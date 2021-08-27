# Intel 8080 Emulator

Yet another Intel 8080 Emulator, written in Rust. So far it passes the TST8080 and 8080PRE tests.

````
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC
 VERSION 1.0  (C) 1980


 CPU IS OPERATIONAL
````

````
8080 Preliminary tests complete
````

TODO:
- pass the other tests
- interrupts
- in / out
- clock

Includes a "cpmloader" which loads and executes basic CP/M programs:

````
cargo run --release --example cpmloader -- bin/helloworld.bin
````

the source code helloworld.asm can be assembled with [Retro Assembler](https://enginedesigns.net/retroassembler/).