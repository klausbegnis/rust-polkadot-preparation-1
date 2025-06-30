use crate::support::{Dispatch};

mod balances;
mod system;
mod support;

// define the types for each generic
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// not implemented yet
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
}

// runtime definition
#[derive(Debug)]
pub struct Runtime {
    balances : balances::Pallet<Self,>,
    system : system::Pallet<Self,>,
}

impl Runtime {
    fn new() -> Self {
        Self { balances: balances::Pallet::new(), system: system::Pallet::new() }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        // increment block number
        self.system.inc_block_number();

        //println!("block number {}", self.system.block_number());
        //println!("new block number {}", block.header.block_number);

        // check if block number from header matches system block number
        if block.header.block_number != self.system.block_number() {
            return Err("block number doesn't match");
        }

        for (i, support::Extrinsic {caller, call}) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _result = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
					"Extrinsic error \n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
            });
        }
        Ok(())
        
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

// implement the system config trait for Runtime
impl system::Config for Runtime {
    type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

// implement the balace config trait for Runtime
impl balances::Config for Runtime {
	type Balance = types::Balance;
}

fn main() {
    // create Runtime
    let mut runtime : Runtime = Runtime::new();

    // users
    let alice : String = String::from("alice");
    let bob : String = String::from("bob");
    let charlie : String = String::from("charlie");
    // set alice's balance to 60
    runtime.balances.set_balance(&alice, 60);
    // create bob's account
    runtime.balances.set_balance(&bob, 0);
    // create charlies's account
    runtime.balances.set_balance(&charlie, 0);

    let new_block = crate::support::Block{ 
        header:crate::support::Header { block_number : 1}, 
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call : RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 20 })
            },
            support::Extrinsic {
                caller: alice.clone(),
                call : RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 10 })
            }
        ]
    };

    let _res = runtime.execute_block(new_block).expect("invalid block");

    println!("{:#?}", runtime);
}
