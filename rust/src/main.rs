use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the server.
    Serve {
        /// The directory to serve.
        #[arg(short, long, default_value_t = String::from("dist"))]
        dir: String,
        /// The port to serve on.
        #[arg(short, long, default_value_t = String::from("8081"))]
        port: String,
    },
}

fn main() -> server::Result<()> {
    match Args::parse().command {
        Command::Serve { dir, .. } => {
            let rt = Runtime::new()?;
            rt.block_on(server::App::start(dir))?;
        }
    }
    Ok(())
}
