// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted very calmly from Rust!", name)
}

mod book;
mod client;
mod db;

#[tauri::command]
fn get_books() -> String {
    let books = db::get_books();
    serde_json::to_string(&books).unwrap()
}

#[tauri::command]
fn add_book() -> String {
    let books = db::get_books();
    serde_json::to_string(&books).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_books])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
