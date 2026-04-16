#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter1 = Coin::Quarter(UsState::Alabama);
    let quarter2 = Coin::Quarter(UsState::Arkansas);

    println!("Penny   = {}c", value_in_cents(penny));
    println!("Nickel  = {}c", value_in_cents(nickel));
    println!("Dime    = {}c", value_in_cents(dime));
    println!("Quarter = {}c", value_in_cents(quarter1));
    println!("Quarter = {}c", value_in_cents(quarter2));

    let five = Some(5);
    println!("Five = {five:?}");

    let six = plus_one(five);
    println!("Five plus one = {six:?}");

    let none = plus_one(None);
    println!("None = {none:?}");
}
