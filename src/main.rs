mod base;
mod coercion;
mod cust_type;
mod data_structs;
mod functions;
mod memory;
mod result;
mod types;

use crate::base::{base_var, expr, lp, mutable_var};
use crate::memory::{assignments, borrowing, lifetime, ownership, rebind, scope, shadow};
use crate::types::{bool, char, float, range, strings, tuples, unit, unsigned};
use crate::functions::{func,closure};

use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::hash::Hash;
use crate::coercion::type_coerc;
use crate::cust_type::{enums, generics, pattern, structs, traits};
use crate::data_structs::{arrays, hashmaps, iterator, vectors};
use crate::result::res;

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
                "sc" => scope(),
                "sh" => shadow(),
                "reb" => rebind(),
                "asn" => assignments(),
                "own" => ownership(),
                "bor" => borrowing(),
                "lft" => lifetime(),
                "tup" => tuples(),
                "uns" => unsigned(),
                "flt" => float(),
                "rng" => range(),
                "chr" => char(),
                "str" => strings(),
                "bool" => bool(),
                "unit" => unit(),
                "fnc" => call_panic_func(args,"fnc"),
                "cls" => closure(),
                "arr" => arrays(),
                "vec" => vectors(),
                "map" => hashmaps(),
                "itr" => iterator(),
                "enm" => enums(),
                "strc" => structs(),
                "ptn" => pattern(),
                "trt" => traits(),
                "gen" => generics(),
                "crc" => type_coerc(),
                "res" => call_panic_func(args,"res"),
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

    //todo readme
}

fn init_arguments() -> HashMap<String, String> {
    let mut arguments = HashMap::new();
    arguments.insert("bv".to_string(), "Base Variables".to_string());
    arguments.insert("mv".to_string(), "Mutable Variables".to_string());
    arguments.insert("expr".to_string(), "Expressions".to_string());
    arguments.insert("lp".to_string(), "Loop".to_string());
    arguments.insert("sc".to_string(), "Scopes of variables".to_string());
    arguments.insert("sh".to_string(), "Shadowing".to_string());
    arguments.insert("reb".to_string(), "Rebinding".to_string());
    arguments.insert("asn".to_string(), "Assignments destructuring".to_string());
    arguments.insert("own".to_string(), "Ownership of variables".to_string());
    arguments.insert("bor".to_string(), "Borrowing of variable values".to_string());
    arguments.insert("lft".to_string(), "Lifetimes usage in Rust".to_string());
    arguments.insert("tup".to_string(), "Tuple example".to_string());
    arguments.insert("uns".to_string(), "Unsigned types demo".to_string());
    arguments.insert("flt".to_string(), "Float type example".to_string());
    arguments.insert("rng".to_string(), "Ranges in Rust".to_string());
    arguments.insert("chr".to_string(), "Character usage".to_string());
    arguments.insert("str".to_string(), "Strings example".to_string());
    arguments.insert("bool".to_string(), "Booleans example".to_string());
    arguments.insert("unit".to_string(), "Unit type usage".to_string());
    arguments.insert("fnc".to_string(),"Functions usage. Requires additional argument true or false to call panic".to_string());
    arguments.insert("cls".to_string(), "Closures examples in rust".to_string());
    arguments.insert("arr".to_string(), "Arrays example".to_string());
    arguments.insert("vec".to_string(), "Vectors example".to_string());
    arguments.insert("map".to_string(), "Maps example".to_string());
    arguments.insert("itr".to_string(), "Iterators example".to_string());
    arguments.insert("enm".to_string(), "Enums example".to_string());
    arguments.insert("strc".to_string(), "Structs and associated functions example".to_string());
    arguments.insert("ptn".to_string(), "Pattern check example".to_string());
    arguments.insert("trt".to_string(), "Traits example".to_string());
    arguments.insert("gen".to_string(), "Generics example".to_string());
    arguments.insert("crc".to_string(), "Type coercion example".to_string());


    arguments
}

fn show_help(arguments: &HashMap<String, String>) {
    println!("List of valid commands:");
    for (arg, description) in arguments {
        println!("  {} -> {}", arg, description);
    }
}

fn call_panic_func(args: Vec<String>,fnc: &str) {
    let arg2 = args.get(2);
    let is_panic = parse_bool(arg2);

    match parse_bool(arg2) {
        Ok(parsed_value) => println!("Parsed value: {}", parsed_value),
        Err(e) => println!("{}", e),
    }

    match fnc {
        "fnc" => func(is_panic.unwrap()),
        "res"=>  res(is_panic.unwrap()),
        _=> panic!("Unknown function {}", fnc)
    }

}

fn parse_bool(input: Option<&String>) -> Result<bool, String> {
    match input {
        Some(value) => value.parse::<bool>().map_err(|_| "Failed to parse the string into a bool.".to_string()),
        None => Err("Not enough arguments.".to_string()),
    }
}
