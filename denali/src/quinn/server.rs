use std::{error::Error, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}, path::{self, Path, 
    PathBuf
}, sync::Arc};

use directories_next::ProjectDirs;
use log::{error, info};
use quinn::{Endpoint, ServerConfig};
// use rustls::KeyLogFile;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, 
    // PrivatePkcs8KeyDer
};
use quinn_proto::crypto::rustls::QuicServerConfig;

use crate::quinn::ALPN_QUIC_HTTP;

// #[derive(Parser, Debug)]
// #[clap(name = "server")]
// struct Opt {
//     /// file to log TLS keys to for debugging
//     // #[clap(long = "keylog")]
//     keylog: bool,
//     /// directory to serve files from
//     root: PathBuf,
//     /// TLS private key in PEM format
//     // #[clap(short = 'k', long = "key", requires = "cert")]
//     key: Option<PathBuf>,
//     /// TLS certificate in PEM format
//     // #[clap(short = 'c', long = "cert", requires = "key")]
//     cert: Option<PathBuf>,
//     /// Enable stateless retries
//     // #[clap(long = "stateless-retry")]
//     stateless_retry: bool,
//     /// Address to listen on
//     // #[clap(long = "listen", default_value = "[::1]:4433")]
//     listen: SocketAddr,
//     /// Client address to block
//     // #[clap(long = "block")]
//     block: Option<SocketAddr>,
//     /// Maximum number of concurrent connections to allow
//     // #[clap(long = "connection-limit")]
//     connection_limit: Option<usize>,
// }

pub async fn start_quinn_server() -> Result<(), Box<dyn Error>>{
    // let options: Opt;

    // Luke - start
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    // Luke - end

    let (certs, key) = 
    // if let (Some(key_path), Some(cert_path)) = (&options.key, &options.cert) {
    //     let key = fs::read(key_path).context("failed to read private key")?;
    //     let key = if key_path.extension().is_some_and(|x| x == "der") {
    //         PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key))
    //     } else {
    //         rustls_pemfile::private_key(&mut &*key)
    //             .context("malformed PKCS #1 private key")?
    //             .ok_or_else(|| anyhow::Error::msg("no private keys found"))?
    //     };
    //     let cert_chain = fs::read(cert_path).context("failed to read certificate chain")?;
    //     let cert_chain = if cert_path.extension().is_some_and(|x| x == "der") {
    //         vec![CertificateDer::from(cert_chain)]
    //     } else {
    //         rustls_pemfile::certs(&mut &*cert_chain)
    //             .collect::<Result<_, _>>()
    //             .context("invalid PEM-encoded certificate")?
    //     };

    //     (cert_chain, key)
    // } 
    // else 
    {
        let dirs = ProjectDirs::from("org", "quinn", "quinn-examples").unwrap();
        let path = dirs.data_local_dir();
        let cert_path = path.join("cert.der");
        let key_path = path.join("key.der");
        let (cert, key) = match fs::read(&cert_path).and_then(|x| Ok((x, fs::read(&key_path)?))) {
            Ok((cert, key)) => (
                CertificateDer::from(cert),
                PrivateKeyDer::try_from(key).unwrap()
                // .map_err(anyhow::Error::msg)?,
            ),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("generating self-signed certificate");
                let _cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
                // let key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());
                // let cert = cert.cert.into();
                fs::create_dir_all(path)
                    // .context("failed to create certificate directory")
                    ?;
                // fs::write(&cert_path, &cert)
                    // .context("failed to write certificate")
                    // ?;
                // fs::write(&key_path, key.secret_pkcs8_der())
            //         .context("failed to write private key")
                    // ?;
                // (cert, key.into())
                todo!()
            }
            Err(_e) => {
            //     bail!("failed to read certificate: {}", e);
                todo!()
            }
        };

        (vec![cert], key)
    };

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
    // if options.keylog {
    //     server_crypto.key_log = Arc::new(KeyLogFile::new());
    // }

    let mut server_config =
        ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    // Luke - Start
    // let root = Arc::<Path>::from(options.root.clone());
    pub const PATH: &str = "./stuffs";
    let root = Arc::<Path>::from(PATH.as_ref());
    // Luke - End
    if !root.exists() {
    //     bail!("root path does not exist");
        todo!()
    }

    // Luke - Start
    // let endpoint = Endpoint::server(server_config, options.listen)?;
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4433);
    let endpoint = Endpoint::server(server_config, socket)?;
    // Luke - End
    println!("listening on {}", endpoint.local_addr()?);

    while let Some(conn) = endpoint.accept().await {
        // if options
        //     .connection_limit
        //     .is_some_and(|n| endpoint.open_connections() >= n)
        // {
        //     info!("refusing due to open connection limit");
        //     conn.refuse();
        // } else if Some(conn.remote_address()) == options.block {
        //     info!("refusing blocked client IP address");
        //     conn.refuse();
        // } else if options.stateless_retry && !conn.remote_address_validated() {
        //     info!("requiring connection to validate its address");
        //     conn.retry().unwrap();
        // } else {
            println!("accepting connection");
            let fut = handle_connection(root.clone(), conn);
            tokio::spawn(async move {
                if let Err(e) = fut.await {
                    println!("connection failed: {reason}", reason = e.to_string())
                }
            });
        // }
    }

    Ok(())
}

async fn handle_connection(root: Arc<Path>, conn: quinn::Incoming) -> Result<(), Box<dyn Error>> {
    let connection = conn.await?;
    // let span = info_span!(
    //     "connection",
    //     remote = %connection.remote_address(),
    //     protocol = %connection
    //         .handshake_data()
    //         .unwrap()
    //         .downcast::<quinn::crypto::rustls::HandshakeData>().unwrap()
    //         .protocol
    //         .map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned())
    // );
    async {
        println!("established");

        // Each stream initiated by the client constitutes a new request.
        loop {
            let stream = connection.accept_bi().await;
            let stream = match stream {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    println!("connection closed");
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => s,
            };
            let fut = handle_request(root.clone(), stream);
            tokio::spawn(
                async move {
                    if let Err(e) = fut.await {
                        println!("failed: {reason}", reason = e.to_string());
                    }
                }
                // .instrument(info_span!("request")),
            );
        }
    }
    // .instrument(span)
    .await?;
    Ok(())
}

async fn handle_request(
    root: Arc<Path>,
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
) -> Result<(), Box<dyn Error>> {
    let req = recv
        .read_to_end(64 * 1024)
        .await
//         .map_err(|e| anyhow!("failed reading request: {}", e))?
        ?;
    // let mut escaped = String::new();
    // for &x in &req[..] {
    //     let part = ascii::escape_default(x).collect::<Vec<_>>();
    //     escaped.push_str(str::from_utf8(&part).unwrap());
    // }
    // info!(content = %escaped);
    // Execute the request
    let resp = process_get(&root, &req).unwrap_or_else(|e| {
        error!("failed: {}", e);
        format!("failed to process request: {e}\n").into_bytes()
    });
    // Write the response
    send.write_all(&resp)
        .await
        // .map_err(|e| anyhow!("failed to send response: {}", e))
        ?;
    // Gracefully terminate the stream
    send.finish().unwrap();
    info!("complete");
    Ok(())
}

fn process_get(root: &Path, x: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if x.len() < 4 || &x[0..4] != b"GET " {
//         bail!("missing GET");
    }
    if x[4..].len() < 2 || &x[x.len() - 2..] != b"\r\n" {
//         bail!("missing \\r\\n");
    }
    let x = &x[4..x.len() - 2];
    let end = x.iter().position(|&c| c == b' ').unwrap_or(x.len());
    let path = str::from_utf8(&x[..end])
        // .unwrap()
        // .context("path is malformed UTF-8")?
        ?
        ;
    let path = Path::new(&path);
    let mut real_path = PathBuf::from(root);
    let mut components = path.components();
    match components.next() {
        Some(path::Component::RootDir) => {}
        _ => {
            // bail!("path must be absolute");
            todo!()
        }
    }
    for c in components {
        match c {
            path::Component::Normal(x) => {
                real_path.push(x);
            }
            _x => {
                // bail!("illegal component in path: {:?}", x);
            }
        }
    }
    let data = fs::read(&real_path)
        // .context("failed reading file")
        ?;
    Ok(data)
}
