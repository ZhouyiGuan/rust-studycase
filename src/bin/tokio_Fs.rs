/* 
    发现io如果spawn出去的话数据有可能是乱序的,就是后来的数据先被处理了.
    与TcpStream的区别是Tcp每个连接之间是独立的,而对于串口读取以及文件读写
    都是需要按照顺序来的
    所以io这里必须是确保顺序执行()
*/

/* 
    API的使用
*/


use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file_handle = File::open("trace_origin").await?;

    loop{
        let mut buffer = [0; 2];
        let n = file_handle.read(&mut buffer[..]).await?;
        if n > 0 {
            // situation1:顺序执行 await每个process完成
            /* process(buffer).await; */

            // situation2:异步执行 创建异步任务,该任务会被tokio放入任务队列
            tokio::spawn(async move {
                process(buffer).await;
            });
        }
        else {
            println!("sleep for 1s");
            sleep(Duration::from_secs(1)).await;
        }
    }
}


async fn process (buffer: [u8;2])  {
    sleep(Duration::from_secs(1)).await;
    println!("The bytes: {:?}", buffer);
}