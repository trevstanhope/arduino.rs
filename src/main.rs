extern crate serial;
extern crate time;

use std::env;
use time::Duration;
use std::io::prelude::*;
use serial::prelude::*;

// Port Settings
const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};

fn main() {
    for arg in env::args_os().skip(1) { // Get arguments
        println!("opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap(); // interact with port
    }
}

// Interact with serial port
fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::seconds(2)));

    loop {
        let mut buf: Vec<u8> = (0..5).collect();

        //println!("writing bytes");
        //try!(port.write(&buf[..]));
        try!(port.read(&mut buf[..]));
        let b = String::from_utf8_lossy(&buf);
        println!("{}", b);
    }
}