use std::{ env, error::Error, process };
use intel8080::*;

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let  a: Vec<String> = env::args().collect();
    let mut c = CPU::new();
    // Loads assembled program into memory
    c.bus.load_bin(&a[1], 0x100).unwrap();
    
    // RET at 0x05 for mocking of CP/M BDOS system calls
    c.bus.write_word(0x0005, 0xc9);

    // Setting PC to 0x0100 (CP/M Binaries are loaded with a 256 byte offset)
    c.pc = 0x0100;

    /* Setting up stack : by reverse engineering assembled CP/M software, it seems
    that the $0006 address is read to set the stack */
    c.bus.write_word(0x0006, 0xFF00);
    
    /* Setting up stack in case of the program does not read the $0006 address
    and does not set any stack. */
    c.sp = 0xFF00;

    loop {
        c.execute();
        if c.pc == 0x0005 { bdos_call(&c) }
        if c.pc == 0x0000 { break }             //  if CP/M warm boot -> we exit
    }
    Ok(())
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
