use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个共享的 Mutex
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // 创建5个工作线程，正常递增计数器
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // 模拟一些工作
            thread::sleep(Duration::from_millis(100 * i));

            let mut num = data_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented to {}", i, *num);
        });
        handles.push(handle);
    }

    // 创建一个会 panic 的线程
    let data_clone = Arc::clone(&data);
    let panic_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(250));

        let mut num = data_clone.lock().unwrap();
        *num += 1;
        println!("Panic thread incremented to {}", *num);

        // 这里故意 panic，导致 Mutex 中毒
        panic!("Panic thread crashed while holding the lock!");
    });
    handles.push(panic_handle);

    // 再创建3个工作线程，可能会遇到中毒情况
    for i in 5..8 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(350 + 100 * (i - 5)));

            // 尝试获取锁，处理可能的中毒情况
            match data_clone.lock() {
                Ok(mut num) => {
                    *num += 1;
                    println!("Thread {} incremented to {}", i, *num);
                }
                Err(poisoned) => {
                    let mut num = poisoned.into_inner();
                    *num += 1;
                    println!(
                        "Thread {} recovered from poison, incremented to {}",
                        i, *num
                    );
                }
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        // 忽略线程 panic 的错误
        let _ = handle.join();
    }

    // 最终检查 Mutex 状态
    match data.lock() {
        Ok(guard) => {
            println!("Final value (no poison): {}", *guard);
        }
        Err(poisoned) => {
            let guard = poisoned.into_inner();
            println!("Final value (recovered from poison): {}", *guard);
        }
    };
}
