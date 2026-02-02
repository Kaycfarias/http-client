use iced::Length::Fill;
use iced::{Element, Length};
use iced::widget::{button, checkbox, column, container, row, text, text_input};
use crate::components::enums::{KeyValue, Message};

pub fn view_key_value_list<'a>(
    items: &'a [KeyValue],
    key_placeholder: &'static str,
    value_placeholder: &'static str,
    on_key_changed: fn(usize, String) -> Message,
    on_value_changed: fn(usize, String) -> Message,
    on_toggle: fn(usize) -> Message,
    on_remove: fn(usize) -> Message,
    on_add: Message,
    add_label: &'static str,
) -> Element<'a, Message> {
    let mut col = column![].spacing(8);

    if !items.is_empty() {
        col = col.push(view_header_row());
    }

    for (index, item) in items.iter().enumerate() {
        col = col.push(view_key_value_row(
            index,
            item,
            key_placeholder,
            value_placeholder,
            on_key_changed,
            on_value_changed,
            on_toggle,
            on_remove,
        ));
    }

    col = col.push(
        button(text(add_label))
            .on_press(on_add)
            .style(button::secondary)
    );

    container(col).padding(16).into()
}

fn view_header_row<'a>() -> Element<'a, Message> {
    row![
        container(text("")).width(40),
        container(text("Key").size(12)).width(250),
        container(text("Value").size(12)).width(Length::Fill),
        container(text("")).width(80), // Espaço para botão delete
    ]
    .spacing(8)
    .into()
}

fn view_key_value_row<'a>(
    index: usize,
    item: &'a KeyValue,
    key_placeholder: &'static str,
    value_placeholder: &'static str,
    on_key_changed: fn(usize, String) -> Message,
    on_value_changed: fn(usize, String) -> Message,
    on_toggle: fn(usize) -> Message,
    on_remove: fn(usize) -> Message,
) -> Element<'a, Message> {
    row![
        container(checkbox(item.enabled).on_toggle(move |_| on_toggle(index))).width(40),
        text_input(key_placeholder, &item.key)
            .on_input(move |v| on_key_changed(index, v))
            .width(250),
        text_input(value_placeholder, &item.value)
            .on_input(move |v| on_value_changed(index, v))
            .width(Length::Fill),
        button(text("Remove"))
            .on_press(on_remove(index))
            .style(button::danger)
            .height(Fill),
    ]
    .spacing(8)
    .into()
}
