use std::net::IpAddr;

use clap::Parser;
use io_core::DEFAULT_PORT;

#[derive(Debug, Parser)]
pub struct Arguments {
    pub address: IpAddr,
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    pub port: u16,
}
