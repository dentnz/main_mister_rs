use cores::api::{Core, KEY_CODE_NONE};

pub struct UnknownCore {}

const UNKNOWN_CORE_ID : u8 = 0x55;

impl Core for UnknownCore {
    fn core_id(&self) -> u8 {
        UNKNOWN_CORE_ID
    }

    fn map_key(&self, _key: u16) -> u32 {
        KEY_CODE_NONE
    }
}