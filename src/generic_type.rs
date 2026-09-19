fn main() {
    let num_list = vec![20, 2324, 4535, 543, 6464, 74574];

    println!("The largest number is: {}", get_largest(num_list));

    let char_list = vec!['f', 's', '5', '6', 'g', '8'];

    println!("The largest char is: {}", get_largest(char_list));

    let point1 = Point { x: 5, y: 10 };
    let point2 = Point { x: 1.0, y: 4.0 };
}

fn get_largest<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut largest = v[0];

    for number in v {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
