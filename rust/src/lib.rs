use axum::routing::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] tokio::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct App {}

impl App {
    pub async fn start(dir: String) -> Result<()> {
        let router = Router::new()
            .route_service("/", ServeDir::new(dir.clone()))
            .with_state(App {});
        let addr = "0.0.0.0:8080";
        let tcp_listener = TcpListener::bind(addr).await?;
        println!("serving {dir} on {addr}");
        axum::serve(tcp_listener, router).await?;
        Ok(())
    }
}
