use affinity;
use num_cpus;
use std::thread;

#[cfg(not(target_os = "macos"))]
fn use_affinity() {
    let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
    println!("{:?}", cores);
    affinity::set_thread_affinity(&cores).unwrap();
}

fn main() {
    let count = thread::available_parallelism().unwrap().get();

    println!("{}", count);

    let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
    println!("{:?}", cores);

    // 获取逻辑核心
    let num = num_cpus::get_physical();
    println!("{}", num);

    use_affinity();
}
