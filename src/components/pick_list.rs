use iced::widget::{column, pick_list};
use iced::{Element};

use crate::components::enums::{HTTPMethod, Message};

impl HTTPMethod {
    const ALL: [HTTPMethod; 4] = [
        HTTPMethod::GET,
        HTTPMethod::POST,
        HTTPMethod::PUT,
        HTTPMethod::DELETE,
    ];
}

pub fn pick_list_view(selected_method: Option<HTTPMethod>) -> Element<'static, Message> {
    column![
        pick_list(
        &HTTPMethod::ALL[..],
        selected_method,
        Message::HTTPSelected,
    )
    .placeholder("Method")
    ]
    .into()
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HTTPMethod::GET => "GET",
                HTTPMethod::POST => "POST",
                HTTPMethod::PUT => "PUT",
                HTTPMethod::PATCH => "PATCH",
                HTTPMethod::DELETE => "DELETE",
            }
        )
    }
}