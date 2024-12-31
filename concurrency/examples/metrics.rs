use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::{thread_rng, Rng};

fn main() -> Result<()> {
    let metrics = Metrics::new();

    task_worker(1, metrics.clone());

    request_worker(metrics.clone());

    loop {
        thread::sleep(Duration::from_millis(1000));

        println!("{:?}", metrics.snapshot());
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc("call.request.worker");
    });
}
