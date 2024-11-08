pub fn type_coerc() {
    println!("type coercion");

    let dec: f32 = 97.123_f32;

    let int: u8 = dec as u8;
    println!("Dec {} as int {}", dec, int);

    let ch: char = int as char;
    println!("Int {} as char {}", int, ch);

    overflow();

    let num: Numb = Numb::from(30);
    println!("Numb from {:?}", num);

    let nm: Numb = 64.into();
    println!("Numb into {:?}", nm)
}

#[allow(overflowing_literals)]
fn overflow() {
    println!("U8 max {}", u8::MAX);

    let over: u8 = 1023 as u8;
    println!("1023 in u8 with overflow {}", over)
}

#[derive(Debug)]
struct Numb {
    val: i32,
}

impl From<i32> for Numb {
    fn from(value: i32) -> Self {
        Self { val: value }
    }
}