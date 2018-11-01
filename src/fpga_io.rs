use libc;
use memmap::MmapOptions;
use std::fs::OpenOptions;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::result;

#[derive(Debug)]
pub enum FPGAError {
    /// IO error
    Io(io::Error)
}

pub type FPGAResult<T> = result::Result<T, FPGAError>;

pub struct FPGA {
    mem_map: Option<memmap::MmapMut>
}

const FPGA_REG_BASE: u64 = 0xFF000000;
const FPGA_REG_SIZE: libc::size_t = 0x01000000;

impl FPGA {
    pub fn new() -> FPGA {
        FPGA {
            mem_map: None
        }
    }

    pub fn io_init(&mut self) -> FPGAResult<()> {
        if self.mem_map.is_some() {
            return Ok(())
        }

        let fpga_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_SYNC)
            .open("/dev/mem")
        {
            Ok(file) => file,
            Err(e) => return Err(FPGAError::Io(e))
        };

        unsafe {
            self.mem_map = match MmapOptions::new()
                .offset(FPGA_REG_BASE)
                .len(FPGA_REG_SIZE)
                .map_mut(&fpga_file) {
                    Ok(mmap_mut) => Some(mmap_mut),
                    Err(e) => return Err(FPGAError::Io(e))
                };
        };

        Ok(())
    }
}