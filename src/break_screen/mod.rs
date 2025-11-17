use cosmic::iced::Length;
use cosmic::widget::{self, container};
use cosmic::{app, Application, Element};

use crate::timer::BreakType;

/// Message types for the break screen
#[derive(Debug, Clone)]
pub enum Message {
    /// Skip the current break
    Skip,
    /// Postpone the break
    Postpone,
    /// Break timer tick (update remaining time)
    Tick,
    /// Close the break screen
    Close,
}

/// Break screen state
pub struct BreakScreen {
    break_type: BreakType,
    remaining_seconds: u64,
    allow_skip: bool,
    allow_postpone: bool,
}

impl BreakScreen {
    pub fn new(
        break_type: BreakType,
        duration_seconds: u64,
        allow_skip: bool,
        allow_postpone: bool,
    ) -> Self {
        Self {
            break_type,
            remaining_seconds: duration_seconds,
            allow_skip,
            allow_postpone,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = match self.break_type {
            BreakType::Short => "Time for a short break!",
            BreakType::Long => "Time for a long break!",
        };

        let message_text = match self.break_type {
            BreakType::Short => "Look away from your screen and rest your eyes",
            BreakType::Long => "Stand up, stretch, and take a walk",
        };

        let minutes = self.remaining_seconds / 60;
        let seconds = self.remaining_seconds % 60;
        let time_text = format!("{:02}:{:02}", minutes, seconds);

        let mut content = widget::column()
            .spacing(20)
            .align_x(cosmic::iced::alignment::Horizontal::Center)
            .push(
                widget::text(title)
                    .size(48)
                    .width(Length::Shrink)
            )
            .push(
                widget::text(message_text)
                    .size(24)
                    .width(Length::Shrink)
            )
            .push(
                widget::text(time_text)
                    .size(72)
                    .width(Length::Shrink)
            );

        // Add buttons if allowed
        if self.allow_skip || self.allow_postpone {
            let mut buttons = widget::row().spacing(20);

            if self.allow_postpone {
                buttons = buttons.push(
                    widget::button::standard("Postpone")
                        .on_press(Message::Postpone)
                );
            }

            if self.allow_skip {
                buttons = buttons.push(
                    widget::button::standard("Skip")
                        .on_press(Message::Skip)
                );
            }

            content = content.push(buttons);
        }

        // Use styled container with solid background
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(cosmic::theme::Container::default())
            .into()
    }

    pub fn update_remaining(&mut self, seconds: u64) {
        self.remaining_seconds = seconds;
    }
}
