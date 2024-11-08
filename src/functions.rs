// returns () by default
pub fn func(is_panic: bool) {
    println!("func");
    let (x, y) = (1, 2);

    let s = sum(x, y);
    println!("{}", s);

    if is_panic {
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

pub fn closure() {
    println!("closure");

    let incr = |i: i32| -> i32 { i + 1 };
    println!("Incr: {}", incr(7));

    let val = || 9;
    println!("{}", val());

    let name = String::from("Arman");

    //greet will own name
    let greet = move || println!("Hello {}", name);
    greet();

    let mut count = 0;
    let mut cnt = move || {
        count += 1;
        println!("{}", count)
    };
    cnt();
    println!("Orig count: {}", count);

    let _reborrow: &i32 = &count;
    cnt();

    let closure_param = || println!("Closure passed as a param");
    func_receiver(fn_param);
    func_receiver(closure_param);
}

fn func_receiver<F: Fn()>(f: F) {
    f();
}

fn fn_param() {
    println!("Function passed as a param")
}