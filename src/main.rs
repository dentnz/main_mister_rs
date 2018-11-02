extern crate libc;
extern crate memmap;

mod fpga_io;

use std::process;
use std::env;

fn main() {
    let mut fpga = match fpga_io::FPGA::init() {
        Ok(fpga) => fpga,
        Err(err) => {
            eprintln!("Failed to initialize FPGA: {:?}", err);
            process::exit(0);
        }
    };
    
    println!("main_mister_rs starting.");

    let args: Vec<String> = env::args().collect();

    let default_core_path = &String::from("");
    let core_path = args.get(0).or(Some(default_core_path)).unwrap();

    println!("Core path: '{0}'", core_path);

    match fpga.is_fpga_ready() {
        Ok(ready) => {
            if !ready {
                eprintln!("FPGA is unitialized or incompatible core loaded.");
                process::exit(0);
            }
        },
        Err(err) => {
            eprintln!("Failed to read FPGA ready state: {:?}", err);
            process::exit(0);
        }
    }
}
