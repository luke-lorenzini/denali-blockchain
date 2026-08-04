use std::{collections::HashMap, sync::Arc};

use tokio::sync::{RwLock, mpsc::Sender};

use crate::{Plugin, messaging::ResponseTx, processor::Processor};

pub mod endpoints;
pub mod web_task;

#[derive(Clone, Debug)]
pub struct WebState {
    pub processor: Arc<RwLock<Processor>>,
    pub tx: Sender<ResponseTx>,
    pub contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
}
