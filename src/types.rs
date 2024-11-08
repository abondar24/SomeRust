pub fn tuples() {
    println!("tuples");

    let tup: (u8, u16) = (0u8, 1u16);
    println!("{:?}", tup);

    let (mut x, y) = (1, 2);
    x += 2;

    println!("x is {}, y is {}", x, y)
}


pub fn unsigned() {
    let v: u8 = 24_u8;
    println!("{} is unsigned 8 int", v);

    println!("Max {}", u8::MAX)
}

pub fn float() {
    println!("floats");
    let x: f32 = 0.12;

    println!("F32 x is {}", x);

    println!("F64 x is {}", x as f64)
}

pub fn range() {
    println!("range");

    let mut sum = 0;

    //in range last one is excluded by default
    for i in -5..5 {
        sum += i;
        println!("sum at iteration {} is {}", i, sum)
    }

    println!("Final sum {}", sum);

    for i in 'a'..='d' {
        println!("{}:{}", i, i as u8)
    }
}

pub fn char() {
    println!("characters");

    let c1 = 'a';

    println!("Size of char 'a' {} bytes ", size_of_val(&c1))
}

pub fn bool() {
    let t: bool = true;

    if t {
        println!("Bool")
    }
}

pub fn unit() {
    println!("unit");

    let _v: () = ();

    println!("Size of unit {}", size_of_val(&_v))
}

pub fn strings() {
    println!("strings");
    let str = String::from("strings");

    let start = &str[0..3];
    println!("{}", start);

    let mut str1: String = String::from("");
    str1.push_str("test");
    str1.push('!');

    println!("{}", str1);

    let str2 = str.replace("ings", "gggg");
    println!("{}", str2);

    let mut str3 = start.to_string();
    str3.push_str("ings are nice");
    println!("{}", str3);

    str3.push_str("\x73\x74\\x3F");
    println!("{}", str3)
}