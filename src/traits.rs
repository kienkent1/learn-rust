struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

struct Data2 {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

impl Data {
    fn new() -> Self {
        Data {
            num1: 0,
            num2: 0,
            str1: String::new(),
            optional: None,
        }
    }
}

trait Transform {
    fn revert(&self) -> String {
        String::from("Default implementation")
    }
}

impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}

impl Transform for Data2 {
    fn revert(&self) -> String {
        (self.num1 + self.num2).to_string()
    }
}

// fn main() {
//     let a = Data::new();
//     let b = Data2 {
//         num1: 10,
//         num2: 20,
//         str1: String::from("Hello"),
//         optional: None,
//     };

//     //println!("Reverted string: {}", a.revert());
//     println!("Sum of numbers: {}", b.revert());
// }

// Trait obj
trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) -> String {
        "Keyboard input".to_owned()
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        "Mouse click".to_owned()
    }
}

fn main() {
    // let x = Keyboard;
    // let x: &dyn Clicky = &Keyboard;

    let x: Box<dyn Clicky> = Box::new(Keyboard);
    let y: Box<dyn Clicky> = Box::new(Mouse);
    let clicker = vec![x, y];

    for i in clicker {
        println!("{}", i.click());
    }
}
