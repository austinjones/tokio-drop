use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::spawn(task());

    tokio::time::sleep(Duration::from_secs(1000)).await;
    //tokio::signal::ctrl_c()
    //    .await
    //    .expect("failed to listen for event");
}

async fn task() {
    println!("Starting drop");
    let _drop = DropValue {};

    tokio::time::sleep(Duration::from_secs(1000)).await;
}

struct DropValue {}

impl Drop for DropValue {
    fn drop(&mut self) {
        println!("Dropping");
    }
}
