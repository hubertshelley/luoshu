use anyhow::Result;
use core::Storage;
use namespace::Namespace;

/// 命名空间存储sled实现
pub struct NamespaceStorage {
    db: sled::Db,
    key: String,
}

impl NamespaceStorage {
    /// 创建命名空间存储
    pub fn new(db: sled::Db) -> Self {
        Self {
            db,
            key: "NamespaceStorage".to_string(),
        }
    }
}

impl Storage for NamespaceStorage {
    type Target = Namespace;

    fn save(&self, values: Vec<Self::Target>) -> Result<()> {
        self.db
            .insert(
                self.key.as_bytes(),
                serde_json::to_string(&values).unwrap().as_bytes(),
            )
            .expect("NamespaceStorage save error");
        Ok(())
    }

    fn load(&mut self) -> Result<Vec<Self::Target>> {
        let _data = self.db.get(self.key.as_bytes()).unwrap();
        match _data {
            None => Ok(vec![]),
            Some(_data) => {
                let _data: Vec<Namespace> = serde_json::from_slice(_data.to_vec().as_slice())?;
                Ok(_data)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::NamespaceStorage;
    use anyhow::Result;
    use namespace::{Namespace, NamespaceStore};

    #[test]
    fn name_space_store_save_test() -> Result<()> {
        let db: sled::Db = sled::open("test_db_namespace1").unwrap();
        let storage = NamespaceStorage::new(db);
        let mut store = NamespaceStore::new(Box::new(storage));
        store.append_namespace(Namespace::new("test_name_space".into()))?;
        store.save()?;
        Ok(())
    }

    #[test]
    fn name_space_store_load_test() -> Result<()> {
        let db: sled::Db = sled::open("test_db_namespace2").unwrap();
        let storage = NamespaceStorage::new(db);
        let mut store = NamespaceStore::new(Box::new(storage));
        store.load()?;
        println!("{:#?}", store.namespaces);
        Ok(())
    }
}
