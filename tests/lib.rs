extern crate arduino;

#[test]
fn foobar() {
    let arg = "/dev/ttyACM0";
    println!("opening port: {:?}", arg);
}
