pub const KEY_CODE_NONE : u32 = 0xFF;
pub const KEY_CAPS_TOGGLE : u32 = 0x040000;

pub trait Core {
    /// Returns this core's id.
    fn core_id(&self) -> u8;

    /// Maps an input device specific keycode to a core specific keycode
    fn map_key(&self, key : u16) -> u32;
}