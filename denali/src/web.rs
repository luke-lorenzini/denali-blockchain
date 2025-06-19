use crate::Transactor;


pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn chain_height() -> &'static str {
    // let res = transactor.get_chain_height();
    "4"
}