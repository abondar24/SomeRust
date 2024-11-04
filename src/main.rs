use crate::Message::{Quit, Start, Write};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

fn main() {
    println!("Let's start the show");

    //todo add arg reading and calling functions by args
    //todo move functions to mods
    base_var();
    mutable_var();
    scope();
    shadow();
    rebind();
    tuples();
    assignments();
    unsigned();
    float();
    range();
    char();
    bool();
    unit();
    expr();
    func(false);
    ownership();
    borrowing();
    strings();
    arrays();
    structs();
    enums();
    lp();
    pattern();
    generics();
    traits();
    vectors();
    hashmaps();
    type_coerc()
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


fn scope() {
    println!("scope");

    let x = 1;
    //scope - var z is not available outside of scope
    {
        let z: i32 = 6;
        println!("x is {} and z is {}", x, z);
    }
}


fn shadow() {
    println!("shadowing");

    let x = 5;

    {
        let x = 7;
        assert_eq!(x, 7)
    }

    println!("{}", x)
}

fn rebind() {
    println!("rebind");

    let mut x: i32 = 1;
    x += 1;
    println!("{}", x);

    let x = 3;
    println!("{}", x);

    let x = "Now it is a string";
    println!("{}", x)
}

fn tuples() {
    println!("tuples");

    let tup: (u8, u16) = (0u8, 1u16);
    println!("{:?}", tup);

    let (mut x, y) = (1, 2);
    x += 2;

    println!("x is {}, y is {}", x, y)
}

fn assignments() {
    println!("assignments destructuring");

    let (x, y);

    (x, ..) = (5, 4);
    [.., y] = [1, 7];

    assert_eq!([x, y], [5, 7]);

    println!("x is {}, y is {}", x, y)
}

fn unsigned() {
    let v: u8 = 24_u8;
    println!("{} is unsigned 8 int", v);

    println!("Max {}", u8::MAX)
}

fn float() {
    println!("floats");
    let x: f32 = 0.12;

    println!("F32 x is {}", x);

    println!("F64 x is {}", x as f64)
}

fn range() {
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

fn char() {
    println!("characters");

    let c1 = 'a';

    println!("Size of char 'a' {} bytes ", size_of_val(&c1))
}

fn bool() {
    let t: bool = true;

    if t {
        println!("Bool")
    }
}

fn unit() {
    println!("unit");

    let _v: () = ();

    println!("Size of unit {}", size_of_val(&_v))
}

fn expr() {
    println!("expressions");

    let x = 5u32;

    let y: u32 = {
        let x_2 = x * x;

        //statement to be returned
        x_2 * x_2
    };

    println!("{:?}", y)
}

// returns () by default
fn func(isPanic: bool) {
    println!("func");
    let (x, y) = (1, 2);

    let s = sum(x, y);
    println!("{}", s);

    if isPanic {
        sum(-100, 0);
    }
}


fn sum(x: i32, y: i32) -> i32 {
    if x == -100 || y == -100 {
        error()
    }

    x + y
}

//diverget func - never returns a val
fn error() -> ! {
    panic!()
}

fn ownership() {
    println!("Ownership");

    let x: String = String::from("hi");
    let y: String = x.clone();

    println!("{},{}", x, y);

    let tup: (String, String) = (String::from("hi"), String::from("man"));

    let _t = tup.0;

    println!("remaining tup: {}", tup.1)
}

fn borrowing() {
    println!("borrowing");

    let mut s = String::from("bh");

    let addr: &String = &s;
    println!("Address is {:p}", addr);

    borrow_string(&mut s)
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

fn strings() {
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

fn arrays() {
    println!("arrays");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr.len());

    let arr1: [_; 3] = ['a', 'b', 'c'];
    println!("{:?}", arr1)
}

fn structs() {
    println!("stucts");

    let user = User {
        name: String::from("test"),
        pass: String::from("tes"),
    };
    println!("{:?}", user.name);
    println!("{:?}", user);

    let point = Point(34, 34, 45);
    println!("{}", point.2);

    let Point(x, _, _) = point;
    println!("Destructured point to {}", x);

    let enc = user.pass_enc();
    println!("Encoded pass {}", enc);

    let new = User::new_user();
    println!("{:?}", new)
}

//tuple struct
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
    name: String,
    pass: String,
}

impl User {
    pub fn pass_enc(&self) -> String {
        let mut enc = String::from("enc");
        enc.push_str(&*self.pass);
        enc
    }

    pub fn new_user() -> Self {
        Self {
            name: String::from("new"),
            pass: String::from("pass"),
        }
    }
}

fn enums() {
    println!("enums");

    println!("{:?}", Number::One);
    println!("{:?}", Number::One as u16);
    println!("{:?}", Num::One);

    let msg_quit: Message = Quit(String::from("Reason"), String::from("Epic"));
    let msg_start: Message = Start { x: 0, y: 32 };
    let msg_write: Message = Write(String::from("Hiii"));

    if let Start { x: a, y: b } = msg_start {
        println!("{}", a)
    }

    if let Write(ref message) = msg_write {
        println!("{}", message);
    }

    let msgs: [Message; 3] = [
        msg_quit,
        msg_start,
        msg_write
    ];

    for msg in msgs {
        println!("{:?}", msg)
    }

    let mut nm: Option<i32> = Some(5);
    nm = nm.map(|v| v + 1);

    match nm {
        Some(v) => println!("{}", v),
        None => println!("no val")
    }
}


#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
enum Num {
    Zero = 0,
    One = 1,
}

#[derive(Debug)]
enum Message {
    Quit(String, String),
    Start {
        x: i32,
        y: i32,
    },
    Write(String),
}

fn lp() {
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

fn pattern() {
    println!("patterns");
    let dir: Direction = Direction::South;

    match dir {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("Highway to hell")
        }
        _ => println!("West")
    }

    let chars = ['a', 'b', 'C'];

    for c in chars {
        let is_big = matches!(c, 'A'..'Z');
        println!("{} is Big: {}", c, is_big)
    }

    for i in 1..10 {
        match i {
            1 => println!("Start"),
            2 | 3 => println!("Push"),
            4..6 => println!("Don't fall"),
            _ => println!("{}", i)
        }
    }
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn generics() {
    println!("generics");

    let _sg = SGen {
        val: "SSS"
    };
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

fn traits() {
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
        _ => panic!()
    }
}

trait Greet {
    fn greet(&self, person: String) -> String {
        let mut gr = String::from("Greeting");
        gr.push(' ');
        gr.push_str(&*person);

        gr
    }
}

#[derive(Debug)]
struct Student {}
impl Greet for Student {
    fn greet(&self, person: String) -> String {
        person
    }
}


struct Teacher {}
impl Greet for Teacher {}


fn vectors() {
    println!("vectors");
    let arr: [u8; 3] = [1, 2, 3];

    let vc: Vec<u8> = Vec::from(arr);

    println!("{:?}", vc);

    let v1: Vec<u8> = vec![4, 5, 6];
    println!("{:?}", v1);

    let mut v2 = Vec::new();

    for i in &v1 {
        v2.push(*i);
    }

    for i in &vc {
        v2.push(*i);
    }

    println!("{:?}", v2.pop());
    println!("{:?}", v2[4]);

    let v3: Vec<u16> = [1; 5].into_iter().collect();
    println!("Vector from iterator: {:?}", v3)
}

fn hashmaps() {
    println!("hashmaps");

    let mut scores = HashMap::new();
    scores.insert("a", 100);
    scores.insert("b", 200);

    println!("{:?}", scores);

    let sa = scores.get("a");
    println!("Value of a is {:?} or {}", sa, scores["a"]);


    let employees: HashMap<Employee, String> = HashMap::from([
        (Employee::new(1, "Armen"), String::from("test")),
        (Employee::new(2, "Arsen"), String::from("test1"))
    ]);

    println!("{:?}", employees)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Employee {
    id: i32,
    name: String,
}

impl Employee {
    fn new(id: i32, name: &str) -> Employee {
        Employee {
            id,
            name: name.to_string(),
        }
    }
}


fn type_coerc(){
    println!("type coercion");

    let dec: f32 = 97.123_f32;

    let int: u8 = dec as u8;
    println!("Dec {} as int {}",dec,int);

    let ch: char = int as char;
    println!("Int {} as char {}",int,ch);

    overflow();

    let num: Numb = Numb::from(30);
    println!("Numb from {:?}",num);

    let nm: Numb = 64.into();
    println!("Numb into {:?}",nm)


}

#[allow(overflowing_literals)]
fn overflow(){
    println!("U8 max {}",u8::MAX);

    let over: u8 = 1023 as u8;
    println!("1023 in u8 with overflow {}", over)
}

#[derive(Debug)]
struct Numb{
    val: i32
}

impl From<i32> for Numb {
    fn from(value: i32) -> Self {
        Self {
            val: value,
        }
    }
}