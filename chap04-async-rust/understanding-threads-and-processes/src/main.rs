use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    // sequential execution
    let now = time::Instant::now();
    let one: i8 = do_something(1);
    let two: i8 = do_something(2);
    let three: i8 = do_something(3);
    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);

    // parallel execution (multi-threading)
    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(
        || do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(
        || do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(
        || do_something(3));
    let result_one = thread_one.join().unwrap();
    let result_two = thread_two.join().unwrap();
    let result_three = thread_three.join().unwrap();
    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", result_one + result_two + result_three);
}

