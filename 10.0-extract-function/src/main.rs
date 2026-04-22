fn main() {
    let number_list = vec![33, 34, 35, 50, 10, 100, 64];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
