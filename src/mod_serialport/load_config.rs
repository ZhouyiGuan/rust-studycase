use lazy_static::lazy_static;
use parking_lot::Mutex;
use crate::mod_serialport::global_static::PORT;

lazy_static! {
    static ref PORTS: Mutex<Vec<PORT>> = Mutex::new(Vec::new());
}


pub fn init () {
    println!("hello");
}

fn init_p () {}