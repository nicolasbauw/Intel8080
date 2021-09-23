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

You can load assembled programs from disk to memory:
```rust
c.pc = 0x0100;                                      // sets pc to $100
c.bus.load_bin("bin/loop.bin", 0x100).unwrap();     // loads file at address $100
loop {
    c.execute();
    if c.pc == 0x0000 { break }
}
```

It's easy to create an interrupt request:
```rust
c.bus.load_bin("bin/interrupt.bin", 0).unwrap();
c.int = (true, 0xcf);               // we create an interrupt request : flag set to true
loop {                              // and its associated RST command
    c.execute();                    // test program is designed to never leave a loop
    if c.pc == 0x0000 { break }     // if it does not execute the interrupt routine
}
```

Starting with 0.8.0, a more stabilized I/O system:
```rust
// Data sent from CPU to device 1 (OUT) ? let's handle it
let value = c.bus.get_io_out(1);
if let Some(v) = value {
... }
```

Includes a "cpmloader" which loads and executes basic CP/M programs:

```
cargo run --release --example cpmloader -- bin/helloworld.bin
```

The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).


License: MIT
