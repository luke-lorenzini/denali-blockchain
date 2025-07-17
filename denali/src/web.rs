use std::{collections::HashMap, sync::Arc};

use tokio::sync::{RwLock, mpsc::Sender};

use crate::{messaging::ResponseTx, transactor::Transactor, Plugin};

pub mod endpoints;
pub mod web_task;

#[derive(Clone, Debug)]
pub struct WebState {
    pub transactor: Arc<RwLock<Transactor>>,
    pub tx: Sender<ResponseTx>,
    pub contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
}
