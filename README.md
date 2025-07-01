# Rust State Machine

This is my study case following the [Rust State Machin](https://dotcodeschool.com/courses/in-browser-rust-state-machine) from Polkadot.

# Anottations

- Pallets translate into rust modules that contain logic specifc for the blockchain runtime

- system pallet: holds the low level functionalities managing the heap and setting runtim code

- block: has a header and list of calls

- extrinsic : external messages from the blockchain - who makes the call and which call they are making

- header : contains information about the block

- Dispatch: trait for allowing to dispach incoming extrinsic to appropriate state transitions

- making as generic as possible - define Config for pallets, types which implement simple traits like zero, copy, etc. This allows for creating the applications on many ways, and checks on compiler level if the types used follow the requirements.

- runtime : containes the balances and system pallets, it implements the configs of each pallet it uses and also implements dispatch.

- Dispatching: implementations of dispatch in a match for each pallet that has runtime extrinsic operations example: transfer balance for the balance pallet. On pub enum RuntimeCall storing the balaces::Call<Runtime> in Balances so on the execute block we can acess each call and send the dispatch.