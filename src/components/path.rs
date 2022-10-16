use std::{fmt::Display, path::PathBuf, env};

use serde_json::Value;

use crate::components::Component;

pub struct Path {
    path: PathBuf,
}

impl Path {
    pub fn new() -> Self {
        Self { path: env::current_dir().unwrap_or(PathBuf::new()) }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl Component for Path {
    fn id(&self) -> String {
        "path".to_string()
    }

    fn include(&self) -> bool {
        true
    }
}
