// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::sync::Arc;

use serde::Deserialize;
use surrealdb::dbs::Session;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{self, Array, Thing, Value};
use surrealdb::Error;
use surrealdb::Surreal;
use tauri::State;

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

struct DbRepoState(SurrealDBRepo);
struct DbState(Surreal<Db>);

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn query(
    // db: State<'_, DbState>,
    db: State<'_, DbRepoState>,
    sql: &str,
    params: HashMap<String, String>,
) -> Result<String, Error> {
    dbg!(sql);
    let mut vars: BTreeMap<String, Value> = BTreeMap::new();
    for param in params {
        vars.insert(param.0, sql::json(param.1.as_str()).unwrap());
    }
    let res = db.0.ds.execute(sql, &db.0.ses, Option::Some(vars)).await?;
    let res_iter = res.into_iter();
    let mut res_array: Array = Array::new();
    for res_item in res_iter {
        let result = sql::to_value(res_item.result).unwrap();
        res_array.push(result);
    }

    Ok(sql::to_value(res_array).unwrap().into_json().to_string())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn query2(
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
    let ds = Arc::new(Datastore::new("speedb://../surreal.db").await.unwrap());
    let ses = Session::default().with_ns("test").with_db("test");
    let repo = SurrealDBRepo { ses, ds };

    let db = Surreal::new::<SpeeDb>("../surreal.db2").await.unwrap();
    db.use_ns("test").use_db("test2").await.unwrap();

    tauri::Builder::default()
        .manage(DbRepoState(repo))
        .manage(DbState(db))
        .invoke_handler(tauri::generate_handler![query, query2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
