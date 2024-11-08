pub(in crate::cust_type) trait Greet {
    fn greet(&self, person: String) -> String {
        let mut gr = String::from("Greeting");
        gr.push(' ');
        gr.push_str(&*person);

        gr
    }
}