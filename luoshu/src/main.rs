use bytes::{Buf, BytesMut};
use clap::Parser;
use error::LuoshuResult;
use luoshu_core::Store;
use luoshu_namespace::Namespace;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod data;
mod error;
mod web;

use crate::data::{Frame, LuoshuData};
use crate::web::run_server;
use anyhow::Result;
use tokio::io::BufWriter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run with web
    #[arg(long, default_value_t = false)]
    web: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();

    let data = LuoshuData::new();

    data.configuration_store.write().await.load()?;
    data.namespace_store.write().await.load()?;
    if data.namespace_store.read().await.values.is_empty() {
        data.namespace_store
            .write()
            .await
            .append_namespace(Namespace::new("default".into()))?;
        data.namespace_store.write().await.save()?;
    }
    let _data = data.clone();
    tokio::task::spawn(async move {
        if args.web {
            run_server("0.0.0.0:19999", _data).await;
        };
    });
    let listener = TcpListener::bind("0.0.0.0:19998").await?;
    tracing::info!("Luoshu listening on: http://0.0.0.0:19998");
    loop {
        let (stream, client) = listener.accept().await?;
        let _data = data.clone();
        tokio::task::spawn(async move {
            let mut connection = Connection::new(stream, client);
            match connection.process(_data).await {
                Ok(_) => {}
                Err(_) => {
                    tracing::info!("{} left", client)
                }
            };
        });
    }
}

#[allow(dead_code)]
struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
    client: SocketAddr,
}

#[allow(dead_code)]
impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream, client: SocketAddr) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            client,
        }
    }

    pub async fn process(&mut self, data: LuoshuData) -> LuoshuResult<()> {
        loop {
            if let Some(frame) = self.read_frame().await? {
                tracing::info!("{}", frame);
                match frame.action {
                    data::ActionEnum::Up => data.append(&frame.data).await?,
                    data::ActionEnum::Down => todo!(),
                    data::ActionEnum::Sync => todo!(),
                };
            }
        }
    }

    pub async fn read_frame(&mut self) -> LuoshuResult<Option<Frame>> {
        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            if self.buffer.is_empty() {
                return Ok(None);
            } else {
                return Err(error::AppError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "connection reset by peer",
                )));
            }
        }
        let data_len = self.buffer.get_u32() as usize;
        tracing::info!("{}", data_len);
        let data = &self.buffer[..data_len];
        match Frame::parse(data) {
            Ok(frame) => Ok(Some(frame)),
            Err(_) => Ok(None),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        let data = serde_json::to_vec(&frame)?;
        let data_len = data.len();
        self.stream.write_u32(data_len as u32).await?;
        self.stream.write_all(&serde_json::to_vec(&frame)?).await?;
        self.stream.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use tokio::net::TcpStream;

    // use crate::{data::Frame, Connection};

    #[tokio::test]
    async fn frame_write_test() -> Result<(), Box<dyn std::error::Error>> {
        // let addr = "127.0.0.1:19998".to_string();
        // let stream = TcpStream::connect(addr.clone()).await.unwrap();
        // let mut connection = Connection::new(stream, addr.parse()?);
        // let frame = Frame {
        //     action: crate::data::ActionEnum::Up,
        //     data: crate::data::NamespaceReg {
        //         name: "test".into(),
        //     }
        //     .into(),
        // };
        // connection.write_frame(&frame).await?;
        // loop {}
        Ok(())
    }
}
