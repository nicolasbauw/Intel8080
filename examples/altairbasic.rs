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
    c.bus.load_bin(&a[1], 0x0)?;

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    c.bus.set_io_in(255, 0x00);

    c.bus.set_io_in(0, 0x80);

    loop {
        //c.debug = true;
        c.execute();
        if c.pc == 0x0000 { break };
    }
    Ok(())
}


