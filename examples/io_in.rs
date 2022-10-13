use std::{ error::Error, process, time::Duration, thread };
use intel8080::CPU;

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let mut c = CPU::new();
    c.debug.io = false;

    // Loads assembled program into memory
    c.bus.load_bin("bin/in_a.bin", 0)?;

    // io.0 is the sender, io.1 is the receiver. Used to send / receive a (device, data) tuple to / from a peripheral.
    let io_sender1 = c.bus.io.0.clone();

    // In this example periph is the entry function that simulates a peripheral. It runs in a separate thread.
    thread::spawn(move || {
        periph(io_sender1);
    });

    // A single loop which waits for the 0xDE byte to be sent by the 0x07 peripheral
    loop {
        c.execute_slice();
        if c.pc == 0x0000 { break }
    }
    Ok(())
}

// Demonstration peripheral 0x07 sends 0xDE on the data bus after 1 second
fn periph(tx: crossbeam_channel::Sender<(u8, u8)>) {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        println!("The 0x07 peripheral puts 0xDE on the data bus");
        tx.send((0x07, 0xDE)).unwrap();
    }
}
