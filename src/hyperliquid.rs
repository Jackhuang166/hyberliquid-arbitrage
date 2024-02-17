use std::collections::HashMap;

use hyperliquid_rust_sdk::{BaseUrl, InfoClient, Message, Subscription};
use tokio::sync::mpsc::unbounded_channel;

pub struct HyperLiquidStruct {
    api_url: String,
    info_client: InfoClient,
}

impl HyperLiquidStruct {
    pub async fn new() -> Self {
        Self {
            api_url: "https://api.hyperliquid.xyz/info".into(),
            info_client: InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap(),
        }
    }

    fn format_hyperliquid_tickers(tickers: &HashMap<String, String>) -> Vec<String> {
        let symbols: Vec<String> = tickers
            .keys()
            .map(|key| {
                let formatted_tickers = if key.starts_with("k") {
                    key.replacen("k", "1000", 1)
                } else {
                    key.to_string()
                };
                format!("{}USDT", formatted_tickers)
            })
            .collect();
        symbols
    }

    pub async fn get_tickers(&self) -> Vec<String> {
        let tickers = self.info_client.all_mids().await.unwrap();
        let format_tickers = Self::format_hyperliquid_tickers(&tickers);
        format_tickers
    }

    pub async fn hyperliquid_ws(mut self) {
        let (sender, mut receiver) = unbounded_channel();
        self.info_client
            .subscribe(Subscription::AllMids, sender)
            .await
            .unwrap();

        while let Some(Message::AllMids(all_mids)) = receiver.recv().await {
            println!("hyperliquid data: {:#?}", all_mids.data.mids);
        }
    }
}
