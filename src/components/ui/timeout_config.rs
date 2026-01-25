use iced::Element;
use iced::Length::Fill;
use iced::widget::{container, row, text, text_input};
use crate::components::enums::Message;
use crate::components::styles;

pub fn view_timeout_config(timeout_ms: &str) -> Element<'_, Message> {
    container(
        row![
            text("⏱ Timeout (ms):").size(14),
            text_input("30000", timeout_ms)
                .on_input(Message::TimeoutChanged)
                .width(Fill),
        ]
        .spacing(12)
    )
    .padding(16)
    .style(styles::config_card)
    .into()
}
