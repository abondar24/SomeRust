use super::enums::direction::Direction;

pub fn pattern() {
    println!("patterns");
    let dir: Direction = Direction::South;

    match dir {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("Highway to hell")
        }
        _ => println!("West"),
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
            _ => println!("{}", i),
        }
    }
}