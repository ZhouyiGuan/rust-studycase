// extern crate lazy_static;
use std::{collections::HashMap, sync::Arc};

use futures_util::StreamExt as _;
use redis::{RedisError, AsyncCommands};
use tokio::{sync::{Mutex, OnceCell}, task::JoinHandle};
use uuid::Uuid;

type CallBack = fn(channel: &str, msg: &[u8]);

pub struct RedisClient {
    client: redis::Client,
    sub_msg_callbacks: Arc<Mutex<HashMap<String, (Uuid, CallBack)>>>,
    handler: Mutex<Option<JoinHandle<Result<(), RedisError>>>>
}

impl RedisClient {
    pub fn new() -> Self {
        Self {
            client: redis::Client::open("redis://192.168.1.33/").unwrap(),
            sub_msg_callbacks: Arc::new(Mutex::new(HashMap::new())),
            handler: Mutex::new(None)
        }
    }

    pub async fn start<'a>(&self) {
        self.stop().await;
        let mut mutex = self.handler.lock().await;
        let map = self.sub_msg_callbacks.lock().await;
        let patterns: Vec<String> = map.keys().cloned().collect();
        *mutex = Some(tokio::spawn(sub_redis_msg(patterns)));
    }

    pub async fn stop(&self) {
        let mutex = self.handler.lock().await;
        if (*mutex).is_some() {
            mutex.as_ref().unwrap().abort();
        }
    }

    pub async fn publish(&self, channel: &str, message: &str) -> redis::RedisResult<()> {
        let mut publish_conn = self.client.get_async_connection().await?;
        publish_conn.publish(channel, message).await?;
        Ok(())
    }

    pub async fn subscribe(&self, pattern: &str, cb: CallBack) -> Uuid {
        println!("sub echo");
        let mut map = self.sub_msg_callbacks.lock().await;
        let uuid = Uuid::new_v4();
        map.insert(String::from(pattern), (uuid, cb));
        uuid
    }

    pub async fn unsubscribe(&self, uuid: Uuid) {
        let mut map = self.sub_msg_callbacks.lock().await;
        let ele = map.iter().find(|&x| x.1 .0 == uuid);
        let key = match ele {
            Some(e) => String::from(e.0),
            None => return
        };
        map.remove(&key);
    }
}

static REDIS_CLIENT: OnceCell<RedisClient> = OnceCell::const_new();
pub async fn get_redis_ins() -> &'static RedisClient {
    REDIS_CLIENT.get_or_init(|| async {
        println!("new redis client");
        RedisClient::new()
    }).await
}

async fn sub_redis_msg(pattern: Vec<String>) -> redis::RedisResult<()> {
    println!("new sub redis msg!!!!!!");
    let mut pubsub_conn = get_redis_ins().await
        .client
        .get_async_connection()
        .await?
        .into_pubsub();
    pubsub_conn.psubscribe(pattern).await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    loop {
        let msg = pubsub_stream.next().await;
        if msg.is_none() {
            continue;
        }
        let msg = msg.unwrap();
        // let pattern = msg.get_pattern::<String>();
        let channel = msg.get_channel_name();
        let payload = msg.get_payload_bytes();
        // println!("channel: {}, data: {:?}", channel, payload);

        // println!("pattern: {}", pattern.unwrap_or(String::from("none")));
        // println!("map: {:?}", REDIS_CLIENT.sub_msg_callbacks);

        let map = get_redis_ins().await.sub_msg_callbacks.lock().await;
        let (_, cb) = match map.get(channel) {
            Some(cb) => cb,
            None => continue,
        };
        cb(channel, payload);

        // tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // let msg = match msg {
        //     Some(msg) => (msg.get_channel_name(), msg.get_payload_bytes()),
        //     None => continue,
        // };
        // println!("channel: {}, data: {:?}", msg.0, msg.1);
    }
}
