#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Employee {
    id: i32,
    name: String,
}

impl Employee {
   pub fn new(id: i32, name: &str) -> Employee {
        Employee {
            id,
            name: name.to_string(),
        }
    }
}