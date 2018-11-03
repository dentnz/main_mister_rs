/// TODO: this file should not exists, instead cores needs to be loaded dynamically
use cores::api::Core;
use cores::{minimig, unknown};

pub struct CoreRegistry {
    pub _core_path: String,
    pub unknown_core: Box<Core>,
    pub cores: Vec<Box<Core>>
}

impl CoreRegistry{
    /// Finds a core in the registry by it's id
    pub fn get_core_by_id(&self, id: u8) -> Option<&Box<Core>> {
        let mut iter = self.cores.iter();
        iter.find(|&c| c.core_id() == id)
    }

    pub fn get_unknown_core(&self) -> &Box<Core> {
        &self.unknown_core
    }
}

pub fn get_default(core_path: &str) -> CoreRegistry {
    CoreRegistry {
        _core_path: core_path.to_string(),
        unknown_core: Box::new(unknown::UnknownCore{}),
        cores: vec![
            Box::new(minimig::MinimigCore::new())
        ]
    }
}