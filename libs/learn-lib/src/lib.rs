// fn call_order() {}

// // mod test {
// //     mod back_house {
// //         fn cook_order() {}
// //         fn fix_order() {
// //             super::super::call_order();
// //             cook_order();
// //         }
// //     }
// // }

// fn eat_at_restaurant() {
//     //self::front_house::hosting::add_to_waitlist();
//     front_house::hosting::add_to_waitlist();
// }

use rand::{Rng, RngCore, SeedableRng};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }
}

fn eat_at_restaurant() {
    let mut order = back_house::Breakfast::monday("white");
    order.toast = String::from("wheat");

    let order1: Breakfast = back_house::Breakfast {
        toast: String::from("white"),
        fruit: String::from("peaches"),
    };

    let order2: Salad = back_house::Salad::Soup;
}
