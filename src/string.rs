//# unicode-segmentation = "1.13.3"
//src/string.rs

use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let s1 = String::from("Con chó kêu meo meo, còn mèo sủa gâu gâu");
    let s2 = String::new();
    let s3 = "hehe 😀".to_string();

    println!("{}", s1 + &s3);

    // for i in s3.chars() {
    //     println!("{}", i);
    // }

    for i in s3.graphemes(true) {
        println!("{}", i);
    }
}
