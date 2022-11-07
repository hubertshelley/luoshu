use chrono::Local;
use serde::{Deserialize, Serialize};

/// 服务
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    /// host
    pub host: String,
    /// port
    pub port: u16,
    #[serde(skip_deserializing)]
    reg_time: i64,
}

impl Service {
    /// 创建一个服务
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            reg_time: Local::now().timestamp(),
        }
    }
    /// 刷新服务存活时间
    pub fn fresh(&mut self) {
        self.reg_time = Local::now().timestamp();
    }
}

impl PartialEq<Self> for Service {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host && self.port == other.port
    }
}

impl Eq for Service {}
