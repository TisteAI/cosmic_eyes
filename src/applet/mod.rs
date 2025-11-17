use cosmic::app::{Core, Task};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{window, Alignment, Length, Subscription};
use cosmic::surface::action;
use cosmic::iced_runtime::core::window::Id as SurfaceId;
use cosmic::widget::{self, button};
use cosmic::{Element, Theme};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::timer::{BreakType, TimerService, TimerState};
use crate::break_screen;

/// Messages that the applet can handle
#[derive(Debug, Clone)]
pub enum Message {
    /// Toggle the popup window
    TogglePopup,
    /// Update timer tick
    Tick,
    /// Timer state updated
    TimerUpdate {
        short_remaining: chrono::Duration,
        long_remaining: chrono::Duration,
        state: TimerState,
    },
    /// Start a break immediately
    StartBreak(BreakType),
    /// Skip current break
    SkipBreak,
    /// Postpone break
    PostponeBreak(BreakType),
    /// Configuration changed
    ConfigChanged(Config),
    /// Popup closed
    PopupClosed(SurfaceId),
    /// Break screen closed
    BreakScreenClosed(SurfaceId),
    /// Break screen action
    BreakScreenAction(break_screen::Message),
    /// Break countdown tick
    BreakTick,
}

/// The applet state
pub struct CosmicEyes {
    core: Core,
    timer_service: Arc<TimerService>,
    config: Config,
    popup: Option<SurfaceId>,
    icon_name: String,
    // Timer display state
    next_short_break: Option<chrono::Duration>,
    next_long_break: Option<chrono::Duration>,
    timer_state: TimerState,
    // Break screen state
    break_window: Option<SurfaceId>,
    break_screen: Option<break_screen::BreakScreen>,
    break_remaining: u64,
    // Notification tracking
    short_break_notified: bool,
    long_break_notified: bool,
}

impl CosmicEyes {
    pub fn new(config: Config) -> Self {
        let timer_service = Arc::new(TimerService::new(config.clone()));

        Self {
            core: Core::default(),
            timer_service,
            config,
            popup: None,
            icon_name: "cosmic-eyes-symbolic".to_string(),
            next_short_break: None,
            next_long_break: None,
            timer_state: TimerState::Running,
            break_window: None,
            break_screen: None,
            break_remaining: 0,
            short_break_notified: false,
            long_break_notified: false,
        }
    }

    /// Format time duration for display
    fn format_duration(duration: chrono::Duration) -> String {
        let total_seconds = duration.num_seconds().max(0);
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl cosmic::Application for CosmicEyes {
    type Executor = cosmic::executor::Default;
    type Flags = Config;
    type Message = Message;
    const APP_ID: &'static str = "com.system76.CosmicEyes";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, config: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = Self::new(config);

        // Start D-Bus service in background
        let timer = app.timer_service.clone();
        tokio::spawn(async move {
            if let Err(e) = crate::dbus::start_service(timer).await {
                eprintln!("Failed to start D-Bus service: {}", e);
            }
        });

        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                if let Some(id) = self.popup.take() {
                    return self.update(Message::Surface(action::destroy_popup(id)));
                } else {
                    let popup_action = action::app_popup::<Self>(
                        |state| {
                            let new_id = SurfaceId::unique();
                            state.popup = Some(new_id);
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );
                            popup_settings.positioner.size_limits = cosmic::iced::Limits::NONE
                                .min_width(300.0)
                                .max_width(400.0)
                                .min_height(200.0)
                                .max_height(600.0);
                            popup_settings
                        },
                        None,
                    );
                    return self.update(Message::Surface(popup_action));
                }
            }
            Message::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(action),
                ));
            }
            Message::Tick => {
                // Query timer service and update display
                let timer = self.timer_service.clone();
                let config = self.config.clone();
                Task::perform(
                    async move {
                        // Check for idle detection
                        if config.idle_detection {
                            let is_idle = crate::idle::is_idle(config.idle_threshold).await;
                            let state = timer.state().await;
                            let is_paused = matches!(state, TimerState::Paused);

                            // Auto-pause when idle, auto-resume when active
                            if is_idle && !is_paused {
                                timer.pause().await;
                            } else if !is_idle && is_paused {
                                timer.resume().await;
                            }
                        }

                        let short_remaining = timer.time_until_short_break().await;
                        let long_remaining = timer.time_until_long_break().await;
                        let state = timer.state().await;

                        // Check if it's time for a break
                        if let Some(break_type) = timer.check_break_time().await {
                            timer.start_break(break_type).await;
                        }

                        Message::TimerUpdate {
                            short_remaining,
                            long_remaining,
                            state,
                        }
                    },
                    |msg| cosmic::Action::App(msg),
                )
            }
            Message::TimerUpdate { short_remaining, long_remaining, state } => {
                // Update display state
                self.next_short_break = Some(short_remaining);
                self.next_long_break = Some(long_remaining);

                // Send pre-break notifications
                let notify_threshold = self.config.notification_before_break as i64;

                // Check short break notification
                if short_remaining.num_seconds() <= notify_threshold && !self.short_break_notified {
                    self.short_break_notified = true;
                    tokio::spawn(async move {
                        crate::notify::send_notification(
                            "Short Break Soon",
                            "Your short break will start soon. Save your work!"
                        ).await;
                    });
                } else if short_remaining.num_seconds() > notify_threshold + 10 {
                    // Reset notification flag when timer is well above threshold
                    self.short_break_notified = false;
                }

                // Check long break notification
                if long_remaining.num_seconds() <= notify_threshold && !self.long_break_notified {
                    self.long_break_notified = true;
                    tokio::spawn(async move {
                        crate::notify::send_notification(
                            "Long Break Soon",
                            "Your long break will start soon. Finish up what you're doing!"
                        ).await;
                    });
                } else if long_remaining.num_seconds() > notify_threshold + 10 {
                    // Reset notification flag when timer is well above threshold
                    self.long_break_notified = false;
                }

                // Check if we're entering a break state
                let entering_break = !matches!(self.timer_state, TimerState::InBreak(_))
                    && matches!(state, TimerState::InBreak(_));

                self.timer_state = state.clone();

                // Create break screen window if entering break
                if entering_break {
                    if let TimerState::InBreak(break_type) = state {
                        let duration_seconds = match break_type {
                            BreakType::Short => self.config.short_break.duration,
                            BreakType::Long => self.config.long_break.duration,
                        };

                        self.break_screen = Some(break_screen::BreakScreen::new(
                            break_type,
                            duration_seconds,
                            self.config.allow_skip && !self.config.strict_mode,
                            self.config.allow_postpone && !self.config.strict_mode,
                        ));
                        self.break_remaining = duration_seconds;

                        let new_id = SurfaceId::unique();
                        self.break_window = Some(new_id);

                        let window_settings = window::Settings {
                            size: cosmic::iced::Size::new(800.0, 600.0),
                            position: window::Position::Centered,
                            decorations: false,
                            exit_on_close_request: false,
                            ..Default::default()
                        };

                        return window::spawn(new_id, window_settings);
                    }
                }

                // Close break screen window if exiting break
                if let Some(window_id) = self.break_window {
                    if !matches!(self.timer_state, TimerState::InBreak(_)) {
                        self.break_window = None;
                        self.break_screen = None;
                        return window::close(window_id);
                    }
                }

                Task::none()
            }
            Message::StartBreak(break_type) => {
                // Start break immediately
                let timer = self.timer_service.clone();
                Task::perform(
                    async move {
                        timer.start_break(break_type).await;
                    },
                    |_| cosmic::Action::App(Message::Tick),
                )
            }
            Message::SkipBreak => {
                let timer = self.timer_service.clone();
                Task::perform(
                    async move {
                        timer.skip_break().await;
                    },
                    |_| cosmic::Action::App(Message::Tick),
                )
            }
            Message::PostponeBreak(break_type) => {
                let timer = self.timer_service.clone();
                Task::perform(
                    async move {
                        timer.postpone_break(break_type).await;
                    },
                    |_| cosmic::Action::App(Message::Tick),
                )
            }
            Message::ConfigChanged(new_config) => {
                self.config = new_config.clone();
                let timer = self.timer_service.clone();
                Task::perform(
                    async move {
                        timer.update_config(new_config).await;
                    },
                    |_| cosmic::Action::App(Message::Tick),
                )
            }
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::BreakScreenClosed(id) => {
                if self.break_window == Some(id) {
                    self.break_window = None;
                    self.break_screen = None;
                    // End the break in timer service
                    let timer = self.timer_service.clone();
                    return Command::perform(
                        async move {
                            timer.end_break().await;
                        },
                        |_| Message::Tick,
                    );
                }
                Command::none()
            }
            Message::BreakScreenAction(action) => {
                match action {
                    break_screen::Message::Skip => {
                        // Close break window and skip break
                        if let Some(window_id) = self.break_window {
                            self.break_window = None;
                            self.break_screen = None;
                            let timer = self.timer_service.clone();
                            return Command::batch(vec![
                                window::close(window_id),
                                Command::perform(
                                    async move {
                                        timer.skip_break().await;
                                    },
                                    |_| Message::Tick,
                                ),
                            ]);
                        }
                    }
                    break_screen::Message::Postpone => {
                        // Close break window and postpone
                        if let Some(window_id) = self.break_window {
                            if let TimerState::InBreak(break_type) = &self.timer_state {
                                let break_type = *break_type;
                                self.break_window = None;
                                self.break_screen = None;
                                let timer = self.timer_service.clone();
                                return Command::batch(vec![
                                    window::close(window_id),
                                    Command::perform(
                                        async move {
                                            timer.postpone_break(break_type).await;
                                        },
                                        |_| Message::Tick,
                                    ),
                                ]);
                            }
                        }
                    }
                    break_screen::Message::Close => {
                        // Just close the window, break will end naturally
                        if let Some(window_id) = self.break_window {
                            self.break_window = None;
                            self.break_screen = None;
                            return window::close(window_id);
                        }
                    }
                    break_screen::Message::Tick => {
                        // This is handled by BreakTick message
                    }
                }
                Command::none()
            }
            Message::BreakTick => {
                // Update break countdown
                if self.break_remaining > 0 {
                    self.break_remaining = self.break_remaining.saturating_sub(1);
                    if let Some(ref mut screen) = self.break_screen {
                        screen.update_remaining(self.break_remaining);
                    }

                    // If break time is up, end the break
                    if self.break_remaining == 0 {
                        if let Some(window_id) = self.break_window {
                            self.break_window = None;
                            self.break_screen = None;
                            let timer = self.timer_service.clone();
                            return Command::batch(vec![
                                window::close(window_id),
                                Command::perform(
                                    async move {
                                        timer.end_break().await;
                                    },
                                    |_| Message::Tick,
                                ),
                            ]);
                        }
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button(&self.icon_name)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: SurfaceId) -> Element<Self::Message> {
        // Check if this is the break screen window
        if Some(id) == self.break_window {
            if let Some(ref screen) = self.break_screen {
                return screen.view().map(Message::BreakScreenAction);
            }
        }

        // Otherwise, render the popup window
        let spacing = cosmic::theme::active().cosmic().spacing;

        // Format timer display
        let short_text = if let Some(duration) = self.next_short_break {
            format!("Short break: {}", Self::format_duration(duration))
        } else {
            "Short break: calculating...".to_string()
        };

        let long_text = if let Some(duration) = self.next_long_break {
            format!("Long break: {}", Self::format_duration(duration))
        } else {
            "Long break: calculating...".to_string()
        };

        let status_text = match &self.timer_state {
            TimerState::Running => "Status: Active",
            TimerState::Paused => "Status: Paused",
            TimerState::InBreak(BreakType::Short) => "Status: In short break",
            TimerState::InBreak(BreakType::Long) => "Status: In long break",
            TimerState::Postponed => "Status: Break postponed",
        };

        let content = widget::column()
            .spacing(spacing.space_m)
            .padding(spacing.space_m)
            .push(
                widget::container(
                    widget::text("Cosmic Eyes")
                        .size(24)
                )
                .width(Length::Fill)
                .center_x(Length::Fill)
            )
            .push(widget::divider::horizontal::default())
            .push(
                widget::column()
                    .spacing(spacing.space_s)
                    .push(widget::text(short_text))
                    .push(widget::text(long_text))
                    .push(widget::text(status_text))
            )
            .push(widget::divider::horizontal::default())
            .push(
                widget::text("Quick Actions")
                    .size(16)
                    .width(Length::Fill)
            )
            .push(
                widget::row()
                    .spacing(spacing.space_s)
                    .push(
                        button::standard("Short Break")
                            .on_press(Message::StartBreak(BreakType::Short))
                    )
                    .push(
                        button::standard("Long Break")
                            .on_press(Message::StartBreak(BreakType::Long))
                    )
            );

        widget::container(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            // Timer tick every second
            cosmic::iced::time::every(std::time::Duration::from_secs(1))
                .map(|_| Message::Tick),
        ];

        // Add break countdown tick when in break
        if self.break_window.is_some() {
            subscriptions.push(
                cosmic::iced::time::every(std::time::Duration::from_secs(1))
                    .map(|_| Message::BreakTick),
            );
        }

        Subscription::batch(subscriptions)
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
