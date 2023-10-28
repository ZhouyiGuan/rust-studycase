use rust_studycase::mod_RedisPubsub::*;


#[tokio::main]
async fn main() {
    // let _ = tokio::spawn(sub_redis_msg());

    // let s1 = String::from("abc");
    // tokio::spawn(REDIS_CLIENT
    //     .subscribe("SOME.d3fs_slave@1.heartbeat.notification", |s, v| {
    //         println!("callback: {}, {:?}", s, v);
    //     })
    // );

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
    // tokio::spawn(REDIS_CLIENT
    //     .subscribe("SOME.gpio.*", |s, v| {
    //         println!("callback: {}, {:?}", s, v);
    //     })
    // );

    //

    // let _ = sub_redis_msg();
    // match ret {
    //     Ok(_) => println!("OK"),
    //     Err(err) => eprintln!("{}", err)
    // }
    println!("test!!!");

    // let ret = ret.await.unwrap().unwrap();
    // println!("test??? {}", ret);

    // let router = get_router();
    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(router.into_make_service())
    //     .await
    //     .unwrap();
}
