use tokio::time::{self, sleep, sleep_until, Duration, Instant};

#[tokio::main]
async fn main() {
    sleep(Duration::from_micros(100)).await;
    println!("100 ms have elapsed");

    sleep_until(Instant::now() + Duration::from_micros(100)).await;

    let mut interval = time::interval(Duration::from_secs(1));
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;

    let start = Instant::now() + Duration::from_secs(1);
    let mut interval = time::interval_at(start, Duration::from_secs(1));
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
}
