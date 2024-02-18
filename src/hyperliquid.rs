use std::{collections::HashMap, sync::Arc};

use hyperliquid_rust_sdk::{BaseUrl, InfoClient, Message, Subscription};
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    compare_price::{self, compare_prices},
    share_state::SharedState,
};

pub struct HyperLiquidStruct {
    info_client: InfoClient,
}

impl HyperLiquidStruct {
    pub async fn new() -> Self {
        Self {
            info_client: InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap(),
        }
    }

    fn format_hyperliquid_tickers(tickers: &HashMap<String, String>) -> Vec<String> {
        let symbols: Vec<String> = tickers
            .keys()
            .map(|key| Self::format_ticker_name(key))
            .collect();
        symbols
    }

    fn format_ticker_name(ticker: &String) -> String {
        let formatted_ticker = if ticker.starts_with("k") {
            ticker.replacen("k", "1000", 1)
        } else {
            ticker.to_string()
        };
        format!("{}USDT", formatted_ticker)
    }

    pub async fn get_tickers(&self) -> Vec<String> {
        let tickers = self.info_client.all_mids().await.unwrap();
        let format_tickers = Self::format_hyperliquid_tickers(&tickers);
        format_tickers
    }

    pub async fn hyperliquid_ws(mut self, shared_state: &Arc<SharedState>) {
        let (sender, mut receiver) = unbounded_channel();
        self.info_client
            .subscribe(Subscription::AllMids, sender)
            .await
            .unwrap();

        while let Some(Message::AllMids(all_mids)) = receiver.recv().await {
            for (ticker, price_str) in all_mids.data.mids.iter() {
                let formatted_ticker = Self::format_ticker_name(&ticker);
                let price: f64 = price_str.parse().unwrap_or(0.0);
                {
                    let mut hyperliquid_prices = shared_state.hyperliquid_prices.write().await;
                    hyperliquid_prices.insert(formatted_ticker.clone(), price);
                }
                // let read_hyperliquid_prices = shared_state.hyperliquid_prices.read().await;
                // println!("share state: {:#?}", read_hyperliquid_prices);
                compare_prices(shared_state, &formatted_ticker).await;
            }
            // println!("hyperliquid data: {:#?}", all_mids.data.mids);
            // println!("hyperliquid data...");
        }
    }
}
