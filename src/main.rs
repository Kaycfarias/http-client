use iced::Length::Fill;
use iced::{Element, Task};
use iced::widget::{button, container, row, text_input};

mod components;
use components::{
    pick_list::{pick_list_view},    
    enums::{Message, HTTPMethod},
    http_client::make_request,
};

struct App {
    selected_method: HTTPMethod,
    url: String,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HTTPSelected(method) => {
                self.selected_method = method;
                Task::none()
            }
            Message::Edit(content) => {
                self.url = content;
                Task::none()
            } 
            Message::Submit => {
                let method = self.selected_method.as_reqwest();
                let url = self.url.clone();
                Task::perform(
                    async move {
                        make_request(method, &url).map_err(|e| e.to_string())
                    },
                    Message::RequestCompleted)
            }
            Message::RequestCompleted(result) => {
                match result {
                    Ok(response) => println!("Request completed: {}", response),
                    Err(e) => println!("Request failed: {}", e),
                }
                Task::none()       
            }
        }
        
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            row![
                pick_list_view(Some(self.selected_method)),
                text_input(
                    "Type URL here...", &self.url
                ).on_input(Message::Edit),
                button("Submit").on_press(Message::Submit)
            ]
        )
        .center(Fill)   
        .into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_method: HTTPMethod::default(),
            url: String::new(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}