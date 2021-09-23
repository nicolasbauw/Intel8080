use std::{fs::File, io::prelude::*,};

/// The AddressBus struct is hosting the 8080 memory map and the pending IO operations for outer handling.
pub struct AddressBus {
    ram: Vec<u8>,
    io_in: Vec<u8>,
    pending_io : PendingIO,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[doc(hidden)]
// Is the requested IO an input, output, or no IO requested ?
pub enum IO {
    /// OUT : from CPU to peripherals
    OUT,
    /// CLR : No I/O requested on IO bus
    CLR,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[doc(hidden)]
// The last requested IO OUT on IO bus. The interface between the emulator, its IO bus and your own code are
// the get_io_out, set_io_in and clear_io functions owned by the AddressBus struct.
pub struct PendingIO {
    pub kind: IO,
    pub device: u8,
    pub value: u8,
}

impl AddressBus {
    #[doc(hidden)]
    pub fn new() -> AddressBus {
        AddressBus {
            ram: vec![0; 65536],
            io_in: vec![0; 256],
            pending_io: PendingIO{
                kind: IO::CLR,
                device: 0,
                value: 0,
            }
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

    #[doc(hidden)]
    // Gets the "data bus" value put by the requested device. Only used by the IN instruction.
    pub fn get_io_in(&self, device: u8) -> u8 {
        self.io_in[usize::from(device)]
    }

    /// Sets a "data bus" value for the selected device, to be read by the IN instruction.
    pub fn set_io_in(&mut self, device: u8, value: u8) {
        self.io_in[usize::from(device)] = value;
    }

    #[doc(hidden)]
    // Sets next IO OUT PendingIO operation, for processing in outer code. Only used by the OUT instruction.
    pub fn set_io_out(&mut self, device: u8, value: u8) {
        self.pending_io.kind = IO::OUT;
        self.pending_io.device = device;
        self.pending_io.value = value;
    }

    /// Gets the "data bus" value put by an OUT instruction, for processing in your own code.
    pub fn get_io_out(&self, device: u8) -> Option<u8> {
        if self.pending_io.kind == IO::OUT && self.pending_io.device == device{ Some(self.pending_io.value) } else { None }
    }

    /// When done with handling of IO OUT, you should clear the pending operation in your own code with the clear_io_out() function.
    pub fn clear_io_out(&mut self,) {
        self.pending_io.kind = IO::CLR;
        self.pending_io.device = 0;
        self.pending_io.value = 0;
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