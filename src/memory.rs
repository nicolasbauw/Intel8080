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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_write_byte() {
        let mut b = AddressBus::new();
        b.write_byte(0x0000, 0xFF);
        assert_eq!(b.read_byte(0x0000), 0xFF);
    }
}