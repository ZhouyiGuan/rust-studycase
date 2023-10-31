use rust_studycase::mod_redispubsub::*;
/* 使用的逻辑顺序: 第一次调用get_redis_ins()时会创建一个redis客户端的实例,然后我们需要指定他订阅哪些channels,这个实例会保存我们订阅的消息;然后调用start()函数,其中会创建一个sub_msg的任务,把我们的订阅消息传进这个任务,我们在这个任务里打开redis连接,就会持续不断监听这些channels传来的消息 */

#[tokio::main]
async fn main() {

    get_redis_ins()
        .await
        .subscribe("SOME.d3fs_slave@1.heartbeat.notification", |s, v| {
            println!("callback: {}, {:?}", s, v);
        })
        .await;

    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

    get_redis_ins()
        .await
        .subscribe("SOME.gpio.heartbeat.notification", |s, v| {
            println!("callback: {}, {:?}", s, v);
        })
        .await;

    get_redis_ins().await.start().await;

    println!("test!!!");
}
