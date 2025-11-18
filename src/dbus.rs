//! D-Bus interface for cosmic-eyes applet
//!
//! Provides IPC between CLI and applet using D-Bus.

use crate::timer::{BreakType, TimerService, TimerState};
use std::sync::Arc;
use zbus::{interface, ConnectionBuilder};

const SERVICE_NAME: &str = "com.github.cosmiceyes";
const OBJECT_PATH: &str = "/com/github/cosmiceyes";

/// D-Bus interface for timer control
pub struct TimerInterface {
    timer: Arc<TimerService>,
}

impl TimerInterface {
    pub fn new(timer: Arc<TimerService>) -> Self {
        Self { timer }
    }
}

#[interface(name = "com.github.cosmiceyes.Timer")]
impl TimerInterface {
    /// Start a break immediately
    async fn start_break(&self, break_type: &str) -> zbus::fdo::Result<()> {
        let bt = match break_type {
            "short" => BreakType::Short,
            "long" => BreakType::Long,
            _ => return Err(zbus::fdo::Error::InvalidArgs("Invalid break type".to_string())),
        };
        self.timer.start_break(bt).await;
        Ok(())
    }

    /// Skip current break
    async fn skip_break(&self) -> zbus::fdo::Result<()> {
        self.timer.skip_break().await;
        Ok(())
    }

    /// Postpone next break
    async fn postpone_break(&self, break_type: &str) -> zbus::fdo::Result<()> {
        let bt = match break_type {
            "short" => BreakType::Short,
            "long" => BreakType::Long,
            _ => return Err(zbus::fdo::Error::InvalidArgs("Invalid break type".to_string())),
        };
        self.timer.postpone_break(bt).await;
        Ok(())
    }

    /// Pause timers
    async fn pause(&self) -> zbus::fdo::Result<()> {
        self.timer.pause().await;
        Ok(())
    }

    /// Resume timers
    async fn resume(&self) -> zbus::fdo::Result<()> {
        self.timer.resume().await;
        Ok(())
    }

    /// Get current status
    async fn get_status(&self) -> zbus::fdo::Result<(String, i64, i64)> {
        let state = self.timer.state().await;
        let short_remaining = self.timer.time_until_short_break().await.num_seconds();
        let long_remaining = self.timer.time_until_long_break().await.num_seconds();

        let state_str = match state {
            TimerState::Running => "Running",
            TimerState::Paused => "Paused",
            TimerState::InBreak(BreakType::Short) => "InBreak:Short",
            TimerState::InBreak(BreakType::Long) => "InBreak:Long",
            TimerState::Postponed => "Postponed",
        };

        Ok((state_str.to_string(), short_remaining, long_remaining))
    }
}

/// Start D-Bus service
pub async fn start_service(timer: Arc<TimerService>) -> zbus::Result<zbus::Connection> {
    let interface = TimerInterface::new(timer);

    ConnectionBuilder::session()?
        .name(SERVICE_NAME)?
        .serve_at(OBJECT_PATH, interface)?
        .build()
        .await
}
