use crate::cust_type::enums::message::Message::{Quit, Start, Write};

use super::number::Number;
use super::num::Num;
use super::message::Message;

pub fn enums() {
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

    let msgs: [Message; 3] = [msg_quit, msg_start, msg_write];

    for msg in msgs {
        println!("{:?}", msg)
    }

    let mut nm: Option<i32> = Some(5);
    nm = nm.map(|v| v + 1);

    match nm {
        Some(v) => println!("{}", v),
        None => println!("no val"),
    }
}