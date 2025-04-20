use tokio::sync::watch;
use tokio::time::{self, Duration, Instant};

use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        Ok(Config {
            timeout: Duration::from_secs(3),
        })
    }
}

async fn my_async_operation() {}

#[tokio::main]
async fn main() {
    let mut config = Config::load_from_file().await.unwrap();
    let (tx, rx) = watch::channel(config.clone());

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(10)).await;
            let new_config = Config::load_from_file().await.unwrap();

            if new_config != config {
                tx.send(new_config.clone()).unwrap();
            }
            config = new_config;
        }
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let mut rx = rx.clone();

        let handle = tokio::spawn(async move {
            let op = my_async_operation();
            tokio::pin!(op);

            let mut conf = rx.borrow().clone();

            let mut op_start = Instant::now();
            let sleep = time::sleep_until(op_start + conf.timeout);
            tokio::pin!(sleep);

            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        op.set(my_async_operation());

                        op_start = Instant::now();

                        sleep.set(time::sleep_until(op_start + conf.timeout));
                    }
                    _ = rx.changed() => {
                        conf = rx.borrow_and_update().clone();

                        sleep.as_mut().reset(op_start + conf.timeout);
                    }
                    _ = &mut op => {
                        return
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }
}
