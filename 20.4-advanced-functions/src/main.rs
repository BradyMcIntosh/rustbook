fn add_one(num: i32) -> i32 {
    num + 1
}

fn do_twice(func: fn(i32) -> i32, arg: i32) -> i32 {
    func(arg) + func(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let number = do_twice(add_one, 1);
    println!("number = {}", number);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    for string in list_of_strings {
        println!("{string}");
    }

    let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();

    for status in list_of_statuses {
        match status {
            Status::Value(x) => println!("Status is number '{x}'"),
            Status::Stop => println!("Status is stopped"),
        }
    }
}
