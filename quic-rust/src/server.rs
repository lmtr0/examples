use anyhow::{anyhow, bail, Context, Result};
use std::sync::Arc;
use std::{ascii, fs, io};
use tracing::{error, info, info_span, warn, Instrument};

use std::{
    path::{self, PathBuf},
    str,
};
use rustls::ConfigBuilder;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt};
use tracing_subscriber::filter::{EnvFilter, Directive};


#[tokio::main]
async fn main() -> Result<()> {

    // ! Logging
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .parse("")?
        .add_directive("quinn_proto=error".parse()?);

    tracing_subscriber::registry().with(fmt::layer()).with(env_filter).init();

    // tracing_subscriber::registry()
    //     .with(filtered_layer)
    //     .with(unfiltered_layer)
    //     .init();
    // tracing::subscriber::set_global_default(
    //     tracing_subscriber::FmtSubscriber::builder()
    //         .with_default_directive(LevelFilter::DEBUG.into())
    //         // .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    //         .finish(),
    // )
    //     .unwrap();

    warn!("Hello world");
    error!("Hello world");

    // ! Certificates

    let dirs = directories_next::ProjectDirs::from("org", "lmtr0", "quinn-rust").unwrap();
    let path = dirs.data_local_dir();
    info!(?path, "data_local_dir");
    let cert_path = path.join("cert.der");
    let key_path = path.join("key.der");
    let (cert, key) = match fs::read(&cert_path).and_then(|x| Ok((x, fs::read(&key_path)?))) {
        Ok(x) => x,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            info!(?path, "generating self-signed certificate");
            let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
            let key = cert.serialize_private_key_der();
            let cert = cert.serialize_der().unwrap();
            fs::create_dir_all(path).context("failed to create certificate directory")?;
            fs::write(&cert_path, &cert).context("failed to write certificate")?;
            fs::write(&key_path, &key).context("failed to write private key")?;
            (cert, key)
        }
        Err(e) => {
            bail!("failed to read certificate: {}", e);
        }
    };

    let key = rustls_pki_types::PrivateKeyDer::Pkcs1(key.into());
    let certs = vec![rustls_pki_types::CertificateDer::from(cert)];
    // let key = rustls::PrivateKey(key.into());
    // let certs = vec![rustls::Certificate(cert)];


    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        // .with_client_cert_verifier()
        // .with_safe_default_cipher_suites()
        // .with_safe_default_kx_groups()
        // .with_safe_default_protocol_versions()
        // .unwrap()
        // .with_safe_defaults()
        // .with_no_client_auth()
        .with_single_cert(certs, key)?;
    server_crypto.alpn_protocols = quic_rust::ALPN_QUIC_HTTP
        .iter()
        .map(|&x| x.into())
        .collect();
    // if options.keylog {
    //     server_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
    // }

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(server_crypto));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    // if options.stateless_retry {
    //     server_config.use_retry(true);
    // }

    // let root = Arc::<Path>::from(options.root.clone());
    // if !root.exists() {
    //     bail!("root path does not exist");
    // }

    let endpoint = quinn::Endpoint::server(server_config, "[::1]:4433".parse().unwrap())?;
    warn!("listening on {}", endpoint.local_addr()?);

    while let Some(conn) = endpoint.accept().await {
        info!("connection incoming");
        let fut = handle_connection(conn);
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                error!("connection failed: {reason}", reason = e.to_string())
            }
        });
    }

    Ok(())
}

// pub async fn handle_connection(root: Arc<Path>, conn: quinn::Connecting) -> Result<()> {
pub async fn handle_connection(conn: quinn::Connecting) -> Result<()> {
    let connection = conn.await?;
    let span = info_span!(
        "connection",
        remote = %connection.remote_address(),
        protocol = %connection
            .handshake_data()
            .unwrap()
            .downcast::<quinn::crypto::rustls::HandshakeData>().unwrap()
            .protocol
            .map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned())
    );
    async {
        info!("established");

        // Each stream initiated by the client constitutes a new request.
        loop {
            let stream = connection.accept_bi().await;
            let stream = match stream {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    info!("connection closed");
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => s,
            };
            let fut = handle_request(stream);
            tokio::spawn(
                async move {
                    if let Err(e) = fut.await {
                        error!("failed: {reason}", reason = e.to_string());
                    }
                }
                .instrument(info_span!("request")),
            );
        }
    }
    .instrument(span)
    .await?;
    Ok(())
}

async fn handle_request(
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
) -> Result<()> {
    let req = recv
        .read_to_end(64 * 1024)
        .await
        .map_err(|e| anyhow!("failed reading request: {}", e))?;
    let mut escaped = String::new();
    for &x in &req[..] {
        let part = ascii::escape_default(x).collect::<Vec<_>>();
        escaped.push_str(std::str::from_utf8(&part).unwrap());
    }
    info!(content = %escaped);
    // Execute the request
    // let resp = process_get(&root, &req).unwrap_or_else(|e| {
    let resp = process_get().await.unwrap_or_else(|e| {
        error!("failed: {}", e);
        format!("failed to process request: {e}\n").into_bytes()
    });
    // Write the response
    send.write_all(&resp)
        .await
        .map_err(|e| anyhow!("failed to send response: {}", e))?;

    send.write_all(&resp)
        .await
        .map_err(|e| anyhow!("failed to send response: {}", e))?;
    // Gracefully terminate the stream
    send.finish()
        .await
        .map_err(|e| anyhow!("failed to shutdown stream: {}", e))?;
    info!("complete");
    Ok(())
}

pub async fn process_get() -> Result<Vec<u8>> {
    let data = "Hello from server".as_bytes().to_vec();

    Ok(data)
}
