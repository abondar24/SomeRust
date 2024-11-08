pub fn lifetime(){
    println!("lifetime");

    let a = 10;
    let b = 20;

    let rs = pass_x(&a, &b);
    println!("{}", rs);

    let cmp = compare(&a,&b);
    println!("{}", cmp);

}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> & 'a i32{
    x
}


fn compare<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y{
        x
    } else {
        y
    }
}
