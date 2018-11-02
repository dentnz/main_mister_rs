/// TODO: this file should not exists, instead cores needs to be loaded dynamically
use cores::api::Core;
use cores::minimig;

pub struct CoreRegistry {
    pub cores: Vec<Box<Core>>
}

impl CoreRegistry{}

pub fn get_default() -> CoreRegistry {
    CoreRegistry {
        cores: vec![Box::new(minimig::MinimigCore::new())]
    }
}