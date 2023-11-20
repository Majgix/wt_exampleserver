use std::{sync::Arc, path::PathBuf};

use anyhow::Context;
use tracing::{level_filters::LevelFilter, info};
use tracing_subscriber::EnvFilter;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "[::]:4443")]
    addr: std::net::SocketAddr,

    #[clap(flatten)]
    pub certs: Certs,
}

#[derive(Debug, Parser)]
pub struct Certs {
    #[clap(
        long,
        short,
        default_value = "./certs/localhost.crt",
        help = "TLS Certificate. If present, `--key` is mandatory."
    )]
    pub cert: PathBuf,

     #[clap(
        long,
        short,
        default_value = "./certs/localhost.key",
        help = "Private key for the certificate."
    )]
    pub key: PathBuf,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    let args = Args::parse();
    
   let Certs { cert, key } = args.certs;
    // Convert a rcgen Certificate to a rustls Certificate
    let cert = rustls::Certificate(std::fs::read(cert)?);
    let key = rustls::PrivateKey(std::fs::read(key)?);
    // let cert = Certificate(fs::read("./certs/localhost.pem")?);
    // let key = PrivateKey(fs::read("./certs/localhost-key.pem")?);
    
    //Quinn setup
    let mut tls_config = rustls::ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key).expect("Invaliddd Keys");

    tls_config.max_early_data_size = u32::MAX;
    tls_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()];

    let config = quinn::ServerConfig::with_crypto(Arc::new(tls_config));

    info!("Listening at addr: https://{:?}", args.addr);

    let server = quinn::Endpoint::server(config, args.addr)?;

    //Accept new connections
    while let Some(conn) = server.accept().await {
        tokio::spawn(async move {
            let _ = handle_webtransport_conn(conn).await;
        });
    }
    Ok(())
}


async fn handle_webtransport_conn(conn: quinn::Connecting) -> anyhow::Result<()> {
    info!("Starting new QUIC connection");

    //wait for QUIC handshake to complete
    let conn = &conn.await.context("failed to accept connection")?;

    //Perform the Webtransport handshake
    let request = webtransport_quinn::accept(conn.clone()).await?;
    info!("received Webtransport request: {}", request.url());

   
    let session = request.ok().await.context("failed to accept session")?;

    let datagram = session.read_datagram().await?;
    let payload = datagram.payload();
    info!("datagram received: {:?}", payload);

    for byte in payload.iter(){
        info!("Byte: {}", byte);
    }

    Ok(())
}


fn init_logging() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_env_filter(env_filter)
        .init();
}
