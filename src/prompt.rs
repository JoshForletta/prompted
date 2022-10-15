use std::{
    ops::Index, collections::HashMap,
};

// mod component;
use crate::component::{Component, NoComponent};

pub struct Prompt {
    no_component: Box<dyn Component>,
    components: HashMap<String, Box<dyn Component>>,
}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt { no_component: Box::new(NoComponent {}), components: HashMap::new() }
    }

    pub fn load_config(&self) -> Result<(), serde_yaml::Error> {
        Ok(())
    }
}

impl Index<&str> for Prompt {
    type Output = Box<dyn Component>;

    fn index(&self, index: &str) -> &Self::Output {
        match self.components.get(index) {
            Some(c) => c,
            None => &self.no_component,
        }
    }
}
