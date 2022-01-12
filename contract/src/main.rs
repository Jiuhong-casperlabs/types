#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_contract::contract_api::{runtime, storage};
use std::collections::BTreeMap;

#[no_mangle]
pub extern "C" fn call() {
    let key1 = String::from("first");
    let key2 = String::from("second");
    let value1 = String::from("firstvalue");
    let value2 = String::from("secondvalue");
    let mut map = BTreeMap::new();

    map.insert(key1.clone(), value1.clone());
    map.insert(key2.clone(), value2.clone());

    let token_metas = vec![map];

    let uref = storage::new_uref(token_metas);

    runtime::put_key("token_metas", uref.into());
}

//reference https://github.com/casper-network/casper-node/blob/e2027dbe979ebf91f10ba8a90ffba1fddbd9fb09/types/src/cl_value/jsonrepr.rs
