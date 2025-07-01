use std::collections::BTreeMap;
use num::{CheckedAdd, CheckedSub, Zero};

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

// generic config trait for balances pallet
pub trait Config : crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T : Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl <T : Config> Pallet<T>
{
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        // .get returns Option<&u128> - &u128 a reference to a value
        // unwrap returns the reference the option -
        // * dereferences the reference and gets te actual value
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn account_exists(&self, who: &T::AccountId) -> Option<&T::Balance> {
        // check if account exists
        self.balances.get(who)
    }

    pub fn transfer_balance(
        &mut self,
        origin : T::AccountId, // use references instead of borrowing
        destination : T::AccountId, 
        amount: T::Balance
        // if sucessfull return nothing - otherwise a string literal with lifetime across from the entire program
    ) -> crate::support::DispatchResult {
        // get current balances
        let origin_balance:T::Balance = self.balance(&origin);
        let destination_balance:T::Balance = self.balance(&destination);
        
        let destination_exists = self.account_exists(&destination);

        // check if destination exists
        match destination_exists  {
            None => {
                return Err("Account doesn't exist.");
            }
            _ => ()
        }

        // check new balances or return error
        let new_origin_balance: T::Balance = origin_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds.")?;

        // aditional check if origin exists



        let new_destination_balance: T::Balance = destination_balance
            .checked_add(&amount)
            .ok_or("Reached limit amount on origin.")?;
        
        // set new balances if no errors
        self.set_balance(&origin, new_origin_balance);
        self.set_balance(&destination, new_destination_balance);
        Ok(())
    }
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	Transfer { to : T::AccountId, amount : T::Balance }
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
            Call::Transfer { to, amount } => {
                self.transfer_balance(caller, to, amount)?
            }
        }
        Ok(())
	}
}



#[cfg(test)]
mod tests {
    use crate::balances;

    // imports the definition of the Pallet from the top module
    use super::Pallet;

    struct TestConfig;
    impl crate::system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl balances::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        
        let alice: String = "alice".to_string();
        
        let mut balances: Pallet<TestConfig> = Pallet::new();
        assert_eq!(balances.balance(&alice), 0);
        balances.set_balance(&alice, 100);
        assert_eq!(balances.balance(&alice), 100);
        
    }

    #[test]
    fn test_transter() {

        // init the pallet
        let mut balances: Pallet<TestConfig> = Pallet::new();

        let origin: String = String::from("Bob");
        let destination: String = String::from("Steve");

        // TEST 1: Send money to inexisting account
        
        let transfer_result = balances.transfer_balance(origin.clone(), destination.clone(), 1);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Account doesn't exist.");

        // TEST 2: Insuficcient funds

        // create an account for steve
        balances.set_balance(&destination, 1);

        let transfer_result = balances.transfer_balance(origin.clone(), destination.clone(), 1);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Not enough funds.");
        
        // TEST 3: Send steve 1 from bob

        // set bob with some balance
        balances.set_balance(&origin, 1);

        let transfer_result = balances.transfer_balance(origin.clone(), destination.clone(), 1);
        assert!(transfer_result.is_ok());
        assert_eq!(balances.balance(&origin), 0);
        assert_eq!(balances.balance(&destination), 2);

        // TEST 4: Overflown on steve's account

        // set bob with a lot of tokens
        balances.set_balance(&origin, u128::MAX);
        let transfer_result = balances.transfer_balance(origin.clone(), destination.clone(), u128::MAX);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "Reached limit amount on origin.");

    }
}