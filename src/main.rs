fn main() {
    println!("Let's start the show");

    base_var();
    mutable_var();
    scope();
    shadow();
    rebind();
    tuples();
assignments();
}

fn base_var() {
    println!("basic var");

    //base variable
    let x: i32;
    x = 1;
    println!("{}", x);
}

fn mutable_var() {
    println!("mutable vars");

    //mutable var
    let mut y: i32 = 2;
    y += 2;
    println!("{}", y);
}


fn scope(){
    println!("scope");

    let x = 1;
    //scope - var z is not available outside of scope
    {
        let z: i32 = 6;
        println!("x is {} and z is {}", x, z);
    }
}


fn shadow(){
    println!("shadowing");

    let x = 5;

    {
        let x = 7;
        assert_eq!(x,7)
    }

      println!("{}",x)
}

fn rebind(){
    println!("rebind");

    let mut x: i32  =1;
    x+=1;
    println!("{}",x);

    let x = 3;
    println!("{}",x);

    let x= "Now it is a string";
    println!("{}", x)

}

fn tuples(){
    println!("tuples destructuring");

    let (mut x,y) = (1,2);
    x +=2;

    println!("x is {}, y is {}",x,y)
}

fn assignments(){
    println!("assignments destructuring");

    let(x,y);

    (x,..) = (5,4);
    [..,y] = [1,7];

    assert_eq!([x,y],[5,7]);

    println!("x is {}, y is {}",x,y)
}