use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    /// The port to host the server.
    #[arg(default_value_t = lib_inputstream::consts::DEFAULT_PORT)]
    pub port: u16,
}
