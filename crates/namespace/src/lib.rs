//! registry for luoshu
#![deny(missing_docs)]

use anyhow::Result;
use core::Storage;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
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
        Self {
            id,
            name,
        }
    }
}

/// 命名空间存储
pub struct NamespaceStore {
    storage: Box<dyn Storage<Target=Namespace>>,
    /// 命名空间内容
    pub namespaces: Vec<Namespace>,
}

impl Deref for NamespaceStore {
    type Target = Vec<Namespace>;

    fn deref(&self) -> &Self::Target {
        &self.namespaces
    }
}

impl DerefMut for NamespaceStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.namespaces
    }
}

impl NamespaceStore {
    /// 创建命名空间存储
    pub fn new(storage: Box<dyn Storage<Target=Namespace>>) -> Self {
        Self {
            storage,
            namespaces: vec![],
        }
    }
    /// 添加命名空间
    pub fn append_namespace(&mut self, namespace: Namespace) -> Result<()> {
        self.namespaces.push(namespace);
        Ok(())
    }

    /// 存储命名空间
    pub fn save(&mut self) -> Result<()> {
        self.storage.save(self.namespaces.clone())
    }

    /// 加载命名空间
    pub fn load(&mut self) -> Result<()> {
        self.namespaces = self.storage.load()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Namespace;

    #[test]
    fn test_default() {
        let namespace = Namespace::default();
        println!("{:#?}", namespace);
    }
}