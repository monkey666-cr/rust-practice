use std::{
    sync::mpsc::{channel, sync_channel},
    thread,
};

fn main() {
    let (tx, rx) = sync_channel(3);

    for _ in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send("ok").unwrap();
        });
    }

    drop(tx);

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }

    println!("completed");
}

// fn main() {
//     let (tx, rx) = channel();
//     for i in 0..10 {
//         let tx = tx.clone();
//         thread::spawn(move || {
//             tx.send(i).unwrap();
//         });
//     }

//     drop(tx);

//     while let Ok(res) = rx.recv() {
//         println!("{}", res);
//     }

//     println!("completed");
// }

// fn main() {
//     let (tx, rx) = channel();
//     thread::spawn(move || {
//         tx.send(10).unwrap();
//     });
//     assert_eq!(rx.recv().unwrap(), 10);
// }
