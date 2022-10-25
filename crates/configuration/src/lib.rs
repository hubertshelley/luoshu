//! configurator for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use luoshu_core::{Connection, Storage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// 配置中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configurator {
    id: String,
    namespace: String,
    luoshu_configuration: HashMap<String, Value>,
}

impl Configurator {
    /// 创建配置中心
    pub fn new(namespace: Option<String>) -> Configurator {
        let id = Uuid::new_v4().to_string();
        let namespace = namespace.unwrap_or_else(|| "default".to_string());
        Configurator {
            id,
            namespace,
            luoshu_configuration: HashMap::new(),
        }
    }
    /// 创建配置
    pub fn set_configuration(&mut self, name: String, config: String) -> Result<()> {
        self.luoshu_configuration
            .insert(name, serde_json::from_str(config.as_str())?);
        Ok(())
    }
}

/// 配置中心存储
pub struct ConfiguratorStore {
    connection: Box<dyn Connection>,
    storage: Box<dyn Storage<Target=Vec<Configurator>>>,
    /// 配置中心列表
    pub configurators: Vec<Configurator>,
}

impl ConfiguratorStore {
    /// 创建配置中心存储
    pub fn new(
        connection: Box<dyn Connection>,
        storage: Box<dyn Storage<Target=Vec<Configurator>>>,
    ) -> Self {
        Self {
            connection,
            storage,
            configurators: vec![],
        }
    }
    /// 添加配置中心
    pub fn append_configurator(&mut self, configurator: Configurator) -> Result<()> {
        self.configurators.push(configurator);
        Ok(())
    }

    /// 存储配置中心
    pub fn save(&mut self) -> Result<()> {
        self.storage.save(self.configurators.clone())
    }

    /// 加载配置中心
    pub fn load(&mut self) -> Result<()> {
        self.configurators = self.storage.load()?;
        Ok(())
    }
}

impl Connection for ConfiguratorStore {
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
