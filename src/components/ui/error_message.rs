use iced::Element;
use iced::widget::{container, row, text};
use crate::components::enums::Message;
use crate::components::styles;

pub fn view_error_message(error: &str) -> Element<'_, Message> {
    container(
        row![
            text("⚠ ").size(16),
            text(error).style(|theme| text::danger(theme)),
        ]
        .spacing(8)
    )
    .padding([12, 16])
    .style(styles::error_card)
    .into()
}

pub fn view_empty_error() -> Element<'static, Message> {
    Element::from(container(text("")).height(0))
}
