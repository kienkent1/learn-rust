//# crossbeam-channel = "0.5"
use ::crossbeam_channel::unbounded;
use std::thread;

enum Message {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}

enum MainMsg {
    ResultSum(i32),
    MainQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();
    let a = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                Message::PrintMsg(data) => println!("{}", data),
                Message::Sum(a, b) => {
                    main_tx.send(MainMsg::ResultSum(a + b));
                }
                Message::Quit => {
                    println!("Worker quit....");
                    main_tx.try_send(MainMsg::MainQuit); // đối vói result có err nên dùng try_send
                    break;
                }
            },
            Err(e) => {
                println!("{:?}", e);
                main_tx.send(MainMsg::MainQuit);
                break;
            }
        }
    });

    worker_tx.send(Message::PrintMsg(
        "Hello, this is the first channel".to_string(),
    ));
    worker_tx.send(Message::Sum(54, 46));
    worker_tx.send(Message::Quit);

    //drop(worker_tx);
    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::ResultSum(sum) => println!("Main sum: {}", sum),
            MainMsg::MainQuit => println!("Main quit..."),
        }
    }
    a.join();
}
