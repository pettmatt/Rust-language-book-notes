#[derive(Debug)] // BOOK: so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Florida,
    NewYork,
    Hawai
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter can include value that holds a name of a US state.
}

// Function accepts coin as a argument and goes through them, matching the type of coin with a value.
// Function ends by returning the final list of the values. 
// *If any possible value was forgotten the compiler would print an error.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // i represents a possible value that Some() holds.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Coin example:");
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("");

    println!("Matching <T>:");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", five, six);
    println!("");
}
