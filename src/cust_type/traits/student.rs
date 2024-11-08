use super::greet::Greet;

#[derive(Debug)]
pub(in crate::cust_type) struct Student {}

impl Greet for Student {
    fn greet(&self, person: String) -> String {
        person
    }
}