pub mod http;
mod stdio_transport;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use std::{error::Error, fmt, pin::Pin};

pub use http::*;
pub use stdio_transport::*;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}

#[derive(Debug)]
pub struct UnauthorizedError {
    pub www_authenticate_header: Option<String>,
}

impl Error for UnauthorizedError {}

impl fmt::Display for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unauthorized")
    }
}
