use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
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
