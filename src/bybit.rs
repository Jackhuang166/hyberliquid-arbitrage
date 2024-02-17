use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::utils::{BybitApiResponse, BybitWsResponse};

pub struct Bybit {
    instrument_api_url: String,
    ws_url: String,
}

impl Bybit {
    pub fn new() -> Self {
        Self {
            instrument_api_url: "https://api.bybit.com/v5/market/instruments-info?category=linear"
                .into(),
            ws_url: "wss://stream.bybit.com/v5/public/linear".into(),
        }
    }

    pub async fn get_tickers(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.instrument_api_url).await?;
        let response_data: BybitApiResponse = response.json().await?;

        let tickers: Vec<String> = response_data
            .result
            .list
            .iter()
            .filter_map(|ticker| {
                if !ticker.symbol.contains("-") {
                    Some(ticker.symbol.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(tickers)
    }
    pub async fn bybit_ws(&self) {
        // pub async fn bybit_ws(&self, common_tickers: &Vec<String>) {
        let (mut ws_stream, _) = connect_async(&self.ws_url)
            .await
            .expect("Failed connecting to bybit ws");
        println!("Bybit ws connected");

        // let args: Vec<String> = common_tickers
        //     .iter()
        //     .map(|ticker| format!("tickers.{}", ticker))
        //     .collect();

        let subscribe_message = serde_json::json!({
            "op": "subscribe",
            "args": "tickers.BTCUSDT"
        })
        .to_string();

        ws_stream
            .send(Message::Text(subscribe_message))
            .await
            .expect("failed subscribing to topic");

        while let Some(message) = ws_stream.next().await {
            match message {
                // Ok(Message::Text(text)) => match serde_json::from_str::<BybitWsResponse>(&text) {
                //     Ok(parse_msg) => println!("bybit data: {:#?}", parse_msg),
                //     Err(e) => eprintln!("failed parsing bybit data: {:#?}", e),
                // },
                Ok(data) => println!("not parse: {:#?}", data),
                Err(e) => eprintln!("bybit ws error: {:#?}", e),
            }
        }
    }
}
