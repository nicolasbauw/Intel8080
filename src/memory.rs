use std::{fs::File, io::prelude::*,};

pub struct AddressBus {
    ram: Vec<u8>,
    io: Vec<u8>
}

/// The AddressBus struct is hosting the 8080 memory map. So far it's a 64 KByte RAM space.
impl AddressBus {
    pub fn new() -> AddressBus {
        AddressBus {
            ram: vec![0; 0xFFFF],
            io: vec![0; 0xFF]
        }
    }

    /// Reads a byte from memory
    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[usize::from(address)]
    }

    /// Writes a byte to memory
    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[usize::from(address)] = data;
    }

    /// Reads a word stored in memory in little endian byte order
    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.ram[usize::from(address)]) | (u16::from(self.ram[usize::from(address + 1)]) << 8)
    }

    /// Writes a word to memory in little endian byte order
    pub fn write_word(&mut self, address: u16, data: u16) {
        self.ram[usize::from(address)] = (data & 0xFF) as u8;
        self.ram[usize::from(address + 1)] = (data >> 8) as u8;
    }

    /// Loads binary data from disk into memory at $0000 + offset
    pub fn load_bin(&mut self, file: &str, org: u16) -> Result<(), std::io::Error> {
        let mut f = File::open(file)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        self.ram[org as usize..(buf.len() + org as usize)].clone_from_slice(&buf[..]);
        Ok(())
    }

    // Reads a byte from device number
    pub fn io_read(&self, device: u8) -> u8 {
        self.io[usize::from(device)]
    }

    // Writes a byte to device number
    pub fn io_write(&mut self, device: u8, data: u8) {
        self.io[usize::from(device)] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rw_byte() {
        let mut b = AddressBus::new();
        b.write_byte(0x0000, 0xFF);
        assert_eq!(b.read_byte(0x0000), 0xFF);
    }

    #[test]
    fn rw_word() {
        let mut b = AddressBus::new();
        b.write_word(0x0000, 0x1be3);
        assert_eq!(b.read_word(0x0000), 0x1be3);
    }
}