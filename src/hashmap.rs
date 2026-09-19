use std::collections::HashMap;

fn main() {
    let mut animals = HashMap::new();
    animals.insert("dog".to_string(), 1);
    animals.insert("cat".to_string(), 2);

    println!("dog: {}", animals.get(&String::from("dog")).unwrap_or(&0));
    for (key, value) in &animals {
        println!("{}: {}", key, value);
    }
}
