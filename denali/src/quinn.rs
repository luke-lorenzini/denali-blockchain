pub mod client;
pub mod server;

const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

// todo integrate this
enum _MessageTypes {
    Syncro,
    Update,
    Notify,
}
