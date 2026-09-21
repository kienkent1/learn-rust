use std::rc::Rc;

#[derive(Debug)]
struct Car {
    number: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Car>,
}
fn main() {
    let car = Rc::new(Car {
        number: "60A - 7843".to_owned(),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car),
    };
    let right_door = Door {
        vehicle: Rc::clone(&car),
    };

    drop(car);

    println!("left: {:?}", left_door.vehicle);
    println!("right: {:?}", right_door.vehicle);
}
