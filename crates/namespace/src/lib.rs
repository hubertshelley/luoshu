//! registry for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use luoshu_core::{Connection, Storage, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 命名空间
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Namespace {
    id: String,
    name: String,
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
            id: Uuid::nil().to_string(),
            name: "default".to_string(),
        }
    }
}

impl Namespace {
    /// 创建命名空间
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self { id, name }
    }
}

/// 命名空间存储
pub struct NamespaceStore<T, U>
where
    T: Storage,
    U: Connection,
{
    connection: U,
    storage: T,
    /// 命名空间内容
    pub values: Vec<Namespace>,
}

impl<T, U> Store for NamespaceStore<T, U>
where
    T: Storage,
    U: Connection,
{
    type Target = Namespace;

    type Storage = T;

    fn get_storage(&self) -> T {
        self.storage.clone()
    }

    fn get_storage_key(&self) -> &str {
        "NamespaceStorage"
    }

    fn get_values(&self) -> Vec<Self::Target> {
        self.values.clone()
    }

    fn set_values(&mut self, values: Vec<Self::Target>) {
        self.values = values;
    }
}

impl<T, U> NamespaceStore<T, U>
where
    T: Storage,
    U: Connection,
{
    /// 创建命名空间存储
    pub fn new(connection: U, storage: T) -> Self {
        Self {
            connection,
            storage,
            values: vec![],
        }
    }
    /// 添加命名空间
    pub fn append_namespace(&mut self, namespace: Namespace) -> Result<()> {
        self.values.push(namespace);
        Ok(())
    }
}

impl<T, U> Connection for NamespaceStore<T, U>
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
