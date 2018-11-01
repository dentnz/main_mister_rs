extern crate libc;
extern crate memmap;

mod fpga_io;

use std::process;

fn main() {
    let _fpga = match fpga_io::FPGA::init() {
        Ok(fpga) => fpga,
        Err(err) => {
            eprintln!("Failed to initialize FPGA: {:?}", err);
            process::exit(1);
        }
    };
    
    println!("Hello, world!");
}
