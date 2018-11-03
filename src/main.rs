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

const APP_NAME : &str = "main_mister_rs";
const ARG_CORE_PATH : &str = "CORE_PATH";
const DEFAULT_CORE_PATH : &str = "";

fn main() {
    let arg_matches = App::new(APP_NAME)
        .version(crate_version!())
        .arg(Arg::with_name(ARG_CORE_PATH)
            .help("The path where cores placed.")
            .required(false)
            .index(1))
        .get_matches();

    info!("{} starting.", APP_NAME);

    let core_path = arg_matches.value_of(ARG_CORE_PATH).unwrap_or(DEFAULT_CORE_PATH);
    info!("Using core path: '{0}'", core_path);

    // Initialize FPGA
    let mut fpga = unwrap_or_exit!(fpga_io::FPGA::init(), "Failed to initialize FPGA");

    // Initialize storage
    let _storage = unwrap_or_exit!(file_io::Storage::init(), "Cannot initialize storage");
    
    // Initialize core registry
    let cores = core_registry::get_default(core_path);

    // TODO: user IO related initializations
    let _current_core = fpga.get_current_core_id()
        .and_then(|id| cores.get_core_by_id(id))
        .or(Some(cores.get_unknown_core()))
        .unwrap();

    // Main application loop
    loop {
        fpga.ensure_ready();

        // TODO: user IO poll
        // TODO: input polling
        // TODO: UI handler tick
    }
}
