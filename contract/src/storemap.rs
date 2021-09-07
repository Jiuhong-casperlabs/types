#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use std::collections::BTreeMap;

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let key1 = String::from("first");
    let key2 = String::from("second");
    let value1 = (123, true, "hello");
    let value2 = (999, false, "world");
    let mut map = BTreeMap::new();
    let mut map2 = BTreeMap::new();

    map.insert(key1.clone(), value1.clone());
    map.insert(key2.clone(), value2.clone());
    map2.insert("aaa", &map);

    let uref = storage::new_uref(map);
    let uref2 = storage::new_uref(map2);
    runtime::put_key("testmap", uref.into());
}

//reference https://github.com/casper-network/casper-node/blob/e2027dbe979ebf91f10ba8a90ffba1fddbd9fb09/types/src/cl_value/jsonrepr.rs
