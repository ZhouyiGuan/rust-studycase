/* 
    实现future trait 
*/


use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

// 为我们的 Delay 类型实现 Future 特征
impl Future for Delay {
    type Output = Vec<u8>; 

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Delay as Future>::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();

            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            // 第一次调用 `poll`，生成计时器线程(模拟等待的资源)
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                // 计时结束，通过调用 `waker` 来通知执行器
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            println!("Hello world");
            let buffer = vec![1,2];
            Poll::Ready(buffer)
        } else {
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let future: Delay = Delay { when:Instant::now() + Duration::from_millis(1000),waker: Option::None };

    // 运行并等待 Future 的完成
    let result = future.await;

    assert_eq!(result,vec![1,2])
}