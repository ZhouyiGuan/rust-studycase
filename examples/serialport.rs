use tokio::io::AsyncReadExt;
use tokio_serial::{Serial, SerialPortSettings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置串口
    let settings = SerialPortSettings::default();
    let mut port = Serial::from_path("/dev/ttyUSB0", &settings)?;

    // 读取数据
    let mut buf = vec![0u8; 1024];
    loop {
        let n = port.read(&mut buf).await?;
        if n == 0 {
            break; // EOF
        }
        println!("Received: {:?}", &buf[..n]);
    }

    Ok(())
}
