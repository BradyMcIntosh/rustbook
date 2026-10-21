use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms")
}