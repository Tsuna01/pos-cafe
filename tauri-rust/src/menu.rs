use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tauri::State;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MenuItemData {
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub price: f64,
    pub category_id: String,
    pub image: String,
    pub description: String,
    pub is_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewMenuItem {
    pub name: String,
    pub name_en: String,
    pub price: f64,
    pub category_id: String,
    pub image: Option<String>,
    pub description: Option<String>,
}

#[tauri::command]
pub async fn get_categories(db: State<'_, Database>) -> Result<Vec<Category>, String> {
    sqlx::query_as::<_, Category>("SELECT id, name, icon FROM categories ORDER BY sort_order")
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_menu_items(
    category: Option<String>,
    db: State<'_, Database>,
) -> Result<Vec<MenuItemData>, String> {
    match category {
        Some(cat) => {
            sqlx::query_as::<_, MenuItemData>(
                "SELECT id, name, name_en, price, category_id, image, description, is_available
                 FROM menu_items
                 WHERE category_id = $1 AND is_available = TRUE
                 ORDER BY id",
            )
            .bind(cat)
            .fetch_all(&db.pool)
            .await
        }
        None => {
            sqlx::query_as::<_, MenuItemData>(
                "SELECT id, name, name_en, price, category_id, image, description, is_available
                 FROM menu_items
                 WHERE is_available = TRUE
                 ORDER BY category_id, id",
            )
            .fetch_all(&db.pool)
            .await
        }
    }
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_menu_item(
    item: NewMenuItem,
    db: State<'_, Database>,
) -> Result<MenuItemData, String> {
    sqlx::query_as::<_, MenuItemData>(
        "INSERT INTO menu_items (name, name_en, price, category_id, image, description)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, name, name_en, price, category_id, image, description, is_available",
    )
    .bind(&item.name)
    .bind(&item.name_en)
    .bind(item.price)
    .bind(&item.category_id)
    .bind(item.image.as_deref().unwrap_or_default())
    .bind(item.description.as_deref().unwrap_or_default())
    .fetch_one(&db.pool)
    .await
    .map_err(|e| format!("เพิ่มเมนูไม่สำเร็จ: {}", e))
}

#[tauri::command]
pub async fn update_menu_item(
    id: i32,
    name: String,
    name_en: String,
    price: f64,
    category_id: String,
    image: Option<String>,
    description: Option<String>,
    db: State<'_, Database>,
) -> Result<bool, String> {
    let rows = sqlx::query(
        "UPDATE menu_items
         SET name = $1, name_en = $2, price = $3, category_id = $4, image = $5, description = $6
         WHERE id = $7",
    )
    .bind(&name)
    .bind(&name_en)
    .bind(price)
    .bind(&category_id)
    .bind(image.as_deref().unwrap_or_default())
    .bind(description.as_deref().unwrap_or_default())
    .bind(id)
    .execute(&db.pool)
    .await
    .map_err(|e| format!("อัปเดตเมนูไม่สำเร็จ: {}", e))?
    .rows_affected();

    Ok(rows > 0)
}

#[tauri::command]
pub async fn delete_menu_item(id: i32, db: State<'_, Database>) -> Result<bool, String> {
    let rows = sqlx::query("UPDATE menu_items SET is_available = FALSE WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| format!("ลบเมนูไม่สำเร็จ: {}", e))?
        .rows_affected();

    Ok(rows > 0)
}
