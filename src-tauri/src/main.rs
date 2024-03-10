// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::sync::Arc;

use serde::Deserialize;
use surrealdb::dbs::{Session, Response};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{self, Thing, Value};
use surrealdb::Error;
use tauri::State;

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

struct DbRepoState(SurrealDBRepo);

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
    let mut result_string: String = "[".into();
    let mut index = 0;
    let res_iter = res.into_iter();
    let res_vec: Vec<Response> = res_iter.collect();
    let len = res_vec.len();
    for res_item in res_vec {
        let result = sql::to_value(res_item.result).unwrap().into_json();// res_item.result.unwrap();
        let mut result_json: String = result["Ok"].to_string();
        if index != (len - 1) {
          result_json.push_str(",");
        }
        result_string.push_str(result_json.as_str());
        index = index + 1;
    }

    Ok((result_string + "]").into())

}

#[tokio::main]
async fn main() {

    let ds = Arc::new(Datastore::new("speedb://../surreal.db").await.unwrap());
    let ses = Session::default().with_ns("test").with_db("test");
    let repo = SurrealDBRepo { ses, ds };

    tauri::Builder::default()
        .manage(DbRepoState(repo))
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}