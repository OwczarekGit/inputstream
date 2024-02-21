use std::sync::OnceLock;

use clap::Parser;
use lib_inputstream::osu_key_args::OsuKey;

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(Config::parse)
}

/// Control your devices remotely.
#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The port to host the server.
    #[arg(default_value_t = lib_inputstream::consts::DEFAULT_PORT)]
    pub port: u16,

    /// Set custom key 1 for osu!.
    #[arg(long, default_value = "z")]
    pub osu_key1: OsuKey,

    /// Set custom key 2 for osu!.
    #[arg(long, default_value = "x")]
    pub osu_key2: OsuKey,
}
