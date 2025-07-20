use std::{
    fs,
    io::{
        self,
        // Write
    },
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use log::{error, info};
use quinn_proto::crypto::rustls::QuicClientConfig;
use rustls::pki_types::{
    CertificateDer,
    // PrivateKeyDer,
    // PrivatePkcs8KeyDer
};
use tokio::sync::RwLock;
use url::Url;

use crate::{processor::Processor, quinn::ALPN_QUIC_HTTP};

pub async fn start_quinn_client(processor: Arc<RwLock<Processor>>) -> Result<()> {
    // Luke - start
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    // Luke - end

    // luke - start
    // let url = options.url;
    let url = "https://localhost:4433/test.txt";
    let url = Url::try_from(url).unwrap();
    // luke - end
    let url_host = strip_ipv6_brackets(url.host_str().unwrap());
    let remote = (url_host, url.port().unwrap_or(4433))
        .to_socket_addrs()?
        .next()
        // .ok_or_else(|| anyhow!("couldn't resolve to an address"))?
        .unwrap();

    let mut roots = rustls::RootCertStore::empty();
    // if let Some(ca_path) = options.ca {
    //     roots.add(CertificateDer::from(fs::read(ca_path)?))?;
    // } else
    {
        let dirs = directories_next::ProjectDirs::from("org", "quinn", "quinn-examples").unwrap();
        match fs::read(dirs.data_local_dir().join("cert.der")) {
            Ok(cert) => {
                roots.add(CertificateDer::from(cert))?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("local server certificate not found");
            }
            Err(e) => {
                error!("failed to open local server certificate: {}", e);
            }
        }
    }
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    client_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
    // if options.keylog {
    //     client_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
    // }

    let client_config =
        quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));
    // luke -start
    // let mut endpoint = quinn::Endpoint::client(options.bind)?;
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4434);
    let mut endpoint = quinn::Endpoint::client(socket)?;
    // luke - end
    endpoint.set_default_client_config(client_config);

    let request = format!("GET {}\r\n", url.path());
    let start = Instant::now();
    // luke - start
    // let rebind = options.rebind;
    let _rebind = false;
    // luke - end
    // luke - start
    // let host = options.host.as_deref().unwrap_or(url_host);
    let host = url_host;
    // luke - end

    eprintln!("connecting to {host} at {remote}");
    let conn = endpoint
        .connect(remote, host)?
        .await
        // .map_err(|e| anyhow!("failed to connect: {}", e))
        ?;
    eprintln!("connected at {:?}", start.elapsed());
    let (mut send, mut recv) = conn
        .open_bi()
        .await
        // .map_err(|e| anyhow!("failed to open stream: {}", e))
        ?;
    // if rebind {
    //     let socket = std::net::UdpSocket::bind("[::]:0").unwrap();
    //     let addr = socket.local_addr().unwrap();
    //     eprintln!("rebinding to {addr}");
    //     endpoint.rebind(socket).expect("rebind failed");
    // }

    send.write_all(request.as_bytes())
        .await
        // .map_err(|e| anyhow!("failed to send request: {}", e))
        ?;
    send.finish().unwrap();
    let response_start = Instant::now();
    eprintln!("request sent at {:?}", response_start - start);
    let resp = recv
        .read_to_end(usize::MAX)
        .await
        // .map_err(|e| anyhow!("failed to read response: {}", e))
        ?;
    processor.write().await.chain.add_received_blocks(resp)?;
    // let duration = response_start.elapsed();
    // eprintln!(
    //     "response received in {:?} - {} KiB/s",
    //     duration,
    //     resp.len() as f32 / (duration_secs(&duration) * 1024.0)
    // );
    // io::stdout().write_all(&resp).unwrap();
    // io::stdout().flush().unwrap();
    // println!("{:?}", resp);
    println!("Complete");
    conn.close(0u32.into(), b"done");

    // Give the server a fair chance to receive the close packet
    endpoint.wait_idle().await;

    Ok(())
}

fn strip_ipv6_brackets(host: &str) -> &str {
    // An ipv6 url looks like eg https://[::1]:4433/Cargo.toml, wherein the host [::1] is the
    // ipv6 address ::1 wrapped in brackets, per RFC 2732. This strips those.
    if host.starts_with('[') && host.ends_with(']') {
        &host[1..host.len() - 1]
    } else {
        host
    }
}

fn _duration_secs(x: &Duration) -> f32 {
    x.as_secs() as f32 + x.subsec_nanos() as f32 * 1e-9
}
