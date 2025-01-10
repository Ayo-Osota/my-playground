// enum is like ts type

enum Direction {
    Up,
    Down,
    Left,
    Right
}

// struct is like ts interface
struct Person {
    name: String,
    age: u8
}

fn repetition_while () {
    let mut x = 0;
    while x < 5 {
        x += 1;
        println!("x is {}", x);
    }
    println!("Loop exited!");
}

fn repetition () {
    let mut x = 0;
    loop {
        x += 1;
        println!("x is {}", x);
        if x == 5 {
            break;
        }
    }
    println!("Loop exited!");
}

// loop breaks out manually, while while loops break out automatically when the condition is false.

// match is like switch                                                                                                                                                                                                    
fn match_keyword () {
    let x = 5;
    match x {
        1 => println!("x is one!"),
        2 => println!("x is two!"),
        3 => println!("x is three!"),
        4 => println!("x is four!"),
        5 => println!("x is five!"),
        _ => println!("x is not one, two, three, four, or five :("),
    }
}

fn conditional () {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    }
     else {
        println!("x is not five :(");
    }
}

fn main() {
    println!("Hello, world!");

    let player_direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }

    repetition_while();
    repetition();
    match_keyword();
    conditional();
}
