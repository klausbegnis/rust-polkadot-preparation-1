use std::{collections::BTreeMap, ops::AddAssign};
use num::{Zero, One};

// generic config trait for system pallet
pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number : T::BlockNumber,
    nonce : BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>{
    pub fn new() -> Self {
        Self {block_number : T::BlockNumber::zero(), nonce : BTreeMap::new()}
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, user : &T::AccountId) {
        let nonce : T::Nonce = *self.nonce.get(user).unwrap_or(&T::Nonce::zero());
        let new_nonce : T::Nonce = nonce+T::Nonce::one();
        self.nonce.insert(user.clone(), new_nonce);
    }

    pub fn get_nonce(&self, user: &T::AccountId) -> T::Nonce {
        return *self.nonce.get(user).unwrap_or(&T::Nonce::zero());
    }
}

#[cfg(test)]
mod test {
    // implement the config trait for TestConfig
    struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

    #[test]
    fn init_system() {
        let mut system_pallet : super::Pallet<TestConfig> = super::Pallet::new();

        // #Test 1: increment block number
        system_pallet.inc_block_number();
        assert_eq!(system_pallet.block_number(), 1);

        // #Test 2: increment nonce of 'alice'
        let alice : String = String::from("alice");
        system_pallet.inc_nonce(&alice);
        assert_eq!(system_pallet.get_nonce(&alice), 1);

        // #Test 3: check if 'bob' nonce is 0
        let bob : String = String::from("bob");
        assert_eq!(system_pallet.get_nonce(&bob),0);

    }
}

