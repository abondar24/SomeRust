mod base;
mod data_structs;
mod memory;
mod types;
mod functions;
mod result;
mod coercion;
mod cust_type;

use std::fmt::Display;
use std::hash::Hash;

fn main() {
    println!("Let's start the show");

    //todo add arg reading and calling functions by args
    //todo readme

    base::base_var();
    base::mutable_var();
    base::expr();
    base::lp();

    memory::scope();
    memory::shadow();
    memory::rebind();
    memory::assignments();
    memory::ownership();
    memory::borrowing();
    memory::lifetime();

    types::tuples();
    types::unsigned();
    types::float();
    types::range();
    types::char();
    types::strings();
    types::bool();
    types::unit();

    functions::func(false);
    functions::closure();

    data_structs::arrays();
    data_structs::vectors();
    data_structs::hashmaps();
    data_structs::iterator();

    cust_type::enums();
    cust_type::structs();
    cust_type::pattern();
    cust_type::traits();
    cust_type::generics();

    coercion::type_coerc();

    result::res(false);

}













