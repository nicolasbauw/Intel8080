# intel8080

Yet another Intel 8080 Emulator. So far it passes the TST8080 and 8080PRE tests.

Basic exemple for a small loop:
```rust
use intel8080::CPU;
let mut c = CPU::new();
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

You can also load assembled programs from disk to memory:
```rust
c.bus.load_bin("bin/helloworld.bin", 0x100).unwrap();   // loads file at address $100
```

Includes a "cpmloader" which loads and executes basic CP/M programs:

```
cargo run --release --example cpmloader -- bin/helloworld.bin
```

The provided example source code helloworld.asm can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).
The assembled version is in the bin/ directory.

TODO:
- pass the other tests
- interrupts
- in / out
- clock

License: MIT
