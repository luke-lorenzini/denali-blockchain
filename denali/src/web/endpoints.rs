use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::join;
use serde::Deserialize;
use tokio::sync::oneshot;

use crate::{
    messaging::{Meta, ResponseTx},
    types::Params,
    web::WebState,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    // todo: change this to H256 to allow fail on parse
    block_hash: String,
}

// struct Qs<T>(T);

// #[axum::async_trait]
// impl<T> FromRequest for Qs<T>
// where
//     T: serde::de::DeserializeOwned,
// {
//     type Rejection = Infallible;

//     async fn from_request(req: &mut RequestParts) -> Result<Self, Self::Rejection> {
//         // TODO: error handling
//         let query = req.uri().query().unwrap();
//         Ok(Self(serde_qs::from_str(query).unwrap()))
//     }
// }

pub async fn root() -> impl IntoResponse {
    "Hello, Denali!"
}

pub async fn get_height(State(state): State<WebState>) -> impl IntoResponse {
    let height = state.processor.read().await.chain.get_height().to_string();
    (StatusCode::OK, height)
}

pub async fn get_tip(State(state): State<WebState>) -> impl IntoResponse {
    let tip = state.processor.read().await.chain.get_tip().to_string();
    (StatusCode::OK, tip)
}

pub async fn get_chain(State(state): State<WebState>) -> impl IntoResponse {
    let chain = state.processor.read().await.chain.get_chain();
    // todo output as json
    let result = format!("{chain:?}");
    (StatusCode::OK, result)
}

pub async fn chain_hash(State(state): State<WebState>) -> impl IntoResponse {
    let chain = state.processor.read().await.chain.get_chain_hash();
    let result = format!("{chain:?}");
    (StatusCode::OK, result)
}

pub async fn get_tx(
    State(state): State<WebState>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let tx_id = params.block_hash.try_into();
    if tx_id.is_ok() {
        let chain = state.processor.read().await.chain.get_tx(&tx_id.unwrap());
        let result = format!("{chain:?}");
        return (StatusCode::OK, result);
    }
    (StatusCode::BAD_REQUEST, String::new())
}

pub async fn submit(
    State(state): State<WebState>,
    Json(payload): Json<Params>,
) -> impl IntoResponse {
    let program = payload.program.to_string();

    let xxx = state.contract_map.read().await;
    match xxx.contains_key(&program) {
        true => {
            let payload = serde_json::to_string(&payload.payload).unwrap();

            let (response_tx, response_rx) = oneshot::channel();
            let message_to_process = ResponseTx {
                one_shot: response_tx,
                metadata: Meta {},
                program,
                payload,
            };

            let ack_result = state.tx.send(message_to_process);
            let tx_result = response_rx;
            let (_, tx_result) = join!(ack_result, tx_result);
            let tx_id: String = tx_result.unwrap().tx_id.into();
            let msg = format!("tx_id:0x{tx_id}\n");
            (StatusCode::OK, msg)
        }
        false => (StatusCode::BAD_REQUEST, "plugin not found".into()),
    }
}

pub async fn is_block(
    Query(params): Query<QueryParams>,
    State(state): State<WebState>,
) -> impl IntoResponse {
    println!("{params:?}");
    let block_hash = params.block_hash.try_into();
    if block_hash.is_ok() {
        // todo - find a way to access chain without processor
        match state
            .processor
            .read()
            .await
            .chain
            .is_block(block_hash.unwrap())
        {
            true => return (StatusCode::OK, "exists"),
            false => return (StatusCode::OK, "doesn't exists"),
        }
    }
    (StatusCode::BAD_REQUEST, "invalid address")
}

pub async fn block_header(
    Query(params): Query<QueryParams>,
    State(state): State<WebState>,
) -> impl IntoResponse {
    println!("{params:?}");
    let block_hash = params.block_hash.try_into();
    if block_hash.is_ok() {
        // todo - find a way to access chain without processor
        match state
            .processor
            .read()
            .await
            .chain
            .get_block_header(block_hash.unwrap())
        {
            Some(b) => {
                // let xxx: String = b.into();
                let xxx = format!("{b:?}");
                return (StatusCode::OK, xxx);
            }
            None => return (StatusCode::OK, "doesn't exists".into()),
        }
    }
    (StatusCode::BAD_REQUEST, "invalid address".into())
}

pub async fn block_transactions(
    Query(params): Query<QueryParams>,
    State(state): State<WebState>,
) -> impl IntoResponse {
    println!("{params:?}");
    let block_hash = params.block_hash.try_into();
    if block_hash.is_ok() {
        match state
            .processor
            .read()
            .await
            .chain
            .get_block_transactions(block_hash.unwrap())
        {
            Some(b) => {
                let xxx = format!("{b:?}");
                return (StatusCode::OK, xxx);
            }
            None => return (StatusCode::OK, "doesn't exists".into()),
        }
    }
    (StatusCode::BAD_REQUEST, "invalid address".into())
}
