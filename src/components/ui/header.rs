use iced::{Element, Length};
use iced::widget::{button, column, container, row, text, text_input};
use crate::components::enums::{HTTPMethod, Message};
use crate::components::pick_list::pick_list_view;
use crate::components::styles;

pub fn view_header<'a>(
    method: HTTPMethod,
    url: &'a str,
    is_loading: bool,
) -> Element<'a, Message> {
    container(
        column![
            text("HTTP Client")
                .size(24)
                .color(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            
            row![
                pick_list_view(Some(method)),
                text_input("https://api.example.com/endpoint", url)
                    .on_input(Message::UrlChanged)
                    .width(Length::Fill)
                    ,
                if is_loading {
                    button(text("Sending...").size(14))
                    .height(Length::Fill)
                } else {
                    button(text("Send").size(14))
                        .on_press(Message::Submit)
                        .height(Length::Fill)
                }
            ]
            .spacing(10)
            .align_y(iced::alignment::Vertical::Center),
        ]
        .spacing(10)
    )
    .padding(18)
    .style(styles::header_container)
    .into()
}
