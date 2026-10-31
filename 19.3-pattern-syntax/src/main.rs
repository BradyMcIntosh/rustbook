struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one!"),
        2 => println!("two..."),
        3 => println!("three?"),
        _ => println!("idk"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50! Booyah!"),
        Some(y) => println!("Looks like y is {y}"),
        _ => println!("As expected, x is {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("thuh-ree"),
        _ => println!("whatevs"),
    }

    let x = 5;

    match x {
        1..=5 => println!("somewhere between one and five"),
        _ => println!("anywhere else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("some other letter"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis at ({x}, {y})"),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move {x} in the x direction, and {y} in the y direction")
        }
        Message::Write(text) => {
            println!("Message: \"{text}\"")
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change colour to RGB ({r},{g},{b})")
        }
    }
}
