mod enum_match;
mod error_handle_and_result;
mod lifetime;
mod structs;
use crate::enum_match::GENDER;
use crate::structs::Member;
use crate::structs::Rectangle;

fn main() {
    let user = Member {
        active: true,
        name: String::from("John"),
        age: 30,
        gender: GENDER::Male,
    };

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let coin = enum_match::Coin::Bitcoin(enum_match::Balance::Small);
    let coin_decimals = enum_match::decimals(coin);

    print_debug(&user);
    println!("=============");
    println!("This is rectangle: {:?}", rectangle.is_rectangle());
    println!("Area of rectangle: {}", rectangle.area());
    println!("Coin decimals: {}", coin_decimals);

    let five = Some(5);
    let six = plus_one(five);
    if let Some(5) = five {
        println!("Five: 5")
    };
    println!("Six: {:#?}", six);
}
use std::fmt::Debug;

fn print_debug<T: Debug>(value: &T) {
    println!("{:#?}", value);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}
