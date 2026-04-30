use std::{time};

async  fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    tokio::time::sleep(two_seconds).await;
    2
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    let now = time::Instant::now();
    let future1 = tokio::spawn(do_something(1));
    let future2 = tokio::spawn(do_something(2));
    let two_seconds = time::Duration::new(2, 0);
    tokio::time::sleep(two_seconds).await;
    let outcome = future1.await.unwrap() + future2.await.unwrap();
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}
