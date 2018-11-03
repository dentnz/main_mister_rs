use libc;
use memmap::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::{io, result, thread, time};
use std::os::unix::fs::OpenOptionsExt;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub enum FPGAError {
    /// IO error
    Io(io::Error),
    /// Generic Initialization failure
    Init(String)
}

pub type FPGAResult<T> = result::Result<T, FPGAError>;

pub struct FPGA {
    memmap: MmapMut
}

const FPGA_REG_BASE: u64 = 0xFF000000;
const FPGA_REG_SIZE: libc::size_t = 0x01000000;

const SOCFPGA_MGR_ADDRESS: u64 = 0xFF706000;
const SOCFPGA_MGR_GPO_REGISTER_ADDRESS : u64 = SOCFPGA_MGR_ADDRESS + 0x10;
const SOCFPGA_MGR_GPI_REGISTER_ADDRESS : u64 = SOCFPGA_MGR_ADDRESS + 0x14;

// TODO: these were manually calculated from the struct size in C code, double-check
const SOCFPGA_RSTMGR_ADDRESS : u64 = 0xffd05000;
const SOCFPGA_RSTMGR_CTRL_ADDRESS : u64 = SOCFPGA_RSTMGR_ADDRESS + 0x04;
const SOCFPGA_RSTMGR_TSTSCRATCH_ADDRESS : u64 = SOCFPGA_RSTMGR_ADDRESS + 0xC0;

const CORE_CHECK_MAGIC_NUMBER : u32 = 0x5CA623;

impl FPGA {
    pub fn init() -> FPGAResult<FPGA> {
        let memmap = FPGA::memmap_init()?;

        let mut fpga = FPGA {
            memmap: memmap
        };

        // First initialization of FPGA
        fpga.gpo_write(0);
        if !fpga.is_ready() {
            return Err(FPGAError::Init("FPGA is uninitialized or incompatible core loaded.".to_owned()));
        }

        Ok(fpga)
    }

    /// Ensures that the FPGA is in ready state. If not, resets the core and reboots.
    pub fn ensure_ready(&mut self) {
        if !self.is_ready() {
            info!("Waiting for FPGA to be ready and rebooting...");

            self.core_reset();

            while !self.is_ready() {
                thread::sleep(time::Duration::from_secs(1));
            }

            self.reboot(false);
        }
    }

    /// Retrieves the current core's ID from the FPGA
    pub fn get_current_core_id(&mut self) -> Option<u8> {
        let mut gpo = self.gpo_read() & 0x7FFFFFFF;
        self.gpo_write(gpo);
        let coretype = self.gpi_read();
        gpo |= 0x80000000;
        self.gpo_write(gpo);

        if (coretype >> 8) != CORE_CHECK_MAGIC_NUMBER { 
            None 
        } else { 
            Some((coretype & 0xFF) as u8) 
        }
    }

    /// Reboots the whole device, including HPS
    pub fn reboot(&mut self, cold: bool) {
        // TODO: sync file changes to disk before rebooting.
        self.core_reset();

        self.writeu32le(if cold { 0 } else { 0x1 }, SOCFPGA_RSTMGR_TSTSCRATCH_ADDRESS);
        self.writeu32le(2, SOCFPGA_RSTMGR_CTRL_ADDRESS);
        loop {} // Wait for the device to reboot in an endless-loop
    }

    /// Checks if the FPGA core is ready.
    /// This is done by checking the first bit of the GPI register which should be 1.
    fn is_ready(&mut self) -> bool {
        self.gpi_read() as i32 >= 0
    }

    /// Resets the FPGA core
    fn core_reset(&mut self) {
        let gpo = self.gpo_read() & !0xC0000000;
        self.gpo_write(gpo | 0x40000000);
    }

    /// Initializes the memory mapped region of the FPGA visible from the HPS
    fn memmap_init() -> FPGAResult<MmapMut> {
        let mem_dev_file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_SYNC)
            .open("/dev/mem")
            .map_err(|err| { FPGAError::Io(err) })?;

        unsafe {
            MmapOptions::new()
                .offset(FPGA_REG_BASE)
                .len(FPGA_REG_SIZE)
                .map_mut(&mem_dev_file)
                .map_err(|err| { FPGAError::Io(err) })
        }
    }

    /// Writes a new value to the FPGA manager's GPO register
    fn gpo_write(&mut self, value: u32) {
        self.writeu32le(value, SOCFPGA_MGR_GPO_REGISTER_ADDRESS)
    }

    // Reads the FPGA manager's GPO register
    fn gpo_read(&mut self) -> u32 {
        self.readu32le(SOCFPGA_MGR_GPO_REGISTER_ADDRESS)
    }
 
    // Reads the FPGA manager's GPI register
    fn gpi_read(&mut self) -> u32 {
        self.readu32le(SOCFPGA_MGR_GPI_REGISTER_ADDRESS)
    }

    // Creates a mutable memory slice from the FPGA memory region for access
    #[inline]
    fn get_memory_slice(&mut self, offset: u64, size: u64) -> &mut [u8] {
        // Strip the lower 24 bit of the offset to protect from going outside the region
        let offset24 = (offset & 0xFFFFFF) >> 2;
        &mut self.memmap[offset24 as usize..(offset24 + size as u64) as usize]
    }

    /// Writes a u32 to an FPGA memory position as little endian value
    #[inline]
    fn writeu32le(&mut self, value: u32, offset: u64) {
        LittleEndian::write_u32(self.get_memory_slice(offset, 3), value);
    }

    // Reads an FPGA memory position as little endian value into an u32 integer
    #[inline]
    fn readu32le(&mut self, offset: u64) -> u32 {
        LittleEndian::read_u32(&self.get_memory_slice(offset, 3))
    }
}