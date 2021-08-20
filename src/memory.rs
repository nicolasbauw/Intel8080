pub struct AddressBus {
    ram: Vec<u8>
}

impl AddressBus {
    pub fn new() -> AddressBus {
        AddressBus {
            ram: vec![0; 0xFFFF]
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[usize::from(address)]
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[usize::from(address)] = data;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.ram[usize::from(address)]) | (u16::from(self.ram[usize::from(address + 1)]) << 8)
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        self.ram[usize::from(address)] = (data & 0xFF) as u8;
        self.ram[usize::from(address + 1)] = (data >> 8) as u8;
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