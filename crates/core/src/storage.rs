/// 存储trait
pub trait Storage {
    /// 保存数据
    fn save(&self);
    /// 加载数据
    fn load(&self);
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Store {}
    impl Storage for Store {
        fn save(&self) {}
        fn load(&self) {}
    }

    #[test]
    fn storage_test() {}
}
