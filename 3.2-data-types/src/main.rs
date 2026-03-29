fn main() {
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

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {t}, f = {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("characters = '{c}','{z}','{heart_eyed_cat}'");
}
