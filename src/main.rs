extern crate byteorder;
extern crate libc;
extern crate memmap;

mod fpga_io;
mod file_io;

use std::{process, env, thread, time};

fn main() {
    let mut fpga = match fpga_io::FPGA::init() {
        Ok(fpga) => fpga,
        Err(err) => {
            eprintln!("Failed to initialize FPGA: {:?}", err);
            process::exit(0);
        }
    };
    
    println!("main_mister_rs starting.");

    if !fpga.is_ready() {
        eprintln!("FPGA is uninitialized or incompatible core loaded.");
        process::exit(0);
    }

    let _storage = match file_io::Storage::init() {
        Ok(storage) => storage,
        Err(err) => {
            eprintln!("Cannot initialize storage: {:?}", err);
            process::exit(0);
        }
    };

    let args: Vec<String> = env::args().collect();

    let default_core_path = &String::from("");
    let core_path = args.get(0).or(Some(default_core_path)).unwrap();

    println!("Core path: '{0}'", core_path);

    // TODO: user_io related initializations

    loop {
        if !fpga.is_ready() {
            println!("Waiting for FPGA to be ready...");

            fpga.core_reset();

            while !fpga.is_ready() {
                thread::sleep(time::Duration::from_secs(1));
            }

            fpga.reboot(false);
        }

        // TODO: user IO poll
        // TODO: input polling
        // TODO: UI handler tick
    }
}
