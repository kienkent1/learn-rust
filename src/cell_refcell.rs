use std::cell::RefCell;

#[derive(Debug)]
struct Channel {
    name: RefCell<String>,
}
fn main() {
    let mychannel = Channel {
        name: RefCell::new("Hoc rust".to_owned()),
    };

    {
        let mut a = mychannel.name.borrow_mut();
        *a = "Hoc rust bai cell va refcell".to_owned();
    }
    {
        mychannel.name.replace("Hoc rust di".to_owned());
    }

    println!("mychannel: {:?}", mychannel.name);
}
