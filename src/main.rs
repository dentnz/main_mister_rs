extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate libc;
#[macro_use]
extern crate log;
extern crate memmap;

mod core_registry;
mod cores;
mod fpga_io;
mod file_io;
#[macro_use]
mod utils;

use clap::{Arg, App};

const ARG_CORE_PATH : &str = "CORE_PATH";

const DEFAULT_CORE_PATH : &str = "";

fn main() {
    let mut fpga = unwrap_or_exit!(fpga_io::FPGA::init(), "Failed to initialize FPGA");
    
    info!("main_mister_rs starting.");

    if !fpga.is_ready() {
        exit!("FPGA is uninitialized or incompatible core loaded.");
    }

    let _storage = unwrap_or_exit!(file_io::Storage::init(), "Cannot initialize storage");
    
    let arg_matches = App::new("main_mister_rs")
        .version(crate_version!())
        .arg(Arg::with_name(ARG_CORE_PATH)
            .help("The path where cores placed.")
            .required(false)
            .index(1))
        .get_matches();

    let core_path = arg_matches.value_of(ARG_CORE_PATH).unwrap_or(DEFAULT_CORE_PATH);

    info!("Core path: {0}", core_path);

    let _cores = core_registry::get_default(core_path);

    // TODO: user_io related initializations

    loop {
        fpga.ensure_ready();

        // TODO: user IO poll
        // TODO: input polling
        // TODO: UI handler tick
    }
}
