pub mod client;
pub mod server;

const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
