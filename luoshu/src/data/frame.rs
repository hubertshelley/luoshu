use crate::error::LuoshuResult;
use serde::{Deserialize, Serialize};

use super::LuoshuDataEnum;

/// 消息帧操作
#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActionEnum {
    /// 上线
    Up(LuoshuDataEnum),
    /// 下线
    Down(LuoshuDataEnum),
    /// 同步
    Sync(LuoshuDataEnum),
    /// 订阅
    Subscribe(String),
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
