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
const SOCFPGA_MGR_GPO_REGISTER_ADDRESS : u32 = SOCFPGA_MGR_ADDRESS + 0x10;
const SOCFPGA_MGR_GPI_REGISTER_ADDRESS : u32 = SOCFPGA_MGR_ADDRESS + 0x14;

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

/// Converts an u8 little endian byte array to u32
#[inline]
fn u8aletou32(a: [u8; 4]) -> u32 {
    a[0] as u32 
        + (a[1] as u32).rotate_left(8) 
        + (a[2] as u32).rotate_left(16)
        + (a[3] as u32).rotate_left(24)
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

    /// Checks whetever the FPGA core is ready.
    /// This is done by checking the first bit of the GPI register which should be 1.
    /// First bit is set to 1 once the core is started running. (TODO: is this correct?)
    pub fn is_fpga_ready(&mut self) -> FPGAResult<bool> {
        return match self.fpga_gpi_read() {
            Ok(value) => Ok(value as i32 >= 0),
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

    /// Writes a new value to the FPGA manager's GPO register
    fn fpga_gpo_write(&mut self, value: u32) -> std::io::Result<()> {
        self.writeu32le(value, SOCFPGA_MGR_GPO_REGISTER_ADDRESS)?;
        Ok(())
    }

    // Reads the FPGA manager's GPI register
    fn fpga_gpi_read(&mut self) -> std::io::Result<u32> {
        self.readu32le(SOCFPGA_MGR_GPI_REGISTER_ADDRESS)
    }

    // Creates a mutable memory slice from the FPGA memory region for access
    #[inline]
    fn get_memory_slice(&mut self, offset: u32, size: u32) -> &mut [u8] {
        let reg24 = (offset & 0xFFFFFF) >> 2;
        &mut self.mem_map[reg24 as usize..(reg24 + size) as usize]
    }

    /// Writes a u32 to an FPGA memory position as little endian value
    #[inline]
    fn writeu32le(&mut self, value: u32, offset: u32) -> std::io::Result<()> {
        let mut slice = self.get_memory_slice(offset, 3);
        slice.write_all(&u32tou8ale(value)[..])?;
        Ok(())
    }

    // Reads an FPGA memory position as little endian value into an u32 integer
    #[inline]
    fn readu32le(&mut self, offset: u32) -> std::io::Result<u32> {
        let slice = self.get_memory_slice(offset, 3);
        let mut u8lea : [u8; 4] = Default::default();
        u8lea.copy_from_slice(&slice[..]);
        Ok(u8aletou32(u8lea))
    }
}