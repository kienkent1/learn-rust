fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let mut plus_one_vec1 = vec![];
    for v in vec1 {
        plus_one_vec1.push((v + 1));
    }

    let vec2 = vec![1, 2, 3, 4, 5];

    let plus_one_vec2: Vec<_> = vec2.iter().map(|num| num + 1).collect();

    println!("plus one vec1: {:?}", plus_one_vec1);
    println!("plus one vec2: {:?}", plus_one_vec2);
}
