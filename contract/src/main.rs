#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, vec::Vec};

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
// use casper_types::RuntimeArgs;
use casper_types::{runtime_args, ApiError, ContractHash, ContractWasmHash, Key, RuntimeArgs};
const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

#[no_mangle]
pub extern "C" fn call() {
    // let token_hash = ContractHash::from_formatted_str(
    //     "contract-127d903e0ea04cb48ebb72da32a53297e55322c9f920af35ade1da53d60bf73d",
    // )
    // .unwrap_or_default();

    let a = [123u8; 32];
    let b = ContractHash::new(a);
    let c = b.to_formatted_string();

    let aa = ContractHash::from_formatted_str(
        "contract-127d903e0ea04cb48ebb72da32a53297e55322c9f920af35ade1da53d60bf73d",
    )
    .unwrap();
    //the input for aa above is "hash-127d903e0ea04cb48ebb72da32a53297e55322c9f920af35ade1da53d60bf73d"
    //under account

    let recipient: Key = Key::from_formatted_str(
        "account-hash-2293223427d59ebb331ac2221c3fcd1b3656a5cb72be924a6cdc9d52cdb6db0f",
    )
    .unwrap();
    // runtime::put_key("whatisrecipient", recipient.into());

    let token_id = Some(String::from("@")); // !!!This cannot be already exsiting one
    let key1 = String::from("aaa");

    let value1 = String::from("bbb");

    let mut token_meta = BTreeMap::new();

    token_meta.insert(key1.clone(), value1.clone());

    // runtime::call_contract::<()>(
    //     aa,
    //     "mint_one",
    //     runtime_args! {
    //         "recipient" => recipient,
    //         "token_id" => token_id,
    //         "token_meta" => token_meta
    //     },
    // );

    let mut test = RuntimeArgs::new();

    let _ = test.insert("recipient", recipient);
    let _ = test.insert("token_id", token_id);
    let _ = test.insert("token_meta", token_meta);

    // Call Counter to increment the value.
    let _: () = runtime::call_contract(aa, "mint_one", test);

    // let args = runtime_args! {
    //     "increment" => 3u64,
    // };
    // // Call Counter to increment the value.
    // let _: () = runtime::call_contract(aa, "mint_one", args);
}
