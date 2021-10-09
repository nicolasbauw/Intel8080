use crate::CPU;

impl CPU {
    pub fn dasm(&self) -> String {
        let opcode = self.bus.read_byte(self.pc);
        match opcode {
            /* Carry bit instructions */
            0x3f => String::from("CMC"),                                    // CMC
            0x37 => String::from("STC"),                                    // STC
    
            /* Single register instructions */
            // INR Increment Register or Memory
            0x04 => String::from("INR B"),                                  // INR B
            0x0C => String::from("INR C"),                                  // INR C
            0x14 => String::from("INR D"),                                  // INR D
            0x1C => String::from("INR E"),                                  // INR E
            0x24 => String::from("INR H"),                                  // INR H
            0x2C => String::from("INR L"),                                  // INR L
            0x3C => String::from("INR A"),                                  // INR A
            0x34 => String::from("INR (HL)"),                               // INR (HL)

            // DCR Decrement Register or Memory
            0x05 => String::from("DCR B"),                                  // DCR B
            0x0D => String::from("DCR C"),                                  // DCR C
            0x15 => String::from("DCR D"),                                  // DCR D
            0x1D => String::from("DCR E"),                                  // DCR E
            0x25 => String::from("DCR H"),                                  // DCR H
            0x2D => String::from("DCR L"),                                  // DCR L
            0x3D => String::from("DCR A"),                                  // DCR A
            0x35 => String::from("DCR (HL)"),                               // DCR (HL)

            // CMA Complement Accumulator
            0x2F => String::from("CMA"),                                    // CMA
    
            // Decimal adjust accumulator
            0x27 => String::from("DAA"),                                    // DAA
    
            // NOP No Operation
            0x00 => String::from("NOP"),                                    // NOP
    
            // MOV Data transfer instructions
            0x40 => String::from("MOV B,B"),                                // MOV B,B
            0x41 => String::from("MOV B,C"),                                // MOV B,C
            0x42 => String::from("MOV B,D"),                                // MOV B,D
            0x43 => String::from("MOV B,E"),                                // MOV B,E
            0x44 => String::from("MOV B,H"),                                // MOV B,H
            0x45 => String::from("MOV B,L"),                                // MOV B,L
            0x46 => String::from("MOV B,(HL)"),                             // MOV B,(HL)
            0x47 => String::from("MOV B,A"),                                // MOV B,A
    
            0x48 => String::from("MOV C,B"),                                // MOV C,B                                                     // MOV B,B
            0x49 => String::from("MOV C,C"),                                // MOV C,C
            0x4A => String::from("MOV C,D"),                                // MOV C,D
            0x4B => String::from("MOV C,E"),                                // MOV C,E
            0x4C => String::from("MOV C,H"),                                // MOV C,H
            0x4D => String::from("MOV C,L"),                                // MOV C,L
            0x4E => String::from("MOV C,(HL)"),                             // MOV C,(HL)
            0x4F => String::from("MOV C,A"),                                // MOV C,A
    
            0x50 => String::from("MOV D,B"),                                // MOV D,B                                                     // MOV B,B
            0x51 => String::from("MOV D,C"),                                // MOV D,C
            0x52 => String::from("MOV D,D"),                                // MOV D,D
            0x53 => String::from("MOV D,E"),                                // MOV D,E
            0x54 => String::from("MOV D,H"),                                // MOV D,H
            0x55 => String::from("MOV D,L"),                                // MOV D,L
            0x56 => String::from("MOV D,(HL)"),                             // MOV D,(HL)
            0x57 => String::from("MOV D,A"),                                // MOV D,A
    
            0x58 => String::from("MOV E,B"),                                // MOV E,B                                                     // MOV B,B
            0x59 => String::from("MOV E,C"),                                // MOV E,C
            0x5A => String::from("MOV E,D"),                                // MOV E,D
            0x5B => String::from("MOV E,E"),                                // MOV E,E
            0x5C => String::from("MOV E,H"),                                // MOV E,H
            0x5D => String::from("MOV E,L"),                                // MOV E,L
            0x5E => String::from("MOV E,(HL)"),                             // MOV E,(HL)
            0x5F => String::from("MOV E,A"),                                // MOV E,A
    
            0x60 => String::from("MOV H,B"),                                // MOV H,B                                                     // MOV B,B
            0x61 => String::from("MOV H,C"),                                // MOV H,C
            0x62 => String::from("MOV H,D"),                                // MOV H,D
            0x63 => String::from("MOV H,E"),                                // MOV H,E
            0x64 => String::from("MOV H,H"),                                // MOV H,H
            0x65 => String::from("MOV H,L"),                                // MOV H,L
            0x66 => String::from("MOV H,(HL)"),                             // MOV H,(HL)
            0x67 => String::from("MOV H,A"),                                // MOV H,A
    
            0x68 => String::from("MOV L,B"),                                // MOV L,B                                                     // MOV B,B
            0x69 => String::from("MOV L,C"),                                // MOV L,C
            0x6A => String::from("MOV L,D"),                                // MOV L,D
            0x6B => String::from("MOV L,E"),                                // MOV L,E
            0x6C => String::from("MOV L,H"),                                // MOV L,H
            0x6D => String::from("MOV L,L"),                                // MOV L,L
            0x6E => String::from("MOV L,(HL)"),                             // MOV L,(HL)
            0x6F => String::from("MOV L,A"),                                // MOV L,A
    
            0x70 => String::from("MOV (HL),B"),                             // MOV (HL), B
            0x71 => String::from("MOV (HL),C"),                             // MOV (HL), C
            0x72 => String::from("MOV (HL),D"),                             // MOV (HL), D
            0x73 => String::from("MOV (HL),E"),                             // MOV (HL), E
            0x74 => String::from("MOV (HL),H"),                             // MOV (HL), H
            0x75 => String::from("MOV (HL),L"),                             // MOV (HL), L
    
            0x76 => String::from("HLT"),                                    // HLT
    
            0x77 => String::from("MOV (HL),A"),                             // MOV (HL), A
    
            0x78 => String::from("MOV A,B"),                                // MOV A,B                                                     // MOV B,B
            0x79 => String::from("MOV A,C"),                                // MOV A,C
            0x7A => String::from("MOV A,D"),                                // MOV A,D
            0x7B => String::from("MOV A,E"),                                // MOV A,E
            0x7C => String::from("MOV A,H"),                                // MOV A,H
            0x7D => String::from("MOV A,L"),                                // MOV A,L
            0x7E => String::from("MOV A,(HL)"),                             // MOV A,(HL)
            0x7F => String::from("MOV A,A"),                                // MOV A,A
    
            // STAX Store accumulator
            0x02 => String::from("STAX B"),                                 // STAX B
            0x12 => String::from("STAX D"),                                 // STAX D
    
            // LDAX Load accumulator
            0x0A => String::from("LDAX B"),                                 // LDAX B
            0x1A => String::from("LDAX D"),                                 // LDAX D

            /* Register or Memory to Accumulator instructions*/
            // ADD register or memory to accumulator
            0x80 => String::from("ADD B"),                              // ADD B
            0x81 => String::from("ADD C"),                              // ADD C
            0x82 => String::from("ADD D"),                              // ADD D
            0x83 => String::from("ADD E"),                              // ADD E
            0x84 => String::from("ADD H"),                              // ADD H
            0x85 => String::from("ADD L"),                              // ADD L
            0x86 => String::from("ADD (HL)"),                           // ADD (HL)
            0x87 => String::from("ADD A"),                              // ADD A
    
            // ADC Add register or memory to accumulator with carry
            0x88 => String::from("ADC B"),                              // ADC B
            0x89 => String::from("ADC C"),                              // ADC C
            0x8A => String::from("ADC D"),                              // ADC D
            0x8B => String::from("ADC E"),                              // ADC E
            0x8C => String::from("ADC H"),                              // ADC H
            0x8D => String::from("ADC L"),                              // ADC L
            0x8E => String::from("ADC (HL)"),                           // ADC (HL)
            0x8F => String::from("ADC A"),                              // ADC A
    
            // SUB Substract register or memory to accumulator
            0x90 => String::from("SUB B"),                              // SUB B
            0x91 => String::from("SUB C"),                              // SUB C
            0x92 => String::from("SUB D"),                              // SUB D
            0x93 => String::from("SUB E"),                              // SUB E
            0x94 => String::from("SUB H"),                              // SUB H
            0x95 => String::from("SUB L"),                              // SUB L
            0x96 => String::from("SUB (HL)"),                           // SUB (HL)
            0x97 => String::from("SUB A"),                              // SUB A
    
            // SBB Substract register or memory to accumulator with borrow
            0x98 => String::from("SBB B"),                              // SBB B
            0x99 => String::from("SBB C"),                              // SBB C
            0x9A => String::from("SBB D"),                              // SBB D
            0x9B => String::from("SBB E"),                              // SBB E
            0x9C => String::from("SBB H"),                              // SBB H
            0x9D => String::from("SBB L"),                              // SBB L
            0x9E => String::from("SBB (HL)"),                           // SBB (HL)
            0x9F => String::from("SBB A"),                              // SBB A
    
            // ANA Logical AND register or memory with accumulator
            0xA0 => String::from("ANA B"),                              // ANA B
            0xA1 => String::from("ANA C"),                              // ANA C
            0xA2 => String::from("ANA D"),                              // ANA D
            0xA3 => String::from("ANA E"),                              // ANA E
            0xA4 => String::from("ANA H"),                              // ANA H
            0xA5 => String::from("ANA L"),                              // ANA L
            0xA6 => String::from("ANA (HL)"),                           // ANA (HL)
            0xA7 => String::from("ANA A"),                              // ANA A
    
            // XRA Logical Exclusive-OR register or memory with accumulator
            0xA8 => String::from("XRA B"),                              // XRA B
            0xA9 => String::from("XRA C"),                              // XRA C
            0xAA => String::from("XRA D"),                              // XRA D
            0xAB => String::from("XRA E"),                              // XRA E
            0xAC => String::from("XRA H"),                              // XRA H
            0xAD => String::from("XRA L"),                              // XRA L
            0xAE => String::from("XRA (HL)"),                           // XRA (HL)
            0xAF => String::from("XRA A"),                              // XRA A
    
            // ORA Logical OR register or memory with accumulator
            0xB0 => String::from("ORA B"),                              // ORA B
            0xB1 => String::from("ORA C"),                              // ORA C
            0xB2 => String::from("ORA D"),                              // ORA D
            0xB3 => String::from("ORA E"),                              // ORA E
            0xB4 => String::from("ORA H"),                              // ORA H
            0xB5 => String::from("ORA L"),                              // ORA L
            0xB6 => String::from("ORA (HL)"),                           // ORA (HL)
            0xB7 => String::from("ORA A"),                               // ORA A
    
            // CMP Compare register or memory with accumulator
            0xB8 => String::from("CMP B"),                               // CMP B
            0xB9 => String::from("CMP C"),                               // CMP C
            0xBA => String::from("CMP D"),                               // CMP D
            0xBB => String::from("CMP E"),                               // CMP E
            0xBC => String::from("CMP H"),                               // CMP H
            0xBD => String::from("CMP L"),                               // CMP L
            0xBE => String::from("CMP (HL)"),                            // CMP (HL)
            0xBF => String::from("CMP A"),                               // CMP A
    
            /* Rotate accumulator instructions */
            0x07 => String::from("RLC"),                                 // RLC
            0x0F => String::from("RRC"),                                 // RRC
            0x17 => String::from("RAL"),                                 // RAL
            0x1F => String::from("RAR"),                                 // RAR
    
            /* Register pair instructions */
            // PUSH data onto stack
            0xC5 => String::from("PUSH B"),                              // PUSH B
            0xD5 => String::from("PUSH D"),                              // PUSH D
            0xE5 => String::from("PUSH H"),                              // PUSH H
            0xF5 => String::from("PUSH PSW"),                            // PUSH PSW
    
            // POP data off stack
            0xC1 => String::from("POP B"),                               // POP B
            0xD1 => String::from("POP D"),                               // POP D
            0xE1 => String::from("POP H"),                               // POP H
            0xF1 => String::from("POP PSW"),                             // POP PSW

            // DAD Double add
            0x09 => String::from("DAD B"),                               // DAD B
            0x19 => String::from("DAD D"),                               // DAD D
            0x29 => String::from("DAD H"),                               // DAD H
            0x39 => String::from("DAD SP"),                              // DAD SP

            // INX Increment register pair
            0x03 => String::from("INX B"),                               // INX B
            0x13 => String::from("INX D"),                               // INX D
            0x23 => String::from("INX H"),                               // INX H
            0x33 => String::from("INX SP"),                              // INX SP             
    
            // DCX Decrement register pair
            0x0B => String::from("DCX B"),                               // DCX B
            0x1B => String::from("DCX D"),                               // DCX D
            0x2B => String::from("DCX H"),                               // DCX H
            0x3B => String::from("DCX SP"),                              // DCX SP
    
            // XCHG Exchange registers
            0xEB => String::from("XCHG"),
    
            // XTHL Exchange stack
            0xE3 => String::from("XTHL"),
    
            // SPHL Load SP from H and L
            0xF9 => String::from("SPHL"),
    
            /* Immediate instructions */
            // LXI Move immediate data
            0x01 => {                                                       // LXI B
                let d16 = self.bus.read_word(self.pc + 1); 
                format!("LXI B,${:4x}", d16)
            },
            0x11 => {                                                       // LXI D
                let d16 = self.bus.read_word(self.pc + 1); 
                format!("LXI D,${:4x}", d16)
            },
            0x21 => {                                                       // LXI H
                let d16 = self.bus.read_word(self.pc + 1); 
                format!("LXI H,${:4x}", d16)
            },
            0x31 => {                                                       // LXI SP
                let d16 = self.bus.read_word(self.pc + 1); 
                format!("LXI SP,${:4x}", d16)
            },

            // MVI Move immediate data
            0x06 => {                                                       // MVI B,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI B,${:2x}", d8)
            },
            0x0E => {                                                       // MVI C,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI C,${:2x}", d8)
            },
            0x16 => {                                                       // MVI D,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI D,${:2x}", d8)
            },
            0x1E => {                                                       // MVI E,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI E,${:2x}", d8)
            },
            0x26 => {                                                       // MVI H,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI H,${:2x}", d8)
            },
            0x2E => {                                                       // MVI L,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI L,${:2x}", d8)
            },
            0x36 => {                                                       // MVI (HL),d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI (HL),${:2x}", d8)
            },
            0x3E => {                                                       // MVI A,d8
                let d8 = self.bus.read_byte(self.pc + 1);
                format!("MVI A,${:2x}", d8)
            },
    
                // ADI add immediate to accumulator
                /*0xC6 => {                                                       // ADI
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
                    self.registers.a = self.bus.get_io_in(self.bus.read_byte(self.pc+1));
                    if self.debug { println!("IN : device : {}, value : {:#04x}", usize::from(self.bus.read_byte(self.pc+1)), self.registers.a); }
                },
                // OUT Output
                0xD3 => {
                    let device = self.bus.read_byte(self.pc+1);
                    self.bus.set_io_out(device, self.registers.a);
                    if self.debug { println!("OUT : device : {}, value : {:#04x}", device, self.registers.a); }
                },*/

                _ => String::new()
        
    }
}
}