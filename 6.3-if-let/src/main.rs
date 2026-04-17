#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc.
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // etc.
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("The maximum is nothing.");
    }

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter1 = Coin::Quarter(UsState::Alabama);
    let quarter2 = Coin::Quarter(UsState::Alaska);

    println!("Penny description:     '{}'", describe_state_quarter(penny).unwrap_or("".to_string()));
    println!("Nickel description:    '{}'", describe_state_quarter(nickel).unwrap_or("".to_string()));
    println!("Dime description:      '{}'", describe_state_quarter(dime).unwrap_or("".to_string()));
    println!("Quarter1 description:  '{}'", describe_state_quarter(quarter1).unwrap_or("".to_string()));
    println!("Quarter2 description:  '{}'", describe_state_quarter(quarter2).unwrap_or("".to_string()));
}
