use std::io;

fn main() {
    println!("NUMBERS:");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {x}, y = {y}");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    println!("BOOLEANS:");

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {t}, f = {f}");

    println!("CHARS:");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("characters = '{c}','{z}','{heart_eyed_cat}'");

    println!("TUPLES:");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;

    println!("tuple (x,y,z) = ({x},{y},{z})");
    println!("tup.0 = {tup0}");
    println!("tup.1 = {tup1}");
    println!("tup.2 = {tup2}");

    println!("ARRAYS:");

    let arra = [1, 2, 3, 4, 5];
    println!("array = {arra:?}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let month0 = months[0];
    let month1 = months[1];
    let month11 = months[11];

    println!("From month array, months 0, 1, and 11: {month0}, {month1}, {month11}");

    let arrb = [3; 5];
    println!("array[3;5] = {arrb:?}");

    println!("This is your array: {arra:?}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arra[index];

    println!("The value of the element at index {index} is: {element}");
}
