//! registry for luoshu
#![deny(missing_docs)]

mod configurator_storage;
mod namespace_storage;
mod registry_storage;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

pub use configurator_storage::*;
pub use namespace_storage::*;
pub use registry_storage::*;

/// 全局存储文件配置
pub static SLED_DB: Lazy<RwLock<sled::Db>> =
    Lazy::new(|| RwLock::new(sled::open("registry.db").unwrap()));
