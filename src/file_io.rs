use std::fs::create_dir_all;
use std::io;

pub struct Storage {
    _full_path: String
}

impl Storage {
    pub fn init() -> io::Result<Storage> {
        Ok(Storage {
            _full_path: Storage::find_storage()?
        })
    }

    fn find_storage() -> io::Result<String> {
        let root_path = Storage::find_storage_root()?;
        create_dir_all(&root_path)?;
        Ok(root_path)
    }

    fn find_storage_root() -> io::Result<String> {
        // TODO: this is now hardwired for SD storage. The original MiSTer code is
        // capable of checking for mounted USB drives and restore 
        // the last selection from config/device.bin
        let sd_storage_path = String::from("/media/fat/config");
        Ok(sd_storage_path)
    }
}