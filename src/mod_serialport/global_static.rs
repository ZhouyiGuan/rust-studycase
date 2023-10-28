use parking_lot::Mutex;
use tokio::fs::File;
use tokio_serial::SerialPort;

pub struct PORT {
    pub port_handle: Mutex<Option<Box<dyn SerialPort>>>, 
    pub tracefile_handle: Mutex<Option<File>>,
    pub forwardport_handle: Mutex<Option<Box<dyn SerialPort>>>, 
}

impl PORT {
    pub fn new() -> Self {
        PORT {
            port_handle : Mutex::new(Option::None),
            tracefile_handle : Mutex::new(Option::None),
            forwardport_handle : Mutex::new(Option::None),
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
    pub fn forwardport_handle(&self,handle: File) {} 
}

