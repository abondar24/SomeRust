use super::user::User;

pub fn structs() {
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

//tuple cust_type
struct Point(i32, i32, i32);
