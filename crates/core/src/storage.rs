use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 存储trait
pub trait Store {
    /// 存储类型
    type Target: Serialize + for<'a> Deserialize<'a>;
    /// 存储
    type Storage: Storage;
    /// 获取存储对象
    fn get_storage(&self) -> Self::Storage;
    /// 获取存储键名
    fn get_storage_key(&self) -> &str;
    /// 获取存储数据
    fn get_values(&self) -> Vec<Self::Target>;
    /// 获取存储数据
    fn set_values(&mut self, values: Vec<Self::Target>);
    /// 保存数据
    fn save(&self) -> Result<()> {
        self.get_storage().save(
            self.get_storage_key(),
            serde_json::to_string(&self.get_values())?.as_bytes(),
        )
    }
    /// 加载数据
    fn load(&mut self) -> Result<()> {
        match self.get_storage().load(self.get_storage_key()) {
            Some(data) => {
                self.set_values(serde_json::from_slice(data.as_slice())?);
            }
            None => self.set_values(vec![]),
        };
        Ok(())
    }
    /// 添加数据
    fn append(&mut self, value: Self::Target) -> Result<()>;
    /// 删除数据
    fn drop(&mut self, value: Self::Target) -> Result<()>;
}

/// 存储trait
pub trait Storage: Clone + Send + Sync {
    /// 保存数据
    fn save(&self, key: &str, values: &[u8]) -> Result<()>;
    /// 加载数据
    fn load(&mut self, key: &str) -> Option<Vec<u8>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Store {}

    impl Storage for Store {
        fn save(&self, _key: &str, _: &[u8]) -> Result<()> {
            Ok(())
        }
        fn load(&mut self, _key: &str) -> Option<Vec<u8>> {
            None
        }
    }

    #[test]
    fn storage_test() {}
}
