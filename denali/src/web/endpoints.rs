use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{types::Params, web::WebState};

pub async fn root() -> impl IntoResponse {
    "Hello, Denali!"
}

pub async fn chain_height(State(state): State<WebState>) -> impl IntoResponse {
    let height = state
        .transactor
        .read()
        .await
        .chain
        .get_chain_height()
        .to_string();
    (StatusCode::OK, height)
}

pub async fn submit(
    State(state): State<WebState>,
    Json(payload): Json<Params>,
) -> impl IntoResponse {
    // println!("{state:?}");
    // println!("{payload:?}");
    let program = payload.program.to_string();

    let xxx = state.contract_map.read().await;
    match xxx.contains_key(&program) {
        true => {
            let payload = serde_json::to_string(&payload.payload).unwrap();
            // let payload = r#"
            //             {
            //                 "candidate": "candidate1"
            //             }"#
            // .into();
            let _res = state.tx.send((program, payload)).await;
            (StatusCode::OK, "plugin found")
        }
        false => (StatusCode::BAD_REQUEST, "plugin not found"),
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {}

pub async fn get_something(Query(params): Query<QueryParams>) -> impl IntoResponse {
    println!("{params:?}");
    "todo!()"
}
