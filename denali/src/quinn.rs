pub mod client;
pub mod server;

const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

#[derive(Clone, PartialEq)]
pub enum Roles {
    Receiver,
    Validator,
    Archiver,
}

// todo integrate this
enum _MessageTypes {
    Syncro,
    Update,
    Notify,
}
