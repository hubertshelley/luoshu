mod frame;
mod mem_data;
mod regs;
mod sled_data;

use anyhow::Result;
use bytes::{Buf, BytesMut};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

use crate::error::{LuoshuError, LuoshuResult};
pub use frame::*;
pub use mem_data::*;
pub use regs::*;
pub use sled_data::*;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
    pub client: SocketAddr,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.client == other.client
    }
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream, client: SocketAddr) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4096), // 4 * 1024
            client,
        }
    }

    pub async fn process(&mut self, data: Arc<RwLock<LuoshuSledData>>) -> LuoshuResult<()> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Frame>();
        loop {
            tokio::select! {
                Ok(Some(frame)) = self.read_frame() => {
                    tracing::info!("Recv: {}: {}",self.client,frame);
                    match frame.data {
                    ActionEnum::Up(frame) => data.write().await.append(&frame).await?,
                    ActionEnum::Down(frame) => data.write().await.remove(&frame).await?,
                    ActionEnum::Sync(frame) => data.write().await.sync(&frame).await?,
                    ActionEnum::Subscribe(config_name) => {
                        data.write().await.subscribe(config_name, tx.clone()).await?
                    }
                }
                }
                Some(frame) = rx.recv() => {
                    tracing::info!("Send: {}: {}",self.client,frame);
                    self.write_frame(&frame).await?;
                }
            }
        }
    }

    pub async fn read_frame(&mut self) -> LuoshuResult<Option<Frame>> {
        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            return if self.buffer.is_empty() {
                Ok(None)
            } else {
                Err(LuoshuError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "connection reset by peer",
                )))
            };
        }
        let data_len = self.buffer.get_u32() as usize;
        let data = &self.buffer[..data_len];
        match Frame::parse(data) {
            Ok(frame) => Ok(Some(frame)),
            Err(_) => Ok(None),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        println!("write_frame {}", frame);
        let data = serde_json::to_vec(&frame)?;
        let data_len = data.len();
        self.stream.write_u32(data_len as u32).await?;
        self.stream.write_all(&serde_json::to_vec(&frame)?).await?;
        self.stream.flush().await?;
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(())
    }
}
