use hyper::{Client};
use std::io::Read;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use rustc_serialize::json;

#[derive(RustcDecodable, Debug)]
pub struct RpcResponse {
	id: usize,
	jsonrpc: f64,
	result: Vec<String>,
}

pub struct Web3Client {
	pub client: Client,
}

impl Web3Client {
	pub fn new() -> Web3Client {
		Web3Client {
			client: Client::new(),
		}
	}

	pub fn get_accounts(&mut self) -> RpcResponse {
		let accounts_command = r#"{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":1}"#;

		let mut result: RpcResponse = match self.client
		.post("http://localhost:8545")
		.body(accounts_command)
		.headers(get_headers())
		.send() {
			Ok(mut res) => {
				let mut buf: String = String::new();
				res.read_to_string(&mut buf).unwrap();
				let decoded: RpcResponse = json::decode(&buf).unwrap();
				decoded
			},
			Err(_) => panic!("error".to_string()),
		};

		result
	}

	pub fn get_work(&mut self) -> RpcResponse {
		let accounts_command = r#"{"jsonrpc":"2.0","method":"eth_getWork","params":[],"id":73}"#;

		let mut result: RpcResponse = match self.client
		.post("http://localhost:8545")
		.body(accounts_command)
		.headers(get_headers())
		.send() {
			Ok(mut res) => {
				let mut buf: String = String::new();
				res.read_to_string(&mut buf).unwrap();
				let decoded: RpcResponse = json::decode(&buf).unwrap();
				decoded
			},
			Err(_) => panic!("Error getting work package".to_string())
		};

		result	
	}
}

fn get_headers() -> Headers {
	let mut headers = Headers::new();
	headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
	headers
}
