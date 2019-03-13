pub mod dummy_dot_product;
pub mod html;

use crate::common::{self, Record};
use std::default::Default;
use std::net::{Ipv4Addr, TcpStream};

pub fn execute() {
    println!("Lib Hello World!");
    common::execute();
}

pub enum Market {
    BTCUSD,
}

pub struct Client {
    _server_addr: Ipv4Addr,
}

pub trait Trader {
    fn get_quote(&self, market: &Market) -> Record;
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
    fn get_quote(&self, _market: &Market) -> Record {
        let mut _stream = TcpStream::connect(&self._server_addr.to_string());
        Record::default()
    }
}
