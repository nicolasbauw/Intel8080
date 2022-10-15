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

    // In this example periph is the entry function that simulates a peripheral. It runs in a separate thread.
    thread::spawn(move || {
        periph(io_receiver1);
    });

    // A basic program which waits a moment then sends the 0xBB byte to the 0x07 peripheral
    loop {
        c.execute_slice();
        if c.pc == 0x0000 { thread::sleep(Duration::from_millis(500)); break }
    }
    Ok(())
}

// Demonstration peripheral 0x07 listens data sent by the CPU
fn periph(rx: crossbeam_channel::Receiver<(u8, u8)>) {
    loop {
        if let Ok((device, data)) = rx.try_recv() {
            if device == 0x07 { println!("The 0x07 peripheral received {:#04X} from the CPU", data) }
        }
    }
}
