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
    /// Surface action (for popups)
    Surface(cosmic::surface::Action),
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
                Task::perform(
                    async move {
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
                self.timer_state = state;
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
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button(&self.icon_name)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: SurfaceId) -> Element<Self::Message> {
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
        // Timer tick every second
        cosmic::iced::time::every(std::time::Duration::from_secs(1))
            .map(|_| Message::Tick)
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
