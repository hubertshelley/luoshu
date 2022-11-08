mod frame;
mod mem_data;
mod regs;
mod sled_data;

use anyhow::Result;
use bytes::{Buf, BytesMut};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

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

    pub async fn process(&mut self, data: LuoshuSledData) -> LuoshuResult<()> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Frame>();
        // tokio::task::spawn(async move {
        //     if let Some(frame) = rx.recv().await {
        //         self.write_frame(&frame);
        //     }
        // });
        loop {
            if let Some(frame) = self.read_frame().await? {
                match frame.data {
                    ActionEnum::Up(frame) => data.append(&frame).await?,
                    ActionEnum::Down(frame) => data.remove(&frame).await?,
                    ActionEnum::Sync(frame) => data.sync(&frame).await?,
                    ActionEnum::Subscribe(config_name) => {
                        data.subscribe(config_name, tx.clone()).await?
                    }
                };
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
        let data = serde_json::to_vec(&frame)?;
        let data_len = data.len();
        self.stream.write_u32(data_len as u32).await?;
        self.stream.write_all(&serde_json::to_vec(&frame)?).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
