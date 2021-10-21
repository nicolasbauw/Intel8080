use crate::CPU;

impl CPU {
    /// Disassembles code at (address)
    pub fn dasm(&self, address: u16) -> String {
        let opcode = self.bus.read_byte(address);
        match opcode {
            /* Carry bit instructions */
            0x3f => String::from("3F        CMC"),                                    // CMC
            0x37 => String::from("37        STC"),                                    // STC
    
            /* Single register instructions */
            // INR Increment Register or Memory
            0x04 => String::from("04        INR B"),                                  // INR B
            0x0C => String::from("0C        INR C"),                                  // INR C
            0x14 => String::from("14        INR D"),                                  // INR D
            0x1C => String::from("1C        INR E"),                                  // INR E
            0x24 => String::from("24        INR H"),                                  // INR H
            0x2C => String::from("2C        INR L"),                                  // INR L
            0x3C => String::from("3C        INR A"),                                  // INR A
            0x34 => String::from("34        INR (HL)"),                               // INR (HL)

            // DCR Decrement Register or Memory
            0x05 => String::from("05        DCR B"),                                  // DCR B
            0x0D => String::from("0D        DCR C"),                                  // DCR C
            0x15 => String::from("15        DCR D"),                                  // DCR D
            0x1D => String::from("1D        DCR E"),                                  // DCR E
            0x25 => String::from("25        DCR H"),                                  // DCR H
            0x2D => String::from("2D        DCR L"),                                  // DCR L
            0x3D => String::from("3D        DCR A"),                                  // DCR A
            0x35 => String::from("35        DCR (HL)"),                               // DCR (HL)

            // CMA Complement Accumulator
            0x2F => String::from("2F        CMA"),                                    // CMA
    
            // Decimal adjust accumulator
            0x27 => String::from("27        DAA"),                                    // DAA
    
            // NOP No Operation
            0x00 => String::from("00        NOP"),                                    // NOP
    
            // MOV Data transfer instructions
            0x40 => String::from("40        MOV B,B"),                                // MOV B,B
            0x41 => String::from("41        MOV B,C"),                                // MOV B,C
            0x42 => String::from("42        MOV B,D"),                                // MOV B,D
            0x43 => String::from("43        MOV B,E"),                                // MOV B,E
            0x44 => String::from("44        MOV B,H"),                                // MOV B,H
            0x45 => String::from("45        MOV B,L"),                                // MOV B,L
            0x46 => String::from("46        MOV B,(HL)"),                             // MOV B,(HL)
            0x47 => String::from("47        MOV B,A"),                                // MOV B,A
    
            0x48 => String::from("48        MOV C,B"),                                // MOV C,B                                                     // MOV B,B
            0x49 => String::from("49        MOV C,C"),                                // MOV C,C
            0x4A => String::from("4A        MOV C,D"),                                // MOV C,D
            0x4B => String::from("4B        MOV C,E"),                                // MOV C,E
            0x4C => String::from("4C        MOV C,H"),                                // MOV C,H
            0x4D => String::from("4D        MOV C,L"),                                // MOV C,L
            0x4E => String::from("4E        MOV C,(HL)"),                             // MOV C,(HL)
            0x4F => String::from("4F        MOV C,A"),                                // MOV C,A
    
            0x50 => String::from("50        MOV D,B"),                                // MOV D,B                                                     // MOV B,B
            0x51 => String::from("51        MOV D,C"),                                // MOV D,C
            0x52 => String::from("52        MOV D,D"),                                // MOV D,D
            0x53 => String::from("53        MOV D,E"),                                // MOV D,E
            0x54 => String::from("54        MOV D,H"),                                // MOV D,H
            0x55 => String::from("55        MOV D,L"),                                // MOV D,L
            0x56 => String::from("56        MOV D,(HL)"),                             // MOV D,(HL)
            0x57 => String::from("57        MOV D,A"),                                // MOV D,A
    
            0x58 => String::from("58        MOV E,B"),                                // MOV E,B                                                     // MOV B,B
            0x59 => String::from("59        MOV E,C"),                                // MOV E,C
            0x5A => String::from("5A        MOV E,D"),                                // MOV E,D
            0x5B => String::from("5B        MOV E,E"),                                // MOV E,E
            0x5C => String::from("5C        MOV E,H"),                                // MOV E,H
            0x5D => String::from("5D        MOV E,L"),                                // MOV E,L
            0x5E => String::from("5E        MOV E,(HL)"),                             // MOV E,(HL)
            0x5F => String::from("5F        MOV E,A"),                                // MOV E,A
    
            0x60 => String::from("60        MOV H,B"),                                // MOV H,B                                                     // MOV B,B
            0x61 => String::from("61        MOV H,C"),                                // MOV H,C
            0x62 => String::from("62        MOV H,D"),                                // MOV H,D
            0x63 => String::from("63        MOV H,E"),                                // MOV H,E
            0x64 => String::from("64        MOV H,H"),                                // MOV H,H
            0x65 => String::from("65        MOV H,L"),                                // MOV H,L
            0x66 => String::from("66        MOV H,(HL)"),                             // MOV H,(HL)
            0x67 => String::from("67        MOV H,A"),                                // MOV H,A
    
            0x68 => String::from("68        MOV L,B"),                                // MOV L,B                                                     // MOV B,B
            0x69 => String::from("69        MOV L,C"),                                // MOV L,C
            0x6A => String::from("6A        MOV L,D"),                                // MOV L,D
            0x6B => String::from("6B        MOV L,E"),                                // MOV L,E
            0x6C => String::from("6C        MOV L,H"),                                // MOV L,H
            0x6D => String::from("6D        MOV L,L"),                                // MOV L,L
            0x6E => String::from("6E        MOV L,(HL)"),                             // MOV L,(HL)
            0x6F => String::from("6F        MOV L,A"),                                // MOV L,A
    
            0x70 => String::from("70        MOV (HL),B"),                             // MOV (HL), B
            0x71 => String::from("71        MOV (HL),C"),                             // MOV (HL), C
            0x72 => String::from("72        MOV (HL),D"),                             // MOV (HL), D
            0x73 => String::from("73        MOV (HL),E"),                             // MOV (HL), E
            0x74 => String::from("74        MOV (HL),H"),                             // MOV (HL), H
            0x75 => String::from("75        MOV (HL),L"),                             // MOV (HL), L
    
            0x76 => String::from("76        HLT"),                                    // HLT
    
            0x77 => String::from("77        MOV (HL),A"),                             // MOV (HL), A
    
            0x78 => String::from("78        MOV A,B"),                                // MOV A,B                                                     // MOV B,B
            0x79 => String::from("79        MOV A,C"),                                // MOV A,C
            0x7A => String::from("7A        MOV A,D"),                                // MOV A,D
            0x7B => String::from("7B        MOV A,E"),                                // MOV A,E
            0x7C => String::from("7C        MOV A,H"),                                // MOV A,H
            0x7D => String::from("7D        MOV A,L"),                                // MOV A,L
            0x7E => String::from("7E        MOV A,(HL)"),                             // MOV A,(HL)
            0x7F => String::from("7F        MOV A,A"),                                // MOV A,A
    
            // STAX Store accumulator
            0x02 => String::from("02        STAX B"),                                 // STAX B
            0x12 => String::from("12        STAX D"),                                 // STAX D
    
            // LDAX Load accumulator
            0x0A => String::from("0A        LDAX B"),                                 // LDAX B
            0x1A => String::from("1A        LDAX D"),                                 // LDAX D

            /* Register or Memory to Accumulator instructions*/
            // ADD register or memory to accumulator
            0x80 => String::from("80        ADD B"),                              // ADD B
            0x81 => String::from("81        ADD C"),                              // ADD C
            0x82 => String::from("82        ADD D"),                              // ADD D
            0x83 => String::from("83        ADD E"),                              // ADD E
            0x84 => String::from("84        ADD H"),                              // ADD H
            0x85 => String::from("85        ADD L"),                              // ADD L
            0x86 => String::from("86        ADD (HL)"),                           // ADD (HL)
            0x87 => String::from("87        ADD A"),                              // ADD A
    
            // ADC Add register or memory to accumulator with carry
            0x88 => String::from("88        ADC B"),                              // ADC B
            0x89 => String::from("89        ADC C"),                              // ADC C
            0x8A => String::from("8A        ADC D"),                              // ADC D
            0x8B => String::from("8B        ADC E"),                              // ADC E
            0x8C => String::from("8C        ADC H"),                              // ADC H
            0x8D => String::from("8D        ADC L"),                              // ADC L
            0x8E => String::from("8E        ADC (HL)"),                           // ADC (HL)
            0x8F => String::from("8F        ADC A"),                              // ADC A
    
            // SUB Substract register or memory to accumulator
            0x90 => String::from("90        SUB B"),                              // SUB B
            0x91 => String::from("91        SUB C"),                              // SUB C
            0x92 => String::from("92        SUB D"),                              // SUB D
            0x93 => String::from("93        SUB E"),                              // SUB E
            0x94 => String::from("94        SUB H"),                              // SUB H
            0x95 => String::from("95        SUB L"),                              // SUB L
            0x96 => String::from("96        SUB (HL)"),                           // SUB (HL)
            0x97 => String::from("97        SUB A"),                              // SUB A
    
            // SBB Substract register or memory to accumulator with borrow
            0x98 => String::from("98        SBB B"),                              // SBB B
            0x99 => String::from("99        SBB C"),                              // SBB C
            0x9A => String::from("9A        SBB D"),                              // SBB D
            0x9B => String::from("9B        SBB E"),                              // SBB E
            0x9C => String::from("9C        SBB H"),                              // SBB H
            0x9D => String::from("9D        SBB L"),                              // SBB L
            0x9E => String::from("9E        SBB (HL)"),                           // SBB (HL)
            0x9F => String::from("9F        SBB A"),                              // SBB A
    
            // ANA Logical AND register or memory with accumulator
            0xA0 => String::from("A0        ANA B"),                              // ANA B
            0xA1 => String::from("A1        ANA C"),                              // ANA C
            0xA2 => String::from("A2        ANA D"),                              // ANA D
            0xA3 => String::from("A3        ANA E"),                              // ANA E
            0xA4 => String::from("A4        ANA H"),                              // ANA H
            0xA5 => String::from("A5        ANA L"),                              // ANA L
            0xA6 => String::from("A6        ANA (HL)"),                           // ANA (HL)
            0xA7 => String::from("A7        ANA A"),                              // ANA A
    
            // XRA Logical Exclusive-OR register or memory with accumulator
            0xA8 => String::from("A8        XRA B"),                              // XRA B
            0xA9 => String::from("A9        XRA C"),                              // XRA C
            0xAA => String::from("AA        XRA D"),                              // XRA D
            0xAB => String::from("AB        XRA E"),                              // XRA E
            0xAC => String::from("AC        XRA H"),                              // XRA H
            0xAD => String::from("AD        XRA L"),                              // XRA L
            0xAE => String::from("AE        XRA (HL)"),                           // XRA (HL)
            0xAF => String::from("AF        XRA A"),                              // XRA A
    
            // ORA Logical OR register or memory with accumulator
            0xB0 => String::from("B0        ORA B"),                              // ORA B
            0xB1 => String::from("B1        ORA C"),                              // ORA C
            0xB2 => String::from("B2        ORA D"),                              // ORA D
            0xB3 => String::from("B3        ORA E"),                              // ORA E
            0xB4 => String::from("B4        ORA H"),                              // ORA H
            0xB5 => String::from("B5        ORA L"),                              // ORA L
            0xB6 => String::from("B6        ORA (HL)"),                           // ORA (HL)
            0xB7 => String::from("B7        ORA A"),                               // ORA A
    
            // CMP Compare register or memory with accumulator
            0xB8 => String::from("B8        CMP B"),                               // CMP B
            0xB9 => String::from("B9        CMP C"),                               // CMP C
            0xBA => String::from("BA        CMP D"),                               // CMP D
            0xBB => String::from("BB        CMP E"),                               // CMP E
            0xBC => String::from("BC        CMP H"),                               // CMP H
            0xBD => String::from("BD        CMP L"),                               // CMP L
            0xBE => String::from("BE        CMP (HL)"),                            // CMP (HL)
            0xBF => String::from("BF        CMP A"),                               // CMP A
    
            /* Rotate accumulator instructions */
            0x07 => String::from("07        RLC"),                                 // RLC
            0x0F => String::from("0F        RRC"),                                 // RRC
            0x17 => String::from("17        RAL"),                                 // RAL
            0x1F => String::from("1F        RAR"),                                 // RAR
    
            /* Register pair instructions */
            // PUSH data onto stack
            0xC5 => String::from("C5        PUSH B"),                              // PUSH B
            0xD5 => String::from("D5        PUSH D"),                              // PUSH D
            0xE5 => String::from("E5        PUSH H"),                              // PUSH H
            0xF5 => String::from("F5        PUSH PSW"),                            // PUSH PSW
    
            // POP data off stack
            0xC1 => String::from("C1        POP B"),                               // POP B
            0xD1 => String::from("D1        POP D"),                               // POP D
            0xE1 => String::from("E1        POP H"),                               // POP H
            0xF1 => String::from("F1        POP PSW"),                             // POP PSW

            // DAD Double add
            0x09 => String::from("09        DAD B"),                               // DAD B
            0x19 => String::from("19        DAD D"),                               // DAD D
            0x29 => String::from("29        DAD H"),                               // DAD H
            0x39 => String::from("39        DAD SP"),                              // DAD SP

            // INX Increment register pair
            0x03 => String::from("03        INX B"),                               // INX B
            0x13 => String::from("13        INX D"),                               // INX D
            0x23 => String::from("23        INX H"),                               // INX H
            0x33 => String::from("33        INX SP"),                              // INX SP             
    
            // DCX Decrement register pair
            0x0B => String::from("0B        DCX B"),                               // DCX B
            0x1B => String::from("1B        DCX D"),                               // DCX D
            0x2B => String::from("2B        DCX H"),                               // DCX H
            0x3B => String::from("3B        DCX SP"),                              // DCX SP
    
            // XCHG Exchange registers
            0xEB => String::from("EB        XCHG"),
    
            // XTHL Exchange stack
            0xE3 => String::from("E3        XTHL"),
    
            // SPHL Load SP from H and L
            0xF9 => String::from("F9        SPHL"),
    
            /* Immediate instructions */
            // LXI Move immediate data
            0x01 => {                                                       // LXI B
                let d16 = self.bus.read_word(address + 1);
                let d16_le = self.bus.read_le_word(address + 1);
                format!("01 {:04x}   LXI B,${:04x}", d16_le , d16)
            },
            0x11 => {                                                       // LXI D
                let d16 = self.bus.read_word(address + 1);
                let d16_le = self.bus.read_le_word(address + 1);
                format!("11 {:04x}   LXI D,${:04x}", d16_le, d16)
            },
            0x21 => {                                                       // LXI H
                let d16 = self.bus.read_word(address + 1);
                let d16_le = self.bus.read_le_word(address + 1);
                format!("21 {:04x}   LXI H,${:04x}", d16_le, d16)
            },
            0x31 => {                                                       // LXI SP
                let d16 = self.bus.read_word(address + 1);
                let d16_le = self.bus.read_le_word(address + 1);
                format!("31 {:04x}   LXI SP,${:04x}", d16_le, d16)
            },

            // MVI Move immediate data
            0x06 => {                                                       // MVI B,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("06 {:02x}     MVI B,${:02x}",d8, d8)
            },
            0x0E => {                                                       // MVI C,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("0E {:02x}     MVI C,${:02x}",d8, d8)
            },
            0x16 => {                                                       // MVI D,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("16 {:02x}     MVI D,${:02x}",d8 ,d8)
            },
            0x1E => {                                                       // MVI E,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("1E {:02x}     MVI E,${:02x}",d8 ,d8)
            },
            0x26 => {                                                       // MVI H,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("26 {:02x}     MVI H,${:02x}",d8 ,d8)
            },
            0x2E => {                                                       // MVI L,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("2E {:02x}     MVI L,${:02x}",d8 ,d8)
            },
            0x36 => {                                                       // MVI (HL),d8
                let d8 = self.bus.read_byte(address + 1);
                format!("36 {:02x}     MVI (HL),${:02x}",d8 ,d8)
            },
            0x3E => {                                                       // MVI A,d8
                let d8 = self.bus.read_byte(address + 1);
                format!("3E {:02x}     MVI A,${:02x}",d8 ,d8)
            },
    
            // ADI add immediate to accumulator
            0xC6 => {                                                       // ADI
                let n = self.bus.read_byte(address + 1);
                format!("C6 {:02x}     ADI ${:02x}",n ,n)
            },
    
            // ACI add immediate to accumulator with carry
            0xCE => {                                                       // ACI
                let n = self.bus.read_byte(address + 1);
                format!("CE {:02x}     ACI ${:02x}",n ,n)
            },
    
            // SUI substract immediate from accumulator
            0xD6 => {                                                       // SUI
                let n = self.bus.read_byte(address + 1);
                format!("D6 {:02x}     SUI ${:02x}",n ,n)
            },
    
            // SBI substract immediate from accumulator with borrow
            0xDE => {                                                       // SBI
                let n = self.bus.read_byte(address + 1);
                format!("DE {:02x}     SBI ${:02x}",n ,n)
            },
    
            // ANI and immediate with accumulator
            0xE6 => {                                                       // ANI
                let n = self.bus.read_byte(address + 1);
                format!("E6 {:02x}     ANI ${:02x}",n ,n)
            },
    
            // XRI exclusive-or immediate with accumulator
            0xEE => {                                                       // XRI
                let n = self.bus.read_byte(address + 1);
                format!("EE {:02x}     XRI ${:02x}",n ,n)
            },
    
            // ORI or immediate with accumulator
            0xF6 => {                                                       // ORI
                let n = self.bus.read_byte(address + 1);
                format!("F6 {:02x}     ORI ${:02x}",n ,n)
            },
    
            // CPI compare immediate with accumulator
            0xFE => {                                                       // CPI
                let n = self.bus.read_byte(address + 1);
                format!("FE {:02x}     CPI ${:02x}",n ,n)
            },
    
            /* Direct addressing instructions */
            // STA Store accumulator direct
            0x32 => {                                                       // STA
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("32 {:04x}   STA ${:04x}",addr_le ,addr)
            },
    
            // LDA Store accumulator direct
            0x3A => {                                                       // LDA
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("3A {:04x}   LDA ${:04x}",addr_le ,addr)
            },
    
            // SHLD Store H and L direct
            0x22 => {                                                       // SHLD
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("22 {:04x}   SHLD ${:04x}",addr_le, addr)
            },
    
            // LHLD Load H and L direct
            0x2A => {                                                       // LHLD
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("2A {:04x}   LHLD ${:04x}",addr_le, addr)
            },

            /* JUMP instructions */
            // Load program counter
            0xE9 => String::from("addressHL"),                                   // addressHL
            // JMP Jump
            0xC3 => {                                                       // JMP
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("C3 {:04x}   JMP ${:04x}", addr_le,addr)
            },
            // JC Jump if carry
            0xDA => {                                                       // JC
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("DA {:04x}   JC ${:04x}", addr_le,addr)
            },
            // JNC Jump if no carry
            0xD2 => {                                                       // JNC
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("D2 {:04x}   JNC ${:04x}", addr_le,addr)
            },
            // JZ Jump if zero
            0xCA => {                                                       // JZ
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("CA {:04x}   JZ ${:04x}", addr_le,addr)
            },
            // JNZ Jump if not zero
            0xC2 => {                                                       // JNZ
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("C2 {:04x}   JNZ ${:04x}", addr_le,addr)
            },
            // JM Jump if minus
            0xFA => {                                                       // JM
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("FA {:04x}   JM ${:04x}", addr_le,addr)
            },
            // JP Jump if positive
            0xF2 => {                                                       // JP
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("F2 {:04x}   JP ${:04x}", addr_le,addr)
            },
            // JPE Jump if parity even
            0xEA => {                                                       // JPE
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("EA {:04x}   JPE ${:04x}", addr_le,addr)
            },
            // JPO Jump if parity odd
            0xE2 => {                                                       // JPO
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("E2 {:04x}   JPO ${:04x}", addr_le,addr)
            },
    
            /* Call subroutine instructions */
            // CALL
            0xCD => {                                                       // CALL
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("CD {:04x}   CALL ${:04x}", addr_le, addr)
            },
            // CC Call if carry
            0xDC => {                                                       // CC
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("DC {:04x}   CC ${:04x}", addr_le, addr)
            },
            // CNC Call if no carry
            0xD4 => {                                                       // CNC
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("D4 {:04x}   CNC ${:04x}", addr_le, addr)
            },
            // CZ Call if zero
            0xCC => {                                                       // CZ
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("CC {:04x}   CZ ${:04x}", addr_le, addr)
            },
            // CNZ Call if not zero
            0xC4 => {                                                       // CNZ
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("C4 {:04x}   CNZ ${:04x}", addr_le, addr)
            },
            // CM Call if minus
            0xFC => {                                                       // CM
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("FC {:04x}   CM ${:04x}", addr_le, addr)
            },
            // CP Call if plus
            0xF4 => {                                                       // CP
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("F4 {:04x}   CP ${:04x}", addr_le, addr)
            },
            // CPE Call if parity even
            0xEC => {                                                       // CPE
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("EC {:04x}   CPE ${:04x}", addr_le, addr)
            },
            // CPO Call if parity odd
            0xE4 => {                                                       // CPO
                let addr = self.bus.read_word(address + 1);
                let addr_le = self.bus.read_le_word(address + 1);
                format!("E4 {:04x}   CPO ${:04x}", addr_le, addr)
            },
    
            /* Return from subroutine instructions */
            // RET Return
            0xC9 => String::from("C9        RET"),                                // RET
            // RC Return if carry
            0xD8 => String::from("D8        RC"),                                 // RC
            // RNC Return if no carry
            0xD0 => String::from("D0        RNC"),                                // RNC
            // RZ Return if zero
            0xC8 => String::from("C8        RZ"),                                 // RZ
            // RNZ Return if not zero
            0xC0 => String::from("C0        RNZ"),                                // RNZ
            // RM Return if minus
            0xF8 => String::from("F8        RM"),                                 // RM
            // RP Return if plus
            0xF0 => String::from("F0        RP"),                                 // RP
            // RPE Return if parity even
            0xE8 => String::from("E8        RPE"),                                // RPE
            // RPO Return if parity odd
            0xE0 => String::from("E0        RPO"),                                // RPO
    
            /* Interrupt flip-flop instructions */
            // EI Enable interrupts
            0xFB => String::from("FB        EI"), 
            // DI Disable Interrupts
            0xF3 => String::from("F3        DI"), 

            /* RST (Restart) instructions */
            0xC7 => String::from("C7        RST 0"), 

            0xCF => String::from("CF        RST 1"),

            0xD7 => String::from("D7        RST 2"),

            0xDF => String::from("DF        RST 3"),

            0xE7 => String::from("E7        RST 4"),

            0xEF => String::from("EF        RST 5"),

            0xF7 => String::from("F7        RST 6"),

            0xFF => String::from("FF        RST 7"),
    
            /* Input / output instructions */
            // IN Input
            0xDB => {
                let device = self.bus.read_byte(address+1);
                format!("DB {:02x}     IN ${:02x}", device, device)
            },
            // OUT Output
            0xD3 => {
                let device = self.bus.read_byte(address+1);
                format!("D3 {:02x}     OUT ${:02x}", device, device)
            },

            _ => String::new()
        
        }
    }
}