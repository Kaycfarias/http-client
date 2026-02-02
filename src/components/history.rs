use super::enums::{HistoryItem, HttpRequest, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::PathBuf;

const MAX_HISTORY_ITEMS: usize = 50;
const HISTORY_FILE_NAME: &str = "history.json";
const APP_NAME: &str = "http-client";

#[derive(Debug, Clone)]
pub struct RequestHistory {
    items: Vec<HistoryItem>,
    file_path: PathBuf,
}

impl RequestHistory {
    pub fn new() -> Self {
        let file_path = Self::get_history_file_path();
        let items = Self::load_from_file(&file_path).unwrap_or_default();
        
        Self { items, file_path }
    }
    
    fn get_history_file_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .map(|dir| dir.join(APP_NAME))
            .unwrap_or_else(|| PathBuf::from("."));
        
        let _ = fs::create_dir_all(&config_dir);
        
        config_dir.join(HISTORY_FILE_NAME)
    }
    
    fn load_from_file(path: &PathBuf) -> Result<Vec<HistoryItem>, String> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read history file: {}", e))?;
        
        let items: Vec<HistoryItem> = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse history file: {}", e))?;
        
        Ok(items)
    }
    
    fn save_to_file(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.items)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;
        
        fs::write(&self.file_path, json)
            .map_err(|e| format!("Failed to write history file: {}", e))?;
        
        Ok(())
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

        if self.items.len() > MAX_HISTORY_ITEMS {
            self.items.truncate(MAX_HISTORY_ITEMS);
        }
        
        if let Err(e) = self.save_to_file() {
            eprintln!("Warning: Failed to save history: {}", e);
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
        if let Err(e) = self.save_to_file() {
            eprintln!("Warning: Failed to save history after clear: {}", e);
        }
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
