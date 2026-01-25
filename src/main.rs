use iced::Length::Fill;
use iced::Theme;
use iced::{Element, Task};
use iced::widget::{button, column, container, row, text, text_input, scrollable, checkbox};

mod components;
use components::{
    enums::{
        BodyType, HTTPMethod, HttpRequest, HttpResponse, KeyValue, Message, RequestTab,
        ResponseTab,
    },
    history::RequestHistory,
    http_client::HttpClient,
    pick_list::pick_list_view,
    utils::{json_formatter, text_formatter, url_validator},
};

struct App {
    // Request state
    method: HTTPMethod,
    url: String,
    headers: Vec<KeyValue>,
    query_params: Vec<KeyValue>,
    body: String,
    body_type: BodyType,
    timeout_ms: String,

    // UI state
    active_tab: RequestTab,
    response_tab: ResponseTab,
    is_loading: bool,
    error_message: Option<String>,

    // Response state
    response: Option<HttpResponse>,

    // History
    history: RequestHistory,

    // HTTP client
    http_client: HttpClient,
}

impl Default for App {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            url: String::new(),
            headers: vec![KeyValue::new(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )],
            query_params: Vec::new(),
            body: String::new(),
            body_type: BodyType::Json,
            timeout_ms: "30000".to_string(),
            active_tab: RequestTab::QueryParams,
            response_tab: ResponseTab::Body,
            is_loading: false,
            error_message: None,
            response: None,
            history: RequestHistory::new(),
            http_client: HttpClient::new(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HTTPSelected(method) => {
                self.method = method;
                if method == HTTPMethod::GET {
                    self.body_type = BodyType::None;
                    self.body.clear();
                }
                Task::none()
            }
            Message::UrlChanged(url) => {
                self.url = url;
                self.error_message = None;
                Task::none()
            }
            Message::HeaderKeyChanged(index, key) => {
                if let Some(header) = self.headers.get_mut(index) {
                    header.key = key;
                }
                Task::none()
            }
            Message::HeaderValueChanged(index, value) => {
                if let Some(header) = self.headers.get_mut(index) {
                    header.value = value;
                }
                Task::none()
            }
            Message::HeaderEnabledToggled(index) => {
                if let Some(header) = self.headers.get_mut(index) {
                    header.enabled = !header.enabled;
                }
                Task::none()
            }
            Message::AddHeader => {
                self.headers.push(KeyValue::empty());
                Task::none()
            }
            Message::RemoveHeader(index) => {
                self.headers.remove(index);
                Task::none()
            }
            Message::QueryParamKeyChanged(index, key) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.key = key;
                }
                Task::none()
            }
            Message::QueryParamValueChanged(index, value) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.value = value;
                }
                Task::none()
            }
            Message::QueryParamEnabledToggled(index) => {
                if let Some(param) = self.query_params.get_mut(index) {
                    param.enabled = !param.enabled;
                }
                Task::none()
            }
            Message::AddQueryParam => {
                self.query_params.push(KeyValue::empty());
                Task::none()
            }
            Message::RemoveQueryParam(index) => {
                self.query_params.remove(index);
                Task::none()
            }
            Message::BodyChanged(body) => {
                self.body = body;
                Task::none()
            }
            Message::BodyTypeChanged(body_type) => {
                self.body_type = body_type;
                Task::none()
            }
            Message::TimeoutChanged(timeout) => {
                self.timeout_ms = timeout;
                Task::none()
            }
            Message::Submit => {
                if let Err(e) = url_validator::validate_and_normalize(&self.url) {
                    self.error_message = Some(e);
                    return Task::none();
                }

                self.is_loading = true;
                self.error_message = None;

                let request = HttpRequest {
                    method: self.method,
                    url: self.url.clone(),
                    headers: self.headers.clone(),
                    query_params: self.query_params.clone(),
                    body: self.body.clone(),
                    body_type: self.body_type,
                    timeout_ms: self.timeout_ms.parse().unwrap_or(30000),
                };

                let client = self.http_client.clone();

                Task::perform(
                    async move { client.send_request(request)},
                    Message::RequestCompleted,
                )
            }
            Message::RequestCompleted(result) => {
                self.is_loading = false;

                match result {
                    Ok(response) => {
                        let request = HttpRequest {
                            method: self.method,
                            url: self.url.clone(),
                            headers: self.headers.clone(),
                            query_params: self.query_params.clone(),
                            body: self.body.clone(),
                            body_type: self.body_type,
                            timeout_ms: self.timeout_ms.parse().unwrap_or(30000),
                        };
                        self.history.add_item(request, response.clone());
                        self.response = Some(response);
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                    }
                }
                Task::none()
            }
            Message::CancelRequest => {
                self.is_loading = false;
                Task::none()
            }
            Message::LoadFromHistory(index) => {
                if let Some(item) = self.history.get_item(index) {
                    self.method = item.request.method;
                    self.url = item.request.url.clone();
                    self.headers = item.request.headers.clone();
                    self.query_params = item.request.query_params.clone();
                    self.body = item.request.body.clone();
                    self.body_type = item.request.body_type;
                    self.timeout_ms = item.request.timeout_ms.to_string();
                    self.response = Some(item.response.clone());
                }
                Task::none()
            }
            Message::ClearHistory => {
                self.history.clear();
                Task::none()
            }
            Message::TabChanged(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::ResponseTabChanged(tab) => {
                self.response_tab = tab;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let main_content = column![
            text("HTTP Client").size(32),
            
            row![
                pick_list_view(Some(self.method)),
                text_input("https://api.example.com/endpoint", &self.url)
                    .on_input(Message::UrlChanged)
                    .width(Fill),
                if self.is_loading {
                    button(text("Sending...")).padding(10)
                } else {
                    button(text("Send"))
                        .on_press(Message::Submit)
                        .padding(10)
                }
            ]
            .spacing(10),
            
            if let Some(error) = &self.error_message {
                container(text(error).style(|theme| text::danger(theme)))
                    .padding(10)
            } else {
                container(text(""))
            },
            
            row![
                text("Timeout (ms):"),
                text_input("30000", &self.timeout_ms)
                    .on_input(Message::TimeoutChanged)
                    .width(100),
            ]
            .spacing(10),
            
            self.view_request_tabs(),
            self.view_active_tab_content(),
            
            if self.response.is_some() {
                self.view_response()
            } else {
                container(text("No response yet"))
                    .padding(20)
                    .into()
            },
            
            self.view_history(),
        ]
        .spacing(15)
        .padding(20);

        scrollable(main_content).into()
    }

    fn view_request_tabs(&self) -> Element<'_, Message> {
        row![
            button(text("Query Params"))
                .on_press(Message::TabChanged(RequestTab::QueryParams))
                .style(if self.active_tab == RequestTab::QueryParams {
                    button::primary
                } else {
                    button::secondary
                }),
            button(text("Headers"))
                .on_press(Message::TabChanged(RequestTab::Headers))
                .style(if self.active_tab == RequestTab::Headers {
                    button::primary
                } else {
                    button::secondary
                }),
            button(text("Body"))
                .on_press(Message::TabChanged(RequestTab::Body))
                .style(if self.active_tab == RequestTab::Body {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(5)
        .into()
    }

    fn view_active_tab_content(&self) -> Element<'_, Message> {
        match self.active_tab {
            RequestTab::QueryParams => self.view_query_params(),
            RequestTab::Headers => self.view_headers(),
            RequestTab::Body => self.view_body(),
        }
    }

    fn view_query_params(&self) -> Element<'_, Message> {
        let mut params_column = column![].spacing(5);

        for (index, param) in self.query_params.iter().enumerate() {
            params_column = params_column.push(
                row![
                    checkbox(param.enabled)
                        .on_toggle(move |_| Message::QueryParamEnabledToggled(index)),
                    text_input("key", &param.key)
                        .on_input(move |v| Message::QueryParamKeyChanged(index, v))
                        .width(200),
                    text_input("value", &param.value)
                        .on_input(move |v| Message::QueryParamValueChanged(index, v))
                        .width(Fill),
                    button(text("✕"))
                        .on_press(Message::RemoveQueryParam(index))
                        .style(button::danger),
                ]
                .spacing(5),
            );
        }

        params_column = params_column.push(
            button(text("+ Add Query Param"))
                .on_press(Message::AddQueryParam)
                .style(button::success),
        );

        container(params_column)
            .padding(10)
            .into()
    }

    fn view_headers(&self) -> Element<'_, Message> {
        let mut headers_column = column![].spacing(5);

        for (index, header) in self.headers.iter().enumerate() {
            headers_column = headers_column.push(
                row![
                    checkbox(header.enabled)
                        .on_toggle(move |_| Message::HeaderEnabledToggled(index)),
                    text_input("Header-Name", &header.key)
                        .on_input(move |v| Message::HeaderKeyChanged(index, v))
                        .width(200),
                    text_input("value", &header.value)
                        .on_input(move |v| Message::HeaderValueChanged(index, v))
                        .width(Fill),
                    button(text("✕"))
                        .on_press(Message::RemoveHeader(index))
                        .style(button::danger),
                ]
                .spacing(5),
            );
        }

        headers_column = headers_column.push(
            button(text("+ Add Header"))
                .on_press(Message::AddHeader)
                .style(button::success),
        );

        container(headers_column)
            .padding(10)
            .into()
    }

    fn view_body(&self) -> Element<'_, Message> {
        if self.method == HTTPMethod::GET {
            return container(text("Body is not available for GET requests"))
                .padding(20)
                .into();
        }

        column![
            row![
                text("Body Type:"),
                button(text("None"))
                    .on_press(Message::BodyTypeChanged(BodyType::None))
                    .style(if self.body_type == BodyType::None {
                        button::primary
                    } else {
                        button::secondary
                    }),
                button(text("Raw"))
                    .on_press(Message::BodyTypeChanged(BodyType::Raw))
                    .style(if self.body_type == BodyType::Raw {
                        button::primary
                    } else {
                        button::secondary
                    }),
                button(text("JSON"))
                    .on_press(Message::BodyTypeChanged(BodyType::Json))
                    .style(if self.body_type == BodyType::Json {
                        button::primary
                    } else {
                        button::secondary
                    }),
            ]
            .spacing(5),
            text_input(
                if self.body_type == BodyType::Json {
                    r#"{"key": "value"}"#
                } else {
                    "Body content..."
                },
                &self.body
            )
            .on_input(Message::BodyChanged)
            .line_height(text::LineHeight::Relative(1.5))
            .width(Fill),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn view_response(&self) -> Element<'_, Message> {
        if let Some(response) = &self.response {
            let status_color = if response.status >= 200 && response.status < 300 {
                iced::Color::from_rgb(0.0, 0.8, 0.0)
            } else if response.status >= 400 {
                iced::Color::from_rgb(0.8, 0.0, 0.0)
            } else {
                iced::Color::from_rgb(0.5, 0.5, 0.5)
            };

            column![
                text("Response").size(24),
                row![
                    text(format!("Status: {} {}", response.status, response.status_text))
                        .color(status_color),
                    text(format!(
                        "Time: {}",
                        text_formatter::format_duration(response.duration_ms)
                    )),
                ]
                .spacing(20),
                row![
                    button(text("Body"))
                        .on_press(Message::ResponseTabChanged(ResponseTab::Body))
                        .style(if self.response_tab == ResponseTab::Body {
                            button::primary
                        } else {
                            button::secondary
                        }),
                    button(text("Headers"))
                        .on_press(Message::ResponseTabChanged(ResponseTab::Headers))
                        .style(if self.response_tab == ResponseTab::Headers {
                            button::primary
                        } else {
                            button::secondary
                        }),
                ]
                .spacing(5),
                match self.response_tab {
                    ResponseTab::Body => {
                        let formatted_body = if json_formatter::is_valid_json(&response.body) {
                            json_formatter::format(&response.body).unwrap_or(response.body.clone())
                        } else {
                            response.body.clone()
                        };
                        container(
                            scrollable(text(formatted_body).font(iced::Font::MONOSPACE))
                                .height(300)
                        )
                        .padding(10)
                    }
                    ResponseTab::Headers => {
                        let headers_text = response
                            .headers
                            .iter()
                            .map(|(k, v)| format!("{}: {}", k, v))
                            .collect::<Vec<_>>()
                            .join("\n");
                        container(
                            scrollable(text(headers_text).font(iced::Font::MONOSPACE)).height(300)
                        )
                        .padding(10)
                    }
                },
            ]
            .spacing(10)
            .into()
        } else {
            text("No response").into()
        }
    }

    fn view_history(&self) -> Element<'_, Message> {
        let mut history_column = column![
            row![
                text("History").size(24),
                button(text("Clear"))
                    .on_press(Message::ClearHistory)
                    .style(button::danger),
            ]
            .spacing(10),
        ]
        .spacing(10);

        if self.history.is_empty() {
            history_column = history_column.push(text("No requests in history"));
        } else {
            for (index, item) in self.history.get_items().iter().enumerate() {
                let formatted_time =
                    RequestHistory::format_timestamp(item.timestamp);
                history_column = history_column.push(
                    button(
                        column![
                            row![
                                text(format!("{}", item.request.method)).size(14),
                                text(&item.request.url).size(14),
                            ]
                            .spacing(10),
                            text(format!(
                                "{} - {}ms",
                                formatted_time, item.response.duration_ms
                            ))
                            .size(12),
                        ]
                        .spacing(5),
                    )
                    .on_press(Message::LoadFromHistory(index))
                    .width(Fill),
                );
            }
        }

        container(history_column)
            .padding(10)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(|_state: &App| Theme::Ferra)
        .run()
}
