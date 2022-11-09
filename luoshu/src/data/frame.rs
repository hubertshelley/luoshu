use crate::error::LuoshuResult;
use serde::{Deserialize, Serialize};

use super::LuoshuDataEnum;

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActionEnum {
    Up(LuoshuDataEnum),
    Down(LuoshuDataEnum),
    Sync(LuoshuDataEnum),
    Subscribe(String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Frame {
    pub data: ActionEnum,
}

impl From<ActionEnum> for Frame {
    fn from(data: ActionEnum) -> Self {
        Self { data }
    }
}

#[allow(dead_code)]
impl Frame {
    pub fn parse(src: &[u8]) -> LuoshuResult<Frame> {
        serde_json::from_slice(src).map_err(|e| e.into())
    }
}
