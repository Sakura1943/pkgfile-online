use backend::{cli::Cli, server::run, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Logger initialize
    std::env::set_var("RUST_LOG", "poem=debug");
    tracing_subscriber::fmt::init();

    let Cli {host, port} = Cli::default();

    // Running server
    run(port, &host).await?;
    Ok(())
}
