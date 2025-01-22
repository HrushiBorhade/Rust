enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin1 = Coin::Penny;
    println!("Value of coin1 : {}", value_of_coin(coin1));
    let coin2 = Coin::Dime;
    println!("Value of coin2 : {}", value_of_coin(coin2));
}
