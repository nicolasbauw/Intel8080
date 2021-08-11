mod register;

use crate::register::Registers;

pub struct CPU {
    registers: Registers,
    flags: Flags,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
        }
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
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
