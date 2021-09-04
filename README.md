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

Includes a "cpmloader" which loads and executes basic CP/M programs (and every pure 8080 program):

```
cargo run --release --example cpmloader -- bin/helloworld.bin
```

It can be used for debugging:

```
cargo run --example cpmloader -- bin/loop.bin > /tmp/test.txt
````

```text
opcode : 0x3e	PC : 0x0102	SP : 0xff00	S : 0	Z : 0	A : 0	P : 0	C : 0
B : 0x00	C : 0x00	D : 0x00	E : 0x00	H : 0x00	L : 0x00	A : 0x0f
-----------------------------------------------------------------------------------------------------
...
```

The provided source code examples can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).
The assembled versions are in the bin/ directory.

TODO:
- pass the other tests
- interrupts (WIP)
- clock

License: MIT
