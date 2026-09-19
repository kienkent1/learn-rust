fn math(a: i32, b: i32, ob: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    ob(a, b)
}

fn main() {
    // let a = |a: i32, b: i32| a + b;

    // println!("{}", a(2, 3));

    let add = |a, b| a + b;
    let mul = |a: i32, b: i32| a * b;
    let add = Box::new(add);
    let mul = Box::new(mul);

    println!("{}", math(2, 3, add));
    println!("{:?}", math(2, 3, mul));
}
