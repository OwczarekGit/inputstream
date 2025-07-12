use std::net::IpAddr;

use clap::Parser;
use io_core::DEFAULT_PORT;

#[derive(Debug, Parser)]
pub struct Arguments {
    pub address: IpAddr,
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    pub port: u16,
    #[arg(long, default_value_t = 1.0)]
    pub mouse_accell: f32,
}
