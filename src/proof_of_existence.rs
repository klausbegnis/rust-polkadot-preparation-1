use crate::support::DispatchResult;
use std::collections::{btree_map::Entry, BTreeMap};
use core::fmt::Debug;

// config type required for the pallet
pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

// create the proof of existence pallet
#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.get_claim(claim)
    }


    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        // check if content already claimed
        if self.claims.contains_key(&claim) {
            return Err("Content already claimed.");
        }

        // overwrites if there was content
        self.claims.insert(claim, caller);
        Ok(())
    }   


    pub fn revoke_claim(&mut self, caller: T::AccountId, claim : T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim)
                .ok_or("No owner for that claim.")?;        
        if !(*owner == caller) {
            return Err("Claim can't be revoked, since doesn't belongs to caller.")
        }
        let _res = self.claims.remove(&claim);
        match _res {
            Some(_) => Ok(()),
            None => Err("Failed to revoke claim.")
        }
    }
}

// create calls from proof of existence pallet
pub enum Call<T : Config> {
    CreateClaim {claim : T::Content},
    RevokeClaim {claim : T::Content},
}

// implement dispatch trait to the pallet
impl <T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
	type Call = Call<T>;

    fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
            Call::CreateClaim { claim } => {
                self.create_claim(caller, claim)?
            },
            Call::RevokeClaim { claim } => {
                self.revoke_claim(caller, claim)?
            }
        }
        Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim("bob", "Hello, world!"),
			Err("Content already claimed.")
		);
		assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
	}
}
