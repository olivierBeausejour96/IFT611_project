pub mod dummy_dot_product;
pub mod html;

use crate::common::Record;
use std::default::Default;
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpStream};
use std::string::ToString;

pub fn execute() {
    println!("Lib Hello World!");
}

pub enum Market {
    BTCUSD,
}

impl ToString for Market {
    fn to_string(&self) -> String {
        match self {
            Market::BTCUSD => String::from("BTCUSD"),
        }
    }
}

pub struct Client {
    _server_addr: Ipv4Addr,
}

pub trait Trader {
    fn get_quote(&self, market: &Market) -> Result<Record, &'static str>;
}

impl Client {
    pub fn new(ip_addr: Ipv4Addr) -> Client {
        Client {
            _server_addr: ip_addr,
        }
    }
}

impl Default for Client {
    fn default() -> Client {
        Client {
            _server_addr: Ipv4Addr::LOCALHOST,
        }
    }
}

impl Trader for Client {
    fn get_quote(&self, market: &Market) -> Result<Record, &'static str> {
        use html::{get_custom_string, Path};

        let path = Path::new(&format!("/{}", market.to_string()));
        let request = get_custom_string(&path);
        if let Ok(mut stream) = TcpStream::connect(&self._server_addr.to_string()) {
            stream.write_all(&request.as_bytes()).unwrap();

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            let r = parse_html_response_to_record(&response);
            Ok(r)
        } else {
            Err("Error occured trying to reach out for server")
        }
    }
}

pub fn parse_html_response_to_record(html_response: &str) -> Record {
    //extract body
    let _extracted_record_string = html_response;
    //record from string
    Record::default()
}
