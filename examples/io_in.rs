use std::{ error::Error, process, thread };
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
    let periph1_sender = c.bus.io_in.0.clone();
    let periph1_req_receiver = c.bus.io_req.1.clone();

    // Demonstration peripheral 0x07 sends the 0xDE message when a IN instruction occurs
    thread::spawn(move || {
        // IN instruction automatically sends a request message through the io_req channel
        // So the peripheral knows when he can send the message to the CPU via the io_in sender
        loop {
            if let Ok(device) = periph1_req_receiver.recv() {
                // IN instruction for the 0x07 device ?
                if device == 0x07 {
                    println!("The 0x07 peripheral puts 0xDE on the data bus");
                    // We send the data through the io_in channel
                    periph1_sender.send((0x07, 0xDE)).unwrap();
                }
            }
        }
    });

    // A single loop which waits for the 0xDE byte to be sent by the 0x07 peripheral
    loop {
        c.execute_slice();
        if c.pc == 0x0000 { break }
    }
    Ok(())
}
