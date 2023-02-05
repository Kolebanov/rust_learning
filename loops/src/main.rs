use std::{thread, time};

fn main() {

    let delay = time::Duration::from_secs(1);

    for number in (1..4).rev() {
        println!("{number}!");
        thread::sleep(delay)
    }
    println!("LAUNCH!")
}