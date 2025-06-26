use std::collections::BTreeMap;

/*Pallets in Polkadot SDK:

"Pallet" is a term specific to the Polkadot SDK, 
    which refers to Rust modules which contain logic specific 
    for your blockchain runtime.

This Pallet will tell you: how much balance each user has,
    provide functions which allow users to transfer those balances,
    and even some low level functions to allow your blockchain system
    to manipulate those balances if needed.
     Think for example if you want to mint new tokens which don't already exist.
*/

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        // .get returns Option<&u128> - &u128 a reference to a value
        // unwrap returns the reference the option -
        // * dereferences the reference and gets te actual value
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn account_exists(&self, who: &String) -> Option<&u128> {
        // check if account exists
        self.balances.get(who)
    }

    pub fn transfer_balance(
        &mut self,
        origin : &String, // use references instead of borrowing
        destination : &String, 
        amount: u128
        // if sucessfull return nothing - otherwise a string literal with lifetime across from the entire program
    ) -> Result<(), &'static str> {
        // get current balances
        let origin_balance:u128 = self.balance(origin);
        let destination_balance:u128 = self.balance(destination);
        
        let destination_exists = self.account_exists(destination);

        // check if destination exists
        match destination_exists  {
            None => {
                return Err("Account doesn't exist.");
            }
            _ => ()
        }

        // check new balances or return error
        let new_origin_balance: u128 = origin_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;

        // aditional check if origin exists



        let new_destination_balance: u128 = destination_balance
            .checked_add(amount)
            .ok_or("Reached limit amount on origin.")?;
        
        // set new balances if no errors
        self.set_balance(origin, new_origin_balance);
        self.set_balance(destination, new_destination_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // imports the definition of the Pallet from the top module
    use super::Pallet;

    #[test]
    fn init_balances() {
        
        let alice: String = "alice".to_string();
        
        let mut balances = Pallet::new();
        assert_eq!(balances.balance(&alice), 0);
        balances.set_balance(&alice, 100);
        assert_eq!(balances.balance(&alice), 100);
        
    }

    #[test]
    fn test_transter() {

        // init the pallet
        let mut balances: Pallet = Pallet::new();

        let origin: String = String::from("Bob");
        let destination: String = String::from("Steve");

        // TEST 1: Send money to inexisting account
        
        let transfer_result = balances.transfer_balance(&origin, &destination, 1);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Account doesn't exist.");

        // TEST 2: Insuficcient funds

        // create an account for steve
        balances.set_balance(&destination, 1);

        let transfer_result = balances.transfer_balance(&origin, &destination, 1);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Not enough funds.");
        
        // TEST 3: Send steve 1 from bob

        // set bob with some balance
        balances.set_balance(&origin, 1);

        let transfer_result = balances.transfer_balance(&origin, &destination, 1);
        assert!(transfer_result.is_ok());
        assert_eq!(balances.balance(&origin), 0);
        assert_eq!(balances.balance(&destination), 2);

        // TEST 4: Overflown on steve's account

        // set bob with a lot of tokens
        balances.set_balance(&origin, u128::MAX);
        let transfer_result = balances.transfer_balance(&origin, &destination, u128::MAX);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Reached limit amount on origin.");

    }
}