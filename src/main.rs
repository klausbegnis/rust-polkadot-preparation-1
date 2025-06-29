mod balances;
mod system; 

#[derive(Debug)]
pub struct Runtime {
    balances : balances::Pallet<Self,>,
    system : system::Pallet<Self,>,
}

impl Runtime {
    fn new() -> Self {
        Self { balances: balances::Pallet::new(), system: system::Pallet::new() }
    }
}

// implement the system config trait for Runtime
impl system::Config for Runtime {
    type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
}

// implement the balace config trait for Runtime
impl balances::Config for Runtime {
	type Balance = u128;
}

fn main() {
    // create Runtime
    let mut runtime : Runtime = Runtime::new();

    // users
    let alice : String = String::from("alice");
    let bob : String = String::from("bob");
    let charlie : String = String::from("charlie");

    // increment the block number
    runtime.system.inc_block_numer();
    assert_eq!(runtime.system.block_number(), 1);

    // set alice's balance to 60
    runtime.balances.set_balance(&alice, 60);
    
    // increment nonce and do a transfer
    // result with error since bob has no account so far
    runtime.system.inc_nonce(&alice);
    let _res= runtime.balances.transfer_balance(&alice, &bob, 30);
    println!("account doesn't exist transaction failed: {}", _res.is_err_and(|err| err == "Account doesn't exist."));
    

    runtime.balances.set_balance(&charlie, 0);
    // increment nonce and do another transfer
    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer_balance(&alice, &charlie, 20);
    println!("transfer succesfull: {}", _res.is_ok());

    println!("Runtime: {:#?}", runtime);
}
