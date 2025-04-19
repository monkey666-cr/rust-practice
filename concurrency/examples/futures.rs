use std::thread;
use std::time::Duration;

use futures::pin_mut;
use futures::{future::FutureExt, select};
use tokio::runtime::Runtime;
// use futures::channel::mpsc;
// use futures::executor::{self, ThreadPool};
// use futures::{pin_mut, StreamExt};

// pub fn futures_async() {
//     let pool = ThreadPool::new().unwrap();
//     let (tx, rx) = mpsc::unbounded::<i32>();

//     let fut_values = async {
//         let fut_tx_result = async move {
//             (0..100).for_each(|v| {
//                 tx.unbounded_send(v).expect("Failed to send");
//             });
//         };
//         pool.spawn_ok(fut_tx_result);

//         let fut_values = rx.map(|x| x * 2).collect();

//         fut_values.await
//     };

//     let values: Vec<i32> = executor::block_on(fut_values);

//     println!("Values={:?}, ", values);
// }

async fn select_all() {
    let future1 = async {
        thread::sleep(Duration::from_secs(1));
        "hello"
    }
    .fuse();
    let future2 = async {
        thread::sleep(Duration::from_secs(2));
        "world"
    }
    .fuse();

    pin_mut!(future1, future2);

    let result = select! {
        result1 = future1 => {result1},
        result2 = future2 => {result2},
    };

    println!("{:?}", result);

    thread::sleep(Duration::from_secs(3));
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(select_all());
}
