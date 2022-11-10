//! registry for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use luoshu_core::{get_default_uuid4, Storage, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 命名空间
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Namespace {
    #[serde(default = "get_default_uuid4")]
    id: String,
    /// 命名空间名称
    pub name: String,
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
            id: Uuid::nil().to_string(),
            name: "default".to_string(),
        }
    }
}

impl From<&str> for Namespace {
    fn from(s: &str) -> Self {
        Self::new(s.into())
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
pub struct NamespaceStore<T>
where
    T: Storage,
{
    storage: T,
    /// 命名空间内容
    pub values: Vec<Namespace>,
}

impl<T> Store for NamespaceStore<T>
where
    T: Storage,
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

    fn append(&mut self, value: Self::Target) -> Result<()> {
        if !self.values.clone().iter().any(|x| x.name == value.name) {
            self.values.push(value);
        }
        Ok(())
    }

    fn remove(&mut self, value: Self::Target) -> Result<()> {
        // 默认命名空间不会删除
        if value.name != *"default" {
            self.values.retain(|x| x.name == value.name);
        }
        Ok(())
    }
}

impl<T> NamespaceStore<T>
where
    T: Storage,
{
    /// 创建命名空间存储
    pub fn new(storage: T) -> Self {
        Self {
            storage,
            values: vec![],
        }
    }
}
