//! registry for luoshu
#![deny(missing_docs)]

mod service;

use anyhow::Result;
use luoshu_core::{Connection, Storage};
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
pub struct RegistryStore {
    connection: Box<dyn Connection>,
    storage: Box<dyn Storage<Target = Registry>>,
    /// 注册中心列表
    pub registries: Vec<Registry>,
}

impl RegistryStore {
    /// 创建注册中心存储
    pub fn new(
        connection: Box<dyn Connection>,
        storage: Box<dyn Storage<Target = Registry>>,
    ) -> Self {
        Self {
            connection,
            storage,
            registries: vec![],
        }
    }
    /// 添加注册中心
    pub fn append_registry(&mut self, registry: Registry) -> Result<()> {
        self.registries.push(registry);
        Ok(())
    }

    /// 存储注册中心
    pub fn save(&mut self) -> Result<()> {
        self.storage.save(self.registries.clone())
    }

    /// 加载注册中心
    pub fn load(&mut self) -> Result<()> {
        self.registries = self.storage.load()?;
        Ok(())
    }
}

impl Connection for RegistryStore {
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
