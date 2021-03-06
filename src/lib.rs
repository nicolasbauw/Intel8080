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
//! You can load assembled programs from disk to memory:
//! ```rust
//! # use intel8080::CPU;
//! # let mut c = CPU::new();
//! c.pc = 0x0100;                                      // sets pc to $100
//! c.bus.load_bin("bin/loop.bin", 0x100).unwrap();     // loads file at address $100
//! loop {
//!     c.execute();
//!     if c.pc == 0x0000 { break }
//! }
//! ```
//! 
//! It's easy to create an interrupt request:
//! ```rust
//! # use intel8080::CPU;
//! # let mut c = CPU::new();
//! c.bus.load_bin("bin/interrupt.bin", 0).unwrap();
//! c.int = (true, 0xcf);               // we create an interrupt request : flag set to true
//! loop {                              // and its associated RST command
//!     c.execute();                    // test program is designed to never leave a loop
//!     if c.pc == 0x0000 { break }     // if it does not execute the interrupt routine
//! }
//! ```
//! 
//! Starting with 0.8.0, a more stabilized I/O system:
//! ```rust
//! # use intel8080::CPU;
//! # let mut c = CPU::new();
//! c.bus.write_byte(0x0000, 0x3e);     // MVI A,$55
//! c.bus.write_byte(0x0001, 0x55);
//! c.bus.write_byte(0x0002, 0xd3);     // OUT 1
//! c.bus.write_byte(0x0003, 0x01);
//! loop {
//!     c.execute();
//!     // Data sent from CPU to device 1 (OUT) ? let's handle it
//!     if let Some(v) = c.bus.get_io_out(1) {
//!         assert_eq!(v, 0x55);
//!         // OUT handled ? let's clear it
//!         c.bus.clear_io_out();
//!     }
//!     if c.pc == 0x0004 { break }
//! }
//! ```
//! 
//! The I/O system has been improved in 0.14.0, you can now use callbacks:
//! ```text
//! c.set_cb_out(out_callback);
//! ...
//! fn out_callback(c: &mut CPU, device: u8, data: u8) -> Option<u8> {
//!     your callback code here
//! }
//! ```
//! The 0.8.0 I/O system still works if no callback is set.
//! 
//! In version 0.13.0, a field has been added to define a read-only area in the address space:
//! ```rust
//! use intel8080::{CPU, memory::ROMSpace};
//! let mut c = CPU::new();
//! c.bus.rom_space = Some(ROMSpace{start: 0xfff0, end: 0xffff});
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
//! You can also check my [Altair 8800 / 88-SIO / teletype emulator](https://crates.io/crates/teletype).
//! 
//! The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).
//! 

#[doc(hidden)]
pub mod register;
pub mod memory;
mod flags;
mod bit;
mod dasm;

use crate::register::Registers;
use crate::memory::{AddressBus};
use crate::flags::Flags;
use std::{time::Duration, time::SystemTime};

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

// Optional callback functions for IN and OUT instructions must have this signature
type Callback = fn(&mut CPU, u8, u8) -> Option<u8>;

pub struct Debug {
    /// Enables / Disables the debug string generation
    pub switch: bool,
    /// The debug information string
    pub string: String,
}

pub struct CPU {
    pub registers: Registers,
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
    pub bus: AddressBus,
    pub halt: bool,
    /// Interrupt request : true / false, instruction to execute (normally a RST command)
    pub int: (bool, u8),
    /// Interrupt enable bit
    pub inte: bool,
    out_callback: Option<Callback>,
    in_callback: Option<Callback>,
    /// Outputs CPU state and disassembled code to stdout after each execute()
    /// ```text
    /// 3E 0f     MVI A,$0f
    /// PC : 0x0003	SP : 0xff00	S : 0	Z : 0	A : 0	P : 0	C : 0
    /// B : 0x00	C : 0x00	D : 0x00	E : 0x00	H : 0x00	L : 0x00 ...
    /// ```
    pub debug: Debug,
    /// Defaults to 1/60FPS = 16ms
    pub slice_duration: u32,
    /// Defaults to 35000 cycles per 16ms slice (2.1 Mhz).
    /// cycles = clock speed in Hz / required frames-per-second
    pub slice_max_cycles: u32,
    slice_current_cycles: u32,
    slice_start_time: SystemTime,
}

impl Debug {
    pub fn new() -> Debug {
        Debug {
            switch: false,
            string: String::new(),
        }
    }
}

impl CPU {
    /// Creates a new CPU instance and its 16 bits address bus.
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
            bus: AddressBus::new(),
            halt: false,
            int: (false, 0),
            inte: false,
            out_callback: None,
            in_callback: None,
            debug: Debug::new(),
            slice_duration: 16,
            slice_max_cycles: 35000,
            slice_current_cycles: 0,
            slice_start_time: SystemTime::now(),
        }
    }

    /// Sets IO OUT callback function used by the OUT instruction.
    pub fn set_cb_out(&mut self, cb: Callback) {
        self.out_callback = Some(cb);
    }

    /// Sets IO IN callback function used by the IN instruction.
    pub fn set_cb_in(&mut self, cb: Callback) {
        self.in_callback = Some(cb);
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

    // SBB register or memory from Accumulator with borrow
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
        self.flags.a = (n | self.registers.a) & 0x08 != 0;
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
        self.flags.a = false;
        self.registers.a = r;
    }

    // ORA Logical AND register or memory with accumulator
    fn ora(&mut self, n: u8) {
        let r = self.registers.a | n;
        self.flags.z = r == 0x00;
        self.flags.s = bit::get(r, 7);
        self.flags.p = r.count_ones() & 0x01 == 0x00;
        self.flags.c = false;
        self.flags.a = false;
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
    
    // RAR Rotate accumulator right through carry
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
        let pointed_by_sp = self.bus.read_word(self.sp);
        let hl = self.registers.get_hl();
        self.bus.write_word(self.sp, hl);
        self.registers.set_hl(pointed_by_sp);
    }

    // Decimal adjust accumulator
    fn daa(&mut self) {
        let mut inc_a: u8 = 0;
        let mut c = self.flags.c;
        let lsb = self.registers.a & 0x0F;
        if (lsb > 9) || self.flags.a {
            inc_a += 0x06;
        }

        let msb = self.registers.a >> 4;
        if (msb > 9) || self.flags.c || (msb >= 9 && lsb > 9) {
            inc_a += 0x60;
            c = true;
        }

        self.add(inc_a);
        self.flags.c = c;
        self.flags.z = self.registers.a == 0x00;
        self.flags.s = bit::get(self.registers.a, 7);
        self.flags.p = self.registers.a.count_ones() & 0x01 == 0x00;
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

    /// Fetches and executes one instruction from (pc), limiting speed to 2,1 Mhz by default. Returns the number of consumed clock cycles.
    pub fn execute_slice(&mut self) -> u32 {
        if self.slice_current_cycles > self.slice_max_cycles {
            self.slice_current_cycles = 0;
            // d = time taken to execute the slice_max_cycles
            if let Ok(d) = self.slice_start_time.elapsed() {
                let sleep_time = self.slice_duration.saturating_sub(d.as_millis() as u32);
                /*println!("Execution time : {:?}", d);
                println!("Sleep time : {:?}", sleep_time);*/

                #[cfg(windows)]
                spin_sleep::sleep(Duration::from_millis(u64::from(sleep_time)));

                #[cfg(not(windows))]
                std::thread::sleep(Duration::from_millis(u64::from(sleep_time)));

                self.slice_start_time = SystemTime::now();
            }
        }
        let cycles = self.execute();
        self.slice_current_cycles += cycles;
        cycles
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
            0x27 => self.daa(),

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
            0x51 => self.registers.d = self.registers.c,                    // MOV D,C
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
            0x62 => self.registers.h = self.registers.d,                    // MOV H,D
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

            0x76 => self.halt = true,                                       // HLT

            0x77 => {                                                       // MOV (HL), A
                let addr = self.registers.get_hl();
                self.bus.write_byte(addr, self.registers.a)
            },

            0x78 => self.registers.a = self.registers.b,                    // MOV A,B                                                     // MOV B,B
            0x79 => self.registers.a = self.registers.c,                    // MOV A,C
            0x7A => self.registers.a = self.registers.d,                    // MOV A,D
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
            0xAE => {                                                       // XNA (HL)
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
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.registers.get_bc());
            },
            0xD5 => {                                                       // PUSH D
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.registers.get_de());
            },
            0xE5 => {                                                       // PUSH H
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_word(self.sp, self.registers.get_hl());
            },
            0xF5 => {                                                       // PUSH PSW
                self.sp = self.sp.wrapping_sub(2);
                self.bus.write_byte(self.sp, self.flags.as_byte());
                self.bus.write_byte(self.sp + 1, self.registers.a);
            },

            // POP data off stack
            0xC1 => {                                                       // POP B
                self.registers.set_bc(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xD1 => {                                                       // POP D
                self.registers.set_de(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xE1 => {                                                       // POP H
                self.registers.set_hl(self.bus.read_word(self.sp));
                self.sp = self.sp.wrapping_add(2);
            },

            0xF1 => {                                                       // POP PSW
                self.registers.a = self.bus.read_byte((self.sp)+1);
                let bflags = self.bus.read_byte(self.sp);
                self.flags.from_byte(bflags);
                self.sp = self.sp.wrapping_add(2);
            },

            // DAD Double add
            0x09 => {                                                       // DAD B
                let reg = self.registers.get_bc();
                self.dad(reg);
            },
            0x19 => {                                                       // DAD D
                let reg = self.registers.get_de();
                self.dad(reg);
            },
            0x29 => {                                                       // DAD H
                let reg = self.registers.get_hl();
                self.dad(reg);
            },
            0x39 => {                                                       // DAD SP
                let reg = self.sp;
                self.dad(reg);
            },

            // INX Increment register pair
            0x03 => {                                                       // INX B
                let mut b = self.registers.get_bc();
                b = b.wrapping_add(1);
                self.registers.set_bc(b);
            },

            0x13 => {                                                       // INX D
                let mut d = self.registers.get_de();
                d = d.wrapping_add(1);
                self.registers.set_de(d);
            },

            0x23 => {                                                       // INX H
                let mut h = self.registers.get_hl();
                h = h.wrapping_add(1);
                self.registers.set_hl(h);
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
                self.registers.set_de(d);
            },

            0x2B => {                                                       // DCX H
                let mut h = self.registers.get_hl();
                h = h.wrapping_sub(1);
                self.registers.set_hl(h);
            }

            0x3B => self.sp = self.sp.wrapping_sub(1),                      // DCX SP

            // XCHG Exchange registers
            0xEB => self.xchg(),

            // XTHL Exchange stack
            0xE3 => self.xthl(),

            // SPHL Load SP from H and L
            0xF9 => self.sp = self.registers.get_hl(),

            /* Immediate instructions */
            // LXI Move immediate data
            0x01 => {                                                       // LXI B
                let d16 = self.bus.read_word(self.pc + 1); 
                self.registers.set_bc(d16);
            },
            0x11 => {                                                       // LXI D
                let d16 = self.bus.read_word(self.pc + 1); 
                self.registers.set_de(d16);
            },
            0x21 => {                                                       // LXI H
                let d16 = self.bus.read_word(self.pc + 1); 
                self.registers.set_hl(d16);
            },
            0x31 => {                                                       // LXI SP
                let d16 = self.bus.read_word(self.pc + 1); 
                self.sp = d16;
            },
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

            // LHLD Load H and L direct
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
                let device = self.bus.read_byte(self.pc+1);
                match self.in_callback {
                    None => { self.registers.a = self.bus.get_io_in(device); },
                    Some(cb) => {
                        if let Some(a) = cb(self, device, 0) { self.registers.a = a; }
                    }
                };
            },
            // OUT Output
            0xD3 => {
                let device = self.bus.read_byte(self.pc+1);
                match self.out_callback {
                    None => { self.bus.set_io_out(device, self.registers.a); },
                    Some(cb) => { cb(self, device, self.registers.a); }
                }
            },

            _ => {}
        }

        if self.debug.switch
        { self.debug.string = match opcode {
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF =>  String::from("RST"),
            _ => format!("{}\nPC : {:#06x}\tSP : {:#06x}\tS : {}\tZ : {}\tA : {}\tP : {}\tC : {}\nB : {:#04x}\tC : {:#04x}\tD : {:#04x}\tE : {:#04x}\tH : {:#04x}\tL : {:#04x}\tA : {:#04x}\t(SP) : {:#06x}\n", self.dasm(pc), pc, self.sp, self.flags.s as i32, self.flags.z as i32, self.flags.a as i32, self.flags.p as i32, self.flags.c as i32, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l, self.registers.a, self.bus.read_word(self.sp)),
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

#[cfg(test)]
mod instructions {
    use super::*;
    use crate::memory::ROMSpace;
    #[test]
    fn ldax_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0a);
        c.bus.write_byte(0x100, 0x65);
        c.registers.set_bc(0x100);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x65);
    }

    #[test]
    fn ldax_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1a);
        c.bus.write_byte(0x100, 0x65);
        c.registers.set_de(0x100);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x65);
    }

    #[test]
    fn lxi_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x01);
        c.bus.write_byte(0x0001, 0x12);
        c.bus.write_byte(0x0002, 0x34);
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.registers.b, 0x34);
        assert_eq!(c.registers.c, 0x12);
    }

    #[test]
    fn lxi_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x11);
        c.bus.write_byte(0x0001, 0x12);
        c.bus.write_byte(0x0002, 0x34);
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.registers.d, 0x34);
        assert_eq!(c.registers.e, 0x12);
    }

    #[test]
    fn lxi_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x21);
        c.bus.write_byte(0x0001, 0x12);
        c.bus.write_byte(0x0002, 0x34);
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.registers.h, 0x34);
        assert_eq!(c.registers.l, 0x12);
    }

    #[test]
    fn lxi_sp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x31);
        c.bus.write_byte(0x0001, 0x12);
        c.bus.write_byte(0x0002, 0x34);
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.sp, 0x3412);
    }

    #[test]
    fn sta() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x32);
        c.bus.write_byte(0x0001, 0x00);
        c.bus.write_byte(0x0002, 0xff);
        c.registers.a = 0x56;
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.bus.read_byte(0xff00), 0x56);
    }

    #[test]
    fn lda() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3a);
        c.bus.write_byte(0x0001, 0x00);
        c.bus.write_byte(0x0002, 0xff);
        c.bus.write_byte(0xff00, 0x56);
        c.execute();
        assert_eq!(c.pc, 0x0003);
        assert_eq!(c.registers.a, 0x56);
    }

    #[test]
    fn stax_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x02);
        c.registers.a = 0x49;
        c.registers.set_bc(0x1234);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.bus.read_byte(0x1234), 0x49);
    }

    #[test]
    fn stax_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x12);
        c.registers.a = 0x49;
        c.registers.set_de(0x1234);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.bus.read_byte(0x1234), 0x49);
    }

    #[test]
    fn inx_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x03);
        c.registers.set_bc(0x1234);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.registers.get_bc(), 0x1235);
    }

    #[test]
    fn inx_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x13);
        c.registers.set_de(0x1234);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.registers.get_de(), 0x1235);
    }

    #[test]
    fn inx_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x23);
        c.registers.a = 0x49;
        c.registers.set_hl(0x1234);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.registers.get_hl(), 0x1235);
    }

    #[test]
    fn inx_sp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x33);
        c.sp = 0x0049;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(c.sp, 0x004A);
    }

    #[test]
    fn cmc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3f);
        c.bus.write_byte(0x0001, 0x3f);
        c.execute();
        assert_eq!(true, c.flags.c);
        assert_eq!(c.pc, 0x0001);
        c.execute();
        assert_eq!(false, c.flags.c);
        assert_eq!(c.pc, 0x0002);
    }

    #[test]
    fn stc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x37);
        c.bus.write_byte(0x0001, 0x37);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(true, c.flags.c);
        c.execute();
        assert_eq!(c.pc, 0x0002);
        assert_eq!(true, c.flags.c);
    }

    #[test]
    fn inrb() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x04);
        c.registers.b = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.b);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inrc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0C);
        c.registers.c = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.c);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inrd() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x14);
        c.registers.d = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.d);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inre() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1C);
        c.registers.e = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.e);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inrh() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x24);
        c.registers.h = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.h);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inrl() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2C);
        c.registers.l = 0xff;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.registers.l);
        assert_eq!(true, c.flags.z);
    }

    #[test]
    fn inrm() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x34);
        c.bus.write_byte(0x0001, 0x34);
        c.bus.write_byte(0x100, 0xff);
        c.registers.set_hl(0x100);
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0, c.bus.read_byte(0x100));
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 0x0002);
        assert_eq!(1, c.bus.read_byte(0x100));
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn inra() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3C);
        c.registers.a = 0x0f;
        c.execute();
        assert_eq!(c.pc, 0x0001);
        assert_eq!(0x10, c.registers.a);
        assert_eq!(false, c.flags.z);
        assert_eq!(true, c.flags.a);
    }

    #[test]
    fn dcr_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x05);
        c.bus.write_byte(0x0001, 0x05);
        c.registers.b = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.b);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.b);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0d);
        c.bus.write_byte(0x0001, 0x0d);
        c.registers.c = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.c);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.c);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x15);
        c.bus.write_byte(0x0001, 0x15);
        c.registers.d = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.d);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.d);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1d);
        c.bus.write_byte(0x0001, 0x1d);
        c.registers.e = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.e);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.e);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x25);
        c.bus.write_byte(0x0001, 0x25);
        c.registers.h = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.h);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.h);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2d);
        c.bus.write_byte(0x0001, 0x2d);
        c.registers.l = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.l);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.l);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x35);
        c.bus.write_byte(0x0001, 0x35);
        c.bus.write_byte(0x100, 0x55);
        c.registers.set_hl(0x0100);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0x54, c.bus.read_byte(0x0100));
        assert_eq!(false, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0x53, c.bus.read_byte(0x0100));
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn dcr_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3d);
        c.bus.write_byte(0x0001, 0x3d);
        c.registers.a = 0x01;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0, c.registers.a);
        assert_eq!(true, c.flags.z);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(0xff, c.registers.a);
        assert_eq!(false, c.flags.z);
    }

    #[test]
    fn cma() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2F);
        c.registers.a = 0b11001100;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0b00110011, c.registers.a);
        
    }

    #[test]
    fn add() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x82);
        c.registers.a = 0x6C;
        c.registers.d = 0x2E;
        c.execute();
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
        assert_eq!(0x0C, c.registers.a);
    }

    #[test]
    fn ora() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xB1);
        c.registers.a = 0x33;
        c.registers.c = 0x0F;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(0x3F, c.registers.a);
    }

    #[test]
    fn cmp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xBB);
        c.registers.a = 0x0A;
        c.registers.e = 0x05;
        c.execute();
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0xE5);
    }

    #[test]
    fn rrc() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0F);
        c.registers.a = 0xF2;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x79);
    }

    #[test]
    fn ral() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x17);
        c.registers.a = 0xB5;
        c.execute();
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xFF);
        assert_eq!(c.flags.s, true);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.p, false);
    }

    #[test]
    fn dad_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x09);
        c.registers.set_bc(0x339F);
        c.registers.set_hl(0xA17B);
        c.execute();
        assert_eq!(c.registers.h, 0xD5);
        assert_eq!(c.registers.l, 0x1A);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dad_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x19);
        c.registers.set_de(0x339F);
        c.registers.set_hl(0xA17B);
        c.execute();
        assert_eq!(c.registers.h, 0xD5);
        assert_eq!(c.registers.l, 0x1A);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dad_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x29);
        c.registers.set_hl(0x339F);
        c.execute();
        assert_eq!(c.registers.h, 0x67);
        assert_eq!(c.registers.l, 0x3e);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dad_sp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x39);
        c.sp = 0x339F;
        c.registers.set_hl(0xA17B);
        c.execute();
        assert_eq!(c.registers.h, 0xD5);
        assert_eq!(c.registers.l, 0x1A);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dcx_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0b);
        c.registers.set_bc(0);
        c.execute();
        assert_eq!(c.registers.get_bc(), 0xffff);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dcx_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1b);
        c.registers.set_de(0);
        c.execute();
        assert_eq!(c.registers.get_de(), 0xffff);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dcx_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2b);
        c.registers.set_hl(0);
        c.execute();
        assert_eq!(c.registers.get_hl(), 0xffff);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn dcx_sp() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3b);
        c.sp = 0xFFFF;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.sp, 0xFFFE);
    }

    #[test]
    fn xchg() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xeb);
        c.registers.set_de(0x3355);
        c.registers.set_hl(0x00FF);
        c.execute();
        assert_eq!(c.pc, 1);
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
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.get_hl(), 0x0df0);
        assert_eq!(c.bus.read_byte(0x10ad), 0x3c);
        assert_eq!(c.bus.read_byte(0x10ae), 0x0b);
    }

    #[test]
    fn mvi_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x06);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.b, 0x88);
    }

    #[test]
    fn mvi_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x0e);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.c, 0x88);
    }

    #[test]
    fn mvi_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x16);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.d, 0x88);
    }

    #[test]
    fn mvi_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x1e);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.e, 0x88);
    }

    #[test]
    fn mvi_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x26);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.h, 0x88);
    }

    #[test]
    fn mvi_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x2e);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.l, 0x88);
    }

    #[test]
    fn mvi_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x36);
        c.bus.write_byte(0x0001, 0x88);
        c.registers.set_hl(0x100);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.bus.read_byte(0x100), 0x88);
    }

    #[test]
    fn mvi_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3e);
        c.bus.write_byte(0x0001, 0x88);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.registers.a, 0x88);
    }

    #[test]
    fn adi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xc6);
        c.bus.write_byte(0x0001, 0x42);
        c.registers.a = 0x14;
        c.execute();
        assert_eq!(c.pc, 2);
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
        assert_eq!(c.pc, 4);
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
        assert_eq!(c.pc, 2);
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
        assert_eq!(c.pc, 3);
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
        assert_eq!(c.pc, 3);
        assert_eq!(c.registers.a, 0x0a);
        assert_eq!(c.flags.p, true);
        assert_eq!(c.flags.a, true);
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
        assert_eq!(c.pc, 2);
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
        assert_eq!(c.pc, 3);
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
        assert_eq!(c.pc, 4);
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
        assert_eq!(c.pc, 3);
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
        assert_eq!(c.pc, 3);
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

    #[test]
    fn daa() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x27);
        c.registers.a = 0x9B;
        c.flags.a = false;
        c.flags.c = false;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 1);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sphl() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xf9);
        c.registers.h = 0x50;
        c.registers.l = 0x6c;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.sp, 0x506c)
    }

    #[test]
    fn nop() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x00);
        c.execute();
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn mov_b() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x252f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x40);
        c.bus.write_byte(0x0001, 0x41);
        c.bus.write_byte(0x0002, 0x42);
        c.bus.write_byte(0x0003, 0x43);
        c.bus.write_byte(0x0004, 0x44);
        c.bus.write_byte(0x0005, 0x45);
        c.bus.write_byte(0x0006, 0x46);
        c.bus.write_byte(0x0007, 0x47);
        c.execute();
        assert_eq!(c.registers.b, 0x11);
        c.execute();
        assert_eq!(c.registers.b, 0x15);
        c.execute();
        assert_eq!(c.registers.b, 0x1f);
        c.execute();
        assert_eq!(c.registers.b, 0x21);
        c.execute();
        assert_eq!(c.registers.b, 0x25);
        c.execute();
        assert_eq!(c.registers.b, 0x2f);
        c.execute();
        assert_eq!(c.registers.b, 0x31);
        c.execute();
        assert_eq!(c.registers.b, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_c() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x252f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x48);
        c.bus.write_byte(0x0001, 0x49);
        c.bus.write_byte(0x0002, 0x4a);
        c.bus.write_byte(0x0003, 0x4b);
        c.bus.write_byte(0x0004, 0x4c);
        c.bus.write_byte(0x0005, 0x4d);
        c.bus.write_byte(0x0006, 0x4e);
        c.bus.write_byte(0x0007, 0x4f);
        c.execute();
        assert_eq!(c.registers.c, 0x11);
        c.execute();
        assert_eq!(c.registers.c, 0x11);
        c.execute();
        assert_eq!(c.registers.c, 0x1f);
        c.execute();
        assert_eq!(c.registers.c, 0x21);
        c.execute();
        assert_eq!(c.registers.c, 0x25);
        c.execute();
        assert_eq!(c.registers.c, 0x2f);
        c.execute();
        assert_eq!(c.registers.c, 0x31);
        c.execute();
        assert_eq!(c.registers.c, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_d() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x252f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x50);
        c.bus.write_byte(0x0001, 0x51);
        c.bus.write_byte(0x0002, 0x52);
        c.bus.write_byte(0x0003, 0x53);
        c.bus.write_byte(0x0004, 0x54);
        c.bus.write_byte(0x0005, 0x55);
        c.bus.write_byte(0x0006, 0x56);
        c.bus.write_byte(0x0007, 0x57);
        c.execute();
        assert_eq!(c.registers.d, 0x11);
        c.execute();
        assert_eq!(c.registers.d, 0x15);
        c.execute();
        assert_eq!(c.registers.d, 0x15);
        c.execute();
        assert_eq!(c.registers.d, 0x21);
        c.execute();
        assert_eq!(c.registers.d, 0x25);
        c.execute();
        assert_eq!(c.registers.d, 0x2f);
        c.execute();
        assert_eq!(c.registers.d, 0x31);
        c.execute();
        assert_eq!(c.registers.d, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_e() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x252f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x58);
        c.bus.write_byte(0x0001, 0x59);
        c.bus.write_byte(0x0002, 0x5a);
        c.bus.write_byte(0x0003, 0x5b);
        c.bus.write_byte(0x0004, 0x5c);
        c.bus.write_byte(0x0005, 0x5d);
        c.bus.write_byte(0x0006, 0x5e);
        c.bus.write_byte(0x0007, 0x5f);
        c.execute();
        assert_eq!(c.registers.e, 0x11);
        c.execute();
        assert_eq!(c.registers.e, 0x15);
        c.execute();
        assert_eq!(c.registers.e, 0x1f);
        c.execute();
        assert_eq!(c.registers.e, 0x1f);
        c.execute();
        assert_eq!(c.registers.e, 0x25);
        c.execute();
        assert_eq!(c.registers.e, 0x2f);
        c.execute();
        assert_eq!(c.registers.e, 0x31);
        c.execute();
        assert_eq!(c.registers.e, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_h() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x2f2f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x60);
        c.bus.write_byte(0x0001, 0x61);
        c.bus.write_byte(0x0002, 0x62);
        c.bus.write_byte(0x0003, 0x63);
        c.bus.write_byte(0x0004, 0x64);
        c.bus.write_byte(0x0005, 0x65);
        c.bus.write_byte(0x0006, 0x66);
        c.bus.write_byte(0x0007, 0x67);
        c.execute();
        assert_eq!(c.registers.h, 0x11);
        c.execute();
        assert_eq!(c.registers.h, 0x15);
        c.execute();
        assert_eq!(c.registers.h, 0x1f);
        c.execute();
        assert_eq!(c.registers.h, 0x21);
        c.execute();
        assert_eq!(c.registers.h, 0x21);
        c.execute();
        assert_eq!(c.registers.h, 0x2f);
        c.execute();
        assert_eq!(c.registers.h, 0x31);
        c.execute();
        assert_eq!(c.registers.h, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_l() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x2525, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x68);
        c.bus.write_byte(0x0001, 0x69);
        c.bus.write_byte(0x0002, 0x6a);
        c.bus.write_byte(0x0003, 0x6b);
        c.bus.write_byte(0x0004, 0x6c);
        c.bus.write_byte(0x0005, 0x6d);
        c.bus.write_byte(0x0006, 0x6e);
        c.bus.write_byte(0x0007, 0x6f);
        c.execute();
        assert_eq!(c.registers.l, 0x11);
        c.execute();
        assert_eq!(c.registers.l, 0x15);
        c.execute();
        assert_eq!(c.registers.l, 0x1f);
        c.execute();
        assert_eq!(c.registers.l, 0x21);
        c.execute();
        assert_eq!(c.registers.l, 0x25);
        c.execute();
        assert_eq!(c.registers.l, 0x25);
        c.execute();
        assert_eq!(c.registers.l, 0x31);
        c.execute();
        assert_eq!(c.registers.l, 0x3f);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn mov_m() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x2f2f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x70);
        c.bus.write_byte(0x0001, 0x71);
        c.bus.write_byte(0x0002, 0x72);
        c.bus.write_byte(0x0003, 0x73);
        c.bus.write_byte(0x0004, 0x74);
        c.bus.write_byte(0x0005, 0x75);
        c.bus.write_byte(0x0006, 0x77);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x11);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x15);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x1f);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x21);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x25);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x2f);
        c.execute();
        assert_eq!(c.bus.read_byte(0x252f), 0x3f);
        assert_eq!(c.pc, 7);
    }

    #[test]
    fn mov_a() {
        let mut c = CPU::new();
        c.registers.b = 0x11;
        c.registers.c = 0x15;
        c.registers.d = 0x1F;
        c.registers.e = 0x21;
        c.registers.h = 0x25;
        c.registers.l = 0x2F;
        c.bus.write_byte(0x252f, 0x31);
        c.registers.a = 0x3F;
        c.bus.write_byte(0x0000, 0x78);
        c.bus.write_byte(0x0001, 0x79);
        c.bus.write_byte(0x0002, 0x7a);
        c.bus.write_byte(0x0003, 0x7b);
        c.bus.write_byte(0x0004, 0x7c);
        c.bus.write_byte(0x0005, 0x7d);
        c.bus.write_byte(0x0006, 0x7e);
        c.bus.write_byte(0x0007, 0x7f);
        c.execute();
        assert_eq!(c.registers.a, 0x11);
        c.execute();
        assert_eq!(c.registers.a, 0x15);
        c.execute();
        assert_eq!(c.registers.a, 0x1f);
        c.execute();
        assert_eq!(c.registers.a, 0x21);
        c.execute();
        assert_eq!(c.registers.a, 0x25);
        c.execute();
        assert_eq!(c.registers.a, 0x2f);
        c.execute();
        assert_eq!(c.registers.a, 0x31);
        c.execute();
        assert_eq!(c.registers.a, 0x31);
        assert_eq!(c.pc, 8);
    }

    #[test]
    fn hlt() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x76);
        c.execute();
        assert_eq!(c.halt, true);
        assert_eq!(c.pc, 1);
    }

    #[test]
    fn add_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x80);
        c.registers.a = 0x0f;
        c.registers.b = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x81);
        c.registers.a = 0x0f;
        c.registers.c = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x82);
        c.registers.a = 0x0f;
        c.registers.d = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x83);
        c.registers.a = 0x0f;
        c.registers.e = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x84);
        c.registers.a = 0x0f;
        c.registers.h = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x85);
        c.registers.a = 0x0f;
        c.registers.l = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x86);
        c.bus.write_byte(0x100, 0x53);
        c.registers.a = 0x0f;
        c.registers.set_hl(0x100);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x62);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn add_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x87);
        c.registers.a = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1e);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x88);
        c.registers.a = 0x0f;
        c.registers.b = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x89);
        c.registers.a = 0x0f;
        c.registers.c = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8a);
        c.registers.a = 0x0f;
        c.registers.d = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8b);
        c.registers.a = 0x0f;
        c.registers.e = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8c);
        c.registers.a = 0x0f;
        c.registers.h = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8d);
        c.registers.a = 0x0f;
        c.registers.l = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8e);
        c.bus.write_byte(0x100, 0x53);
        c.registers.a = 0x0f;
        c.registers.set_hl(0x100);
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x63);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn adc_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x8f);
        c.registers.a = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x1f);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x90);
        c.registers.a = 0x0f;
        c.registers.b = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x91);
        c.registers.a = 0x0f;
        c.registers.c = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x92);
        c.registers.a = 0x0f;
        c.registers.d = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x93);
        c.registers.a = 0x0f;
        c.registers.e = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x94);
        c.registers.a = 0x0f;
        c.registers.h = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x95);
        c.registers.a = 0x0f;
        c.registers.l = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x96);
        c.registers.a = 0x0f;
        c.bus.write_byte(0x100, 2);
        c.registers.set_hl(0x100);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x0d);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sub_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x97);
        c.registers.a = 0x0f;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sbb_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x98);
        c.registers.a = 0x0f;
        c.registers.b = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x99);
        c.registers.a = 0x0f;
        c.registers.c = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9a);
        c.registers.a = 0x0f;
        c.registers.d = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9b);
        c.registers.a = 0x0f;
        c.registers.e = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9c);
        c.registers.a = 0x0f;
        c.registers.h = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9d);
        c.registers.a = 0x0f;
        c.registers.l = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn sbb_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9e);
        c.registers.a = 0x0f;
        c.bus.write_byte(0x100, 2);
        c.registers.set_hl(0x100);
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0x0c);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, true);
        assert_eq!(c.flags.c, false);
    }

    #[test]
    fn sbb_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x9f);
        c.registers.a = 0x0f;
        c.flags.c = true;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.registers.a, 0xff);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.a, false);
        assert_eq!(c.flags.c, true);
    }

    #[test]
    fn rst_0() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xc7);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_1() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xcf);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 8);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_2() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xd7);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x10);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_3() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xdf);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x18);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_4() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xe7);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x20);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_5() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xef);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x28);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_6() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xf7);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x30);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn rst_7() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xff);
        c.sp = 0xff00;
        c.execute();
        assert_eq!(c.pc, 0x38);
        assert_eq!(c.sp, 0xfefe);
    }

    #[test]
    fn cmp_b() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xb8);
        c.bus.write_byte(0x0001, 0xb8);
        c.registers.a = 0x12;
        c.registers.b = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.b, 0x12);
        c.registers.b = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.b, 0x27);
    }

    #[test]
    fn cmp_c() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xb9);
        c.bus.write_byte(0x0001, 0xb9);
        c.registers.a = 0x12;
        c.registers.c = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.c, 0x12);
        c.registers.c = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.c, 0x27);
    }

    #[test]
    fn cmp_d() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xba);
        c.bus.write_byte(0x0001, 0xba);
        c.registers.a = 0x12;
        c.registers.d = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.d, 0x12);
        c.registers.d = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.d, 0x27);
    }

    #[test]
    fn cmp_e() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xbb);
        c.bus.write_byte(0x0001, 0xbb);
        c.registers.a = 0x12;
        c.registers.e = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.e, 0x12);
        c.registers.e = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.e, 0x27);
    }

    #[test]
    fn cmp_h() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xbc);
        c.bus.write_byte(0x0001, 0xbc);
        c.registers.a = 0x12;
        c.registers.h = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.h, 0x12);
        c.registers.h = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.h, 0x27);
    }

    #[test]
    fn cmp_l() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xbd);
        c.bus.write_byte(0x0001, 0xbd);
        c.registers.a = 0x12;
        c.registers.l = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.l, 0x12);
        c.registers.l = 0x27;
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.registers.l, 0x27);
    }

    #[test]
    fn cmp_m() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xbe);
        c.bus.write_byte(0x0001, 0xbe);
        c.registers.a = 0x12;
        c.registers.set_hl(0x100);
        c.bus.write_byte(0x100, 0x12);
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.bus.read_byte(0x100), 0x12);
        c.bus.write_byte(0x100, 0x27);
        c.execute();
        assert_eq!(c.pc, 2);
        assert_eq!(c.flags.z, false);
        assert_eq!(c.flags.c, true);
        assert_eq!(c.registers.a, 0x12);
        assert_eq!(c.bus.read_byte(0x100), 0x27);
    }

    #[test]
    fn cmp_a() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0xbf);
        c.bus.write_byte(0x0001, 0xbf);
        c.registers.a = 0x12;
        c.execute();
        assert_eq!(c.pc, 1);
        assert_eq!(c.flags.z, true);
        assert_eq!(c.flags.c, false);
        assert_eq!(c.registers.a, 0x12);
    }

    #[test]
    fn dasm() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x35);
        c.registers.set_hl(0x3412);
        assert_eq!(c.dasm(0), String::from("35        DCR (HL)"));
    }

    #[test]
    fn dasm_mvi() {
        let mut c = CPU::new();
        c.bus.write_byte(0x0000, 0x3E);
        c.bus.write_byte(0x0001, 0x55);
        assert_eq!(c.dasm(0), String::from("3E 55     MVI A,$55"));
    }

    #[test]
    fn rom_space_byte() {
        let mut c = CPU::new();
        c.bus.rom_space = Some(ROMSpace{start: 0xfff0, end: 0xffff});
        c.bus.write_byte(0xffef, 0x3E);
        c.bus.write_byte(0xfff0, 0x55);
        c.bus.write_byte(0xffff, 0x55);
        c.bus.write_byte(0x0000, 0x55);
        assert_eq!(c.bus.read_byte(0xffef), 0x3e);
        assert_eq!(c.bus.read_byte(0xfff0), 0);
        assert_eq!(c.bus.read_byte(0xffff), 0);
        assert_eq!(c.bus.read_byte(0x0000), 0x55);
    }

    #[test]
    fn rom_space_word() {
        let mut c = CPU::new();
        c.bus.rom_space = Some(ROMSpace{start: 0xfff0, end: 0xffff});
        c.bus.write_word(0xffee, 0x3E3E);
        c.bus.write_word(0xfff0, 0x5566);
        assert_eq!(c.bus.read_word(0xffee), 0x3e3e);
        assert_eq!(c.bus.read_byte(0xfff0), 0);
    }
}
