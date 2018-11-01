use libc;
use memmap::*;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::result;

#[derive(Debug)]
pub enum FPGAError {
    /// Gerneric IO error
    Io(io::Error)
}

pub type FPGAResult<T> = result::Result<T, FPGAError>;

pub struct FPGA {
    mem_map: MmapMut
}

const FPGA_REG_BASE: u64 = 0xFF000000;
const FPGA_REG_SIZE: libc::size_t = 0x01000000;

const SOCFPGA_MGR_ADDRESS: u32 = 0xFF706000;

/// Converts u32 to u8 little endian byte array
#[inline]
fn u32tou8ale(v: u32) -> [u8; 4] {
    [
        v as u8,
        (v >> 8) as u8,
        (v >> 16) as u8,
        (v >> 24) as u8,
    ]
}

impl FPGA {
    pub fn init() -> FPGAResult<FPGA> {
        let mem_map = match FPGA::mem_map_init() {
            Ok(mem_map) => mem_map,
            Err(e) => return Err(e)
        };

        let mut fpga = FPGA {
            mem_map: mem_map
        };

        return match fpga.fpga_gpo_write(0) {
            Ok(_) => Ok(fpga),
            Err(e) => Err(FPGAError::Io(e))
        };
    }

    /// Initializes the memory mapped region of the FPGA visible from the HPS
    fn mem_map_init() -> FPGAResult<MmapMut> {
        let mem_dev_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_SYNC)
            .open("/dev/mem")
        {
            Ok(file) => file,
            Err(e) => return Err(FPGAError::Io(e))
        };

        unsafe {
            let mmap_mut = match MmapOptions::new()
                .offset(FPGA_REG_BASE)
                .len(FPGA_REG_SIZE)
                .map_mut(&mem_dev_file) {
                    Ok(mmap_mut) => mmap_mut,
                    Err(e) => return Err(FPGAError::Io(e))
                };
            return Ok(mmap_mut);
        };
    }

    /// Writes a u32 to the FPGA manager's GPO register
    fn fpga_gpo_write(&mut self, value: u32) -> std::io::Result<()> {
        self.writeu32le(value, SOCFPGA_MGR_ADDRESS + 0x10)?;
        Ok(())
    }

    /// Writes a u32 to an FPGA memory position as little endian value
    #[inline]
    fn writeu32le(&mut self, value: u32, offset: u32) -> std::io::Result<()> {
        let reg24 = (offset & 0xFFFFFF) >> 2;
        (&mut self.mem_map[reg24 as usize..(reg24+3) as usize])
            .write_all(&u32tou8ale(value)[..])?;
        Ok(())
    }
}