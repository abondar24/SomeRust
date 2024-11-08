pub fn borrowing() {
    println!("borrowing");

    let mut s = String::from("bh");

    let addr: &String = &s;
    println!("Address is {:p}", addr);

    borrow_string(&mut s);

    let borrow;
    {
        borrow = &s;
        println!("Borrow {}",borrow)
    }

    println!("{}", borrow)
}


fn borrow_string(st: &mut String) {
    st.push_str("fdgdfgfd");
    println!("{}", st);

    //the code doesn't compile as we can't have two mutable references
    // let st1 = st;
    // let st2 = st;
    //
    // println!("{},{}", st1,st2)

}