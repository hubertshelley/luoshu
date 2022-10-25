use anyhow::Result;

/// 存储trait
pub trait Storage {
    /// 存储数据类型
    type Target;
    /// 保存数据
    fn save(&self, values: Vec<Self::Target>) -> Result<()>;
    /// 加载数据
    fn load(&mut self) -> Result<Vec<Self::Target>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Store {}

    impl Storage for Store {
        type Target = u8;

        fn save(&self, _: Vec<Self::Target>) -> Result<()> {
            Ok(())
        }
        fn load(&mut self) -> Result<Vec<Self::Target>> {
            Ok(vec![])
        }
    }

    #[test]
    fn storage_test() {}
}
