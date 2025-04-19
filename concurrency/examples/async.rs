// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         println!("hello");
//     });
// }

pub fn tokio_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello from tokio");
        rt.spawn(async {
            println!("Hello from a tokio task!");
            println!("in spawn")
        })
        .await
        .unwrap();
    });

    rt.spawn_blocking(|| println!("in spawn_blocking"));
}

fn main() {
    tokio_async();
}
