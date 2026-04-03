fn main() {
    let number = 3;

    if number != 0 {
        println!("{number} was something other than zero");
    }

    println!("Is {number} less than 5?");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 7;

    println!("Is {number2} less than 5?");

    if number2 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number3 = 6;

    println!("Examining {number3}...");

    if number3 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number3 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number4 = if condition { 8 } else { 9 };

    println!("Since condition is {condition}, the next number is {number4}");
}
