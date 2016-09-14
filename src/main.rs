mod web3_api;

extern crate eth_rpc;
extern crate hyper;
extern crate rustc_serialize;

use web3_api::{ Web3Client };

fn main() {
	let web3 = Web3Client::new();
	println!("{:?}", web3.get_accounts());
}
