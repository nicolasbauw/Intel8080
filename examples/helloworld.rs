use intel8080::*;

fn main() {
    let mut c = CPU::new();
    // Loads assembled program into memory
    c.bus.load_bin("examples/helloworld.bin", 0x100).unwrap();
    
    // RET at 0x05 for mocking of CP/M BDOS system calls
    c.bus.write_word(0x0005, 0xc9);

    // Setting PC to 0x0100 (CP/M Binaries are loaded with a 256 byte offset)
    c.pc = 0x0100;

    // Setting stack
    c.sp = 0xFF00;

    loop {
        c.execute();
        if c.pc == 0x0005 { bdos_call(&c) }
        if c.pc == 0x0000 { break }
    }
}

fn bdos_call(c: &CPU) {
    #[cfg(debug_assertions)]
    {
        println!("BDOS CALL");
    }

    if c.registers.c == 0x09 {
        let mut a = c.registers.get_de();
        loop {
            let c = c.bus.read_byte(a);
            if c as char == '$' {
                println!("");
                break;
            } else {
                a += 1;
            }
            print!("{}", c as char);
        }
    }
    if c.registers.c == 0x02 {
        print!("{}", c.registers.e as char);
    }
}
