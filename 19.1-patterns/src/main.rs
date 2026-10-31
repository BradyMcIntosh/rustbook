fn main() {
    let five = 5;
    println!("five = {five}");

    let (x, y, z) = (1, 2, 3);
    println!("(x, y, z) = ({x}, {y}, {z})");

    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Your favourite colour is {colour}!");
    } else if is_tuesday {
        println!("It's Tuesday, that means we're greening it up.");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Purple: an auspicious colour for an auspicious age.");
        } else {
            println!("Babies like orange, right?");
        }
    } else {
        println!("Blue is a nice, safe, average colour. Let's go with that.");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("Message: {value}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{index}: \"{value}\"");
    }
}
