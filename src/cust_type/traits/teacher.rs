use super::greet::Greet;

pub(in crate::cust_type) struct Teacher {}
impl Greet for Teacher {}