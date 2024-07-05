use bytes::Bytes;
use std::pin::Pin;
use tracing::instrument;
use tracing::info;

use anyhow::Result;

use serde::Deserialize;
use crate::Properties;
use futures::stream::Stream;
use std::task::{Context, Poll};

pub const PROVIDER: &str = "vserial";

#[instrument(skip(server))]
pub async fn start_provider(name: String, server: Server) {
    let console = VSerial::new(name.clone());
    let properties = Properties::new(name.clone());
    let id = server.register_console(properties.clone(), console);
    info!("vserial start_provider {:?} {:?} {:?}", name, properties, id);
}

#[derive(Debug)]
pub(crate) struct VSerial {
}

use crate::Server;

impl VSerial {
    pub fn new(path: String) -> Self {
        info!("VSerial::new {:?}", path);
        VSerial {}
    }

    pub async fn open(&self) -> Result<()> {
        info!("VSerial::open");
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::Console for VSerial {
    fn configure(
        &self,
        parameters: Box<dyn erased_serde::Deserializer>,
    ) -> Result<(), crate::ConsoleError> {
        #[derive(serde::Deserialize)]
        struct Config {
            rate: u32,
        }
        let config = Config::deserialize(parameters).unwrap();
        info!("VSerial::configure {:?}", config.rate);
        Ok(())
    }

    async fn input(
        &self,
    ) -> Result<
        Pin<Box<dyn futures::Sink<Bytes, Error = crate::ConsoleError> + Send>>,
        crate::ConsoleError,
    > {
        info!("VSerial::input");
        Err(crate::ConsoleError::Closed)
    }

    async fn output(
        &self,
    ) -> Result<
        futures::stream::BoxStream<'static, Result<Bytes, crate::ConsoleError>>,
        crate::ConsoleError,
    > {
        info!("VSerial::output");
        Ok(Box::pin(VSerialOutput::new()))
    }
}

pub struct VSerialOutput {
}

impl VSerialOutput {
    fn new() -> Self {
        Self { }
    }
}

impl Stream for VSerialOutput {
    type Item = Result<Bytes, crate::ConsoleError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = Ok(Bytes::from("Hello, world! "));
        Poll::Ready(Some(result))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

