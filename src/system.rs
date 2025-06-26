use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number : u32,
    nonce : BTreeMap<String, u32>,
}

impl Pallet{
    pub fn new() -> Self {
        Self {block_number : 0, nonce : BTreeMap::new()}
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_numer(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, user : &String) {
        let nonce : u32 = *self.nonce.get(user).unwrap_or(&0);
        let new_nonce : u32 = nonce+1;
        self.nonce.insert(user.clone(), new_nonce);
    }

    pub fn get_nonce(&self, user: &String) -> u32 {
        return *self.nonce.get(user).unwrap_or(&0);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system_pallet : super::Pallet = super::Pallet::new();

        // #Test 1: increment block number
        system_pallet.inc_block_numer();
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

