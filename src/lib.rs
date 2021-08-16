mod register;
mod flags;
mod memory;

use crate::register::Registers;
use crate::memory::AddressBus;
use crate::flags::Flags;

pub struct CPU {
    pub registers: Registers,
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
    pub bus: AddressBus
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
            bus: AddressBus::new(),
        }
    }

    // Increment functions
    pub fn inr(&self, n: u8) -> u8 {
        n.wrapping_add(1)
        // TODO : update of flags
    }

    // Decrement functions
    pub fn dcr(&self, n: u8) -> u8 {
        n.wrapping_sub(1)
        // TODO : update of flags
    }

    // fetches and executes instruction from (pc)
    pub fn execute(&mut self) {
        let opcode = self.bus.read_byte(self.pc);
        match opcode {
            /* Carry bit instructions */
            0x3f => self.flags.carry = !self.flags.carry,                   // CMC
            0x37 => self.flags.carry = true,                                // STC

            /* Single register instructions */
            // Increment Register or Memory
            0x04 => self.registers.b = self.inr(self.registers.b),          // INR B
            0x0C => self.registers.c = self.inr(self.registers.c),          // INR C
            0x14 => self.registers.d = self.inr(self.registers.d),          // INR D
            0x1C => self.registers.e = self.inr(self.registers.e),          // INR E
            0x24 => self.registers.h = self.inr(self.registers.h),          // INR H
            0x2C => self.registers.l = self.inr(self.registers.l),          // INR L
            0x3C => self.registers.a = self.inr(self.registers.a),          // INR A
            0x34 => {                                                       // INR (HL)
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.inr(self.bus.read_byte(addr)))
            },

            // Decrement Register or Memory
            0x05 => self.registers.b = self.dcr(self.registers.b),          // DCR B
            0x0D => self.registers.c = self.dcr(self.registers.c),          // DCR C
            0x15 => self.registers.d = self.dcr(self.registers.d),          // DCR D
            0x1D => self.registers.e = self.dcr(self.registers.e),          // DCR E
            0x25 => self.registers.h = self.dcr(self.registers.h),          // DCR H
            0x2D => self.registers.l = self.dcr(self.registers.l),          // DCR L
            0x3D => self.registers.a = self.dcr(self.registers.a),          // DCR A
            0x35 => {                                                       // DCR (HL)
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.dcr(self.bus.read_byte(addr)))
            },

            // Complement Accumulator
            0x2F => self.registers.a = !self.registers.a,                   // CMA

            // Decimal adjust accumulator
            // TODO : DAA

            // No Operation
            0x00 => {},                                                     // NOP

            // Data transfer instructions
            0x40 => {},                                                     // MOV B,B
            0x41 => self.registers.b = self.registers.c,                    // MOV B,C
            0x42 => self.registers.b = self.registers.d,                    // MOV B,D
            0x43 => self.registers.b = self.registers.e,                    // MOV B,E
            0x44 => self.registers.b = self.registers.h,                    // MOV B,H
            0x45 => self.registers.b = self.registers.l,                    // MOV B,L
            0x46 => {                                                       // MOV B,(HL)
                let addr = self.registers.get_hl();
                self.registers.b = self.bus.read_byte(addr)
            },
            0x47 => self.registers.b = self.registers.a,                    // MOV B,A

            0x48 => self.registers.c = self.registers.b,                    // MOV C,B                                                     // MOV B,B
            0x49 => {},                                                     // MOV C,C
            0x4A => self.registers.c = self.registers.d,                    // MOV C,D
            0x4B => self.registers.c = self.registers.e,                    // MOV C,E
            0x4C => self.registers.c = self.registers.h,                    // MOV C,H
            0x4D => self.registers.c = self.registers.l,                    // MOV C,L
            0x4E => {                                                       // MOV C,(HL)
                let addr = self.registers.get_hl();
                self.registers.c = self.bus.read_byte(addr)
            },
            0x4F => self.registers.c = self.registers.a,                    // MOV C,A

            0x50 => self.registers.d = self.registers.b,                    // MOV D,B                                                     // MOV B,B
            0x51 => self.registers.d = self.registers.b,                    // MOV D,C
            0x52 => {},                                                     // MOV D,D
            0x53 => self.registers.d = self.registers.e,                    // MOV D,E
            0x54 => self.registers.d = self.registers.h,                    // MOV D,H
            0x55 => self.registers.d = self.registers.l,                    // MOV D,L
            0x56 => {                                                       // MOV D,(HL)
                let addr = self.registers.get_hl();
                self.registers.d = self.bus.read_byte(addr)
            },
            0x57 => self.registers.d = self.registers.a,                    // MOV D,A

            0x58 => self.registers.e = self.registers.b,                    // MOV E,B                                                     // MOV B,B
            0x59 => self.registers.e = self.registers.c,                    // MOV E,C
            0x5A => self.registers.e = self.registers.d,                    // MOV E,D
            0x5B => {},                                                     // MOV E,E
            0x5C => self.registers.e = self.registers.h,                    // MOV E,H
            0x5D => self.registers.e = self.registers.l,                    // MOV E,L
            0x5E => {                                                       // MOV E,(HL)
                let addr = self.registers.get_hl();
                self.registers.e = self.bus.read_byte(addr)
            },
            0x5F => self.registers.e = self.registers.a,                    // MOV E,A

            0x60 => self.registers.h = self.registers.b,                    // MOV H,B                                                     // MOV B,B
            0x61 => self.registers.h = self.registers.c,                    // MOV H,C
            0x62 => self.registers.h = self.registers.b,                    // MOV H,D
            0x63 => self.registers.h = self.registers.e,                    // MOV H,E
            0x64 => {},                                                     // MOV H,H
            0x65 => self.registers.h = self.registers.l,                    // MOV H,L
            0x66 => {                                                       // MOV H,(HL)
                let addr = self.registers.get_hl();
                self.registers.h = self.bus.read_byte(addr)
            },
            0x67 => self.registers.h = self.registers.a,                    // MOV H,A

            0x68 => self.registers.l = self.registers.b,                    // MOV L,B                                                     // MOV B,B
            0x69 => self.registers.l = self.registers.c,                    // MOV L,C
            0x6A => self.registers.l = self.registers.d,                    // MOV L,D
            0x6B => self.registers.l = self.registers.e,                    // MOV L,E
            0x6C => self.registers.l = self.registers.h,                    // MOV L,H
            0x6D => {},                                                     // MOV L,L
            0x6E => {                                                       // MOV L,(HL)
                let addr = self.registers.get_hl();
                self.registers.l = self.bus.read_byte(addr)
            },
            0x6F => self.registers.l = self.registers.a,                    // MOV L,A

            0x70 => {                                                       // MOV (HL), B
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.b)
            },
            0x71 => {                                                       // MOV (HL), C
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.c)
            },
            0x72 => {                                                       // MOV (HL), D
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.d)
            },
            0x73 => {                                                       // MOV (HL), E
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.e)
            },
            0x74 => {                                                       // MOV (HL), H
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.h)
            },
            0x75 => {                                                       // MOV (HL), L
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.l)
            },

            0x76 => {/* TODO : HLT */},
            0x77 => {                                                       // MOV (HL), A
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.a)
            },

            0x78 => self.registers.a = self.registers.b,                    // MOV A,B                                                     // MOV B,B
            0x79 => self.registers.a = self.registers.c,                    // MOV A,C
            0x7A => self.registers.a = self.registers.b,                    // MOV A,D
            0x7B => self.registers.a = self.registers.e,                    // MOV A,E
            0x7C => self.registers.a = self.registers.h,                    // MOV A,H
            0x7D => self.registers.a = self.registers.l,                    // MOV A,L
            0x7E => {                                                       // MOV A,(HL)
                let addr = self.registers.get_hl();
                self.registers.a = self.bus.read_byte(addr)
            },
            0x7F => {},                                                     // MOV A,A

            0x02 => {
                let addr = self.registers.get_bc();                         // STAX B
                self.bus.write_byte(addr, self.registers.a)
            }
            0x12 => {                                                       // STAX D
                let addr = self.registers.get_de();
                self.bus.write_byte(addr, self.registers.a)
            },
            0x0A => {
                let addr = self.registers.get_bc();                         // LDAX B
                self.registers.a = self.bus.read_byte(addr)
            },
            0x1A => {                                                       // LDAX D
                let addr = self.registers.get_de();
                self.registers.a = self.bus.read_byte(addr)
            },

            /* Register or Memory to Accumulator instructions*/

            _ => {}
        }

        self.pc +=1;
    }
}

#[cfg(test)]
mod instructions {
    use super::*;
    #[test]
    fn complement_carry() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3f);
        c.execute();
        assert_eq!(true, c.flags.carry);
    }

    #[test]
    fn set_carry() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x37);
        c.execute();
        assert_eq!(true, c.flags.carry);
    }

    #[test]
    fn inr_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x04);
        c.execute();
        assert_eq!(1, c.registers.b);
        // TODO : test flags
    }

    #[test]
    fn dcr_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x05);
        c.execute();
        assert_eq!(255, c.registers.b);
        // TODO : test flags
    }

    #[test]
    fn cma() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2F);
        c.registers.a = 0b11001100;
        c.execute();
        assert_eq!(0b00110011, c.registers.a);
        
    }
}
