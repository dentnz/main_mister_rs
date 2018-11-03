extern crate byteorder;
extern crate libc;
extern crate memmap;

mod fpga_io;
mod file_io;
mod core_registry;
mod cores;

#[macro_use]
mod utils;

const DEFAULT_CORE_PATH : &str = "Test";

use std::{env};

fn main() {
    let mut fpga = unwrap_or_exit!(fpga_io::FPGA::init(), "Failed to initialize FPGA");
    
    println!("main_mister_rs starting.");

    if !fpga.is_ready() {
        exit!("FPGA is uninitialized or incompatible core loaded.");
    }

    let _storage = unwrap_or_exit!(file_io::Storage::init(), "Cannot initialize storage");

    let args: Vec<String> = env::args().collect();

    let default_core_path = DEFAULT_CORE_PATH.to_string();
    let _core_path = args.get(0).unwrap_or(&default_core_path);

    println!("Core path: '{0}'", _core_path);

    let _cores = core_registry::get_default();

    // TODO: user_io related initializations

    loop {
        fpga.ensure_ready();

        // TODO: user IO poll
        // TODO: input polling
        // TODO: UI handler tick
    }
}
