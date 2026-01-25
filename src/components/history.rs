use super::enums::{HistoryItem, HttpRequest, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_HISTORY_ITEMS: usize = 50;

#[derive(Debug, Clone)]
pub struct RequestHistory {
    items: Vec<HistoryItem>,
}

impl RequestHistory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, request: HttpRequest, response: HttpResponse) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let item = HistoryItem {
            request,
            response,
            timestamp,
        };

        self.items.insert(0, item);

        // Manter apenas os últimos MAX_HISTORY_ITEMS
        if self.items.len() > MAX_HISTORY_ITEMS {
            self.items.truncate(MAX_HISTORY_ITEMS);
        }
    }

    pub fn get_items(&self) -> &[HistoryItem] {
        &self.items
    }

    pub fn get_item(&self, index: usize) -> Option<&HistoryItem> {
        self.items.get(index)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn format_timestamp(timestamp: i64) -> String {
        use chrono::{Local, TimeZone};
        
        let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
        datetime.format("%d/%m/%Y %H:%M:%S").to_string()
    }
}

impl Default for RequestHistory {
    fn default() -> Self {
        Self::new()
    }
}
