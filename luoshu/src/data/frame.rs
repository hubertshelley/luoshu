use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::LuoshuDataEnum;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub(crate) enum ActionEnum {
    Up,
    Down,
    Sync,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Frame {
    pub action: ActionEnum,
    pub data: LuoshuDataEnum,
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self.action {
            ActionEnum::Up => "up",
            ActionEnum::Down => "down",
            ActionEnum::Sync => "sync",
        };
        write!(f, "{}", action)
    }
}

#[allow(dead_code)]
impl Frame {
    pub fn parse(src: &[u8]) -> Result<Frame> {
        // tracing::info!("{:?}", src.get_ref());
        Ok(serde_json::from_slice(src)?)
        // Ok(Frame {
        //     action: ActionEnum::Up,
        //     data: NamespaceReg {
        //         name: "test".to_string(),
        //     }
        //     .into(),
        // })
    }
}
