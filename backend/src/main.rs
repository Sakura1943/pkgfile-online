use backend::{cli::Cli, server::run, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Logger initialize
    std::env::set_var("RUST_LOG", "poem=debug");
    tracing_subscriber::fmt::init();

    let cli = Cli::default();
    let host = cli.host;
    let port = cli.port;

    // Running server
    run(port, &host).await?;
    Ok(())
}
