use std::{env, error::Error, net::SocketAddr};

use rlvr_verifiers::server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rlvr_verifiers=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = parse_addr(env::args().skip(1))?;
    tracing::info!(%addr, "starting verifier server");
    server::serve(addr).await?;
    Ok(())
}

fn parse_addr(args: impl IntoIterator<Item = String>) -> Result<SocketAddr, Box<dyn Error>> {
    let mut args = args.into_iter().peekable();
    let mut addr = env::var("RLVR_VERIFIER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "serve" => {}
            "--addr" | "-a" => {
                addr = args.next().ok_or("--addr requires a socket address")?;
            }
            "--help" | "-h" => {
                println!("Usage: rlvr-verifiers [serve] [--addr 127.0.0.1:8080]");
                std::process::exit(0);
            }
            _ if arg.starts_with("--addr=") => {
                addr = arg["--addr=".len()..].to_string();
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }

    Ok(addr.parse()?)
}
