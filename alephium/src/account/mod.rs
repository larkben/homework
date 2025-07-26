// alephium account logic

pub mod account {

    pub struct Token {
        address: String,    // how do I derive this?
        name: String,
        symbol: String,
        decimals: u64,
        supply: u64
    }

    pub struct Account {
        alph_amount: u64,
        tokens: Vec<Token>
    }

}