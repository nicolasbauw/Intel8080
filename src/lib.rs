mod register;

use crate::register::Registers;

pub struct CPU {
    registers: Registers,
    flags: Flags,
    pc: u16,
    sp: u16,
    mem: Vec<u8>
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
            mem: vec![0; 0xFFFF]
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.mem[usize::from(self.pc)];
        match opcode {
            0x3f => self.flags.carry = !self.flags.carry,                   // CMC
            0x37 => self.flags.carry = true,                                // STC
            _ => {}
        }

        self.pc +=1;
    }
}

pub struct Flags {
    pub sign: bool,                 // bit 7
    pub zero: bool,                 // bit 6
    pub auxiliary_carry: bool,      // bit 4
    pub parity: bool,               // bit 2
    pub carry: bool                 // bit 0
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            sign: false,
            zero: false,
            auxiliary_carry: false,
            parity: false,
            carry: false
        }
    }
}

#[cfg(test)]
mod instructions {
    use super::*;
    #[test]
    fn complement_carry() {
        let mut c = CPU::new();
        c.mem[0] = 0x3f;
        c.execute();
        assert_eq!(true, c.flags.carry);
    }

    #[test]
    fn set_carry() {
        let mut c = CPU::new();
        c.mem[0] = 0x37;
        c.execute();
        assert_eq!(true, c.flags.carry);
    }
}
