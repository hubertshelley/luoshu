//! registry for luoshu
#![deny(missing_docs)]
extern crate chrono;

mod service;

pub use service::Service;

use anyhow::Result;
use luoshu_core::{default_namespace, get_default_uuid4, Storage, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 注册中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    #[serde(default = "get_default_uuid4")]
    id: String,
    #[serde(default = "default_namespace")]
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
    pub fn register_service(&mut self, service: Service) -> Result<()> {
        self.services.push(service);
        Ok(())
    }
}

/// 注册中心存储
pub struct RegistryStore<T>
    where
        T: Storage,
{
    storage: T,
    /// 注册中心列表
    pub values: Vec<Registry>,
}

impl<T> Store for RegistryStore<T>
    where
        T: Storage,
{
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

impl<T> RegistryStore<T>
    where
        T: Storage,
{
    /// 创建注册中心存储
    pub fn new(storage: T) -> Self {
        Self {
            storage,
            values: vec![],
        }
    }
    /// 添加注册中心
    pub fn append_registry(&mut self, registry: Registry) -> Result<()> {
        match self
            .values
            .iter_mut()
            .find(|x| x.namespace == registry.namespace && x.name == registry.name)
        {
            None => {
                self.values.push(registry);
            }
            Some(value) => {
                for service in &registry.services {
                    for sub_value in value.services.iter_mut() {
                        if sub_value == service {
                            sub_value.fresh();
                        }
                    }
                }
            }
        };
        Ok(())
    }
}
