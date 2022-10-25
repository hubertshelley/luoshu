use anyhow::Result;

/// 存储trait
pub trait Storage {
    /// 存储数据类型
    type Target;
    /// 保存数据
    fn save(&self, values: Self::Target) -> Result<()>;
    /// 加载数据
    fn load(&mut self) -> Result<Self::Target>;
    /// 加载数据
    fn load_value(&mut self, key: &str) -> Result<Self::Target>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Store {}

    impl Storage for Store {
        type Target = u8;

        fn save(&self, _: Self::Target) -> Result<()> {
            Ok(())
        }
        fn load(&mut self) -> Result<Self::Target> {
            Ok(0)
        }

        fn load_value(&mut self, _key: &str) -> Result<Self::Target> {
            Ok(0)
        }
    }

    #[test]
    fn storage_test() {}
}
