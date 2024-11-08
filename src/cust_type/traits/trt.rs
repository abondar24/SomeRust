use super::greet::Greet;
use super::student::Student;
use super::teacher::Teacher;

pub fn traits() {
    println!("Traits");

    let st = greet_factory("student");
    let tch = greet_factory("teacher");

    let greets: [Box<dyn Greet>; 2] = [st, tch];
    for gr in greets {
        let g = gr.greet(String::from("Alex"));
        println!("{}", g);
    }
}

//returns a trait object
fn greet_factory(obj_type: &str) -> Box<dyn Greet> {
    match obj_type {
        "student" => Box::new(Student {}),
        "teacher" => Box::new(Teacher {}),
        _ => panic!(),
    }
}
