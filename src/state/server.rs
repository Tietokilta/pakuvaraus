use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("BIND_ADDR and PORT do not form a valid socket address: {addr}")]
    InvalidAddr {
        addr: String,
        #[source]
        source: std::net::AddrParseError,
    },
    #[error("failed to bind listener on {addr}")]
    Bind {
        addr: SocketAddr,
        #[source]
        source: std::io::Error,
    },
    #[error("server stopped with an error")]
    Serve(#[source] std::io::Error),
}

pub struct ServerConfig {
    pub addr: SocketAddr,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        let ip = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

        let raw = format!("{ip}:{port}");
        let addr = raw
            .parse()
            .map_err(|source| ServerError::InvalidAddr { addr: raw, source })?;

        Ok(Self { addr })
    }
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn from_config(config: &ServerConfig) -> Result<Self, ServerError> {
        let listener =
            TcpListener::bind(config.addr)
                .await
                .map_err(|source| ServerError::Bind {
                    addr: config.addr,
                    source,
                })?;
        println!("Listening on {}", config.addr);

        Ok(Self { listener })
    }

    pub async fn create() -> Result<Self, ServerError> {
        Self::from_config(&ServerConfig::from_env()?).await
    }

    pub async fn run(self, app: Router) -> Result<(), ServerError> {
        axum::serve(self.listener, app.into_make_service())
            .await
            .map_err(ServerError::Serve)
    }
}
