use std::sync::Once;

static INIT: Once = Once::new();
static mut GLOBAL_CONFIG: Option<String> = None;

fn init_global_config() {
    unsafe { GLOBAL_CONFIG = Some("Initialized global configuration".to_string()) };
}

fn get_global_config() -> &'static str {
    INIT.call_once(|| {
        init_global_config();
    });
    unsafe { GLOBAL_CONFIG.as_ref().unwrap() }
}

fn main() {
    println!("{}", get_global_config());
    println!("{}", get_global_config());
}
