use tokio::sync::OnceCell;

static ONCE: OnceCell<u32> = OnceCell::const_new();

async fn get_global_integer() -> &'static u32 {
    ONCE.get_or_init(|| async { 1 + 1 }).await
}

#[tokio::main]
async fn main() {
    let result = get_global_integer().await;

    assert_eq!(*result, 2);
}
