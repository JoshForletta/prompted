use std::{env, fmt::Display};

use serde::Deserialize;
use serde_json::{Value};

use crate::{components::Component};

const DEFAULT_CONFIG: LognameConfig = LognameConfig {};

#[derive(Deserialize)]
pub struct LognameConfig;

pub struct Logname {
    config: LognameConfig,
    logname: String,
}

impl Logname {
    pub fn new() -> Self {
        let logname = env::var("LOGNAME").unwrap_or(String::new());

        Self {
            config: DEFAULT_CONFIG,
            logname,
        }
    }
}

impl Display for Logname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(&self.logname);
        write!(f, "{}", self.logname)
    }
}

impl Component for Logname {
    fn id(&self) -> String {
        "logname".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {}
}
