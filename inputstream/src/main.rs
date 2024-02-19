use clap::Parser;

pub use error::{Error, Result};
use event_handlers::{
    channels::create_channels,
    handler::{EventHandler, OsuEventHandler},
};
use server::Server;

mod config;
mod error;
mod event_handlers;
mod senders;
mod server;

fn main() -> Result<()> {
    let config = config::Config::parse();
    let mut server = Server::new(config.port)?;

    let (senders, osu_recv, keyboard_recv, mouse_recv) = create_channels();

    OsuEventHandler.run_detached(osu_recv);

    server.start(senders)?;

    Ok(())
}
