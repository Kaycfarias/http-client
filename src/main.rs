use iced::{Element, Task};
use iced::widget::{button, container, row, text_editor};

mod components;
use components::{
    pick_list::{pick_list_view},
    
    enums::{Message, HTTPMethod},
};

struct App {
    selected_method: Option<HTTPMethod>,
    url: text_editor::Content,
    is_dirty: bool,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HTTPSelected(method) => {
                self.selected_method = Some(method);
                Task::none()
            }
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.url.perform(action);

                Task::none()
            }            
        }
        
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            row![
                pick_list_view(self.selected_method),
                text_editor(&self.url)
                .placeholder("Type something here...")
                .on_action(Message::Edit),
                button("Submit")
            ]
        ).into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_method: None,
            url: text_editor::Content::new(),
            is_dirty: false,
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}