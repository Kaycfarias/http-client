use iced::Length::Fill;
use iced::Theme;
use iced::{Element, Task, Length};
use iced::widget::{column, container, row, scrollable, text_editor};

mod components;
use components::{
    enums::{
        BodyType, HTTPMethod, HttpRequest, HttpResponse, KeyValue, Message, RequestTab,
        ResponseTab, DEFAULT_TIMEOUT_MS,
    },
    history::RequestHistory,
    http_client::HttpClient,
    utils::url_validator,
    ui,
    styles,
};

struct App {
    // Request state
    method: HTTPMethod,
    url: String,
    headers: Vec<KeyValue>,
    query_params: Vec<KeyValue>,
    body: String,
    body_content: text_editor::Content,
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
            body_content: text_editor::Content::new(),
            body_type: BodyType::Json,
            timeout_ms: DEFAULT_TIMEOUT_MS.to_string(),
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
                    self.body_content = text_editor::Content::new();
                }
            }
            UrlChanged(url) => {
                self.url = url;
                self.error_message = None;
            }
            HeaderKeyChanged(i, key) => Self::update_list_item(&mut self.headers, i, |h| h.key = key),
            HeaderValueChanged(i, val) => Self::update_list_item(&mut self.headers, i, |h| h.value = val),
            HeaderEnabledToggled(i) => Self::update_list_item(&mut self.headers, i, |h| h.enabled = !h.enabled),
            AddHeader => self.headers.push(KeyValue::empty()),
            RemoveHeader(i) => { self.headers.remove(i); }
            QueryParamKeyChanged(i, key) => Self::update_list_item(&mut self.query_params, i, |p| p.key = key),
            QueryParamValueChanged(i, val) => Self::update_list_item(&mut self.query_params, i, |p| p.value = val),
            QueryParamEnabledToggled(i) => Self::update_list_item(&mut self.query_params, i, |p| p.enabled = !p.enabled),
            AddQueryParam => self.query_params.push(KeyValue::empty()),
            RemoveQueryParam(i) => { self.query_params.remove(i); }
            BodyChanged(body) => self.body = body,
            BodyEditorAction(action) => {
                self.body_content.perform(action);
                self.body = self.body_content.text();
            }
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

    /// Atualiza um item em uma lista se o índice for válido
    fn update_list_item<T>(
        list: &mut Vec<T>,
        index: usize,
        update_fn: impl FnOnce(&mut T),
    ) {
        if let Some(item) = list.get_mut(index) {
            update_fn(item);
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
            timeout_ms: self.timeout_ms.parse().unwrap_or(DEFAULT_TIMEOUT_MS),
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
            Err(error) => {
                self.error_message = Some(error);
            }
        }
    }

    fn load_from_history(&mut self, index: usize) {
        if let Some(item) = self.history.get_item(index) {
            self.method = item.request.method;
            self.url = item.request.url.clone();
            self.headers = item.request.headers.clone();
            self.query_params = item.request.query_params.clone();
            self.body = item.request.body.clone();
            self.body_content = text_editor::Content::with_text(&item.request.body);
            self.body_type = item.request.body_type;
            self.timeout_ms = item.request.timeout_ms.to_string();
            self.response = Some(item.response.clone());
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Sidebar do histórico (esquerda, largura fixa)
        let history_sidebar = container(
            scrollable(
                ui::view_history(&self.history)
            )
        )
        .width(300)
        .height(Length::Fill);

        // Conteúdo principal (direita, ocupa espaço restante)
        let main_content = column![
            // Header com controles principais
            ui::view_header(self.method, &self.url, self.is_loading),
            
            // Mensagem de erro (se houver)
            if let Some(error) = &self.error_message {
                ui::view_error_message(error)
            } else {
                ui::view_empty_error()
            },
            
            // Configuração de timeout
            ui::view_timeout_config(&self.timeout_ms),
            
            // Container de request (tabs + conteúdo)
            container(
                column![
                    ui::view_request_tabs(self.active_tab),
                    self.view_active_tab_content(),
                ]
                .spacing(0)
                .width(Fill)
            )
            .style(styles::request_container),
            
            // Response ou placeholder
            if let Some(response) = &self.response {
                ui::view_response(response, self.response_tab)
            } else {
                ui::view_no_response()
            },
        ]
        .spacing(10)
        .padding([16, 20]);

        // Layout de duas colunas: histórico | conteúdo principal
        let layout = row![
            history_sidebar,
            scrollable(main_content).width(Length::Fill),
        ]
        .spacing(0);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_active_tab_content(&self) -> Element<'_, Message> {
        match self.active_tab {
            RequestTab::QueryParams => self.view_query_params(),
            RequestTab::Headers => self.view_headers(),
            RequestTab::Body => ui::view_body_editor(self.method, self.body_type, &self.body_content),
        }
    }

    fn view_query_params(&self) -> Element<'_, Message> {
        ui::view_key_value_list(
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
        ui::view_key_value_list(
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
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(|_state: &App| Theme::Oxocarbon)
        .run()
}
