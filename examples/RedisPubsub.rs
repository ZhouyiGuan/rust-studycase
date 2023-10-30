use rust_studycase::mod_redispubsub::*;


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
