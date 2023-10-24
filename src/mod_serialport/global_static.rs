use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::fs::File;
use tokio_serial::SerialPort;

pub struct PORT {
    pub port_handle: Mutex<Option<Box<dyn SerialPort>>>, 
    pub tracefile_handle: Mutex<Option<File>>,
}

impl PORT {
    pub fn new() -> Self {
        PORT {
            port_handle : Mutex::new(Option::None),
            tracefile_handle : Mutex::new(Option::None),
        }
    }
    pub fn port_handle(&self,handle: Box<dyn SerialPort>) {
        let mut port_handle = self.port_handle.lock();
        *port_handle = Option::Some(handle);
        drop(port_handle);
    } 
    pub fn tracefile_handle(&self,handle: File) {
        let mut port_handle = self.tracefile_handle.lock();
        *port_handle = Option::Some(handle);
        drop(port_handle);
    } 
}

lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}