//! Yet another Intel 8080 Emulator. It passes the TST8080, 8080PRE, CPUTEST and 8080EXM tests.
//! 
//! Example for a small loop:
//! ```rust
//! use intel8080::CPU;
//! let mut c = CPU::new();
//! c.pc = 0x0100;                      // sets pc to $100
//! // Here we create a small machine code program for demo purpose.
//! // Usually you will rather load an assembled code in memory (see below).
//! c.bus.write_byte(0x0100, 0x3e);     // MVI A,$0F
//! c.bus.write_byte(0x0101, 0x0F);
//! c.bus.write_byte(0x0102, 0x3d);     // DCR A
//! c.bus.write_byte(0x0103, 0xc2);     // JNZ $0102
//! c.bus.write_word(0x0104, 0x0102);
//! c.bus.write_byte(0x0106, 0xc9);     // RET
//! loop {
//!     c.execute();
//!     if c.pc == 0x0000 { break }
//! }
//! ```
//! 
//! Debug mode outputs CPU state and disassembled code to an internal string after each execute():
//! ```text
//! 3E 0f     MVI A,$0f
//! PC : 0x0003	SP : 0xff00	S : 0	Z : 0	A : 0	P : 0	C : 0
//! B : 0x00	C : 0x00	D : 0x00	E : 0x00	H : 0x00	L : 0x00 ...
//! ```
//! 
//! Includes a "cpmloader" which loads and executes basic CP/M programs:
//! 
//! ```text
//! cargo run --release --example cpmloader -- bin/helloworld.bin
//! ```
//! 
//! You can also check my [Altair 8800 / 88-SIO / teletype emulator](https://github.com/nicolasbauw/Altair8800).
//! 
//! The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).
//! 

#[doc(hidden)]
pub mod register;
pub mod memory;
mod flags;
mod bit;
mod dasm;
#[cfg(test)]
mod tests;

use crate::register::Registers;
use crate::memory::Bus;
use crate::flags::Flags;
use std::time::SystemTime;

const CYCLES: [u8; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4,
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4 ,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    5, 10, 10, 10, 11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11,
    5, 10, 10, 10, 11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11,
    5, 10, 10, 18, 11, 11, 7, 11, 5, 5, 10, 5, 11, 17, 7, 11,
    5, 10, 10, 4, 11, 11, 7, 11, 5, 5, 10, 4, 11, 17, 7, 11,
];

pub struct Debug {
    /// Enables / Disables the debug string generation
    pub switch: bool,
    /// Debug for IO messages
    pub io: bool,
    /// The debug information string
    pub string: String,
}

pub struct CPU {
    pub reg: Registers,
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
    pub bus: Bus,
    pub halt: bool,
    /// Interrupt request : true / false, instruction to execute (normally a RST command)
    pub int: (bool, u8),
    /// Interrupt enable bit
    pub inte: bool,
    /// Outputs CPU state and disassembled code to stdout after each execute()
    /// ```text
    /// 3E 0f     MVI A,$0f
    /// PC : 0x0003	SP : 0xff00	S : 0	Z : 0	A : 0	P : 0	C : 0
    /// B : 0x00	C : 0x00	D : 0x00	E : 0x00	H : 0x00	L : 0x00 ...
    /// ```
    pub debug: Debug,
    // Defaults to 1/60FPS = 16ms
    slice_duration: u32,
    // Defaults to 35000 cycles per 16ms slice (2.1 Mhz).
    // cycles = clock speed in Hz / required frames-per-second
    slice_max_cycles: u32,
    slice_current_cycles: u32,
    slice_start_time: SystemTime,
}

impl Debug {
    pub fn new() -> Debug {
        Debug {
            switch: false,
            io: false,
            string: String::new(),
        }
    }
}

impl CPU {
    /// Creates a new CPU instance and its 16 bits address bus.
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
            bus: Bus::new(),
            halt: false,
            int: (false, 0),
            inte: false,
            debug: Debug::new(),
            slice_duration: 16,
            slice_max_cycles: 35000,
            slice_current_cycles: 0,
            slice_start_time: SystemTime::now(),
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
        self.flags.a = (r & 0x0f) != 0x0f;
        r
    }

    // ADD register or memory to Accumulator
    fn add(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a.wrapping_add(n);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a & 0x0f) + (n & 0x0f) > 0x0f;
        self.flags.c = u16::from(a) + u16::from(n) > 0xff;
        self.reg.a = r;
    }

    // ADD register or memory to Accumulator with carry
    fn adc(&mut self, n: u8) {
        let c: u8 = match self.flags.c {
            false => 0,
            true => 1,
        };
        let a = self.reg.a;
        let r = a.wrapping_add(n).wrapping_add(c);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a & 0x0f) + (n & 0x0f) + c > 0x0f;
        self.flags.c = u16::from(a) + u16::from(n) + u16::from(c) > 0xff;
        self.reg.a = r;
    }

    // SUB register or memory from Accumulator
    fn sub(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a.wrapping_sub(n);
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a as i8 & 0x0f) - (n as i8 & 0x0f) >= 0x00;
        self.flags.c = u16::from(a) < u16::from(n);
        self.reg.a = r;
    }

    // SBB register or memory from Accumulator with borrow
    fn sbb(&mut self, n: u8) {
        let c: u8 = match self.flags.c {
            false => 0,
            true => 1,
        };
        let a = self.reg.a;
        let r = a.wrapping_sub(n.wrapping_add(c));
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (a as i8 & 0x0f) - (n as i8 & 0x0f) - (c as i8) >= 0x00;
        self.flags.c = u16::from(a) < u16::from(n) + u16::from(c);
        self.reg.a = r;
    }

    // ANA Logical AND register or memory with accumulator
    fn ana(&mut self, n: u8) {
        let r = self.reg.a & n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.a = (n | self.reg.a) & 0x08 != 0;
        self.flags.c = false;
        self.reg.a = r;
    }

    // XRA Logical Exclusive-OR register or memory with accumulator
    fn xra(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a ^ n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.flags.a = false;
        self.reg.a = r;
    }

    // ORA Logical AND register or memory with accumulator
    fn ora(&mut self, n: u8) {
        let r = self.reg.a | n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.flags.a = false;
        self.reg.a = r;
    }

    // CMP Compare register or memory with accumulator
    fn cmp(&mut self, n: u8) {
        let r = self.reg.a;
        self.sub(n);
        self.reg.a = r;
    }

    // Rotate accumulator left
    fn rlc(&mut self) {
        self.flags.c = bit::get(self.reg.a, 7);
        self.reg.a = (self.reg.a << 1) | u8::from(self.flags.c);
    }

    // Rotate accumulator right
    fn rrc(&mut self) {
        self.flags.c = bit::get(self.reg.a, 0);
        self.reg.a = if self.flags.c {0x80 | (self.reg.a >> 1) } else { self.reg.a >> 1 };
    }

    // RAL Rotate accumulator left through carry
    fn ral(&mut self) {
        let c = self.flags.c;
        self.flags.c = bit::get(self.reg.a, 7);
        self.reg.a = match c {
            true => (self.reg.a << 1) | 0x01,
            false => self.reg.a << 1
        }
    }
    
    // RAR Rotate accumulator right through carry
    fn rar(&mut self) {
        let c = self.flags.c;
        self.flags.c = bit::get(self.reg.a, 0);
        self.reg.a = match c {
            true => (self.reg.a >> 1) | 0x80,
            false => self.reg.a >> 1
        }
    }

    // DAD Double add
    fn dad(&mut self, n: u16) {
        let h = self.reg.get_hl();
        let r = h.wrapping_add(n);
        self.reg.set_hl(r);
        self.flags.c = u32::from(h) + u32::from(n) > 0xffff;
    }

    // XCHG Exchange reg
    fn xchg(&mut self) {
        let d = self.reg.d;
        let e = self.reg.e;
        let h = self.reg.h;
        let l = self.reg.l;
        self.reg.d = h;
        self.reg.e = l;
        self.reg.h = d;
        self.reg.l = e;
    }

    // XTHL Exchange stack
    fn xthl(&mut self) {
        let pointed_by_sp = self.bus.read_word(self.sp);
        let hl = self.reg.get_hl();
        self.bus.write_word(self.sp, hl);
        self.reg.set_hl(pointed_by_sp);
    }

    // Decimal adjust accumulator
    fn daa(&mut self) {
        let mut inc_a: u8 = 0;
        let mut c = self.flags.c;
        let lsb = self.reg.a & 0x0F;
        if (lsb > 9) || self.flags.a {
            inc_a += 0x06;
        }

        let msb = self.reg.a >> 4;
        if (msb > 9) || self.flags.c || (msb >= 9 && lsb > 9) {
            inc_a += 0x60;
            c = true;
        }

        self.add(inc_a);
        self.flags.c = c;
        self.flags.z = self.reg.a == 0x00;
        self.flags.s = bit::get(self.reg.a, 7);
        self.flags.p = self.reg.a.count_ones() & 0x01 == 0x00;
    }

    // subroutine stack push
    fn subroutine_stack_push(&mut self) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.write_word(self.sp , self.pc.wrapping_add(3));
    }

    // subroutine stack pop
    fn subroutine_stack_pop(&mut self) {
        self.pc = self.bus.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
    }

    // interrupt stack push
    fn interrupt_stack_push(&mut self) {
        self.sp = self.sp.wrapping_sub(2);
        self.bus.write_word(self.sp , self.pc);
    }

    /// Sets CPU frequency (MHz)
    /// ```rust
    /// use intel8080::CPU;
    /// let mut c = CPU::new();
    /// c.set_freq(1.7);            // CPU will run at 1.7 Mhz
    /// ```
    pub fn set_freq(&mut self, f: f32) {
        let cycles = (f * 1000000 as f32) / (1000/self.slice_duration) as f32;
        self.slice_max_cycles = cycles as u32;
    }

    /// Fetches and executes one instruction from (pc). Returns the sleep time when slice_max_cycles is reached.
    pub fn execute_timed(&mut self) -> Option<u32> {
        let mut sleep_time: Option<u32> = None;
        if self.slice_current_cycles > self.slice_max_cycles {
            self.slice_current_cycles = 0;
            // d = time taken to execute the slice_max_cycles
            if let Ok(d) = self.slice_start_time.elapsed() {
                sleep_time = Some(self.slice_duration.saturating_sub(d.as_millis() as u32));
                self.slice_start_time = SystemTime::now();
            }
        }
        let cycles = self.execute();
        self.slice_current_cycles += cycles;
        sleep_time
    }

    /// Fetches and executes one instruction from (pc). Returns the number of consumed clock cycles. No execution speed limit.
    pub fn execute(&mut self) -> u32 {
        if self.halt { return 0 };
        
        // Saving current PC for debug output
        let pc = self.pc;

        let opcode = match self.inte {
            false => self.bus.read_byte(self.pc),
            // interrupts enabled : is there a pending interrupt ?
            true => match self.int.0 {
                false => self.bus.read_byte(self.pc),
                true => self.int.1,
            }
        };

        let mut cycles = CYCLES[opcode as usize].into();
        
        // if opcode is RST : is it called via an interrupt, or via the program ?
        let direct_rst = if self.inte && self.int.0 { false } else { true };

        // interrupts enable and pending interrupt : we disable interrupts and clear interrupt request
        if self.inte && self.int.0 {
            self.inte = false;
            self.int = (false, 0);
        }

        match opcode {
            /* Carry bit instructions */
            0x3f => self.flags.c = !self.flags.c,                           // CMC
            0x37 => self.flags.c = true,                                    // STC

            /* Single register instructions */
            // INR Increment Register or Memory
            0x04 => self.reg.b = self.inr(self.reg.b),          // INR B
            0x0C => self.reg.c = self.inr(self.reg.c),          // INR C
            0x14 => self.reg.d = self.inr(self.reg.d),          // INR D
            0x1C => self.reg.e = self.inr(self.reg.e),          // INR E
            0x24 => self.reg.h = self.inr(self.reg.h),          // INR H
            0x2C => self.reg.l = self.inr(self.reg.l),          // INR L
            0x3C => self.reg.a = self.inr(self.reg.a),          // INR A
            0x34 => {                                                       // INR (HL)
                let addr = self.reg.get_hl();
                let r = self.inr(self.bus.read_byte(addr));
                self.bus.write_byte(addr, r);
            },

            // DCR Decrement Register or Memory
            0x05 => self.reg.b = self.dcr(self.reg.b),          // DCR B
            0x0D => self.reg.c = self.dcr(self.reg.c),          // DCR C
            0x15 => self.reg.d = self.dcr(self.reg.d),          // DCR D
            0x1D => self.reg.e = self.dcr(self.reg.e),          // DCR E
            0x25 => self.reg.h = self.dcr(self.reg.h),          // DCR H
            0x2D => self.reg.l = self.dcr(self.reg.l),          // DCR L
            0x3D => self.reg.a = self.dcr(self.reg.a),          // DCR A
            0x35 => {                                                       // DCR (HL)
                let addr = self.reg.get_hl();
                let r = self.dcr(self.bus.read_byte(addr));
                self.bus.write_byte(addr, r);
            },

            // CMA Complement Accumulator
            0x2F => self.reg.a = !self.reg.a,                   // CMA

            // Decimal adjust accumulator
            0x27 => self.daa(),

            // NOP No Operation
            0x00 => {},                                                     // NOP

            // MOV Data transfer instructions
            0x40 => {},                                                     // MOV B,B
            0x41 => self.reg.b = self.reg.c,                    // MOV B,C
            0x42 => self.reg.b = self.reg.d,                    // MOV B,D
            0x43 => self.reg.b = self.reg.e,                    // MOV B,E
            0x44 => self.reg.b = self.reg.h,                    // MOV B,H
            0x45 => self.reg.b = self.reg.l,                    // MOV B,L
            0x46 => {                                                       // MOV B,(HL)
                let addr = self.reg.get_hl();
                self.reg.b = self.bus.read_byte(addr)
            },
            0x47 => self.reg.b = self.reg.a,                    // MOV B,A

            0x48 => self.reg.c = self.reg.b,                    // MOV C,B                                                     // MOV B,B
            0x49 => {},                                                     // MOV C,C
            0x4A => self.reg.c = self.reg.d,                    // MOV C,D
            0x4B => self.reg.c = self.reg.e,                    // MOV C,E
            0x4C => self.reg.c = self.reg.h,                    // MOV C,H
            0x4D => self.reg.c = self.reg.l,                    // MOV C,L
            0x4E => {                                                       // MOV C,(HL)
                let addr = self.reg.get_hl();
                self.reg.c = self.bus.read_byte(addr)
            },
            0x4F => self.reg.c = self.reg.a,                    // MOV C,A

            0x50 => self.reg.d = self.reg.b,                    // MOV D,B                                                     // MOV B,B
            0x51 => self.reg.d = self.reg.c,                    // MOV D,C
            0x52 => {},                                                     // MOV D,D
            0x53 => self.reg.d = self.reg.e,                    // MOV D,E
            0x54 => self.reg.d = self.reg.h,                    // MOV D,H
            0x55 => self.reg.d = self.reg.l,                    // MOV D,L
            0x56 => {                                                       // MOV D,(HL)
                let addr = self.reg.get_hl();
                self.reg.d = self.bus.read_byte(addr)
            },
            0x57 => self.reg.d = self.reg.a,                    // MOV D,A

            0x58 => self.reg.e = self.reg.b,                    // MOV E,B                                                     // MOV B,B
            0x59 => self.reg.e = self.reg.c,                    // MOV E,C
            0x5A => self.reg.e = self.reg.d,                    // MOV E,D
            0x5B => {},                                                     // MOV E,E
            0x5C => self.reg.e = self.reg.h,                    // MOV E,H
            0x5D => self.reg.e = self.reg.l,                    // MOV E,L
            0x5E => {                                                       // MOV E,(HL)
                let addr = self.reg.get_hl();
                self.reg.e = self.bus.read_byte(addr)
            },
            0x5F => self.reg.e = self.reg.a,                    // MOV E,A

            0x60 => self.reg.h = self.reg.b,                    // MOV H,B                                                     // MOV B,B
            0x61 => self.reg.h = self.reg.c,                    // MOV H,C
            0x62 => self.reg.h = self.reg.d,                    // MOV H,D
            0x63 => self.reg.h = self.reg.e,                    // MOV H,E
            0x64 => {},                                                     // MOV H,H
            0x65 => self.reg.h = self.reg.l,                    // MOV H,L
            0x66 => {                                                       // MOV H,(HL)
                let addr = self.reg.get_hl();
                self.reg.h = self.bus.read_byte(addr)
            },
            0x67 => self.reg.h = self.reg.a,                    // MOV H,A

            0x68 => self.reg.l = self.reg.b,                    // MOV L,B                                                     // MOV B,B
            0x69 => self.reg.l = self.reg.c,                    // MOV L,C
            0x6A => self.reg.l = self.reg.d,                    // MOV L,D
            0x6B => self.reg.l = self.reg.e,                    // MOV L,E
            0x6C => self.reg.l = self.reg.h,                    // MOV L,H
            0x6D => {},                                                     // MOV L,L
            0x6E => {                                                       // MOV L,(HL)
                let addr = self.reg.get_hl();
                self.reg.l = self.bus.read_byte(addr)
            },
            0x6F => self.reg.l = self.reg.a,                    // MOV L,A

            0x70 => {                                                       // MOV (HL), B
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.b)
            },
            0x71 => {                                                       // MOV (HL), C
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.c)
            },
            0x72 => {                                                       // MOV (HL), D
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.d)
            },
            0x73 => {                                                       // MOV (HL), E
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.e)
            },
            0x74 => {                                                       // MOV (HL), H
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.h)
            },
            0x75 => {                                                       // MOV (HL), L
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.l)
            },

            0x76 => self.halt = true,                                       // HLT

            0x77 => {                                                       // MOV (HL), A
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, self.reg.a)
            },

            0x78 => self.reg.a = self.reg.b,                    // MOV A,B                                                     // MOV B,B
            0x79 => self.reg.a = self.reg.c,                    // MOV A,C
            0x7A => self.reg.a = self.reg.d,                    // MOV A,D
            0x7B => self.reg.a = self.reg.e,                    // MOV A,E
            0x7C => self.reg.a = self.reg.h,                    // MOV A,H
            0x7D => self.reg.a = self.reg.l,                    // MOV A,L
            0x7E => {                                                       // MOV A,(HL)
                let addr = self.reg.get_hl();
                self.reg.a = self.bus.read_byte(addr)
            },
            0x7F => {},                                                     // MOV A,A

            // STAX Store accumulator
            0x02 => {                                                       // STAX B
                let addr = self.reg.get_bc();
                self.bus.write_byte(addr, self.reg.a)
            }
            0x12 => {                                                       // STAX D
                let addr = self.reg.get_de();
                self.bus.write_byte(addr, self.reg.a)
            },

            // LDAX Load accumulator
            0x0A => {                                                       // LDAX B
                let addr = self.reg.get_bc();
                self.reg.a = self.bus.read_byte(addr)
            },
            0x1A => {                                                       // LDAX D
                let addr = self.reg.get_de();
                self.reg.a = self.bus.read_byte(addr)
            },

            /* Register or Memory to Accumulator instructions*/
            // ADD register or memory to accumulator
            0x80 => self.add(self.reg.b),                             // ADD B
            0x81 => self.add(self.reg.c),                             // ADD C
            0x82 => self.add(self.reg.d),                             // ADD D
            0x83 => self.add(self.reg.e),                             // ADD E
            0x84 => self.add(self.reg.h),                             // ADD H
            0x85 => self.add(self.reg.l),                             // ADD L
            0x86 => {                                                       // ADD (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.add(n)
            },
            0x87 => self.add(self.reg.a),                             // ADD A

            // ADC Add register or memory to accumulator with carry
            0x88 => self.adc(self.reg.b),                             // ADC B
            0x89 => self.adc(self.reg.c),                             // ADC C
            0x8A => self.adc(self.reg.d),                             // ADC D
            0x8B => self.adc(self.reg.e),                             // ADC E
            0x8C => self.adc(self.reg.h),                             // ADC H
            0x8D => self.adc(self.reg.l),                             // ADC L
            0x8E => {                                                       // ADC (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.adc(n)
            },
            0x8F => self.adc(self.reg.a),                             // ADC A

            // SUB Substract register or memory to accumulator
            0x90 => self.sub(self.reg.b),                             // SUB B
            0x91 => self.sub(self.reg.c),                             // SUB C
            0x92 => self.sub(self.reg.d),                             // SUB D
            0x93 => self.sub(self.reg.e),                             // SUB E
            0x94 => self.sub(self.reg.h),                             // SUB H
            0x95 => self.sub(self.reg.l),                             // SUB L
            0x96 => {                                                       // SUB (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.sub(n)
            },
            0x97 => self.sub(self.reg.a),                             // SUB A

            // SBB Substract register or memory to accumulator with borrow
            0x98 => self.sbb(self.reg.b),                             // SBB B
            0x99 => self.sbb(self.reg.c),                             // SBB C
            0x9A => self.sbb(self.reg.d),                             // SBB D
            0x9B => self.sbb(self.reg.e),                             // SBB E
            0x9C => self.sbb(self.reg.h),                             // SBB H
            0x9D => self.sbb(self.reg.l),                             // SBB L
            0x9E => {                                                       // SBB (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.sbb(n)
            },
            0x9F => self.sbb(self.reg.a),                             // SBB A

            // ANA Logical AND register or memory with accumulator
            0xA0 => self.ana(self.reg.b),                             // ANA B
            0xA1 => self.ana(self.reg.c),                             // ANA C
            0xA2 => self.ana(self.reg.d),                             // ANA D
            0xA3 => self.ana(self.reg.e),                             // ANA E
            0xA4 => self.ana(self.reg.h),                             // ANA H
            0xA5 => self.ana(self.reg.l),                             // ANA L
            0xA6 => {                                                       // ANA (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.ana(n)
            },
            0xA7 => self.ana(self.reg.a),                             // ANA A

            // XRA Logical Exclusive-OR register or memory with accumulator
            0xA8 => self.xra(self.reg.b),                             // XRA B
            0xA9 => self.xra(self.reg.c),                             // XRA C
            0xAA => self.xra(self.reg.d),                             // XRA D
            0xAB => self.xra(self.reg.e),                             // XRA E
            0xAC => self.xra(self.reg.h),                             // XRA H
            0xAD => self.xra(self.reg.l),                             // XRA L
            0xAE => {                                                       // XNA (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.xra(n)
            },
            0xAF => self.xra(self.reg.a),                             // XRA A

            // ORA Logical OR register or memory with accumulator
            0xB0 => self.ora(self.reg.b),                             // ORA B
            0xB1 => self.ora(self.reg.c),                             // ORA C
            0xB2 => self.ora(self.reg.d),                             // ORA D
            0xB3 => self.ora(self.reg.e),                             // ORA E
            0xB4 => self.ora(self.reg.h),                             // ORA H
            0xB5 => self.ora(self.reg.l),                             // ORA L
            0xB6 => {                                                       // ORA (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.ora(n)
            },
            0xB7 => self.ora(self.reg.a),                             // ORA A

            // CMP Compare register or memory with accumulator
            0xB8 => self.cmp(self.reg.b),                             // CMP B
            0xB9 => self.cmp(self.reg.c),                             // CMP C
            0xBA => self.cmp(self.reg.d),                             // CMP D
            0xBB => self.cmp(self.reg.e),                             // CMP E
            0xBC => self.cmp(self.reg.h),                             // CMP H
            0xBD => self.cmp(self.reg.l),                             // CMP L
            0xBE => {                                                       // CMP (HL)
                let addr = self.reg.get_hl();
                let n = self.bus.read_byte(addr);
                self.cmp(n)
            },
            0xBF => self.cmp(self.reg.a),                             // CMP A

            /* Rotate accumulator instructions */
            0x07 => self.rlc(),                                             // RLC
            0x0F => self.rrc(),                                             // RRC
            0x17 => self.ral(),                                             // RAL
            0x1F => self.rar(),                                             // RAR

            /* Register pair instructions */
            // PUSH data onto stack
            0xC5 => {                                                       // PUSH B
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.reg.get_bc());
            },
            0xD5 => {                                                       // PUSH D
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.reg.get_de());
            },
            0xE5 => {                                                       // PUSH H
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.reg.get_hl());
            },
            0xF5 => {                                                       // PUSH PSW
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_byte(self.sp, self.flags.as_byte());
                self.bus.write_byte(self.sp + 1, self.reg.a);
            },

            // POP data off stack
            0xC1 => {                                                       // POP B
                self.reg.set_bc(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xD1 => {                                                       // POP D
                self.reg.set_de(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xE1 => {                                                       // POP H
                self.reg.set_hl(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xF1 => {                                                       // POP PSW
                self.reg.a = self.bus.read_byte((self.sp)+1);
                let bflags = self.bus.read_byte(self.sp);
                self.flags.from_byte(bflags);
                self.sp = self.sp.wrapping_add(2);
            },

            // DAD Double add
            0x09 => {                                                       // DAD B
                let reg = self.reg.get_bc();
                self.dad(reg);
            },
            0x19 => {                                                       // DAD D
                let reg = self.reg.get_de();
                self.dad(reg);
            },
            0x29 => {                                                       // DAD H
                let reg = self.reg.get_hl();
                self.dad(reg);
            },
            0x39 => {                                                       // DAD SP
                let reg = self.sp;
                self.dad(reg);
            },

            // INX Increment register pair
            0x03 => {                                                       // INX B
                let mut b = self.reg.get_bc();
                b = b.wrapping_add(1);
                self.reg.set_bc(b);
            },

            0x13 => {                                                       // INX D
                let mut d = self.reg.get_de();
                d = d.wrapping_add(1);
                self.reg.set_de(d);
            },

            0x23 => {                                                       // INX H
                let mut h = self.reg.get_hl();
                h = h.wrapping_add(1);
                self.reg.set_hl(h);
            }

            0x33 => self.sp = self.sp.wrapping_add(1),                      // INX SP             

            // DCX Decrement register pair
            0x0B => {                                                       // DCX B
                let mut b = self.reg.get_bc();
                b = b.wrapping_sub(1);
                self.reg.set_bc(b);
            },

            0x1B => {                                                       // DCX D
                let mut d = self.reg.get_de();
                d = d.wrapping_sub(1);
                self.reg.set_de(d);
            },

            0x2B => {                                                       // DCX H
                let mut h = self.reg.get_hl();
                h = h.wrapping_sub(1);
                self.reg.set_hl(h);
            }

            0x3B => self.sp = self.sp.wrapping_sub(1),                      // DCX SP

            // XCHG Exchange reg
            0xEB => self.xchg(),

            // XTHL Exchange stack
            0xE3 => self.xthl(),

            // SPHL Load SP from H and L
            0xF9 => self.sp = self.reg.get_hl(),

            /* Immediate instructions */
            // LXI Move immediate data
            0x01 => {                                                       // LXI B
                let d16 = self.bus.read_word(self.pc + 1); 
                self.reg.set_bc(d16);
            },
            0x11 => {                                                       // LXI D
                let d16 = self.bus.read_word(self.pc + 1); 
                self.reg.set_de(d16);
            },
            0x21 => {                                                       // LXI H
                let d16 = self.bus.read_word(self.pc + 1); 
                self.reg.set_hl(d16);
            },
            0x31 => {                                                       // LXI SP
                let d16 = self.bus.read_word(self.pc + 1); 
                self.sp = d16;
            },
            // MVI Move immediate data
            0x06 => {                                                       // MVI B,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.b = d8;
            },
            0x0E => {                                                       // MVI C,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.c = d8;
            },
            0x16 => {                                                       // MVI D,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.d = d8;
            },
            0x1E => {                                                       // MVI E,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.e = d8;
            },
            0x26 => {                                                       // MVI H,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.h = d8;
            },
            0x2E => {                                                       // MVI L,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.l = d8;
            },
            0x36 => {                                                       // MVI (HL),d8
                let d8 = self.bus.read_byte(self.pc + 1);
                let addr = self.reg.get_hl();
                self.bus.write_byte(addr, d8);
            },
            0x3E => {                                                       // MVI A,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                self.reg.a = d8;
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
                self.bus.write_byte(addr, self.reg.a);
            },

            // LDA Store accumulator direct
            0x3A => {                                                       // LDA
                let addr = self.bus.read_word(self.pc + 1);
                self.reg.a = self.bus.read_byte(addr);
            },

            // SHLD Store H and L direct
            0x22 => {                                                       // SHLD
                let d = self.reg.get_hl();
                let addr = self.bus.read_word(self.pc + 1);
                self.bus.write_word(addr, d);
            },

            // LHLD Load H and L direct
            0x2A => {                                                       // LHLD
                let addr = self.bus.read_word(self.pc + 1);
                let d = self.bus.read_word(addr);
                self.reg.set_hl(d);
            },

            /* JUMP instructions */
            // Load program counter
            0xE9 => { self.pc = self.reg.get_hl(); },                 // PCHL
            // JMP Jump
            0xC3 => {                                                       // JMP
                let addr = self.bus.read_word(self.pc + 1);
                self.pc = addr;
            },
            // JC Jump if carry
            0xDA => {                                                       // JC
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.c { self.pc = addr; } else { self.pc += 3 }
            },
            // JNC Jump if no carry
            0xD2 => {                                                       // JNC
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.c { self.pc = addr; } else { self.pc += 3 }
            },
            // JZ Jump if zero
            0xCA => {                                                       // JZ
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.z { self.pc = addr; } else { self.pc += 3 }
            },
            // JNZ Jump if not zero
            0xC2 => {                                                       // JNZ
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.z { self.pc = addr; } else { self.pc += 3 }
            },
            // JM Jump if minus
            0xFA => {                                                       // JM
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.s { self.pc = addr; } else { self.pc += 3 }
            },
            // JP Jump if positive
            0xF2 => {                                                       // JP
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.s { self.pc = addr; } else { self.pc += 3 }
            },
            // JPE Jump if parity even
            0xEA => {                                                       // JPE
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.p { self.pc = addr; } else { self.pc += 3 }
            },
            // JPO Jump if parity odd
            0xE2 => {                                                       // JPO
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.p { self.pc = addr; } else { self.pc += 3 }
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
                if self.flags.c {
                    self.subroutine_stack_push();
                    self.pc = addr;
                } else { self.pc += 3 }
            },
            // CNC Call if no carry
            0xD4 => {                                                       // CNC
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.c {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },
            // CZ Call if zero
            0xCC => {                                                       // CZ
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.z {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },
            // CNZ Call if not zero
            0xC4 => {                                                       // CNZ
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.z {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                 } else { self.pc += 3 }
            },
            // CM Call if minus
            0xFC => {                                                       // CM
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.s {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },
            // CP Call if plus
            0xF4 => {                                                       // CP
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.s {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },
            // CPE Call if parity even
            0xEC => {                                                       // CPE
                let addr = self.bus.read_word(self.pc + 1);
                if self.flags.p {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },
            // CPO Call if parity odd
            0xE4 => {                                                       // CPO
                let addr = self.bus.read_word(self.pc + 1);
                if !self.flags.p {
                    self.subroutine_stack_push();
                    self.pc = addr;
                    cycles += 6;
                } else { self.pc += 3 }
            },

            /* Return from subroutine instructions */
            // RET Return
            0xC9 => self.subroutine_stack_pop(),                                                    // RET
            // RC Return if carry
            0xD8 => if self.flags.c { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },         // RC
            // RNC Return if no carry
            0xD0 => if !self.flags.c { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },        // RNC
            // RZ Return if zero
            0xC8 => if self.flags.z { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },         // RZ
            // RNZ Return if not zero
            0xC0 => if !self.flags.z { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },        // RNZ
            // RM Return if minus
            0xF8 => if self.flags.s { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },         // RM
            // RP Return if plus
            0xF0 => if !self.flags.s { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },        // RP
            // RPE Return if parity even
            0xE8 => if self.flags.p { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },         // RPE
            // RPO Return if parity odd
            0xE0 => if !self.flags.p { self.subroutine_stack_pop(); cycles += 6; } else { self.pc +=1; },        // RPO

            /* Interrupt flip-flop instructions */
            // EI Enable interrupts
            0xFB => self.inte = true,
            // DI Disable Interrupts
            0xF3 => self.inte = false,

            /* RST (Restart) instructions */
            0xC7 => {                                                       // RST 0
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0000;
            },

            0xCF => {                                                       // RST 1
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0008;
            },

            0xD7 => {                                                       // RST 2
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0010;
            },

            0xDF => {                                                       // RST 3
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0018;
            },

            0xE7 => {                                                       // RST 4
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0020;
            },

            0xEF => {                                                       // RST 5
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0028;
            },

            0xF7 => {                                                       // RST 6
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0030;
            },

            0xFF => {                                                       // RST 7
                match direct_rst {
                    false => self.interrupt_stack_push(),
                    true => { self.pc +=1; self.interrupt_stack_push(); }
                }
                self.pc = 0x0038;
            },

            /* Input / output instructions */
            // IN Input
            0xDB => {
                // To implement yourself
            },

            // OUT Output
            0xD3 => {
                // To implement yourself
            },

            _ => {}
        }

        if self.debug.switch
        { self.debug.string = match opcode {
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF =>  String::from("RST"),
            _ => format!("{}\nPC : {:#06x}\tSP : {:#06x}\tS : {}\tZ : {}\tA : {}\tP : {}\tC : {}\nB : {:#04x}\tC : {:#04x}\tD : {:#04x}\tE : {:#04x}\tH : {:#04x}\tL : {:#04x}\tA : {:#04x}\t(SP) : {:#06x}\n", self.dasm(pc), pc, self.sp, self.flags.s as i32, self.flags.z as i32, self.flags.a as i32, self.flags.p as i32, self.flags.c as i32, self.reg.b, self.reg.c, self.reg.d, self.reg.e, self.reg.h, self.reg.l, self.reg.a, self.bus.read_word(self.sp)),
            }
        }

        match opcode {
            0xe9 | 0xc3 | 0xDA | 0xD2 | 0xCA | 0xC2 | 0xFA | 0xF2 | 0xEA | 0xE2 |
            0xCD | 0xDC | 0xD4 | 0xCC | 0xC4 | 0xFC | 0xF4 | 0xEC | 0xE4 |
            0xC9 | 0xD8 | 0xD0 | 0xC8 | 0xC0 | 0xF8 | 0xF0 | 0xE8 | 0xE0 | 
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {},
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E |
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE |
            0xDB | 0xD3 => self.pc += 2,
            0x32 | 0x3A | 0x22 | 0x2A | 0x01 | 0x11 | 0x21 | 0x31 => self.pc += 3,
            _ => self.pc +=1,
        }

        cycles

    }
}
