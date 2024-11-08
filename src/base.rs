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

pub fn lp() {
    let mut count = 0;
    loop {
        println!("{}", count);
        count += 1;

        if count == 7 {
            println!("loop stop");

            break;
        }
    }
}


pub fn expr() {
    println!("expressions");

    let x = 5u32;

    let y: u32 = {
        let x_2 = x * x;

        //statement to be returned
        x_2 * x_2
    };

    println!("{:?}", y)
}