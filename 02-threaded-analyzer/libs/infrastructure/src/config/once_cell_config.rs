use once_cell::sync::OnceCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppConfig {
    pub thread_counts: HashMap<String, usize>,
    pub chunk_size: usize,
}

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn init_config(cfg: AppConfig) {
    CONFIG.set(cfg).expect("Config already initialized");
}

pub fn get_config() -> &'static AppConfig {
    CONFIG.get().expect("Config not initialized")
}

impl AppConfig {
    pub fn new() -> Self {
        let threads = num_cpus::get();
        let chunk = threads;
        Self {
            thread_counts: HashMap::new(),
            chunk_size: chunk.max(1),
        }
    }
}
