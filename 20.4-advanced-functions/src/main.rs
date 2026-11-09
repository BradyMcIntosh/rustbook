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

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
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

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("handler output is {output}");
    }
}
