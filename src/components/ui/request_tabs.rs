use iced::Element;
use iced::Length::Fill;
use iced::widget::{button, container, row, text, Button};
use crate::components::enums::{Message, RequestTab};

pub fn view_request_tabs(active_tab: RequestTab) -> Element<'static, Message> {
    container(
        row![
            tab_button("Query Params", RequestTab::QueryParams, active_tab),
            tab_button("Headers", RequestTab::Headers, active_tab),
            tab_button("Body", RequestTab::Body, active_tab),
        ]
        .spacing(8)
    )
    .padding([16, 16])
    .into()
}

fn tab_button(
    label: &'static str,
    tab: RequestTab,
    active_tab: RequestTab,
) -> Button<'static, Message> {
    let btn = button(text(label).size(14))
        .on_press(Message::TabChanged(tab))
        .padding([8, 10])
        .width(Fill);
    
    if active_tab == tab {
        btn.style(button::primary)
    } else {
        btn.style(button::secondary)
    }
}
