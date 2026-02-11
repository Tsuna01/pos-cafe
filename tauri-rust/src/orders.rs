use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tauri::State;
use uuid::Uuid;

use crate::database::Database;

// ── Request / Response types ──

#[derive(Debug, Deserialize)]
pub struct OrderItemInput {
    pub item_id: i32,
    pub item_name: String,
    pub qty: i32,
    pub price: f64,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResult {
    pub success: bool,
    pub order_id: Option<String>,
    pub order_number: Option<i32>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderSummary {
    pub id: String,
    pub order_number: i32,
    pub total: f64,
    pub payment_method: String,
    pub cashier_name: String,
    pub created_at: DateTime<Utc>,
    pub item_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderDetail {
    pub id: String,
    pub order_number: i32,
    pub total: f64,
    pub payment_method: String,
    pub cashier_id: String,
    pub cashier_name: String,
    pub created_at: DateTime<Utc>,
    #[sqlx(skip)]
    pub items: Vec<OrderItemDetail>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderItemDetail {
    pub item_id: i32,
    pub item_name: String,
    pub qty: i32,
    pub price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize)]
pub struct DailySummary {
    pub date: String,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub cash_total: f64,
    pub promptpay_total: f64,
    pub card_total: f64,
}

// ── Helpers ──

async fn next_order_number(pool: &sqlx::PgPool) -> i32 {
    let today = Local::now().date_naive();

    let row: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(order_number) FROM orders WHERE created_at::date = $1")
            .bind(today)
            .fetch_one(pool)
            .await
            .unwrap_or((None,));

    row.0.unwrap_or(0) + 1
}

// ── Commands ──

#[tauri::command]
pub async fn create_order(
    items: Vec<OrderItemInput>,
    total: f64,
    payment_method: String,
    cashier_id: String,
    cashier_name: String,
    db: State<'_, Database>,
) -> Result<CreateOrderResult, String> {
    let order_id = Uuid::new_v4().to_string();
    let order_number = next_order_number(&db.pool).await;

    let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;

    if let Err(e) = sqlx::query(
        "INSERT INTO orders (id, order_number, total, payment_method, cashier_id, cashier_name)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&order_id)
    .bind(order_number)
    .bind(total)
    .bind(&payment_method)
    .bind(&cashier_id)
    .bind(&cashier_name)
    .execute(&mut *tx)
    .await
    {
        return Ok(CreateOrderResult {
            success: false,
            order_id: None,
            order_number: None,
            error: Some(format!("บันทึกออเดอร์ไม่สำเร็จ: {}", e)),
        });
    }

    for item in &items {
        if let Err(e) = sqlx::query(
            "INSERT INTO order_items (order_id, item_id, item_name, qty, price)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&order_id)
        .bind(item.item_id)
        .bind(&item.item_name)
        .bind(item.qty)
        .bind(item.price)
        .execute(&mut *tx)
        .await
        {
            return Ok(CreateOrderResult {
                success: false,
                order_id: None,
                order_number: None,
                error: Some(format!("บันทึกรายการไม่สำเร็จ: {}", e)),
            });
        }
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(CreateOrderResult {
        success: true,
        order_id: Some(order_id),
        order_number: Some(order_number),
        error: None,
    })
}

#[tauri::command]
pub async fn get_orders(
    date: Option<String>,
    db: State<'_, Database>,
) -> Result<Vec<OrderSummary>, String> {
    let target = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    sqlx::query_as::<_, OrderSummary>(
        "SELECT o.id, o.order_number, o.total, o.payment_method, o.cashier_name, o.created_at,
                COALESCE(SUM(oi.qty), 0) AS item_count
         FROM orders o
         LEFT JOIN order_items oi ON oi.order_id = o.id
         WHERE o.created_at::date = $1::date
         GROUP BY o.id
         ORDER BY o.created_at DESC",
    )
    .bind(&target)
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_order_detail(
    order_id: String,
    db: State<'_, Database>,
) -> Result<Option<OrderDetail>, String> {
    let order = sqlx::query_as::<_, OrderDetail>(
        "SELECT id, order_number, total, payment_method, cashier_id, cashier_name, created_at
         FROM orders WHERE id = $1",
    )
    .bind(&order_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    match order {
        Some(mut detail) => {
            detail.items = sqlx::query_as::<_, OrderItemDetail>(
                "SELECT item_id, item_name, qty, price, (qty::double precision * price) AS subtotal
                 FROM order_items WHERE order_id = $1",
            )
            .bind(&order_id)
            .fetch_all(&db.pool)
            .await
            .map_err(|e| e.to_string())?;

            Ok(Some(detail))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn get_daily_summary(
    date: Option<String>,
    db: State<'_, Database>,
) -> Result<DailySummary, String> {
    let target = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    let row: (i64, f64, f64, f64, f64) = sqlx::query_as(
        "SELECT
            COUNT(*)::bigint,
            COALESCE(SUM(total), 0),
            COALESCE(SUM(total) FILTER (WHERE payment_method = 'cash'), 0),
            COALESCE(SUM(total) FILTER (WHERE payment_method = 'promptpay'), 0),
            COALESCE(SUM(total) FILTER (WHERE payment_method = 'card'), 0)
         FROM orders
         WHERE created_at::date = $1::date",
    )
    .bind(&target)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(DailySummary {
        date: target,
        total_orders: row.0,
        total_revenue: row.1,
        cash_total: row.2,
        promptpay_total: row.3,
        card_total: row.4,
    })
}
