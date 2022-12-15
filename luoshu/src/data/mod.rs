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
pub use luoshu_registry::Service;
pub use mem_data::*;
pub use regs::*;
pub use sled_data::*;

/// 连接
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
    /// 客户端地址
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

    /// 服务端执行器
    pub async fn process(&mut self, data: Arc<RwLock<LuoshuSledData>>) -> LuoshuResult<()> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Frame>();
        loop {
            tokio::select! {
                frame = self.read_frame() => {
                    if let Ok(Some(frame)) = frame{
                        tracing::info!("Recv: {}: {:?}",self.client,frame);
                        match frame.data {
                            ActionEnum::Up(frame) => data.write().await.append(&frame, Some(self.client), Some(&tx)).await?,
                            ActionEnum::Down(frame) => data.write().await.remove(&frame).await?,
                            ActionEnum::Sync(frame) => data.write().await.sync(&frame).await?,
                            ActionEnum::Subscribe(subscribe) => data.write().await.subscribe(subscribe, &tx).await?,
                            ActionEnum::Ping => {
                                    match tx.send(ActionEnum::Pong.into()){
                                        Ok(_) => {},
                                        Err(_) => data.write().await.broken(self.client).await?,
                                    }
                                }
                            ActionEnum::Pong => {}
                        }
                    }else {
                        data.write().await.broken(self.client).await?;
                        return Ok(());
                    }
                }
                Some(frame) = rx.recv() => {
                    tracing::info!("Send: {}: {:?}",self.client,frame);
                    self.write_frame(&frame).await?;
                }
            }
        }
    }

    /// 读取一条消息帧
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
            Ok(frame) => {
                self.buffer.clear();
                Ok(Some(frame))
            }
            Err(_) => {
                self.buffer.clear();
                Ok(None)
            }
        }
    }

    /// 写入一条消息帧
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        let data = serde_json::to_vec(&frame)?;
        let data_len = data.len();
        self.stream.write_u32(data_len as u32).await?;
        self.stream.write_all(&serde_json::to_vec(&frame)?).await?;
        self.stream.flush().await?;
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(())
    }
}
