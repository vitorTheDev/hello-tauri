// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::{self, Array};
use surrealdb::Error;
use surrealdb::Surreal;
use tauri::State;

struct DbState(Surreal<Db>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn query(
    // db: State<'_, DbState>,
    db: State<'_, DbState>,
    sql: &str,
    params: HashMap<String, serde_json::Value>,
) -> Result<String, Error> {
    dbg!(sql);
    let mut qr = db.0.query(sql);
    for param in params {
        qr = qr.bind(param);
    }
    let mut res = qr.await?;
    let mut index = 0;
    let len = res.num_statements();
    let mut result = Array::new();
    while index < len {
        result.push(res.take(index)?);
        index = index + 1;
    }
    Ok(sql::to_value(result).unwrap().into_json().to_string())
}

#[tokio::main]
async fn main() {
    let db = Surreal::new::<SpeeDb>("../surreal.db2").await.unwrap();
    db.use_ns("test").use_db("test2").await.unwrap();

    tauri::Builder::default()
        .manage(DbState(db))
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
