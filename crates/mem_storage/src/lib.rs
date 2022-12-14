//! registry for luoshu
#![deny(missing_docs)]

use luoshu_core::Storage;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 全局存储文件配置
static MEM_DB: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(HashMap::new);

/// 洛书数据持久化Sled实现
#[derive(Debug, Clone)]
pub struct LuoshuMemStorage {
    /// 存储对象
    pub storage: HashMap<String, Vec<u8>>,
}

impl Default for LuoshuMemStorage {
    fn default() -> Self {
        Self {
            storage: MEM_DB.clone(),
        }
    }
}

impl LuoshuMemStorage {
    /// 创建存储
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for LuoshuMemStorage {
    fn save(&mut self, key: &str, values: &[u8]) -> anyhow::Result<()> {
        self.storage.insert(key.into(), values.to_vec());
        Ok(())
    }

    fn load(&mut self, key: &str) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }
}
