use crate::components::enums::Message;
use crate::components::styles;
use iced::Element;
use iced::widget::{container, row, text};

pub fn view_error_message(error: &str) -> Element<'_, Message> {
    container(
        row![
            text("⚠ ").size(16),
            text(error).style(text::danger),
        ]
        .spacing(8),
    )
    .padding([12, 16])
    .style(styles::error_card)
    .into()
}

pub fn view_empty_error() -> Element<'static, Message> {
    Element::from(container(text("")).height(0))
}
