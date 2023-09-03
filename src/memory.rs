use std::{fs::File, io::prelude::*,};

/// The Bus struct is hosting the 8080 memory map and the pending IO operations for outer handling.
pub struct Bus {
    address_space: Vec<u8>,
    rom_space: Option<ROMSpace>,
}

/// Start and end addresses of read-only (ROM) area.
pub struct ROMSpace {
    pub start: u16,
    pub end: u16,
}

impl Bus {
    #[doc(hidden)]
    pub fn new() -> Bus {
        Bus {
            address_space: vec![0; 65536],
            rom_space: None,
        }
    }

    /// Sets a ROM space. Write operations will be ineffective in this address range.
    pub fn set_romspace(&mut self, start: u16, end: u16) {
        self.rom_space = Some(ROMSpace{start, end});
    }

    /// Reads a byte from memory
    pub fn read_byte(&self, address: u16) -> u8 {
        self.address_space[usize::from(address)]
    }

    /// Writes a byte to memory
    pub fn write_byte(&mut self, address: u16, data: u8) {
        // if rom space is declared, and write operation is requested in rom area : we exit
        if self.rom_space.is_some() && address >= self.rom_space.as_ref().unwrap().start && address <= self.rom_space.as_ref().unwrap().end { return };
        self.address_space[usize::from(address)] = data;
    }

    /// Reads a word stored in memory in little endian byte order, returns this word in BE byte order
    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.address_space[usize::from(address)]) | (u16::from(self.address_space[usize::from(address + 1)]) << 8)
    }

    // Reads a word stored in memory in little endian byte order, returns this word in LE byte order
    pub fn read_le_word(&self, address: u16) -> u16 {
        u16::from(self.address_space[usize::from(address)]) << 8 | (u16::from(self.address_space[usize::from(address + 1)]))
    }

    /// Writes a word to memory in little endian byte order
    pub fn write_word(&mut self, address: u16, data: u16) {
        // if rom space is declared, and write operation is requested in rom area : we exit
        if self.rom_space.is_some() && address >= self.rom_space.as_ref().unwrap().start && address <= self.rom_space.as_ref().unwrap().end { return };
        self.address_space[usize::from(address)] = (data & 0xFF) as u8;
        self.address_space[usize::from(address + 1)] = (data >> 8) as u8;
    }

    /// Loads binary data from disk into memory at $0000 + offset
    pub fn load_bin(&mut self, file: &str, org: u16) -> Result<(), std::io::Error> {
        let mut f = File::open(file)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        self.address_space[org as usize..(buf.len() + org as usize)].clone_from_slice(&buf[..]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rw_byte() {
        let mut b = Bus::new();
        b.write_byte(0x0000, 0xFF);
        assert_eq!(b.read_byte(0x0000), 0xFF);
    }

    #[test]
    fn rw_word() {
        let mut b = Bus::new();
        b.write_word(0x0000, 0x1be3);
        assert_eq!(b.read_word(0x0000), 0x1be3);
    }

    #[test]
    fn rw_le_word() {
        let mut b = Bus::new();
        b.write_word(0x0000, 0x1be3);
        assert_eq!(b.read_le_word(0x0000), 0xe31b);
    }
}