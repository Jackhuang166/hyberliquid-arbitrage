use std::collections::HashMap;

use tokio::sync::RwLock;

#[derive(Debug)]
pub struct SharedState {
    pub bybit_prices: RwLock<HashMap<String, f64>>,
    pub hyperliquid_prices: RwLock<HashMap<String, f64>>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            bybit_prices: RwLock::new(HashMap::new()),
            hyperliquid_prices: RwLock::new(HashMap::new()),
        }
    }
}
