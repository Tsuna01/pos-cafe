use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await?;

        let db = Database { pool };
        db.init_tables().await?;

        Ok(db)
    }

    async fn init_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                icon TEXT NOT NULL,
                sort_order INTEGER DEFAULT 0
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS menu_items (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                name_en TEXT NOT NULL,
                price DOUBLE PRECISION NOT NULL,
                category_id TEXT NOT NULL REFERENCES categories(id),
                image TEXT DEFAULT '',
                description TEXT DEFAULT '',
                is_available BOOLEAN DEFAULT TRUE
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
                role TEXT NOT NULL CHECK(role IN ('admin', 'cashier'))
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS orders (
                id TEXT PRIMARY KEY,
                order_number INTEGER NOT NULL,
                total DOUBLE PRECISION NOT NULL,
                payment_method TEXT NOT NULL,
                cashier_id TEXT NOT NULL REFERENCES users(id),
                cashier_name TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS order_items (
                id SERIAL PRIMARY KEY,
                order_id TEXT NOT NULL REFERENCES orders(id),
                item_id INTEGER NOT NULL,
                item_name TEXT NOT NULL,
                qty INTEGER NOT NULL,
                price DOUBLE PRECISION NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
