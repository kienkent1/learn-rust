mod structs;
mod enum_match;
use crate::structs::Member;
use crate::enum_match::GENDER;
use crate::structs::Rectangle;
fn main() {
    let user = Member {
        active: true,
        name: String::from("John"),
        age: 30,
        gender: GENDER::male
    };

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    print_debug(&user);
    println!("=============");
    println!("This is rectangle: {:?}", rectangle.is_rectangle());
    println!("Area of rectangle: {}", rectangle.area());
}
use std::fmt::Debug;

fn print_debug<T: Debug>(value: &T) {
    println!("{:#?}", value);
}
