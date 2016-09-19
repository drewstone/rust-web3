use hyper::{Client};
use std::io::Read;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::client::response::Response;

use rustc_serialize::json;

#[derive(RustcDecodable, Debug)]
pub struct RpcResponse {
	id: usize,
	jsonrpc: f64,
	result: Vec<String>,
}

pub struct Web3Client {
	pub client: Client,
	pub url: String,
}

impl Web3Client {
	pub fn new(url: String) -> Web3Client {
		Web3Client {
			client: Client::new(),
			url: url
		}
	}

	fn request(&mut self, command: String) -> RpcResponse {
		let result: RpcResponse = match self.client
		.post(&self.url)
		.body(&command)
		.headers(get_headers())
		.send() {
			Ok(res) => decode_response(res),
			Err(_) => panic!("error".to_string()),
		};

		result
	}

	pub fn client_version(&mut self) -> RpcResponse {
		let result: RpcResponse = self.request(r#"{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":67}"#.to_string());
		result
	}

	pub fn get_accounts(&mut self) -> RpcResponse {
		let result: RpcResponse = self.request(r#"{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":1}"#.to_string());
		result
	}

	pub fn get_work(&mut self) -> RpcResponse {
		let result: RpcResponse = self.request(r#"{"jsonrpc":"2.0","method":"eth_getWork","params":[],"id":73}"#.to_string());
		result	
	}
}

fn decode_response(mut response: Response) -> RpcResponse {
	let mut buf: String = String::new();
	response.read_to_string(&mut buf).unwrap();
	let decoded: RpcResponse = json::decode(&buf).unwrap();
	decoded
}

fn get_headers() -> Headers {
	let mut headers = Headers::new();
	headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
	headers
}
