use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use arc_swap::ArcSwap;
use jieba_rs::Jieba;

fn main() {
    let shared_jieba = Arc::new(ArcSwap::new(Arc::new(RwLock::new(Jieba::new()))));

    let mut handlers = vec![];

    let handler = {
        let shared_jieba = shared_jieba.clone();
        std::thread::spawn(move || {
            for _ in 0..15 {
                let jieba = shared_jieba.load();
                let res = jieba.read().unwrap().cut("华为场景AI是个小组", false);
                println!("{:?}", res);
                std::thread::sleep(Duration::from_secs(1));
            }
        })
    };
    handlers.push(handler);

    let add_word_handler = {
        let shared_jieba = shared_jieba.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(3));
            println!("执行add_word");
            shared_jieba
                .load()
                .write()
                .unwrap()
                .add_word("场景AI", None, None);
        })
    };
    handlers.push(add_word_handler);

    let reset_word_handler = {
        let shared_jieba = shared_jieba.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(6));
            println!("重置jieba");
            shared_jieba.store(Arc::new(RwLock::new(Jieba::new())));
        })
    };
    handlers.push(reset_word_handler);

    for handle in handlers {
        handle.join().unwrap();
    }
}
