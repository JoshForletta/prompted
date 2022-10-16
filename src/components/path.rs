use std::{
    env,
    fmt::Display,
    path::{Display as IoDisplay, PathBuf},
};

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::{Color, Component, Sgr, SGR_RESET};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CwdConfig {
    foreground: Option<Color>,
    bold: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PathConfig {
    foreground: Option<Color>,
    bold: Option<bool>,
    cwd: Option<CwdConfig>,
}

#[derive(Debug, Default)]
pub struct Path {
    config: PathConfig,
    path: PathBuf,
}

impl Path {
    pub fn new() -> Self {
        let path = env::current_dir().unwrap_or_else(|e| {
            error!("Failed to get current directory: {e}");
            PathBuf::new()
        });

        Self {
            path,
            ..Default::default()
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sgr = Sgr {
            foreground: self.config.foreground,
            bold: self.config.bold,
        };

        let get_path_components = || {
            let p = self.path.parent()?.display();
            let cwd = self.path.file_name()?.to_str()?;

            Some((p, cwd))
        };

        if let (Some(cwd_config), Some((p, cwd))) = (&self.config.cwd, get_path_components()) {
            let cwd_sgr = Sgr {
                foreground: cwd_config.foreground,
                bold: cwd_config.bold,
            };

            return write!(f, "{sgr}{p}/{cwd_sgr}{cwd}{SGR_RESET}");
        };

        write!(f, "{sgr}{}{SGR_RESET}", self.path.display())
    }
}

impl Component for Path {
    fn id(&self) -> String {
        "path".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {
        if config == Value::Null {
            return;
        }

        self.config = serde_json::from_value(config).unwrap_or_else(|e| {
            error!("Failed parsing path config: {}", e);
            Default::default()
        });
    }
}
