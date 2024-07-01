use tokio::runtime::Runtime;
use tracing_subscriber::EnvFilter;

// Set logging level through RUST_LOG env variable.
//
// (Example): RUST_LOG=server=debug cargo run --release
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives

fn main() -> server::Result<()> {
    let args = &mut std::env::args();
    let quiet = args.nth(1).is_some_and(|e| e == "--quiet");

    if !quiet {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("server=trace".parse()?))
            .pretty()
            .init();
        tracing::info!("tracing enabled")
    }

    assert!(
        std::path::Path::new(server::DIST).exists(),
        "/dist directory must be located at the root of the guppee directory"
    );

    let rt = Runtime::new()?;
    rt.block_on(server::start())?;
    Ok(())
}
