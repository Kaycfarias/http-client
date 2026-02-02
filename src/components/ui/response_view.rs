use crate::components::enums::{HttpResponse, Message, ResponseTab};
use crate::components::styles;
use crate::components::utils::{json_formatter, text_formatter};
use iced::Element;
use iced::widget::{Button, button, column, container, row, scrollable, text};

pub fn view_response<'a>(
    response: &'a HttpResponse,
    response_tab: ResponseTab,
) -> Element<'a, Message> {
    let (status_color, status_icon) = get_status_info(response.status);

    container(
        column![
            text("Response").size(20),
            view_response_info(response, status_color, status_icon),
            view_response_tabs(response_tab),
            view_response_content(response, response_tab),
        ]
        .spacing(16),
    )
    .padding(20)
    .style(styles::response_container)
    .into()
}

pub fn view_no_response() -> Element<'static, Message> {
    container(
        container(
            text("📭 No response yet")
                .size(14)
                .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .padding(32)
        .style(styles::empty_state_card),
    )
    .padding(0)
    .into()
}

fn get_status_info(status: u16) -> (iced::Color, &'static str) {
    if (200..300).contains(&status) {
        (iced::Color::from_rgb(0.0, 0.7, 0.0), "✓")
    } else if status >= 400 {
        (iced::Color::from_rgb(0.8, 0.0, 0.0), "✗")
    } else {
        (iced::Color::from_rgb(0.7, 0.5, 0.0), "⚠")
    }
}

fn view_response_info<'a>(
    response: &'a HttpResponse,
    status_color: iced::Color,
    status_icon: &'static str,
) -> Element<'a, Message> {
    row![
        view_status_badge(response, status_color, status_icon),
        view_timing_badge(response),
    ]
    .spacing(12)
    .into()
}

fn view_status_badge<'a>(
    response: &'a HttpResponse,
    status_color: iced::Color,
    status_icon: &'static str,
) -> Element<'a, Message> {
    container(
        row![
            text(status_icon).size(16).color(status_color),
            text(format!(
                "Status: {} {}",
                response.status, response.status_text
            ))
            .color(status_color)
            .size(14),
        ]
        .spacing(8),
    )
    .padding([8, 12])
    .style(styles::status_badge(status_color))
    .into()
}

fn view_timing_badge(response: &HttpResponse) -> Element<'_, Message> {
    container(
        text(format!(
            "⏱ Time: {}",
            text_formatter::format_duration(response.duration_ms)
        ))
        .size(14),
    )
    .padding([8, 12])
    .style(styles::timing_card)
    .into()
}

fn view_response_tabs(response_tab: ResponseTab) -> Element<'static, Message> {
    row![
        response_tab_button("Body", ResponseTab::Body, response_tab),
        response_tab_button("Headers", ResponseTab::Headers, response_tab),
    ]
    .spacing(8)
    .into()
}

fn response_tab_button(
    label: &'static str,
    tab: ResponseTab,
    active_tab: ResponseTab,
) -> Button<'static, Message> {
    let btn = button(text(label).size(14))
        .on_press(Message::ResponseTabChanged(tab))
        .padding([10, 18]);

    if active_tab == tab {
        btn.style(button::primary)
    } else {
        btn.style(button::secondary)
    }
}

fn view_response_content<'a>(
    response: &'a HttpResponse,
    response_tab: ResponseTab,
) -> Element<'a, Message> {
    match response_tab {
        ResponseTab::Body => view_response_body(&response.body),
        ResponseTab::Headers => view_response_headers(&response.headers),
    }
}

fn view_response_body(body: &str) -> Element<'_, Message> {
    let formatted_body = if json_formatter::is_valid_json(body) {
        json_formatter::format(body).unwrap_or(body.to_string())
    } else {
        body.to_string()
    };

    container(scrollable(text(formatted_body).font(iced::Font::MONOSPACE)).height(300))
        .padding(10)
        .into()
}

fn view_response_headers(
    headers: &std::collections::HashMap<String, String>,
) -> Element<'_, Message> {
    let headers_text = headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    container(scrollable(text(headers_text).font(iced::Font::MONOSPACE)).height(300))
        .padding(10)
        .into()
}
