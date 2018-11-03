/// TODO: this file should not exists, instead cores needs to be loaded dynamically
use cores::api::Core;
use cores::minimig;

pub struct CoreRegistry {
    pub _core_path: String,
    pub cores: Vec<Box<Core>>
}

impl CoreRegistry{}

pub fn get_default(core_path: &str) -> CoreRegistry {
    CoreRegistry {
        _core_path: core_path.to_string(),
        cores: vec![
            Box::new(minimig::MinimigCore::new())
        ]
    }
}