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
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
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

    if args.web {
        run_server("0.0.0.0:19999", data).await;
    }
    let listener = TcpListener::bind("0.0.0.0:19998").await?;
    loop {
        let (stream, client) = listener.accept().await?;
        tokio::task::spawn(async move {
            let _connection = Connection::new(stream, client);
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

    pub async fn read_frame(&mut self) -> LuoshuResult<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    // return Err("connection reset by peer".into());
                    return Err(error::AppError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "connection reset by peer",
                    )));
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;

                // Discard the parsed data from the read buffer.
                //
                // When `advance` is called on the read buffer, all of the data
                // up to `len` is discarded. The details of how this works is
                // left to `BytesMut`. This is often done by moving an internal
                // cursor, but it may be done by reallocating and copying data.
                self.buffer.advance(len);

                Ok(Some(frame))
            }
            _ => Ok(None),
            // Err(Incomplete) => Ok(None),
            // Err(e) => Err(e.into()),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        self.stream.write_all(&serde_json::to_vec(&frame)?).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
