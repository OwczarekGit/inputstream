use clap::Parser;
use lib_inputstream::consts::DEFAULT_PORT;

#[derive(Debug, Parser)]
pub struct Config {
    /// IP address of the server to connect to
    pub address: String,
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
}
