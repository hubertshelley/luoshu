//! configurator for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use luoshu_core::{Connection, Storage, Store};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// 配置中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configurator {
    id: String,
    namespace: String,
    configuration: HashMap<String, Value>,
}

impl Configurator {
    /// 创建配置中心
    pub fn new(namespace: Option<String>) -> Configurator {
        let id = Uuid::new_v4().to_string();
        let namespace = namespace.unwrap_or_else(|| "default".to_string());
        Configurator {
            id,
            namespace,
            configuration: HashMap::new(),
        }
    }
    /// 创建配置
    pub fn set_configuration(&mut self, name: String, config: Value) -> Result<()> {
        self.configuration.insert(name, config);
        Ok(())
    }
}

/// 配置中心存储
pub struct ConfiguratorStore<T, U>
where
    T: Storage,
    U: Connection,
{
    connection: U,
    storage: T,
    /// 配置中心列表
    pub values: Vec<Configurator>,
}

impl<T, U> Store for ConfiguratorStore<T, U>
where
    T: Storage,
    U: Connection,
{
    type Target = Configurator;

    type Storage = T;

    fn get_storage(&self) -> T {
        self.storage.clone()
    }

    fn get_storage_key(&self) -> &str {
        "ConfiguratorStorage"
    }

    fn get_values(&self) -> Vec<Self::Target> {
        self.values.clone()
    }

    fn set_values(&mut self, values: Vec<Self::Target>) {
        self.values = values;
    }
}

impl<T, U> ConfiguratorStore<T, U>
where
    T: Storage,
    U: Connection,
{
    /// 创建配置中心存储
    pub fn new(connection: U, storage: T) -> Self {
        Self {
            connection,
            storage,
            values: vec![],
        }
    }
    /// 添加配置中心
    pub fn append_configurator(&mut self, configurator: Configurator) -> Result<()> {
        match self
            .values
            .iter_mut()
            .find(|x| x.namespace == configurator.namespace)
        {
            None => {
                self.values.push(configurator);
            }
            Some(value) => {
                for (name, config) in configurator.configuration {
                    value.configuration.insert(name, config);
                }
            }
        };
        Ok(())
    }
}

impl<T, U> Connection for ConfiguratorStore<T, U>
where
    T: Storage,
    U: Connection,
{
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
