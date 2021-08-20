mod register;
mod flags;
mod memory;
mod bit;

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
    fn inr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_add(1);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (n & 0x0f) + 0x01 > 0x0f;
        r
    }

    // Decrement functions
    fn dcr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_sub(1);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (n & 0x0f) + 0x01 > 0x0f;
        r
    }

    // ADD register or memory to Accumulator
    fn add(&mut self, n: u8) {
        let a = self.registers.a;
        let r = a.wrapping_add(n);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a & 0x0f) + (n & 0x0f) > 0x0f;
        self.flags.c = u16::from(a) + u16::from(n) > 0xff;
        self.registers.a = r;
    }

    // ADD register or memory to Accumulator with carry
    fn adc(&mut self, n: u8) {
        let c: u8 = match self.flags.c {
            false => 0,
            true => 1,
        };
        let a = self.registers.a;
        let r = a.wrapping_add(n).wrapping_add(c);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a & 0x0f) + (n & 0x0f) + c > 0x0f;
        self.flags.c = u16::from(a) + u16::from(n) + u16::from(c) > 0xff;
        self.registers.a = r;
    }

    // SUB register or memory from Accumulator
    fn sub(&mut self, n: u8) {
        let a = self.registers.a;
        let r = a.wrapping_sub(n);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a as i8 & 0x0f) - (n as i8 & 0x0f) >= 0x00;
        self.flags.c = u16::from(a) < u16::from(n);
        self.registers.a = r;
    }

    // SUB register or memory from Accumulator with borrow
    fn sbb(&mut self, n: u8) {
        let c: u8 = match self.flags.c {
            false => 0,
            true => 1,
        };
        let a = self.registers.a;
        let r = a.wrapping_sub(n.wrapping_add(c));
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a as i8 & 0x0f) - (n as i8 & 0x0f) - (c as i8) >= 0x00;
        self.flags.c = u16::from(a) < u16::from(n) + u16::from(c);
        self.registers.a = r;
    }

    // ANA Logical AND register or memory with accumulator
    fn ana(&mut self, n: u8) {
        let r = self.registers.a & n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.registers.a = r;
    }

    // XRA Logical Exclusive-OR register or memory with accumulator
    fn xra(&mut self, n: u8) {
        let a = self.registers.a;
        let r = a ^ n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.flags.a = (a & 0x0f) + (n & 0x0f) > 0x0f;
        self.registers.a = r;
    }

    // ORA Logical AND register or memory with accumulator
    fn ora(&mut self, n: u8) {
        let r = self.registers.a | n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.registers.a = r;
    }

    // CMP Compare register or memory with accumulator
    fn cmp(&mut self, n: u8) {
        let r = self.registers.a;
        self.sub(n);
        self.registers.a = r;
    }

    // Rotate accumulator left
    fn rlc(&mut self) {
        self.flags.c = bit::get(self.registers.a, 7);
        self.registers.a = (self.registers.a << 1) | u8::from(self.flags.c);
    }

    // Rotate accumulator right
    fn rrc(&mut self) {
        self.flags.c = bit::get(self.registers.a, 0);
        self.registers.a = if self.flags.c {0x80 | (self.registers.a >> 1) } else { self.registers.a >> 1 };
    }

    // RAL Rotate accumulator left through carry
    fn ral(&mut self) {
        let c = self.flags.c;
        self.flags.c = bit::get(self.registers.a, 7);
        self.registers.a = match c {
            true => (self.registers.a << 1) | 0x01,
            false => self.registers.a << 1
        }
    }
    
    // RAL Rotate accumulator right through carry
    fn rar(&mut self) {
        let c = self.flags.c;
        self.flags.c = bit::get(self.registers.a, 0);
        self.registers.a = match c {
            true => (self.registers.a >> 1) | 0x80,
            false => self.registers.a >> 1
        }
    }

    // DAD Double add
    fn dad(&mut self, n: u16) {
        let h = self.registers.get_hl();
        let r = h.wrapping_add(n);
        self.registers.set_hl(r);
        self.flags.c = u32::from(h) + u32::from(n) > 0xffff;
    }

    // XCHG Exchange registers
    fn xchg(&mut self) {
        let d = self.registers.d;
        let e = self.registers.e;
        let h = self.registers.h;
        let l = self.registers.l;
        self.registers.d = h;
        self.registers.e = l;
        self.registers.h = d;
        self.registers.l = e;
    }

    // XTHL Exchange stack
    fn xthl(&mut self) {
        let sph = self.bus.read_byte(self.sp + 1);
        let spl = self.bus.read_byte(self.sp);
        let h = self.registers.h;
        let l = self.registers.l;
        self.bus.write_byte(self.sp, l);
        self.bus.write_byte(self.sp + 1, h);
        self.registers.h = sph;
        self.registers.l = spl;
    }

    // subroutine stack push
    fn subroutine_stack_push(&mut self) {
        self.bus.write_word((self.sp) -2 , self.pc + 3);
        self.sp = self.sp - 2;
    }

    // fetches and executes instruction from (pc)
    pub fn execute(&mut self) {
        let opcode = self.bus.read_byte(self.pc);
        match opcode {
            /* Carry bit instructions */
            0x3f => self.flags.c = !self.flags.c,                           // CMC
            0x37 => self.flags.c = true,                                    // STC

            /* Single register instructions */
            // INR Increment Register or Memory
            0x04 => self.registers.b = self.inr(self.registers.b),          // INR B
            0x0C => self.registers.c = self.inr(self.registers.c),          // INR C
            0x14 => self.registers.d = self.inr(self.registers.d),          // INR D
            0x1C => self.registers.e = self.inr(self.registers.e),          // INR E
            0x24 => self.registers.h = self.inr(self.registers.h),          // INR H
            0x2C => self.registers.l = self.inr(self.registers.l),          // INR L
            0x3C => self.registers.a = self.inr(self.registers.a),          // INR A
            0x34 => {                                                       // INR (HL)
                let addr = self.registers.get_hl();
                let r = self.inr(self.bus.read_byte(addr));
                self.bus.write_byte(addr, r);
            },

            // DCR Decrement Register or Memory
            0x05 => self.registers.b = self.dcr(self.registers.b),          // DCR B
            0x0D => self.registers.c = self.dcr(self.registers.c),          // DCR C
            0x15 => self.registers.d = self.dcr(self.registers.d),          // DCR D
            0x1D => self.registers.e = self.dcr(self.registers.e),          // DCR E
            0x25 => self.registers.h = self.dcr(self.registers.h),          // DCR H
            0x2D => self.registers.l = self.dcr(self.registers.l),          // DCR L
            0x3D => self.registers.a = self.dcr(self.registers.a),          // DCR A
            0x35 => {                                                       // DCR (HL)
                let addr = self.registers.get_hl();
                let r = self.dcr(self.bus.read_byte(addr));
                self.bus.write_byte(addr, r);
            },

            // CMA Complement Accumulator
            0x2F => self.registers.a = !self.registers.a,                   // CMA

            // Decimal adjust accumulator
            // TODO : DAA

            // NOP No Operation
            0x00 => {},                                                     // NOP

            // MOV Data transfer instructions
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

            // STAX Store accumulator
            0x02 => {                                                       // STAX B
                let addr = self.registers.get_bc();
                self.bus.write_byte(addr, self.registers.a)
            }
            0x12 => {                                                       // STAX D
                let addr = self.registers.get_de();
                self.bus.write_byte(addr, self.registers.a)
            },

            // LDAX Load accumulator
            0x0A => {                                                       // LDAX B
                let addr = self.registers.get_bc();
                self.registers.a = self.bus.read_byte(addr)
            },
            0x1A => {                                                       // LDAX D
                let addr = self.registers.get_de();
                self.registers.a = self.bus.read_byte(addr)
            },

            /* Register or Memory to Accumulator instructions*/
            // ADD register or memory to accumulator
            0x80 => self.add(self.registers.b),                             // ADD B
            0x81 => self.add(self.registers.c),                             // ADD C
            0x82 => self.add(self.registers.d),                             // ADD D
            0x83 => self.add(self.registers.e),                             // ADD E
            0x84 => self.add(self.registers.h),                             // ADD H
            0x85 => self.add(self.registers.l),                             // ADD L
            0x86 => {                                                       // ADD (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.add(n)
            },
            0x87 => self.add(self.registers.a),                             // ADD A

            // ADC Add register or memory to accumulator with carry
            0x88 => self.adc(self.registers.b),                             // ADC B
            0x89 => self.adc(self.registers.c),                             // ADC C
            0x8A => self.adc(self.registers.d),                             // ADC D
            0x8B => self.adc(self.registers.e),                             // ADC E
            0x8C => self.adc(self.registers.h),                             // ADC H
            0x8D => self.adc(self.registers.l),                             // ADC L
            0x8E => {                                                       // ADC (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.adc(n)
            },
            0x8F => self.adc(self.registers.a),                             // ADC A

            // SUB Substract register or memory to accumulator
            0x90 => self.sub(self.registers.b),                             // SUB B
            0x91 => self.sub(self.registers.c),                             // SUB C
            0x92 => self.sub(self.registers.d),                             // SUB D
            0x93 => self.sub(self.registers.e),                             // SUB E
            0x94 => self.sub(self.registers.h),                             // SUB H
            0x95 => self.sub(self.registers.l),                             // SUB L
            0x96 => {                                                       // SUB (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.sub(n)
            },
            0x97 => self.sub(self.registers.a),                             // SUB A

            // SBB Substract register or memory to accumulator with borrow
            0x98 => self.sbb(self.registers.b),                             // SBB B
            0x99 => self.sbb(self.registers.c),                             // SBB C
            0x9A => self.sbb(self.registers.d),                             // SBB D
            0x9B => self.sbb(self.registers.e),                             // SBB E
            0x9C => self.sbb(self.registers.h),                             // SBB H
            0x9D => self.sbb(self.registers.l),                             // SBB L
            0x9E => {                                                       // SBB (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.sbb(n)
            },
            0x9F => self.sbb(self.registers.a),                             // SBB A

            // ANA Logical AND register or memory with accumulator
            0xA0 => self.ana(self.registers.b),                             // ANA B
            0xA1 => self.ana(self.registers.c),                             // ANA C
            0xA2 => self.ana(self.registers.d),                             // ANA D
            0xA3 => self.ana(self.registers.e),                             // ANA E
            0xA4 => self.ana(self.registers.h),                             // ANA H
            0xA5 => self.ana(self.registers.l),                             // ANA L
            0xA6 => {                                                       // ANA (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.ana(n)
            },
            0xA7 => self.ana(self.registers.a),                             // ANA A

            // XRA Logical Exclusive-OR register or memory with accumulator
            0xA8 => self.xra(self.registers.b),                             // XRA B
            0xA9 => self.xra(self.registers.c),                             // XRA C
            0xAA => self.xra(self.registers.d),                             // XRA D
            0xAB => self.xra(self.registers.e),                             // XRA E
            0xAC => self.xra(self.registers.h),                             // XRA H
            0xAD => self.xra(self.registers.l),                             // XRA L
            0xAE => {                                                       // ANA (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.xra(n)
            },
            0xAF => self.xra(self.registers.a),                             // XRA A

            // ORA Logical OR register or memory with accumulator
            0xB0 => self.ora(self.registers.b),                             // ORA B
            0xB1 => self.ora(self.registers.c),                             // ORA C
            0xB2 => self.ora(self.registers.d),                             // ORA D
            0xB3 => self.ora(self.registers.e),                             // ORA E
            0xB4 => self.ora(self.registers.h),                             // ORA H
            0xB5 => self.ora(self.registers.l),                             // ORA L
            0xB6 => {                                                       // ORA (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.ora(n)
            },
            0xB7 => self.ora(self.registers.a),                             // ORA A

            // CMP Compare register or memory with accumulator
            0xB8 => self.cmp(self.registers.b),                             // CMP B
            0xB9 => self.cmp(self.registers.c),                             // CMP C
            0xBA => self.cmp(self.registers.d),                             // CMP D
            0xBB => self.cmp(self.registers.e),                             // CMP E
            0xBC => self.cmp(self.registers.h),                             // CMP H
            0xBD => self.cmp(self.registers.l),                             // CMP L
            0xBE => {                                                       // CMP (HL)
                let addr = self.registers.get_hl();
                let n = self.bus.read_byte(addr);
                self.cmp(n)
            },
            0xBF => self.cmp(self.registers.a),                             // CMP A

            /* Rotate accumulator instructions */
            0x07 => self.rlc(),                                             // RLC
            0x0F => self.rrc(),                                             // RRC
            0x17 => self.ral(),                                             // RAL
            0x1F => self.rar(),                                             // RAR

            /* Register pair instructions */
            // PUSH data onto stack
            0xC5 => {                                                       // PUSH B
                self.bus.write_byte((self.sp) - 1, self.registers.b);
                self.bus.write_byte((self.sp) - 2, self.registers.c);
                self.sp = self.sp - 2;
            },
            0xD5 => {                                                       // PUSH D
                self.bus.write_byte((self.sp) - 1, self.registers.d);
                self.bus.write_byte((self.sp) - 2, self.registers.e);
                self.sp = self.sp - 2;
            },
            0xE5 => {                                                       // PUSH H
                self.bus.write_byte((self.sp) - 1, self.registers.h);
                self.bus.write_byte((self.sp) - 2, self.registers.l);
                self.sp = self.sp - 2;
            },
            0xF5 => {                                                       // PUSH PSW
                self.bus.write_byte((self.sp) - 1, self.registers.a);
                self.bus.write_byte((self.sp) - 2, self.flags.as_byte());
                self.sp = self.sp - 2;
            },

            // POP data off stack
            0xC1 => {                                                       // POP B
                self.registers.b = self.bus.read_byte((self.sp)+1);
                self.registers.c = self.bus.read_byte(self.sp);
                self.sp += 2;
            },

            0xD1 => {                                                       // POP D
                self.registers.d = self.bus.read_byte((self.sp)+1);
                self.registers.e = self.bus.read_byte(self.sp);
                self.sp += 2;
            },

            0xE1 => {                                                       // POP H
                self.registers.h = self.bus.read_byte((self.sp)+1);
                self.registers.l = self.bus.read_byte(self.sp);
                self.sp += 2;
            },

            0xF1 => {                                                       // POP PSW
                self.registers.a = self.bus.read_byte((self.sp)+1);
                let bflags = self.bus.read_byte(self.sp);
                self.flags.from_byte(bflags);
                self.sp += 2;
            },

            // DAD Double add
            0x09 => self.dad(self.registers.get_bc()),                      // DAD B
            0x19 => self.dad(self.registers.get_de()),                      // DAD D
            0x29 => self.dad(self.registers.get_hl()),                      // DAD H
            0x39 => self.dad(self.sp),                                      // DAD SP

            // INX Increment register pair
            0x03 => {                                                       // INX B
                let mut b = self.registers.get_bc();
                b = b.wrapping_add(1);
                self.registers.set_bc(b);
            },

            0x13 => {                                                       // INX D
                let mut d = self.registers.get_de();
                d = d.wrapping_add(1);
                self.registers.set_bc(d);
            },

            0x23 => {                                                       // INX H
                let mut h = self.registers.get_hl();
                h = h.wrapping_add(1);
                self.registers.set_bc(h);
            }

            0x33 => self.sp = self.sp.wrapping_add(1),                      // INX SP

            // DCX Decrement register pair
            0x0B => {                                                       // DCX B
                let mut b = self.registers.get_bc();
                b = b.wrapping_sub(1);
                self.registers.set_bc(b);
            },

            0x1B => {                                                       // DCX D
                let mut d = self.registers.get_de();
                d = d.wrapping_sub(1);
                self.registers.set_bc(d);
            },

            0x2B => {                                                       // DCX H
                let mut h = self.registers.get_hl();
                h = h.wrapping_sub(1);
                self.registers.set_bc(h);
            }

            0x3B => self.sp = self.sp.wrapping_sub(1),                      // DCX SP

            // XCHG Exchange registers
            0xEB => self.xchg(),

            // XTHL Exchange stack
            0xE3 => self.xthl(),

            /* Immediate instructions */
            // MVI Move immediate data
            0x06 => {                                                       // MVI B,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.b = d8;
            },
            0x0E => {                                                       // MVI C,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.c = d8;
            },
            0x16 => {                                                       // MVI D,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.d = d8;
            },
            0x1E => {                                                       // MVI E,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.e = d8;
            },
            0x26 => {                                                       // MVI H,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.h = d8;
            },
            0x2E => {                                                       // MVI L,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.l = d8;
            },
            0x36 => {                                                       // MVI (HL),d8
                let d8 = self.bus.read_byte(self.pc + 1);
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, d8);
            },
            0x3E => {                                                       // MVI A,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.registers.a = d8;
            },

            // ADI add immediate to accumulator
            0xC6 => {                                                       // ADI
                let n = self.bus.read_byte(self.pc + 1);
                self.add(n);
            },

            // ACI add immediate to accumulator with carry
            0xCE => {                                                       // ACI
                let n = self.bus.read_byte(self.pc + 1);
                self.adc(n);
            },

            // SUI substract immediate from accumulator
            0xD6 => {                                                       // SUI
                let n = self.bus.read_byte(self.pc + 1);
                self.sub(n);
            },

            // SBI substract immediate from accumulator with borrow
            0xDE => {                                                       // SBI
                let n = self.bus.read_byte(self.pc + 1);
                self.sbb(n);
            },

            // ANI and immediate with accumulator
            0xE6 => {                                                       // ANI
                let n = self.bus.read_byte(self.pc + 1);
                self.ana(n);
            },

            // XRI exclusive-or immediate with accumulator
            0xEE => {                                                       // XRI
                let n = self.bus.read_byte(self.pc + 1);
                self.xra(n);
            },

            // ORI or immediate with accumulator
            0xF6 => {                                                       // ORI
                let n = self.bus.read_byte(self.pc + 1);
                self.ora(n);
            },

            // CPI compare immediate with accumulator
            0xFE => {                                                       // CPI
                let n = self.bus.read_byte(self.pc + 1);
                self.cmp(n);
            },

            /* Direct addressing instructions */
            // STA Store accumulator direct
            0x32 => {                                                       // STA
                let addr = self.bus.read_word(self.pc + 1);
                self.bus.write_byte(addr, self.registers.a);
            },

            // LDA Store accumulator direct
            0x3A => {                                                       // LDA
                let addr = self.bus.read_word(self.pc + 1);
                self.registers.a = self.bus.read_byte(addr);
            },

            // SHLD Store H and L direct
            0x22 => {                                                       // SHLD
                let d = self.registers.get_hl();
                let addr = self.bus.read_word(self.pc + 1);
                self.bus.write_word(addr, d);
            },

            // LHLD Store H and L direct
            0x2A => {                                                       // LHLD
                let addr = self.bus.read_word(self.pc + 1);
                let d = self.bus.read_word(addr);
                self.registers.set_hl(d);
            },

            /* JUMP instructions */
            // Load program counter
            0xE9 => { self.pc = self.registers.get_hl(); },                 // PCHL
            // JMP Jump
            0xC3 => {                                                       // JMP
                let addr = self.bus.read_word(self.pc + 1);
                self.pc = addr;
            },
            // JC Jump if carry
            0xDA => {                                                       // JC
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.c { self.pc = addr; }
            },
            // JNC Jump if no carry
            0xD2 => {                                                       // JNC
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.c { self.pc = addr; }
            },
            // JZ Jump if zero
            0xCA => {                                                       // JZ
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.z { self.pc = addr; }
            },
            // JNZ Jump if not zero
            0xC2 => {                                                       // JNZ
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.z { self.pc = addr; }
            },
            // JM Jump if minus
            0xFA => {                                                       // JM
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.s { self.pc = addr; }
            },
            // JP Jump if positive
            0xF2 => {                                                       // JP
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.s { self.pc = addr; }
            },
            // JPE Jump if parity even
            0xEA => {                                                       // JPE
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.p { self.pc = addr; }
            },
            // JPO Jump if parity odd
            0xE2 => {                                                       // JPO
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.p { self.pc = addr; }
            },

            /* Call subroutine instructions */
            // CALL
            0xCD => {                                                       // CALL
                let addr = self.bus.read_word(self.pc + 1);
                self.subroutine_stack_push();
                self.pc = addr;
            },
            // CC Call if carry
            0xDC => {                                                       // CC
                let addr = self.bus.read_word(self.pc + 1);
                self.subroutine_stack_push();
                if self.flags.c { self.pc = addr; }
            },
            // CNC Call if no carry
            0xD4 => {                                                       // CNC
                let addr = self.bus.read_word(self.pc + 1);
                self.subroutine_stack_push();
                if !self.flags.c { self.pc = addr; }
            },


            _ => {}
        }

        match opcode {
            0xe9 | 0xc3 | 0xDA | 0xD2 | 0xCA | 0xC2 | 0xFA | 0xF2 | 0xEA | 0xE2 |
            0xCD => {},
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E |
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => self.pc += 2,
            0x32 | 0x3A | 0x22 | 0x2A => self.pc += 3,
            _ => self.pc +=1,
        }
    }
}

#[cfg(test)]
mod instructions {
    use super::*;
    #[test]
    fn cmc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3f);
        c.execute();
        assert_eq!(true, c.flags.c);
    }

    #[test]
    fn stc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x37);
        c.execute();
        assert_eq!(true, c.flags.c);
    }

    #[test]
    fn inr() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x04);
        c.execute();
        assert_eq!(1, c.registers.b);
        // TODO : test flags
    }

    #[test]
    fn dcr() {
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

    #[test]
    fn add() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x82);
        c.registers.a = 0x6C;
        c.registers.d = 0x2E;
        c.execute();
        assert_eq!(0x9A, c.registers.a);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.s, true);
        assert_eq!(c.flags.a, true);
    }

    #[test]
    fn adc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x89);
        c.registers.a = 0x42;
        c.registers.c = 0x3D;
        c.execute();
        assert_eq!(0x7F, c.registers.a);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.flags.p, false);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.a, false);
    }

    #[test]
    fn sub() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x97);
        c.registers.a = 0x3E;
        c.execute();
        assert_eq!(0x00, c.registers.a);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.a, true);
    }

    #[test]
    fn sbb() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9D);
        c.registers.a = 0x04;
        c.flags.c = true;
        c.registers.l = 0x02;
        c.execute();
        assert_eq!(0x01, c.registers.a);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.flags.p, false);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.a, true);
    }

    #[test]
    fn ana() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xA1);
        c.registers.a = 0xFC;
        c.registers.c = 0x0F;
        c.execute();
        assert_eq!(0x0C, c.registers.a);
    }

    #[test]
    fn ora() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xB1);
        c.registers.a = 0x33;
        c.registers.c = 0x0F;
        c.execute();
        assert_eq!(0x3F, c.registers.a);
    }

    #[test]
    fn cmp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xBB);
        c.registers.a = 0x0A;
        c.registers.e = 0x05;
        c.execute();
        assert_eq!(0x0A, c.registers.a);
        assert_eq!(0x05, c.registers.e);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn rlc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x07);
        c.registers.a = 0xF2;
        c.execute();
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0xE5);
    }

    #[test]
    fn rrc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0F);
        c.registers.a = 0xF2;
        c.execute();
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x79);
    }

    #[test]
    fn ral() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x17);
        c.registers.a = 0xB5;
        c.execute();
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x6A);
    }

    #[test]
    fn rar() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1F);
        c.registers.a = 0x6A;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0xB5);
    }

    #[test]
    fn push() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xD5);
        c.registers.d = 0x8F;
        c.registers.e = 0x9D;
        c.sp = 0x3A2C;
        c.execute();
        assert_eq!(c.sp, 0x3A2A);
        assert_eq!(c.bus.read_byte(0x3A2B), 0x8F);
        assert_eq!(c.bus.read_byte(0x3A2A), 0x9D);
    }

    #[test]
    fn push_psw() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xF5);
        c.registers.a = 0x1F;
        c.flags.c = true;
        c.flags.z = true;
        c.flags.p = true;
        c.flags.s = false;
        c.flags.a = false;
        c.sp = 0x502A;
        c.execute();
        assert_eq!(c.sp, 0x5028);
        assert_eq!(c.bus.read_byte(0x5029), 0x1F);
        assert_eq!(c.bus.read_byte(0x5028), 0x47);
    }

    #[test]
    fn pop() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xE1);
        c.bus.write_byte(0x1239, 0x3D);
        c.bus.write_byte(0x123A, 0x93);
        c.sp = 0x1239;
        c.execute();
        assert_eq!(c.sp, 0x123B);
        assert_eq!(c.registers.l, 0x3D);
        assert_eq!(c.registers.h, 0x93);
    }

    #[test]
    fn pop_psw() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xF1);
        c.bus.write_byte(0x2C00, 0xC3);
        c.bus.write_byte(0x2C01, 0xFF);
        c.sp = 0x2C00;
        c.execute();
        assert_eq!(c.registers.a, 0xFF);
        assert_eq!(c.flags.s, true);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.p, false);
    }

    #[test]
    fn dad() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x09);
        c.registers.set_bc(0x339F);
        c.registers.set_hl(0xA17B);
        c.execute();
        assert_eq!(c.registers.h, 0xD5);
        assert_eq!(c.registers.l, 0x1A);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn inx() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x03);
        c.registers.set_bc(0xFFFF);
        c.execute();
        assert_eq!(c.registers.get_bc(), 0x00);
    }

    #[test]
    fn dcx() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0b);
        c.sp = 0xFFFF;
        c.execute();
        assert_eq!(c.sp, 0xFFFF);
    }

    #[test]
    fn xchg() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xeb);
        c.registers.set_de(0x3355);
        c.registers.set_hl(0x00FF);
        c.execute();
        assert_eq!(c.registers.get_de(), 0x00FF);
        assert_eq!(c.registers.get_hl(), 0x3355);
    }

    #[test]
    fn xthl() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xe3);
        c.sp = 0x10AD;
        c.registers.set_hl(0x0B3C);
        c.bus.write_byte(0x10ad, 0xF0);
        c.bus.write_byte(0x10ae, 0x0d);
        c.execute();
        assert_eq!(c.registers.get_hl(), 0x0df0);
        assert_eq!(c.bus.read_byte(0x10ad), 0x3c);
        assert_eq!(c.bus.read_byte(0x10ae), 0x0b);
    }

    #[test]
    fn mvi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3e);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.registers.a, 0x88);
    }

    #[test]
    fn adi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xc6);
        c.bus.write_byte(0x0001, 0x42);
        c.registers.a = 0x14;
        c.execute();
        assert_eq!(c.registers.a, 0x56);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn aci() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xce);
        c.bus.write_byte(0x0001, 0xbe);
        c.bus.write_byte(0x0002, 0xce);
        c.bus.write_byte(0x0003, 0x42);
        c.registers.a = 0x56;
        c.flags.c = false;
        c.execute();
        c.execute();
        assert_eq!(c.registers.a, 0x57);
        assert_eq!(c.flags.p, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sui() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xd6);
        c.bus.write_byte(0x0001, 0x01);
        c.registers.a = 0x00;
        c.execute();
        assert_eq!(c.registers.a, 0xFF);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.s, true);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xaf);
        c.bus.write_byte(0x0001, 0xde);
        c.bus.write_byte(0x0002, 0x01);
        c.execute();
        c.execute();
        assert_eq!(c.registers.a, 0xFF);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.s, true);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn ani() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x79);
        c.bus.write_byte(0x0001, 0xe6);
        c.bus.write_byte(0x0002, 0x0f);
        c.registers.c = 0x3a;
        c.execute();
        c.execute();
        assert_eq!(c.registers.a, 0x0a);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.s, false);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn xri() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xee);
        c.bus.write_byte(0x0001, 0x81);
        c.registers.a = 0x3b;
        c.execute();
        assert_eq!(c.registers.a, 0b1011_1010);
    }

    #[test]
    fn ori() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x79);
        c.bus.write_byte(0x0001, 0xf6);
        c.bus.write_byte(0x0002, 0x0f);
        c.registers.c = 0xb5;
        c.execute();
        c.execute();
        assert_eq!(c.registers.a, 0xbf);
    }

    #[test]
    fn cpi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3e);
        c.bus.write_byte(0x0001, 0x4a);
        c.bus.write_byte(0x0002, 0xfe);
        c.bus.write_byte(0x0003, 0x40);
        c.execute();
        c.execute();
        assert_eq!(c.flags.c, false);
        assert_eq!(c.flags.z, false);
    }

    #[test]
    fn shld() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x22);
        c.bus.write_byte(0x0001, 0x0a);
        c.bus.write_byte(0x0002, 0x01);
        c.registers.set_hl(0xae29);
        c.execute();
        assert_eq!(c.bus.read_word(0x010a), 0xae29);
    }

    #[test]
    fn lhld() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2a);
        c.bus.write_byte(0x0001, 0x5b);
        c.bus.write_byte(0x0002, 0x02);
        c.bus.write_byte(0x025b, 0xff);
        c.bus.write_byte(0x025c, 0x03);
        c.execute();
        assert_eq!(c.registers.l, 0xff);
        assert_eq!(c.registers.h, 0x03);
    }

    #[test]
    fn pchl() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xe9);
        c.registers.h = 0x41;
        c.registers.l = 0x3e;
        c.execute();
        assert_eq!(c.pc, 0x413e);
    }

    #[test]
    fn jmp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xc3);
        c.bus.write_byte(0x0001, 0x00);
        c.bus.write_byte(0x0002, 0x3e);
        c.execute();
        assert_eq!(c.pc, 0x3e00);
    }
}
