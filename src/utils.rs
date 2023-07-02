use crate::HONEY_CONTRACT_ADDRESS;
use std::ops::Div;
use substreams_solana::pb::sol::v1::TokenBalance;

pub fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

pub fn fetch_account_to(account_keys: &Vec<Vec<u8>>, position: u8) -> String {
    // Instruction account will contain the list of accounts to fetch in the accounts list
    // inst account pos 0 -> mint_info
    // inst account pos 1 -> destination_account_info
    // inst account pos 2 -> owner_info
    return bs58::encode(&account_keys[position as usize]).into_string();
}

pub fn valid_honey_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &String) -> bool {
    for token_balance in pre_token_balances.iter() {
        if token_balance.owner.eq(account) && token_balance.mint.eq(HONEY_CONTRACT_ADDRESS) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use crate::utils::amount_to_decimals;

    #[test]
    pub fn test_amount_to_decimals() {
        let amount = 4983184141.0;
        let expected = 4.983184141;

        let actual = amount_to_decimals(amount, 9 as f64);
        println!("expected {:?} actual {:?}", expected, actual);
        assert_eq!(expected, actual)
    }
}
