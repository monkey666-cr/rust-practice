use std::sync::Arc;

use tokio::sync::Barrier;

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10));
    for _ in 0..10 {
        let c = barrier.clone();
        handles.push(tokio::spawn(async move {
            println!("before wait");
            let wait_result = c.wait().await;
            println!("after wait");
            wait_result
        }));
    }

    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    assert_eq!(num_leaders, 1);
}
