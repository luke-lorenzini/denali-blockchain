use std::{
    fs,
    io::{
        self,
        // Write
    },
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    sync::Arc,
};

use anyhow::Result;
use log::{error, info};
use quinn::Connection;
use quinn_proto::crypto::rustls::QuicClientConfig;
use rustls::pki_types::{
    CertificateDer,
    // PrivateKeyDer,
    // PrivatePkcs8KeyDer
};
use tokio::sync::{Notify, RwLock};
use url::Url;

use crate::{processor::Processor, quinn::ALPN_QUIC_HTTP};

// todo: clean me!
pub async fn quinn_one_shot_sync(port_number: u16) -> Result<(Vec<u8>, Vec<u8>)> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let url = "https://localhost:4433";
    let url = Url::try_from(url)?;
    let url_host = strip_ipv6_brackets(url.host_str().unwrap());
    let remote = (url_host, url.port().unwrap_or(4433))
        .to_socket_addrs()?
        .next()
        .unwrap();

    let mut roots = rustls::RootCertStore::empty();
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
                error!("failed to open local server certificate: {e}");
            }
        }
    }
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    client_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    let client_config =
        quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_number);
    let mut endpoint = quinn::Endpoint::client(socket)?;
    endpoint.set_default_client_config(client_config);

    let host = url_host;
    let connection = endpoint.connect(remote, host)?.await?;

    let (mut notify_send, _) = connection.open_bi().await?;
    notify_send.write_all(b"NOTIFY").await?;
    notify_send.finish()?;

    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"VSYNCX").await?;
    send.finish()?;
    let encoded_state = recv.read_to_end(usize::MAX).await?;

    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"SYNCRO").await?;
    send.finish()?;
    let encoded_blocks = recv.read_to_end(usize::MAX).await?;

    Ok((encoded_state, encoded_blocks))
}

pub async fn start_quinn_client(processor: Arc<RwLock<Processor>>, port_number: u16) -> Result<()> {
    // Luke - start
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    // Luke - end

    // luke - start
    // let url = options.url;
    let url = "https://localhost:4433";
    let url = Url::try_from(url)?;
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
                error!("failed to open local server certificate: {e}");
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
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_number);
    let mut endpoint = quinn::Endpoint::client(socket)?;
    // luke - end
    endpoint.set_default_client_config(client_config);

    // new stuff
    let host = url_host;
    let conn = endpoint.connect(remote, host)?.await?;
    connect_and_listen(conn, processor).await?;

    // Start July 21
    // // let request = format!("NOTIFY {}\r\n", url.path());
    // let start = Instant::now();
    // // luke - start
    // // let rebind = options.rebind;
    // // let _rebind = false;
    // // luke - end
    // // luke - start
    // // let host = options.host.as_deref().unwrap_or(url_host);
    // let host = url_host;
    // // luke - end

    // eprintln!("connecting to {host} at {remote}");
    // let conn = endpoint
    //     .connect(remote, host)?
    //     .await
    //     // .map_err(|e| anyhow!("failed to connect: {}", e))
    //     ?;
    // eprintln!("connected at {:?}", start.elapsed());
    // let (mut notify_send, mut notify_recv) = conn.open_bi().await?;
    // // if rebind {
    // //     let socket = std::net::UdpSocket::bind("[::]:0").unwrap();
    // //     let addr = socket.local_addr().unwrap();
    // //     eprintln!("rebinding to {addr}");
    // //     endpoint.rebind(socket).expect("rebind failed");
    // // }

    // notify_send.write_all(b"NOTIFY").await?;

    // let (mut send, mut recv) = conn.open_bi().await?;

    // tokio::spawn(async move {
    //     let mut buf = [0u8; 1024];
    //     while let Ok(Some(n)) = notify_recv.read(&mut buf).await {
    //         println!("got notification: {:?}", &buf[..n]);
    //         // act based on content, maybe open a REQUES stream here

    //         // let (mut send, mut recv) = conn.open_bi().await.unwrap();
    //         // send.write_all(b"RQSTXX").await.unwrap();

    //         // let resp = recv
    //         //     .read_to_end(usize::MAX)
    //         //     .await
    //         //     // .map_err(|e| anyhow!("failed to read response: {}", e))
    //         //     .unwrap();
    //         // processor.write().await.chain.add_received_blocks(resp).unwrap();
    //     }
    // });

    // // loop {
    // //     // Example: request new block
    // // let (mut send, mut recv) = conn.open_bi().await?;
    // send.write_all(b"SYNCXX").await?;
    // //     send.write_all(b"GET NEW BLOCK\n").await?;
    // //     // read response if needed
    // // }

    // send.finish().unwrap();
    // // let response_start = Instant::now();
    // // eprintln!("request sent at {:?}", response_start - start);
    // let resp = recv
    //     .read_to_end(usize::MAX)
    //     .await
    //     // .map_err(|e| anyhow!("failed to read response: {}", e))
    //     ?;
    // // println!("resp: {resp:?}");
    // // if resp[0] == '0' && resp[1] == '0' && resp[2] == '0' {
    // //     println!("client has been notified");
    // // } else {
    // processor.write().await.chain.add_received_blocks(resp)?;
    // // }
    // // let duration = response_start.elapsed();
    // // eprintln!(
    // //     "response received in {:?} - {} KiB/s",
    // //     duration,
    // //     resp.len() as f32 / (duration_secs(&duration) * 1024.0)
    // // );
    // // io::stdout().write_all(&resp).unwrap();
    // // io::stdout().flush().unwrap();
    // // println!("{:?}", resp);
    // // println!("Complete");
    // // conn.close(0u32.into(), b"done");

    // // // Give the server a fair chance to receive the close packet
    // // endpoint.wait_idle().await;
    // End July 21

    Ok(())
}

async fn _connect_to_server() -> Result<Connection> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4434);
    let url = "https://localhost:4433/test.txt";
    let url = Url::try_from(url)?;
    let url_host = strip_ipv6_brackets(url.host_str().unwrap());
    let host = url_host;
    let endpoint = quinn::Endpoint::client(socket)?;
    let remote = (url_host, url.port().unwrap_or(4433))
        .to_socket_addrs()?
        .next()
        .unwrap();
    let conn = endpoint.connect(remote, host)?.await?;
    Ok(conn)
}

async fn connect_and_listen(
    connection: Connection,
    processor: Arc<RwLock<Processor>>,
) -> Result<()> {
    // let connection = connect_to_server().await?;
    let notify = Arc::new(Notify::new());
    // === Register for notifications
    let (mut notify_send, mut notify_recv) = connection.open_bi().await?;
    // Send the string required by server
    notify_send.write_all(b"NOTIFY").await?;
    notify_send.finish()?;
    println!("Notified server");

    let notify_clone = notify.clone();
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        while let Ok(Some(_n)) = notify_recv.read(&mut buf).await {
            // println!("got notification: {:?}", &buf[..n]);
            notify_clone.notify_one();
            // act based on content, maybe open a REQUES stream here
        }
    });

    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"SYNCRO").await?;
    send.finish()?;
    let resp = recv.read_to_end(usize::MAX).await?;
    processor.write().await.chain.add_received_blocks(resp)?;
    // === Later, issue requests
    loop {
        notify.notified().await;
        // println!("notified!");
        // Example: request new block
        let (mut send, mut recv) = connection.open_bi().await?;
        // send.write_all(b"REQUES").await?;
        // send.write_all(b"GET NEW BLOCK\n").await?;
        // read response if needed
        let tip = processor.read().await.chain.get_tip();
        // let payload = format!("UPDATE{}", tip);
        // println!("{payload:?}");
        send.write_all(b"UPDATE").await?;
        send.write_all(tip.unwrap().as_ref()).await?;
        send.finish()?;
        let resp = recv.read_to_end(usize::MAX).await?;
        processor.write().await.chain.add_received_blocks(resp)?;
    }
    // Ok(())
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
