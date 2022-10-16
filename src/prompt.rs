use std::{collections::HashMap, error::Error, fs::File, ops::Index};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::{
    git::Git, hostname::Hostname, logname::Logname, path::Path, Component, NoComponent,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub components: HashMap<String, Value>,
}

pub struct Prompt {
    no_component: Box<dyn Component>,
    pub components: HashMap<String, Box<dyn Component>>,
    pub config: HashMap<String, Value>,
}

impl Prompt {
    pub fn new() -> Self {
        let mut p = Prompt {
            no_component: Box::new(NoComponent {}),
            components: HashMap::new(),
            config: Default::default(),
        };

        p.add_component(Box::new(Logname::new()));
        p.add_component(Box::new(Hostname::new()));
        p.add_component(Box::new(Path::new()));
        p.add_component(Box::new(Git::new()));

        p
    }

    pub fn load_config(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;

        self.config = serde_json::from_reader(&file)?;

        for (cid, c) in &mut self.components {
            c.load_config(self.config.remove(cid).unwrap_or(Value::Null));
        }

        Ok(())
    }

    pub fn add_component(&mut self, c: Box<dyn Component>) {
        self.components.insert(c.id(), c);
    }
}

impl Index<&str> for Prompt {
    type Output = Box<dyn Component>;

    fn index(&self, index: &str) -> &Self::Output {
        self.components.get(index).unwrap_or(&self.no_component)
    }
}
