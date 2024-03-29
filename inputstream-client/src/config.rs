use std::net::Ipv4Addr;

use clap::Parser;
use lib_inputstream::consts::DEFAULT_PORT;

/// Simple client for inputstream using SDL2
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// IP address of the server to connect to
    pub address: Ipv4Addr,
    #[arg(default_value_t = DEFAULT_PORT)]
    pub port: u16,
    /// Mouse acceleration
    #[arg(short = 'a', long, default_value_t = 1.0)]
    pub mouse_accel: f32,
    /// Duration between the event pooling in ms
    /// 1 means 1000 pools/sec
    #[arg(short, long, default_value_t = 1)]
    pub rate: u64,
    /// Ignore super/windows key
    #[arg(long, default_value_t = false)]
    pub ignore_super: bool,
    /// Disable mouse grabbing
    #[arg(long = "nograb", default_value_t = false)]
    pub disable_mouse: bool,
    /// Disable gamepad capture
    #[arg(long = "nojoy", default_value_t = false)]
    pub disable_gamepad: bool,
}
