use std::{env, fmt::Display};

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::{Color, Component, Sgr, SGR_RESET};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LognameConfig {
    foreground: Option<Color>,
    bold: Option<bool>,
}

pub struct Logname {
    config: LognameConfig,
    logname: String,
}

impl Logname {
    pub fn new() -> Self {
        let logname = env::var("LOGNAME").unwrap_or(String::new());

        Self {
            config: Default::default(),
            logname,
        }
    }
}

impl Display for Logname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sgr = Sgr {
            foreground: self.config.foreground,
            bold: self.config.bold,
        };
        write!(f, "{sgr}{}{SGR_RESET}", self.logname)
    }
}

impl Component for Logname {
    fn id(&self) -> String {
        "logname".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {
        if config == Value::Null {
            return;
        }
        self.config = serde_json::from_value(config).unwrap_or_else(|e| {
            error!("Failed parsing logname config: {}", e);
            Default::default()
        });
    }
}
