use std::fmt::Display;

pub fn generics() {
    println!("generics");

    let _sg = SGen { val: "SSS" };
    generic(100);
    generic(_sg.val)
}

struct A;
struct S(A);
struct SGen<T: Display> {
    val: T,
}

fn generic<T: Display>(a: T) {
    println!("{}", a)
}