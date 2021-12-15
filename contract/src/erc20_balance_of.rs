#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    runtime_args, CLType, CLValue, ContractHash, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, RuntimeArgs, U256,
};

#[no_mangle]
pub extern "C" fn call() {
    let contract_hash = ContractHash::from_formatted_str(
        "contract-4120116565bd608fAe6a45078055F320a2f429f426C86797b072B4EFD15B186A",
    )
    .unwrap();

    let delegator = Key::from_formatted_str(
        "account-hash-0fb415867b2799432c8cd341ff31800780f3aa5a975e1a526387b81c9e881b92",
    )
    .unwrap();

    let args = runtime_args! {
        "address" => delegator,

    };

    let balances: U256 = runtime::call_contract(contract_hash, "balance_of", args);
    runtime::put_key("resulttype", storage::new_uref(balances).into());
}
