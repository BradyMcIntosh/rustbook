use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

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

    // vvvv this code will panic
    // let _greeting_file_2 = File::open("world.txt")
    //     .expect("world.txt could not be opened");

    // vvvv this code will panic
    // read_username_from_file().expect("could not read username from file...");

    read_username_from_file_2().expect("could not read username from file...");
}
