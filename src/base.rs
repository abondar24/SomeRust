pub fn base_var() {
    println!("basic var");

    //base variable
    let x: i32;
    x = 1;
    println!("{}", x);
}


pub fn mutable_var() {
    println!("mutable vars");

    //mutable var
    let mut y: i32 = 2;
    y += 2;
    println!("{}", y);
}
