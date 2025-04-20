// use std::sync::Arc;

// use tokio::sync::Notify;

// #[tokio::main]
// async fn main() {
//     let notify = Arc::new(Notify::new());
//     let notify2 = notify.clone();

//     let handle = tokio::spawn(async move {
//         notify2.notified().await;
//         println!("received notification");
//     });

//     println!("sending notification");
//     notify.notify_one();

//     handle.await.unwrap();
// }

use std::sync::Arc;

use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified1 = notify.notified();
    let notified2 = notify.notified();

    let _handle = tokio::spawn(async move {
        println!("sending notifications");
        notify2.notify_waiters();
    });

    notified1.await;
    notified2.await;

    println!("received notifications");
}
