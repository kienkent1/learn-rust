#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GENDER {
    Male,
    Female,
    Other,
}
//unwrap_or(value) = ?? value

//Match Option
#[derive(Debug)]
pub enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

#[derive(Debug)]
pub enum Coin {
    Solana,
    Etherrum,
    Near,
    Bitcoin(Balance),
}

pub fn decimals(coin: Coin) -> u8 {
    match coin {
        Coin::Solana => 9,
        Coin::Etherrum => 18,
        Coin::Near => 24,
        Coin::Bitcoin(balance) => {
            println!("Bitcoin balance: {:#?}", balance);
            30
        }
    }
}
