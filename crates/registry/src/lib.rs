//! registry for luoshu
#![deny(missing_docs)]
use core::{Connection, Storage};

/// 注册中心结构
pub struct Registry {
    connection: Box<dyn Connection>,
    storage: Box<dyn Storage>,
}
impl Registry {
    /// 创建注册中心
    pub fn new(connection: Box<dyn Connection>, storage: Box<dyn Storage>) -> Registry {
        Registry {
            connection,
            storage,
        }
    }
}
impl Connection for Registry {
    fn send(&self) {
        self.connection.send()
    }

    fn recv(&self) {
        self.connection.recv()
    }

    fn connected(&self) {
        self.connection.connected()
    }

    fn disconnected(&self) {
        self.connection.disconnected()
    }

    fn get_ipaddr(&self) -> std::net::SocketAddr {
        self.connection.get_ipaddr()
    }
}

impl Storage for Registry {
    fn save(&self) {
        self.storage.save()
    }

    fn load(&self) {
        self.storage.load()
    }
}
