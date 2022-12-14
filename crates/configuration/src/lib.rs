//! configurator for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use luoshu_core::{get_default_uuid4, Storage, Store};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// 配置中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configurator {
    #[serde(default = "get_default_uuid4")]
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
    /// 获取配置
    pub fn get_configuration(&mut self, name: String) -> Option<Value> {
        self.configuration.get(&name).cloned()
    }
    /// 判断配置是否存在
    pub fn exists(&mut self, name: String) -> bool {
        self.configuration.contains_key(&name)
    }
}

/// 配置中心存储
pub struct ConfiguratorStore<T>
where
    T: Storage,
{
    storage: T,
    /// 配置中心列表
    values: Vec<Configurator>,
}

impl<T> Store for ConfiguratorStore<T>
where
    T: Storage,
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

    fn append(&mut self, value: Self::Target) -> Result<()> {
        match self
            .values
            .iter_mut()
            .find(|x| x.namespace == value.namespace)
        {
            None => {
                self.values.push(value);
            }
            Some(config_map) => {
                for (name, config) in value.configuration {
                    config_map.configuration.insert(name, config);
                }
            }
        };
        Ok(())
    }

    fn remove(&mut self, value: Self::Target) -> Result<()> {
        match self
            .values
            .iter_mut()
            .find(|x| x.namespace == value.namespace)
        {
            None => {}
            Some(config_map) => {
                for (name, _config) in value.configuration {
                    config_map.configuration.remove(&name);
                }
            }
        };
        Ok(())
    }
}

impl<T> ConfiguratorStore<T>
where
    T: Storage,
{
    /// 创建配置中心存储
    pub fn new(storage: T) -> Self {
        Self {
            storage,
            values: vec![],
        }
    }
    /// 获取命名空间下的配置
    pub fn get_configurations_by_namespace(&self, namespace: String) -> Option<Configurator> {
        self.get_values()
            .iter()
            .cloned()
            .find(|x| x.namespace == namespace)
    }
}
