use tokio::{
    sync::oneshot,
    time::{timeout, Duration},
};

#[tokio::main]
async fn main() {
    let (_tx, rx) = oneshot::channel::<u32>();

    if let Err(_) = timeout(Duration::from_secs(1), rx).await {
        println!("did not receive value within 1 s");
    }
}
