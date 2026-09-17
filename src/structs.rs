use crate::enum_match::GENDER;
#[derive(Debug)]
pub struct Member {
    pub active: bool,
    pub name: String,
    pub age: u32,
    pub gender: GENDER,
}
//====================

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_rectangle(&self) -> bool {
        if self.width > self.height {
            false
        } else {
            true
        }
    }
}
// fn dien_tich(kichthuoc: (u32, u32)) -> u32 {
//     kichthuoc.0 * kichthuoc.1
// }
