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
    handle_sub_msg: Mutex<Option<JoinHandle<Result<(), RedisError>>>> /* handle_sub_msg保存了sub_msg这个持续运行的任务的句柄(开始任务的时候可以存入,想要终止任务的时候可以关闭这个句柄即可) */
}

impl RedisClient {
    pub fn new() -> Self {
        Self {
            client: redis::Client::open("redis://192.168.1.33/").unwrap(),
            sub_msg_callbacks: Arc::new(Mutex::new(HashMap::new())),
            handle_sub_msg: Mutex::new(None)
        }
    }

    pub async fn start<'a>(&self) {
        self.stop().await;
        let mut mutex = self.handle_sub_msg.lock().await;
        let map = self.sub_msg_callbacks.lock().await;
        let channels: Vec<String> = map.keys().cloned().collect();/* 获取map中所有的keys的迭代器,然后cloned会克隆迭代器中每一项,最后collect会收集到一个集合中 */
        *mutex = Some(tokio::spawn(sub_redis_msg(channels)));
    }

    pub async fn stop(&self) {
        let mutex = self.handle_sub_msg.lock().await;
        if (*mutex).is_some() {
            mutex.as_ref().unwrap().abort();
        }
    }

    pub async fn publish(&self, channel: &str, message: &str) -> redis::RedisResult<()> {
        let mut publish_conn = self.client.get_async_connection().await?;
        publish_conn.publish(channel, message).await?;
        Ok(())
    }

    pub async fn subscribe(&self, channel: &str, cb: CallBack) -> Uuid {
        println!("sub to channel:{}",channel);
        let mut map = self.sub_msg_callbacks.lock().await;
        let uuid = Uuid::new_v4();
        map.insert(String::from(channel), (uuid, cb));/* 每个channel会有一个自己的uuid以及对应的消息处理函数 */
        uuid/* 使用这个函数可以获取channel的uuid */
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


static REDIS_CLIENT: OnceCell<RedisClient> = OnceCell::const_new();/* 全局变量(唯一单例) */

/* 调用这个函数就是获取这个单例的引用.和我们用lasy_static然后解锁是一样的 */
pub async fn get_redis_ins() -> &'static RedisClient {
    REDIS_CLIENT.get_or_init(|| async {
        println!("init a redis client");
        RedisClient::new()
    }).await
}

/* 关键!!!! 把loop放进一个async的task中!!!这样的话这个loop就会异步执行,不需要创建线程单独跑一个任务.我们只需要把不同的任务spawn进一个任务 */
async fn sub_redis_msg(channels: Vec<String>) -> redis::RedisResult<()> {
    println!("listening to sub channels...");
    let mut pubsub_conn = get_redis_ins().await
        .client
        .get_async_connection()
        .await?
        .into_pubsub();
    pubsub_conn.psubscribe(channels).await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    loop {
        let msg = pubsub_stream.next().await;/* 这里的next就是检查这个流是否有数据,没数据就挂起等待 */
        if msg.is_none() {/* 这里是none的可能性是流被关闭了,而不是检查是否有数据 */
            continue;
        }
        let msg = msg.unwrap();
        let channel = msg.get_channel_name();
        let payload = msg.get_payload_bytes();

        let map = get_redis_ins().await.sub_msg_callbacks.lock().await;
        let (_, cb) = match map.get(channel) {
            Some(cb) => cb,
            None => continue,
        };
        cb(channel, payload);
    }
}
