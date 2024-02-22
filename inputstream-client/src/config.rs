use clap::Parser;
use lib_inputstream::consts::DEFAULT_PORT;

#[derive(Debug, Parser)]
pub struct Config {
    pub address: String,
    #[arg(default_value_t = DEFAULT_PORT)]
    pub port: u16,
    #[arg(short = 'a', long, default_value_t = 1.0)]
    pub mouse_accel: f32,
}
