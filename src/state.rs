use crate::config::Config;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct State {
    pub urls: Mutex<HashMap<String, String>>,
    pub config: Config,
}

impl State {
    pub fn new(config: Config) -> State {
        State {
            config,
            urls: Mutex::default(),
        }
    }
}
