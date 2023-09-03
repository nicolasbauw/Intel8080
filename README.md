# intel8080

[![Current Crates.io Version](https://img.shields.io/crates/v/intel8080.svg)](https://crates.io/crates/intel8080)
[![Current docs Version](https://docs.rs/intel8080/badge.svg)](https://docs.rs/intel8080)
[![Downloads badge](https://img.shields.io/crates/d/intel8080.svg)](https://crates.io/crates/intel8080)

Yet another Intel 8080 Emulator. It passes the TST8080, 8080PRE, CPUTEST and 8080EXM tests.

Example for a small loop:
```rust
use intel8080::CPU;
let mut c = CPU::new();
c.pc = 0x0100;                      // sets pc to $100
// Here we create a small machine code program for demo purpose.
// Usually you will rather load an assembled code in memory (see below).
c.bus.write_byte(0x0100, 0x3e);     // MVI A,$0F
c.bus.write_byte(0x0101, 0x0F);
c.bus.write_byte(0x0102, 0x3d);     // DCR A
c.bus.write_byte(0x0103, 0xc2);     // JNZ $0102
c.bus.write_word(0x0104, 0x0102);
c.bus.write_byte(0x0106, 0xc9);     // RET
loop {
    c.execute();
    if c.pc == 0x0000 { break }
}
```

Debug mode outputs CPU state and disassembled code to an internal string after each execute():
```
3E 0f     MVI A,$0f
PC : 0x0003	SP : 0xff00	S : 0	Z : 0	A : 0	P : 0	C : 0
B : 0x00	C : 0x00	D : 0x00	E : 0x00	H : 0x00	L : 0x00 ...
```

Includes a "cpmloader" which loads and executes basic CP/M programs:

```
cargo run --release --example cpmloader -- bin/helloworld.bin
```

You can also check my [Altair 8800 / 88-SIO / teletype emulator](https://github.com/nicolasbauw/Altair8800).

The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).


License: MIT
