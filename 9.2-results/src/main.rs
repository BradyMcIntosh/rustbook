use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Creating file: hello.txt");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    println!("File has been opened: hello.txt");

    let _greeting_file_2 = File::open("world.txt")
        .expect("world.txt could not be opened");
}
