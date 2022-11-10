use crate::data::ConfigurationReg;
use crate::error::LuoshuResult;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use super::LuoshuDataEnum;

/// 消息订阅数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub(crate) namespace: String,
    pub(crate) name: String,
}

impl Subscribe {
    /// 创建订阅内容
    pub fn new(namespace: String, name: String) -> Self {
        Self { namespace, name }
    }
}

impl From<&ConfigurationReg> for Subscribe {
    fn from(config: &ConfigurationReg) -> Self {
        Self {
            namespace: config.get_namespace(),
            name: config.name.clone(),
        }
    }
}

impl Display for Subscribe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.namespace, self.name)
    }
}

/// 消息帧操作
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActionEnum {
    /// 上线
    Up(LuoshuDataEnum),
    /// 下线
    Down(LuoshuDataEnum),
    /// 同步
    Sync(LuoshuDataEnum),
    /// 订阅
    Subscribe(Subscribe),
    /// 心跳包：ping
    Ping,
    /// 心跳包：pong
    Pong,
}

/// 消息帧
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Frame {
    /// 消息数据
    pub data: ActionEnum,
}

impl From<ActionEnum> for Frame {
    fn from(data: ActionEnum) -> Self {
        Self { data }
    }
}

#[allow(dead_code)]
impl Frame {
    /// 消息分析
    pub fn parse(src: &[u8]) -> LuoshuResult<Frame> {
        serde_json::from_slice(src).map_err(|e| e.into())
    }
}
