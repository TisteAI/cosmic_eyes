use chrono::{DateTime, Duration, Local};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::{BreakConfig, Config};

/// Type of break
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakType {
    Short,
    Long,
}

/// Current state of the timer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerState {
    /// Timer is running normally
    Running,
    /// Timer is paused (e.g., due to idle detection)
    Paused,
    /// A break is active
    InBreak(BreakType),
    /// Break was postponed
    Postponed,
}

/// Timer service that manages break intervals
pub struct TimerService {
    config: Arc<RwLock<Config>>,
    state: Arc<RwLock<TimerState>>,
    short_break_next: Arc<RwLock<DateTime<Local>>>,
    long_break_next: Arc<RwLock<DateTime<Local>>>,
    break_end_time: Arc<RwLock<Option<DateTime<Local>>>>,
}

impl TimerService {
    pub fn new(config: Config) -> Self {
        let now = Local::now();
        let short_interval = Duration::minutes(config.short_break.interval as i64);
        let long_interval = Duration::minutes(config.long_break.interval as i64);

        Self {
            config: Arc::new(RwLock::new(config)),
            state: Arc::new(RwLock::new(TimerState::Running)),
            short_break_next: Arc::new(RwLock::new(now + short_interval)),
            long_break_next: Arc::new(RwLock::new(now + long_interval)),
            break_end_time: Arc::new(RwLock::new(None)),
        }
    }

    /// Get current timer state
    pub async fn state(&self) -> TimerState {
        self.state.read().await.clone()
    }

    /// Get time until next short break
    pub async fn time_until_short_break(&self) -> Duration {
        let next = *self.short_break_next.read().await;
        next - Local::now()
    }

    /// Get time until next long break
    pub async fn time_until_long_break(&self) -> Duration {
        let next = *self.long_break_next.read().await;
        next - Local::now()
    }

    /// Get the next break that will occur
    pub async fn next_break(&self) -> (BreakType, Duration) {
        let short_time = self.time_until_short_break().await;
        let long_time = self.time_until_long_break().await;

        if short_time < long_time {
            (BreakType::Short, short_time)
        } else {
            (BreakType::Long, long_time)
        }
    }

    /// Start a break
    pub async fn start_break(&self, break_type: BreakType) {
        let config = self.config.read().await;
        let duration = match break_type {
            BreakType::Short => config.short_break.duration,
            BreakType::Long => config.long_break.duration,
        };

        *self.state.write().await = TimerState::InBreak(break_type);
        *self.break_end_time.write().await = Some(
            Local::now() + Duration::seconds(duration as i64)
        );
    }

    /// End the current break and reset timers
    pub async fn end_break(&self) {
        let state = self.state.read().await.clone();

        if let TimerState::InBreak(break_type) = state {
            let config = self.config.read().await;
            let now = Local::now();

            match break_type {
                BreakType::Short => {
                    let interval = Duration::minutes(config.short_break.interval as i64);
                    *self.short_break_next.write().await = now + interval;
                }
                BreakType::Long => {
                    let interval = Duration::minutes(config.long_break.interval as i64);
                    *self.long_break_next.write().await = now + interval;
                }
            }

            *self.state.write().await = TimerState::Running;
            *self.break_end_time.write().await = None;
        }
    }

    /// Skip the current break
    pub async fn skip_break(&self) {
        self.end_break().await;
    }

    /// Postpone the next break
    pub async fn postpone_break(&self, break_type: BreakType) {
        let config = self.config.read().await;
        let postpone_duration = Duration::minutes(config.postpone_duration as i64);

        match break_type {
            BreakType::Short => {
                let mut next = self.short_break_next.write().await;
                *next = *next + postpone_duration;
            }
            BreakType::Long => {
                let mut next = self.long_break_next.write().await;
                *next = *next + postpone_duration;
            }
        }

        *self.state.write().await = TimerState::Postponed;
    }

    /// Pause the timer (e.g., when user is idle)
    pub async fn pause(&self) {
        *self.state.write().await = TimerState::Paused;
    }

    /// Resume the timer
    pub async fn resume(&self) {
        *self.state.write().await = TimerState::Running;
    }

    /// Check if it's time for a break
    pub async fn check_break_time(&self) -> Option<BreakType> {
        let state = self.state.read().await;

        // Don't trigger new breaks if already in one
        if !matches!(*state, TimerState::Running) {
            return None;
        }

        let now = Local::now();
        let short_next = *self.short_break_next.read().await;
        let long_next = *self.long_break_next.read().await;

        if now >= long_next {
            Some(BreakType::Long)
        } else if now >= short_next {
            Some(BreakType::Short)
        } else {
            None
        }
    }

    /// Update configuration
    pub async fn update_config(&self, config: Config) {
        *self.config.write().await = config;
    }
}
