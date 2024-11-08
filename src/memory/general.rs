pub fn scope() {
    println!("scope");

    let x = 1;
    //scope - var z is not available outside of scope
    {
        let z: i32 = 6;
        println!("x is {} and z is {}", x, z);
    }
}

pub fn shadow() {
    println!("shadowing");

    let x = 5;

    {
        let x = 7;
        assert_eq!(x, 7)
    }

    println!("{}", x)
}


pub fn rebind() {
    println!("rebind");

    let mut x: i32 = 1;
    x += 1;
    println!("{}", x);

    let x = 3;
    println!("{}", x);

    let x = "Now it is a string";
    println!("{}", x)
}

pub fn assignments() {
    println!("assignments destructuring");

    let (x, y);

    (x, ..) = (5, 4);
    [.., y] = [1, 7];

    assert_eq!([x, y], [5, 7]);

    println!("x is {}, y is {}", x, y)
}


pub fn ownership() {
    println!("Ownership");

    let x: String = String::from("hi");
    let y: String = x.clone();

    println!("{},{}", x, y);

    let tup: (String, String) = (String::from("hi"), String::from("man"));

    let _t = tup.0;

    println!("remaining tup: {}", tup.1)
}

