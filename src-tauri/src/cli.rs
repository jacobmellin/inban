mod client;
use crate::client::*;

fn main() {
 let client = FinTSClient::new(String::from("test123"));
 println!("{}", client.get_statements());
}
