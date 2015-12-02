// Crates
extern crate serial;
extern crate time;
extern crate rustc_serialize;

// Modules
mod lib;
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
        thread::sleep_ms(1000);
        // Basic
        loop {
            interact(&mut port).unwrap(); // interact with port
        }
    }
}

// Interact with serial port
fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    let in_buf: Vec<u8> = (0..15).collect();
    let mut out_buf: Vec<u8> = (0..17).collect();
    let mut t1 = time::precise_time_ns();
    let mut t2 = time::precise_time_ns();
    // Set configuration
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::seconds(3)));
    thread::sleep_ms(350); // fails when slower

    // Write value
    println!("[WARN] writing to device...");
    t2 = time::precise_time_ns();
    try!(port.write(&in_buf[..]));
    println!("\tok, {} ns", (time::precise_time_ns() - t2));

    // Read responses
    println!("[WARN] reading from device...");
    t2 = time::precise_time_ns();
    try!(port.read(&mut out_buf[..]));
    println!("\tok, {} ns", (time::precise_time_ns() - t2));

    // As JSON obj
    println!("[WARN] parsing to JSON...");
    let s = String::from_utf8_lossy(&out_buf);
    println!("\tstring: {}", s);
    let optb = json::Json::from_str("{\"foo\":null,\"bar\":null}").unwrap();
    let d = json::Json::from_str(&s).unwrap_or(optb); // if fails, default None obj
    println!("\tdata: {}", d);
    println!("\tis object: {}", d.is_object());
    let obj = d.as_object().unwrap();

    // Get 'foo'
    let foo = obj.get("bar").unwrap();
    println!("\tis foo u64? {}", foo.is_u64());

    // Iterate through object
    for (key, value) in obj.iter() {
        println!("\t{}: {}", key, match *value {
            json::Json::U64(v) => format!("{} (u64)", v),
            json::Json::Null => format!("null"),
            json::Json::String(ref v) => format!("{} (string)", v),
            _ => format!("other")
        });
    }
    t2 = time::precise_time_ns();
    println!("\tdone, {} Hz", 1000000000 / (t2 - t1));
    return Ok(()); // return Result type
}
