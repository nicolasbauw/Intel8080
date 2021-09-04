# intel8080

Yet another Intel 8080 Emulator. It passes the TST8080 and 8080PRE tests.

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
c.inte = false;                     // we start with interrupts disabled, for testing
c.int = (true, 0xcf);               // we create an interrupt request : flag set to true
loop {                              // with its assiociated RST command
    c.execute();                    // test program is designed to never leave the loop
    if c.pc == 0x0000 { break }     // if it does not execute interrupt routine
}
```

Includes a "cpmloader" which loads and executes basic CP/M programs:

```
cargo run --release --example cpmloader -- bin/helloworld.bin
```

The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).
The assembled versions are in the bin/ directory.

TODO:
- pass the other tests
- clock

License: MIT
