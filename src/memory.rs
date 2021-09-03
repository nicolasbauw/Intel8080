use std::{fs::File, io::prelude::*,};

/// The AddressBus struct is hosting the 8080 memory map and the pending IO operations for outer handling.
pub struct AddressBus {
    ram: Vec<u8>,
    pending_io : PendingIO,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Is the requested IO an input, output, or no IO requested ?
pub enum IO {
    /// IN : from peripherals to the CPU
    IN,
    /// OUT : from CPU to peripherals
    OUT,
    /// CLR : No I/O requested on IO bus
    CLR,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// The last requested IO on IO bus. The interface between the emulator, its IO bus and your own code are
/// the get_io, set_io and clear_io functions owned by the AddressBus struct.
pub struct PendingIO {
    pub kind: IO,
    pub device: u8,
    pub value: u8,
}

impl AddressBus {
    #[doc(hidden)]
    pub fn new() -> AddressBus {
        AddressBus {
            ram: vec![0; 0xFFFF],
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

    /// Gets the pending IO operation
    /// ```rust
    /// # use intel8080::{CPU, memory::*};
    /// let mut c = CPU::new();
    /// c.bus.write_byte(0x0100, 0x3e);     // MVI A,$55
    /// c.bus.write_byte(0x0101, 0x55);
    /// c.bus.write_byte(0x0102, 0xd3);     // OUT 0
    /// c.bus.write_byte(0x0103, 0x00);
    /// loop {
    ///    c.execute();
    ///     let io = c.bus.get_io();
    ///     // you want this part of your code to handle output from CPU to device 0
    ///     if io.kind == IO::OUT && io.device == 0 {
    ///         /* handle your IO then clear pending IO */
    ///         c.bus.clear_io();
    ///         break;
    ///     }
    /// }
    /// ```
    /// IMPORTANT : after a OUT instruction execution, once you have handled the data,
    /// you must call clear_io in your own code.
    pub fn get_io(&self) -> PendingIO {
        self.pending_io.clone()
    }

    /// Sets next IO operation
    /// ```rust
    /// # use intel8080::{CPU, memory::*};
    /// let mut c = CPU::new();
    /// c.bus.write_byte(0x0000, 0xdb);     // IN 0
    /// c.bus.write_byte(0x0001, 0x00);
    /// c.bus.set_io(IO::IN, 0, 0x55);      // device 0 puts 0x55 on "data bus" (output for peripheral, input for the CPU, hence the IO::IN)
    /// c.execute();                        // the CPU executes the IN instruction, so accumulator equals input data 0x55
    /// assert_eq!(c.registers.a, 0x55);
    /// ```
    /// IMPORTANT : after a IN instruction execution, a clear_io is done automatically.
    pub fn set_io(&mut self, kind: IO, device: u8, value: u8) {
        self.pending_io.kind = kind;
        self.pending_io.device = device;
        self.pending_io.value = value;
    }

    /// When done with IO handling, you should clear the pending operation in your own code
    /// ```rust
    /// # use intel8080::{CPU, memory::*};
    /// # let mut c = CPU::new();
    /// c.bus.clear_io();
    /// ```
    pub fn clear_io(&mut self,) {
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