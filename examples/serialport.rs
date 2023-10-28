use serialport::SerialPort;
use std::time::Duration;
use tokio::io::{unix::AsyncFd, ReadBuf};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;


struct AsyncSerialPort {
   // inner: AsyncFd<Arc<Mutex<Box<dyn SerialPort>>>>,
}

#[tokio::main]
async fn main() {
   // 打开串口  
   let mut vec_ports: Vec<Box<dyn SerialPort>> = Vec::new();
   vec_ports.push(serialport::new("/dev/ttyUSB1",115200).open().expect("Failed to open port"));
   vec_ports.push(serialport::new("/dev/ttyUSB2",115200).open().expect("Failed to open port"));
   vec_ports.push(serialport::new("/dev/ttyUSB3",115200).open().expect("Failed to open port"));
   vec_ports.push(serialport::new("/dev/ttyUSB4",115200).open().expect("Failed to open port"));

   // 注册监听串口
   register(vec_ports);

   loop {
      // 等待串口触发事件
      // let port_handle: Box<dyn SerialPort> = ports.accept().await.unwrap();


      tokio::spawn(async move {
         let buffer = read_from_serialport().await;

         process(&buffer);
     });
   }
}

fn register(vec_port:Vec<Box<dyn SerialPort>>){}

async fn read_from_serialport() -> Vec<u8> {
   vec![1,1]
}

async fn write_to_tracefile(buffer: &Vec<u8>) {}
async fn write_to_forwardport(buffer: &Vec<u8>) {}
async fn process(buffer: &Vec<u8>) {
   write_to_tracefile(&buffer).await;
   write_to_forwardport(&buffer).await;
}
