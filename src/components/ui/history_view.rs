use crate::components::enums::{HistoryItem, Message};
use crate::components::history::RequestHistory;
use crate::components::styles;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};

pub fn view_history(history: &RequestHistory) -> Element<'_, Message> {
    let mut history_column = column![view_history_header(),].spacing(10);

    if history.is_empty() {
        history_column = history_column.push(view_empty_history());
    } else {
        for (index, item) in history.get_items().iter().enumerate() {
            history_column = history_column.push(view_history_item(index, item));
        }
    }

    container(history_column)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(styles::history_container)
        .into()
}

fn view_history_header() -> Element<'static, Message> {
    column![
        text("📜 History")
            .size(18)
            .color(iced::Color::from_rgb(0.9, 0.9, 0.9)),
        button(text("Clear").size(12))
            .on_press(Message::ClearHistory)
            .padding([6, 12])
            .width(Length::Fill)
            .style(button::danger)
    ]
    .spacing(10)
    .into()
}

fn view_empty_history() -> Element<'static, Message> {
    container(
        text("No requests yet")
            .size(13)
            .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
    )
    .padding(12)
    .center(Length::Fill)
    .into()
}

fn view_history_item(index: usize, item: &HistoryItem) -> Element<'_, Message> {
    let formatted_time = RequestHistory::format_timestamp(item.timestamp);
    let status_color = get_status_color(item.response.status);
    let duration_ms = item.response.duration_ms;

    container(
        button(
            column![
                view_history_item_main(item, status_color),
                text(format!("{} • {}ms", formatted_time, duration_ms))
                    .size(11)
                    .color(iced::Color::from_rgb(0.55, 0.55, 0.55)),
            ]
            .spacing(6),
        )
        .on_press(Message::LoadFromHistory(index))
        .padding(10)
        .width(Length::Fill)
        .style(button::secondary),
    )
    .style(styles::history_item)
    .into()
}

fn view_history_item_main(item: &HistoryItem, status_color: iced::Color) -> Element<'_, Message> {
    column![
        row![
            view_method_badge(&item.request.method),
            view_status_badge(item.response.status, status_color),
        ]
        .spacing(8),
        text(&item.request.url)
            .size(12)
            .color(iced::Color::from_rgb(0.8, 0.8, 0.8)),
    ]
    .spacing(6)
    .into()
}

fn view_method_badge(method: &crate::components::enums::HTTPMethod) -> Element<'_, Message> {
    container(text(format!("{}", method)).size(12))
        .padding([4, 8])
        .style(styles::method_badge)
        .into()
}

fn view_status_badge(status: u16, color: iced::Color) -> Element<'static, Message> {
    container(text(format!("{}", status)).size(12).color(color))
        .padding([4, 8])
        .style(styles::status_badge_border(color))
        .into()
}

fn get_status_color(status: u16) -> iced::Color {
    if (200..300).contains(&status) {
        iced::Color::from_rgb(0.0, 0.7, 0.0)
    } else if status >= 400 {
        iced::Color::from_rgb(0.8, 0.0, 0.0)
    } else {
        iced::Color::from_rgb(0.7, 0.5, 0.0)
    }
}
