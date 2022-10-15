use std::{ error::Error, process, thread, time::Duration };
use intel8080::CPU;

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let mut c = CPU::new();
    c.debug.io = true;

    // Loads assembled program into memory
    c.bus.load_bin("bin/out_a.bin", 0)?;

    // io.0 is the sender, io.1 is the receiver. Used to send / receive a (device, data) tuple to / from a peripheral.
    let io_receiver1 = c.bus.io_out.1.clone();

    // Demonstration peripheral 0x07 listens data sent by the CPU
    thread::spawn(move || {
        loop {
            if let Ok((device, data)) = io_receiver1.try_recv() {
                if device == 0x07 { println!("The 0x07 peripheral received {:#04X} from the CPU", data) }
            }
        }
    });

    // A basic program which waits a moment then sends the 0xBB byte to the 0x07 peripheral
    loop {
        c.execute_slice();
        if c.pc == 0x0000 { thread::sleep(Duration::from_millis(500)); break }
    }
    Ok(())
}
