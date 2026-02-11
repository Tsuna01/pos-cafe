#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod auth;
mod database;
mod menu;
mod orders;
mod printer;

use database::Database;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = Database::new()
        .await
        .expect("Failed to connect database");

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            printer::check_printer_connection,
            printer::test_print,
            auth::login,
            auth::get_users,
            menu::get_categories,
            menu::get_menu_items,
            menu::add_menu_item,
            menu::update_menu_item,
            menu::delete_menu_item,
            orders::create_order,
            orders::get_orders,
            orders::get_order_detail,
            orders::get_daily_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
