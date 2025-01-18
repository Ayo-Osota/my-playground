/// This is a documentation comment, it will be used to generate documentation for the code.

// Hashmap is like js object
// Option is like ts union type Some(<T>) | None

// String is owned and used in struct, &str is borrowed and used in function

// derive is a special macro used on enums and structs 
// to automatically implement traits like 
// Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, and Default.

#[derive(Debug)]
enum MenuChoice {
    MAinMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MAinMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice".to_owned())
    }
}

enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket {
    event: String,
    price: i32,
}


// vector is like array but can grow in size
fn vector () {
    let mut my_numbers: Vec<i32> = Vec::new();
    my_numbers.push(5);
    my_numbers.push(6);
    my_numbers.pop();
    println!("number of elements is {}", my_numbers.len());
    println!("{:?}", my_numbers);

    let my_boys = vec!["James", "Cody", "Gakpo"];

    for boy in &my_boys {
        println!("{}", boy)  
    }

    // using SLF
    match &my_boys.is_empty() {
        true => println!("There are girls"),
        false => println!("There are boys")
    }
}

// impl is like ts class
impl Person {
    fn new(name: String, age: i32) -> Person {
        Person {
            name,
            age: age as u8,
            direction: Direction::Up
        }
    }
    fn say_hello(&self) {
        println!("Hello, my name is {}!", self.name);
    }  
}

// tuple is like array but can also be destructured like objects
fn tuple () {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
  
    let (x, y, _z) = tup;  // destructuring
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", tup.2); // access by index
}

// enum is like ts type
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// struct is like ts interface
struct Person {
    name: String,
    age: u8,
    direction: Direction
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

    let n = 3;
    match n {
        3 => println!("three"),
        other => print!("not three {}", other),
    };

    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("Flat discount of 2!"),
        Discount::Flat(amount) => println!("Flat discount of {}!", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 50,
    };

    match concert {
        Ticket { price: 50, event } => println!("50 event is {}", event),
        Ticket { price, .. } => println!("The price is ${}", price),
    }

    let gakpo = Person::new(String::from("James"), 25);
    gakpo.say_hello();

    let player_direction = Direction::Down;

    let player = Person {
        name: String::from("Cody"),
        age: 25,
        direction: player_direction.clone()
    };

    println!("{} is {} years old and is facing {}", player.name, player.age,  match player.direction {
        Direction::Up => String::from("up!"),
        Direction::Down => String::from("down!"),
        Direction::Left => String::from("left!"),
        Direction::Right => String::from("right!"),
    });

    match player_direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }

   let choice = get_choice("mainmenu err");

   println!("choice = {:?}", choice);
    vector();
    tuple();
    repetition_while();
    repetition();
    match_keyword();
    conditional();
}
