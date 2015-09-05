// Crates
extern crate serial;
extern crate time;
extern crate rustc_serialize;

// Modules
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

// Functions
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_broke() {
        assert!(false);
    }

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

}
