use std::sync::Arc;
use serialport::SerialPort;
use tokio::{sync::OnceCell, task::JoinHandle};
use parking_lot::{RwLock,Mutex};

pub struct COM_CONFIG {
    pub portname: String,
    pub baudrate: u32,
    pub portpath: String,
    pub uuid: String,
    pub activated: bool,
    pub traceformat: String,
    pub forwardports: Vec<String>,
    pub autosend: bool,
    pub default_config: String
}

pub struct COM {
    port_handle: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    port_config: RwLock<COM_CONFIG>,
}

static GLOBAL_COMS: OnceCell<Vec<COM>> = OnceCell::const_new();

pub fn init_global_coms(configs: Vec<COM_CONFIG>) {
    if GLOBAL_COMS.get().is_some() {
        println!("GLOBAL_COMS is already initialized.");
        return;
    }

    println!("init COMS");
    let mut vec_coms: Vec<COM> = Vec::new();
    for config in configs {
        vec_coms.push(COM::new(config));
    }

    if let Err(_) = GLOBAL_COMS.set(vec_coms) {
        println!("Failed to initialize COMS.");
    }
}

pub fn get_global_coms() -> Option<&'static Vec<COM>> {
    GLOBAL_COMS.get()
}



impl COM {
    pub fn new(config: COM_CONFIG) -> COM {
        COM {
            port_handle: Arc::new(Mutex::new(None)),
            port_config: RwLock::new(config),
        }
    }
}
