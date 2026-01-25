use iced::Element;
use iced::widget::{button, column, container, row, text, text_editor, Button};
use crate::components::enums::{BodyType, HTTPMethod, Message};
use crate::components::styles;

pub fn view_body_editor<'a>(
    method: HTTPMethod,
    body_type: BodyType,
    body_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    if method == HTTPMethod::GET {
        return view_body_unavailable();
    }

    column![
        view_body_type_selector(body_type),
        view_body_input(body_type, body_content),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn view_body_unavailable<'a>() -> Element<'a, Message> {
    container(
        text("📝 Body is not available for GET requests")
            .size(14)
            .color(iced::Color::from_rgb(0.5, 0.5, 0.5))
    )
    .padding(24)
    .into()
}

fn view_body_type_selector(body_type: BodyType) -> Element<'static, Message> {
    row![
        text("Body Type:").size(14),
        body_type_button("None", BodyType::None, body_type),
        body_type_button("Raw", BodyType::Raw, body_type),
        body_type_button("JSON", BodyType::Json, body_type),
    ]
    .spacing(8)
    .into()
}

fn view_body_input<'a>(body_type: BodyType, body_content: &'a text_editor::Content) -> Element<'a, Message> {
    let placeholder = if body_type == BodyType::Json {
        "Enter JSON: {\"key\": \"value\"}"
    } else {
        "Enter body content..."
    };

    container(
        text_editor(body_content)
            .on_action(Message::BodyEditorAction)
            .placeholder(placeholder)
            .height(200)
    )
    .padding(12)
    .style(styles::body_input_border)
    .into()
}

fn body_type_button(
    label: &'static str,
    body_type: BodyType,
    current_type: BodyType,
) -> Button<'static, Message> {
    let btn = button(text(label).size(13))
        .on_press(Message::BodyTypeChanged(body_type))
        .padding([8, 14]);
    
    if current_type == body_type {
        btn.style(button::primary)
    } else {
        btn.style(button::secondary)
    }
}
