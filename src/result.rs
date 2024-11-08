pub fn res(is_panic: bool) {
    println!("result handling");

    let res = err_func("dfdf");
    println!("{}", res.unwrap());

    let err_res = err_func("");

    if is_panic {
        println!("{}", err_res.unwrap())
    }

    match hanlder("") {
        Ok(_) => println!("No error"),
        Err(e) => println!("{}", e),
    }

    match hanlder("test") {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }
}

fn hanlder(arg: &str) -> Result<i32, String> {
    let res = err_func(arg)?;
    println!("{}", res);

    Ok(res)
}

fn err_func(arg: &str) -> Result<i32, String> {
    let arg_len = arg.len();

    if arg_len > 0 {
        Ok(arg_len as i32)
    } else {
        Err("Can't be empty".to_string())
    }
}