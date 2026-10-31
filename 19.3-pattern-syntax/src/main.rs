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
}
