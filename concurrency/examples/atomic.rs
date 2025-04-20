use std::sync::atomic::{AtomicI64, Ordering};

fn main() {
    let atomic_num = AtomicI64::new(0);

    // 原子加载
    let num = atomic_num.load(Ordering::Relaxed);
    println!("{num}");

    // 原子加法并返回旧值
    let old = atomic_num.fetch_add(10, Ordering::SeqCst);
    println!("{old}");

    // 原子比较并交换
    let res = atomic_num.compare_exchange(10, 100, Ordering::Acquire, Ordering::Relaxed);
    println!("{res:?}");

    // 原子交换
    let swapped = atomic_num.swap(200, Ordering::Release);
    println!("{swapped}");

    // 原子存储
    atomic_num.store(10000, Ordering::Relaxed);
}
