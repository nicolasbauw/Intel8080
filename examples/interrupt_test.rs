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
    c.debug.switch = true;

    // Loads assembled program into memory
    c.bus.load_bin(&a[1], 0x000)?;

    c.sp = 0xff00;
    c.inte = false;
    c.int = (true, 0xcf);

    loop {
        c.execute();
        println!("{}\n", c.debug.string);
        if c.pc == 0x0000 { break }             //  if CP/M warm boot -> we exit
    }
    Ok(())
}
