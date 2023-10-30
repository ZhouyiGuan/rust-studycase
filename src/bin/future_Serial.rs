use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use serialport::SerialPort;

const BUFFER_SIZE: usize = 100 * 1024;

// struct AsyncSerialPortReceive {
//     port_handle: Option<Arc<Mutex<Box<dyn SerialPort>>>>, 
// }

// impl  AsyncSerialPortReceive { 
//     async fn bind(port: Box<dyn SerialPort>) -> Self {
//         AsyncSerialPortReceive {
//             port_handle: Some(Arc::new(Mutex::new(port))),
//         }
//     }
// }

// impl Future for AsyncSerialPortReceive {
//     type Output = [u8; BUFFER_SIZE];

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<AsyncSerialPortReceive as Future>::Output> {
//         if /* 串口有数据 */ {
//             let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
//             Poll::Ready(buffer)

//         } 
//         else /* 串口没有数据 */ {
//             /* epoll wake() */
//             Poll::Pending

//         }
//     }
// }

#[tokio::main]
async fn main() {
//     let port = serialport::new("/dev/ttyUSB0",115200).open().unwrap();
//     let listener: AsyncSerialPortReceive = AsyncSerialPortReceive::bind(port).await;
//     loop{
//         let vec1 = listener.accept().await.unwrap();

//         let result = future.await;
//         tokio::spawn(async move {
//             process(vec1).await;
//         });
//         assert_eq!(result,[0; BUFFER_SIZE])
//     }
}