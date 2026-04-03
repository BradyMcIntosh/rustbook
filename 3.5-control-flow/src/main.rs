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


    println!("Here's a value that starts at 0, increments 10 times, then doubles...");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!("Now let's do some counting...");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("Ready to launch?");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("Here's my collection");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value of {0} is: {1}", index, a[index]);

        index += 1;
    }

    println!("Here's my collection, but in a for loop :)");

    for element in a {
        println!("the value is: {element}");
    }

    println!("And here's another launch.");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let b = [1;2];
}
