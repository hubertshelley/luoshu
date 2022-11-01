//! registry for luoshu
#![deny(missing_docs)]
extern crate chrono;

mod service;

use anyhow::Result;
use chrono::Local;
use luoshu_core::{Connection, Storage, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 服务
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    /// host
    pub host: String,
    /// port
    pub port: u16,
    #[serde(skip_deserializing)]
    reg_time: i64,
}

impl PartialEq<Self> for Service {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host && self.port == other.port
    }
}

impl Eq for Service {}

/// 注册中心
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    #[serde(default)]
    id: String,
    #[serde(default)]
    namespace: String,
    name: String,
    services: Vec<Service>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            namespace: "default".to_string(),
            name: Default::default(),
            services: Default::default(),
        }
    }
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
    pub fn register_service(&mut self, host: String, port: u16) -> Result<()> {
        self.services.push(Service {
            host,
            port,
            reg_time: Local::now().timestamp(),
        });
        Ok(())
    }
}

/// 注册中心存储
pub struct RegistryStore<T, U>
where
    T: Storage,
    U: Connection,
{
    connection: U,
    storage: T,
    /// 注册中心列表
    pub values: Vec<Registry>,
}

impl<T, U> Store for RegistryStore<T, U>
where
    T: Storage,
    U: Connection,
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

impl<T, U> RegistryStore<T, U>
where
    T: Storage,
    U: Connection,
{
    /// 创建注册中心存储
    pub fn new(connection: U, storage: T) -> Self {
        Self {
            connection,
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
                    let mut edit_index = 0;
                    let mut is_edit = false;
                    for sub_value in &value.services {
                        if sub_value == service {
                            is_edit = true;
                            break;
                        }
                        edit_index += 1;
                    }
                    if is_edit {
                        value.services.get_mut(edit_index).unwrap().reg_time = service.reg_time
                    }
                }
            }
        };
        Ok(())
    }
}

impl<T, U> Connection for RegistryStore<T, U>
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
