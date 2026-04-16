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
}
