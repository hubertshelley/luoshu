use std::fmt::Display;

use crate::error::LuoshuResult;
use serde::{Deserialize, Serialize};

use super::LuoshuDataEnum;

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub enum ActionEnum {
    Up(LuoshuDataEnum),
    Down(LuoshuDataEnum),
    Sync(LuoshuDataEnum),
    Subscribe(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Frame {
    pub data: ActionEnum,
}

impl From<ActionEnum> for Frame {
    fn from(data: ActionEnum) -> Self {
        Self { data }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self.data {
            ActionEnum::Up(_) => "up",
            ActionEnum::Down(_) => "down",
            ActionEnum::Sync(_) => "sync",
            _ => "other",
        };
        write!(f, "{}", action)
    }
}

#[allow(dead_code)]
impl Frame {
    pub fn parse(src: &[u8]) -> LuoshuResult<Frame> {
        serde_json::from_slice(src).map_err(|e| e.into())
    }
}
