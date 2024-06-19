use tokio::runtime::Runtime;
use tracing_subscriber::EnvFilter;

fn main() -> server::Result<()> {
    let args = &mut std::env::args();
    let quiet = args.nth(1).is_some_and(|e| e == "--quiet");

    if !quiet {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("server=info".parse()?))
            .pretty()
            .init();
        tracing::info!("tracing enabled")
    }

    let rt = Runtime::new()?;
    rt.block_on(server::start())?;
    Ok(())
}
