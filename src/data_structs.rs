use std::collections::HashMap;
use crate::cust_type::employee::Employee;

pub fn arrays() {
    println!("arrays");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr.len());

    let arr1: [_; 3] = ['a', 'b', 'c'];
    println!("{:?}", arr1)
}

pub fn vectors() {
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

pub fn hashmaps() {
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

pub fn iterator(){
    println!("iterator");

    let v = vec![1,2,3];

    //n ownership transfer
    for x in v.iter(){
        println!("{}",x)
    }

    //ownership is transferred
    for x in v.into_iter() {
        println!("{}", x)
    }

    let arr = [0;10];
    for i in arr {
        println!("{}",arr[i])
    }
}

