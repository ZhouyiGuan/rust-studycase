use tokio::time::Duration;
use tokio::time::sleep;
use tokio::io::Error;
use serialport::SerialPort;

#[tokio::main]
async fn main()  {
    tokio::spawn(async move {
        let result = recv().await;
    });
    sleep(Duration::from_secs(20)).await;
}

async fn recv ()  {
    let mut port = serialport::new("/dev/ttyS0", 115_200)
        .open()
        .expect("Failed to open port"); 
    loop{
        let mut buf: [u8;1024] = [0;1024];
        let num: usize = port.read(buf.as_mut_slice()).unwrap();
        process(buf,num).await; 
    }
    
}

async fn process (buf: [u8;1024],num: usize)  {
   println!("process data...");
}