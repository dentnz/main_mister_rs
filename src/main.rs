extern crate libc;
extern crate memmap;

mod fpga_io;

use std::process;

fn main() {
    let mut fpga = fpga_io::FPGA::new();
    if let Err(err) = fpga.io_init() {
        eprintln!("Failed to initialize FPGA IO: {:?}", err);
        process::exit(1);
    }
    
    println!("Hello, world!");
}
