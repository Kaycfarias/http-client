use iced::Length::Fill;
use iced::Theme;
use iced::{Element, Task};
use iced::widget::{button, column, container, row, text, text_input, scrollable, checkbox, Button};

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
        use Message::*;
        
        match message {
            HTTPSelected(method) => {
                self.method = method;
                if method == HTTPMethod::GET {
                    self.body_type = BodyType::None;
                    self.body.clear();
                }
            }
            UrlChanged(url) => {
                self.url = url;
                self.error_message = None;
            }
            HeaderKeyChanged(i, key) => self.update_header(i, |h| h.key = key),
            HeaderValueChanged(i, val) => self.update_header(i, |h| h.value = val),
            HeaderEnabledToggled(i) => self.update_header(i, |h| h.enabled = !h.enabled),
            AddHeader => self.headers.push(KeyValue::empty()),
            RemoveHeader(i) => { self.headers.remove(i); }
            QueryParamKeyChanged(i, key) => self.update_param(i, |p| p.key = key),
            QueryParamValueChanged(i, val) => self.update_param(i, |p| p.value = val),
            QueryParamEnabledToggled(i) => self.update_param(i, |p| p.enabled = !p.enabled),
            AddQueryParam => self.query_params.push(KeyValue::empty()),
            RemoveQueryParam(i) => { self.query_params.remove(i); }
            BodyChanged(body) => self.body = body,
            BodyTypeChanged(body_type) => self.body_type = body_type,
            TimeoutChanged(timeout) => self.timeout_ms = timeout,
            Submit => return self.submit_request(),
            RequestCompleted(result) => self.handle_response(result),
            CancelRequest => self.is_loading = false,
            LoadFromHistory(i) => self.load_from_history(i),
            ClearHistory => self.history.clear(),
            TabChanged(tab) => self.active_tab = tab,
            ResponseTabChanged(tab) => self.response_tab = tab,
        }
        
        Task::none()
    }

    fn update_header(&mut self, index: usize, update_fn: impl FnOnce(&mut KeyValue)) {
        if let Some(header) = self.headers.get_mut(index) {
            update_fn(header);
        }
    }

    fn update_param(&mut self, index: usize, update_fn: impl FnOnce(&mut KeyValue)) {
        if let Some(param) = self.query_params.get_mut(index) {
            update_fn(param);
        }
    }

    fn submit_request(&mut self) -> Task<Message> {
        if let Err(e) = url_validator::validate_and_normalize(&self.url) {
            self.error_message = Some(e);
            return Task::none();
        }

        self.is_loading = true;
        self.error_message = None;

        let request = self.build_request();
        let client = self.http_client.clone();

        Task::perform(
            async move { client.send_request(request) },
            Message::RequestCompleted,
        )
    }

    fn build_request(&self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url.clone(),
            headers: self.headers.clone(),
            query_params: self.query_params.clone(),
            body: self.body.clone(),
            body_type: self.body_type,
            timeout_ms: self.timeout_ms.parse().unwrap_or(30000),
        }
    }

    fn handle_response(&mut self, result: Result<HttpResponse, String>) {
        self.is_loading = false;
        match result {
            Ok(response) => {
                self.history.add_item(self.build_request(), response.clone());
                self.response = Some(response);
                self.error_message = None;
            }
            Err(e) => self.error_message = Some(e),
        }
    }

    fn load_from_history(&mut self, index: usize) {
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
            self.tab_button("Query Params", RequestTab::QueryParams),
            self.tab_button("Headers", RequestTab::Headers),
            self.tab_button("Body", RequestTab::Body),
        ]
        .spacing(5)
        .into()
    }

    fn tab_button(&self, label: &'static str, tab: RequestTab) -> Button<'_, Message> {
        button(text(label))
            .on_press(Message::TabChanged(tab))
            .style(if self.active_tab == tab {
                button::primary
            } else {
                button::secondary
            })
    }

    fn response_tab_button(&self, label: &'static str, tab: ResponseTab) -> Button<'_, Message> {
        button(text(label))
            .on_press(Message::ResponseTabChanged(tab))
            .style(if self.response_tab == tab {
                button::primary
            } else {
                button::secondary
            })
    }

    fn view_active_tab_content(&self) -> Element<'_, Message> {
        match self.active_tab {
            RequestTab::QueryParams => self.view_query_params(),
            RequestTab::Headers => self.view_headers(),
            RequestTab::Body => self.view_body(),
        }
    }

    fn view_query_params(&self) -> Element<'_, Message> {
        self.view_key_value_list(
            &self.query_params,
            "key",
            "value",
            Message::QueryParamKeyChanged,
            Message::QueryParamValueChanged,
            Message::QueryParamEnabledToggled,
            Message::RemoveQueryParam,
            Message::AddQueryParam,
            "+ Add Query Param",
        )
    }

    fn view_headers(&self) -> Element<'_, Message> {
        self.view_key_value_list(
            &self.headers,
            "Header-Name",
            "value",
            Message::HeaderKeyChanged,
            Message::HeaderValueChanged,
            Message::HeaderEnabledToggled,
            Message::RemoveHeader,
            Message::AddHeader,
            "+ Add Header",
        )
    }

    fn view_key_value_list(
        &self,
        items: &[KeyValue],
        key_placeholder: &'static str,
        value_placeholder: &'static str,
        on_key_changed: fn(usize, String) -> Message,
        on_value_changed: fn(usize, String) -> Message,
        on_toggle: fn(usize) -> Message,
        on_remove: fn(usize) -> Message,
        on_add: Message,
        add_label: &'static str,
    ) -> Element<'_, Message> {
        let mut col = column![].spacing(5);

        for (index, item) in items.iter().enumerate() {
            col = col.push(
                row![
                    checkbox(item.enabled).on_toggle(move |_| on_toggle(index)),
                    text_input(key_placeholder, &item.key)
                        .on_input(move |v| on_key_changed(index, v))
                        .width(200),
                    text_input(value_placeholder, &item.value)
                        .on_input(move |v| on_value_changed(index, v))
                        .width(Fill),
                    button(text("✕"))
                        .on_press(on_remove(index))
                        .style(button::danger),
                ]
                .spacing(5),
            );
        }

        col = col.push(
            button(text(add_label))
                .on_press(on_add)
                .style(button::success),
        );

        container(col).padding(10).into()
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
                self.body_type_button("None", BodyType::None),
                self.body_type_button("Raw", BodyType::Raw),
                self.body_type_button("JSON", BodyType::Json),
            ]
            .spacing(5),
            text_input(
                if self.body_type == BodyType::Json { r#"{"key": "value"}"# } else { "Body content..." },
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

    fn body_type_button(&self, label: &'static str, body_type: BodyType) -> Button<'_, Message> {
        button(text(label))
            .on_press(Message::BodyTypeChanged(body_type))
            .style(if self.body_type == body_type {
                button::primary
            } else {
                button::secondary
            })
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
                    self.response_tab_button("Body", ResponseTab::Body),
                    self.response_tab_button("Headers", ResponseTab::Headers),
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
        .theme(|_state: &App| Theme::CatppuccinFrappe)
        .run()
}
