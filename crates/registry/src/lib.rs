//! registry for luoshu
#![deny(missing_docs)]

mod service;

use anyhow::Result;
use luoshu_core::{Connection, Storage, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 服务
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Service {
    host: String,
    port: u32,
}

/// 注册中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    id: String,
    namespace: String,
    name: String,
    services: Vec<Service>,
}

impl Registry {
    /// 创建注册中心
    pub fn new(namespace: Option<String>, name: String) -> Registry {
        let id = Uuid::new_v4().to_string();
        let namespace = namespace.unwrap_or_else(|| "default".to_string());
        Registry {
            id,
            namespace,
            name,
            services: vec![],
        }
    }
    /// 注册服务
    pub fn register_service(&mut self, host: String, port: u32) -> Result<()> {
        self.services.push(Service { host, port });
        Ok(())
    }
}

/// 注册中心存储
pub struct RegistryStore<T: Storage> {
    connection: Box<dyn Connection>,
    storage: T,
    /// 注册中心列表
    pub values: Vec<Registry>,
}

impl<T: Storage> Store for RegistryStore<T> {
    type Target = Registry;

    type Storage = T;

    fn get_storage(&self) -> Self::Storage {
        self.storage.clone()
    }

    fn get_storage_key(&self) -> &str {
        "RegistryStorage"
    }

    fn get_values(&self) -> Vec<Self::Target> {
        self.values.clone()
    }

    fn set_values(&mut self, values: Vec<Self::Target>) {
        self.values = values;
    }
}

impl<T: Storage> RegistryStore<T> {
    /// 创建注册中心存储
    pub fn new(connection: Box<dyn Connection>, storage: T) -> Self {
        Self {
            connection,
            storage,
            values: vec![],
        }
    }
    /// 添加注册中心
    pub fn append_registry(&mut self, registry: Registry) -> Result<()> {
        self.values.push(registry);
        Ok(())
    }
}

impl<T: Storage> Connection for RegistryStore<T> {
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
