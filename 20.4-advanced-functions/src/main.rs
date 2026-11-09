fn add_one(num: i32) -> i32 {
    num + 1
}

fn do_twice(func: fn(i32) -> i32, arg: i32) -> i32 {
    func(arg) + func(arg)
}

fn main() {
    let number = do_twice(add_one, 1);
    println!("number = {}", number);
}
