enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("Penny   = {}c", value_in_cents(penny));
    println!("Nickel  = {}c", value_in_cents(nickel));
    println!("Dime    = {}c", value_in_cents(dime));
    println!("Quarter = {}c", value_in_cents(quarter));
}
