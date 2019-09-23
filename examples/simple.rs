extern crate web3_rs_wasm;

use web3_rs_wasm::futures::Future;

fn main() {
    let (_eloop, http) = web3_rs_wasm::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3_rs_wasm::Web3::new(http);
    let accounts = web3.eth().accounts().wait().unwrap();

    println!("Accounts: {:?}", accounts);
}