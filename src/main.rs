extern crate serial;
extern crate time;
extern crate rustc_serialize;

use std::thread;
use std::env;
use time::Duration;
use std::io::prelude::*;
use serial::prelude::*;
use rustc_serialize::json;

// Port Settings
const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};

// Main
fn main() {
    for arg in env::args_os().skip(1) { // Get arguments
        println!("opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap(); // interact with port
    }
}

// Interact with serial port
fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    let in_buf: Vec<u8> = (0..15).collect();
    let mut out_buf: Vec<u8> = (0..20).collect();

    // Set configuration
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::seconds(3)));
    thread::sleep_ms(1000);

    // Write value
    println!("writing to device...");
    try!(port.write(&in_buf[..]));
    thread::sleep_ms(1000);

    // Read responses
    println!("reading from device...");
    try!(port.read(&mut out_buf[..]));
    thread::sleep_ms(100);

    // As String    
    //let s = String::from_utf8_lossy(&out_buf);
    let s = "{\"foo\":0,\"bar\":1}";
    println!("string: {}", s);

    // As JSON obj
    let d = json::Json::from_str(&s).unwrap();
    println!("data: {}", d);
    println!("is object: {}", d.is_object());
    let obj = d.as_object().unwrap();

    // Get 'foo'
    let foo = obj.get("bar").unwrap();
    println!("is foo u64? {}", foo.is_u64());

    // Iterate through object
    for (key, value) in obj.iter() {
        println!("{}: {}", key, match *value {
            json::Json::U64(v) => format!("{} (u64)", v),
            json::Json::String(ref v) => format!("{} (string)", v),
            _ => format!("other")
        });
    }
    return Ok(()); // return Result type
}
