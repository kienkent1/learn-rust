fn main() {
    //let a = [2, 3, 4];
    //let v = vec![1, 2, 3, 4, 5, 6];
    // let mut v2 = Vec::new();
    // v2.push(2);
    // v2.push(3);
    // v2.push(4);

    // let four = &v[3];
    // println!("four: {}", four);

    // match v.get(20) {
    //     Some(four) => println!("four: {}", four),
    //     None => println!("No element at index 20"),
    // }

    // for i in &v {
    //     print!("{} ", i);
    // }

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SheetCell::Int(3),
        SheetCell::Float(10.12),
        SheetCell::Text(String::from("blue")),
    ];

    match &row[1] {
        SheetCell::Int(i) => println!("Integer: {}", i),
        SheetCell::Float(f) => println!("Float: {}", f),
        SheetCell::Text(s) => println!("Text: {}", s),
        _ => println!("Unknown type"),
    }
}
