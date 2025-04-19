use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use rayon::ThreadPoolBuilder;

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));

    return a + b;
}

fn rayon_pool_create() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("work-{}", i))
        .build()
        .unwrap();

    let n = pool.install(|| fib(20));

    println!("{}", n);
}

scoped_tls::scoped_thread_local!(static POOL_DATA: Vec<i32>);

fn rayon_thread_scoped() {
    let pool_data = vec![1, 2, 3];

    assert!(!POOL_DATA.is_set());

    rayon::ThreadPoolBuilder::new()
        .build_scoped(
            |thread| {
                let data = pool_data.clone();
                POOL_DATA.set(&data, || thread.run())
            },
            |pool| {
                pool.install(|| {
                    thread::sleep(Duration::from_secs(1));
                    POOL_DATA.with(|d| {
                        println!("{:?}", d);
                    })
                });
                pool.install(|| {
                    POOL_DATA.with(|d| {
                        println!("{:?}", d);
                    })
                });
            },
        )
        .unwrap();

    drop(pool_data);

    thread::sleep(Duration::from_secs(5));
}

fn threadpool_create() {
    let pool = threadpool::ThreadPool::new(4);

    let (sender, receiver) = channel();

    for i in 0..8 {
        let sender = sender.clone();
        pool.execute(move || {
            let result = i * 2;
            sender.send(result).unwrap();
        });
    }

    for _ in 0..8 {
        let result = receiver.recv().expect("接受失败");
        println!("任务结果 {}", result);
    }
}

fn rusty_pool_example() {
    let pool = rusty_pool::ThreadPool::default();

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool!");
        });
    }

    pool.join();

    let handle = pool.evaluate(|| {
        thread::sleep(Duration::from_secs(5));
        return 4;
    });
    let result = handle.await_complete();
    println!("{}", result);
}

async fn some_async_fn(x: i32, y: i32) -> i32 {
    return x + y;
}

fn rusty_pool_example2() {
    let pool = rusty_pool::ThreadPool::default();

    let handle = pool.complete(async {
        let a = some_async_fn(4, 5).await;
        let b = some_async_fn(a, 5).await;
        let c = some_async_fn(b, 5).await;
        some_async_fn(c, 5).await
    });
    handle.await_complete();

    pool.join();
}

fn scheduled_thread_pool() {
    let (sender, reciever) = channel();

    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let handle = pool.execute_after(Duration::from_secs(1), move || {
        println!("Hello from a scheduled_thread_pool!");
        sender.send("done").unwrap();
    });

    let _ = handle;
    reciever.recv().unwrap();
}

fn main() {
    rayon_pool_create();

    // rayon_thread_scoped();

    // threadpool_create();

    // rusty_pool_example();

    // rusty_pool_example2();
    scheduled_thread_pool();
}
