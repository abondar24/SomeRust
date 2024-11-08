mod base;
mod coercion;
mod cust_type;
mod data_structs;
mod functions;
mod memory;
mod result;
mod types;

use crate::base::{base_var, expr, lp, mutable_var};
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::hash::Hash;

fn main() {
    let arguments = init_arguments();
    let args: Vec<String> = env::args().collect();
    let arg1 = args.get(1);

    if arg1.is_some() {
        let arg_val = arg1.unwrap().as_str();

        if arguments.contains_key(arg_val) {
            match arg_val {
                "bv" => base_var(),
                "mv" => mutable_var(),
                "expr" => expr(),
                "lp" => lp(),

                _ => println!("Unsupported action"),
            }
        } else if arg_val == "help" {
            show_help(&arguments)
        } else {
            panic!("Unknown argument {}", arg_val);
        }
    } else {
        println!("Please provide an argument! Use cargo run help to see the available commands.");
    }

    //todo add arg reading and calling functions by args
    //todo readme

    // memory::scope();
    // memory::shadow();
    // memory::rebind();
    // memory::assignments();
    // memory::ownership();
    // memory::borrowing();
    // memory::lifetime();
    //
    // types::tuples();
    // types::unsigned();
    // types::float();
    // types::range();
    // types::char();
    // types::strings();
    // types::bool();
    // types::unit();
    //
    // functions::func(false);
    // functions::closure();
    //
    // data_structs::arrays();
    // data_structs::vectors();
    // data_structs::hashmaps();
    // data_structs::iterator();
    //
    // cust_type::enums();
    // cust_type::structs();
    // cust_type::pattern();
    // cust_type::traits();
    // cust_type::generics();
    //
    // coercion::type_coerc();
    //
    // result::res(false);
}

fn init_arguments() -> HashMap<String, String> {
    let mut arguments = HashMap::new();
    arguments.insert("bv".to_string(), "Base Variables".to_string());
    arguments.insert("mv".to_string(), "Mutable Variables".to_string());
    arguments.insert("expr".to_string(), "Expressions".to_string());
    arguments.insert("lp".to_string(), "Loop".to_string());

    arguments
}

fn show_help(arguments: &HashMap<String, String>) {
    println!("Valid arguments:");
    for (arg, description) in arguments {
        println!("  {} -> {}", arg, description);
    }
}
