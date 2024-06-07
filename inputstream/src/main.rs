use config::config;
pub use error::{Error, Result};
use event_handlers::{
    channels::create_channels,
    handler::{
        EventHandler, GamepadEventHandler, KeyboardEventHandler, MouseEventHandler, OsuEventHandler,
    },
};
use lib_inputstream::server::TcpServer;

mod config;
mod error;
mod event_handlers;
mod senders;
mod server;

fn main() -> Result<()> {
    let config = config();
    let mut server = TcpServer::new(config.port)?;

    let (senders, osu_recv, keyboard_recv, mouse_recv, gamepad_recv) = create_channels();

    OsuEventHandler.run_detached(osu_recv);
    MouseEventHandler.run_detached(mouse_recv);
    KeyboardEventHandler.run_detached(keyboard_recv);
    GamepadEventHandler.run_detached(gamepad_recv);

    server.listen(|conn| {
        let senders = senders.clone();
        std::thread::spawn(move || crate::server::listen(conn, senders));
    })?;

    Ok(())
}
