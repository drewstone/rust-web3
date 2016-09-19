mod web3_api;

extern crate eth_rpc;
extern crate hyper;
extern crate rustc_serialize;

use web3_api::{ Web3Client };

fn main() {
	let mut web3 = Web3Client::new("http://localhost:8545".to_string());
	println!("{:?}", web3.get_accounts());
	println!("{:?}", web3.get_work());
}
