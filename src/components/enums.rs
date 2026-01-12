use iced::widget::text_editor::Action as TextEditAction;

#[derive(Debug, Clone)]
pub enum Message {
    HTTPSelected(HTTPMethod),
    Edit(TextEditAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HTTPMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}