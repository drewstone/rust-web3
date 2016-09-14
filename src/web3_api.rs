use hyper::{Client};
use std::io::Read;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use rustc_serialize::json;

pub struct Result 

#[derive(RustcDecodable)]
pub struct RpcResponse {
	id: usize,
	jsonrpc: f64,
	result: Vec<String>,
	error: Option<String>
}

pub struct Web3Client {
	pub client: Client,
	headers: Headers,
}

impl Web3Client {
	pub fn new() -> Web3Client {
		let mut headers = Headers::new();
		headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));

		Web3Client {
			client: Client::new(),
			headers: headers,
		}
	}

	pub fn get_accounts(self) -> RpcResponse {
		let accounts_command = r#"{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":1}"#;

		match self.client
		.post("http://localhost:8545")
		.body(accounts_command)
		.headers(self.headers)
		.send() {
			Ok(mut response) => {
				let mut buf = String::new();
    			response.read_to_string(&mut buf).unwrap();
    			let decoded: RpcResponse = json::decode(&buf).unwrap()
			},
			Err(_) => RpcResponse {
				id: 0,
				jsonrpc: 0.0,
				result: !vec(),
				error: Some("Error"),
			},
		}
	}
}
