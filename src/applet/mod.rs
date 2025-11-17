use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::{window, Alignment, Length, Subscription};
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
}

/// The applet state
pub struct CosmicEyes {
    core: Core,
    timer_service: Arc<TimerService>,
    config: Config,
    popup: Option<SurfaceId>,
    icon_name: String,
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

    fn init(core: Core, config: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self::new(config);
        (app, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                if let Some(id) = self.popup.take() {
                    destroy_popup(id)
                } else {
                    let new_id = SurfaceId::unique();
                    self.popup = Some(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        SurfaceId::MAIN,
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

                    get_popup(popup_settings)
                }
            }
            Message::Tick => {
                // Timer logic handled by subscription
                Command::none()
            }
            Message::StartBreak(break_type) => {
                // Start break immediately
                let timer = self.timer_service.clone();
                Command::perform(
                    async move {
                        timer.start_break(break_type).await;
                    },
                    |_| Message::Tick,
                )
            }
            Message::SkipBreak => {
                let timer = self.timer_service.clone();
                Command::perform(
                    async move {
                        timer.skip_break().await;
                    },
                    |_| Message::Tick,
                )
            }
            Message::PostponeBreak(break_type) => {
                let timer = self.timer_service.clone();
                Command::perform(
                    async move {
                        timer.postpone_break(break_type).await;
                    },
                    |_| Message::Tick,
                )
            }
            Message::ConfigChanged(new_config) => {
                self.config = new_config.clone();
                let timer = self.timer_service.clone();
                Command::perform(
                    async move {
                        timer.update_config(new_config).await;
                    },
                    |_| Message::Tick,
                )
            }
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
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

    fn view_window(&self, _id: SurfaceId) -> Element<Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        // Get current timer information
        let timer_service = self.timer_service.clone();

        let content = widget::column()
            .spacing(spacing.space_m)
            .padding(spacing.space_m)
            .push(
                widget::text("Cosmic Eyes")
                    .size(24)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
            )
            .push(widget::divider::horizontal::default())
            .push(
                widget::column()
                    .spacing(spacing.space_s)
                    .push(widget::text("Next break: Loading..."))
                    .push(widget::text("Status: Active"))
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

    fn style(&self) -> Option<<Theme as cosmic::iced_runtime::core::application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}
