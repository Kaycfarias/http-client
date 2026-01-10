use iced::{Element, Task};

#[derive(Debug, Clone)]
enum Message {}

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        iced::widget::text("App").into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}