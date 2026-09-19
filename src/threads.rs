use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    // let iterations = 10;
    // let a: JoinHandle<usize> = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(1));
    //     50
    // });
    // // let b: JoinHandle<usize> = thread::spawn(move || {
    // //     for i in 1..12 {
    // //         print!("{} ", i);
    // //     }
    // // });

    // println!("Waiting for data");
    // match a.join() {
    //     Ok(value) => println!("value: {}", value),
    //     Err(e) => println!("err: {:?}", e),
    // }
    //b.join();

    let hello = || print!("hello, ");
    let name = || print!("my name is ");
    let kien = || print!("Phung Xuong Kien!");

    let one: JoinHandle<()> = thread::spawn(move || hello());
    let two: JoinHandle<()> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        name()
    });
    let three: JoinHandle<()> = thread::spawn(move || kien());

    one.join();
    two.join();
    three.join();
}
