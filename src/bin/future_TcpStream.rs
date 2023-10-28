use tokio::net::{TcpListener, TcpStream};
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::io::Error;

#[tokio::main]
async fn main()  {
    let listener =  TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process (socket : TcpStream)  {
    sleep(Duration::from_secs(1)).await;
}