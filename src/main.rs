mod applet;
mod break_screen;
mod config;
mod timer;
mod dbus;
mod idle;
mod notify;

use applet::CosmicEyes;
use config::Config;

fn main() -> cosmic::iced::Result {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}. Using defaults.", e);
        Config::default()
    });

    // Run the applet
    cosmic::applet::run::<CosmicEyes>(config)
}
