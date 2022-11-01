//! luoshu_connection impl for luoshu
#![deny(missing_docs)]
use std::net::SocketAddr;

use luoshu_core::Connection;

#[derive(Clone)]
/// 连接实现
pub struct Connector {}

impl Connection for Connector {
    fn send(&self) {
        todo!()
    }

    fn recv(&self) {
        todo!()
    }

    fn connected(&self) {
        todo!()
    }

    fn disconnected(&self) {
        todo!()
    }

    fn get_ipaddr(&self) -> SocketAddr {
        todo!()
    }
}
