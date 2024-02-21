use crate::share_state::SharedState;
use bybit::Bybit;
use hyperliquid::HyperLiquidStruct;
use std::{sync::Arc, time::Duration};
use tokio::time::interval;

mod bybit;
mod compare_price;
mod create_tweet;
mod hyperliquid;
mod share_state;
mod utils;

fn get_common_tickers(bybit_tickers: Vec<String>, hyperliquid_tickers: Vec<String>) -> Vec<String> {
    let common_tickers: Vec<String> = bybit_tickers
        .iter()
        .filter(|ticker| hyperliquid_tickers.contains(&ticker))
        .cloned()
        .collect();
    common_tickers
}

#[tokio::main]
async fn main() {
    let hyper_liquid = HyperLiquidStruct::new().await;
    let bybit = Bybit::new();
    let shared_state = Arc::new(SharedState::new());

    let hyperliquid_tickers = hyper_liquid.get_tickers().await;

    let bybit_tickers = bybit
        .get_tickers()
        .await
        .expect("Error calling bybit get tickers");

    let common_tickers = get_common_tickers(bybit_tickers, hyperliquid_tickers);

    {
        let mut bybit_prices = shared_state.bybit_prices.write().await;
        let mut hyperliquid_price = shared_state.hyperliquid_prices.write().await;
        for ticker in &common_tickers {
            bybit_prices.insert(ticker.clone(), 0.0);
            hyperliquid_price.insert(ticker.clone(), 0.0);
        }
    }

    let shared_state_clone_reset = Arc::clone(&shared_state);
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            {
                let mut tweet_symbols = shared_state_clone_reset.tweeted_symbols.write().await;
                tweet_symbols.clear();
                println!("**************************************** Tweet symbols reset ****************************************");
            }
        }
    });

    tokio::join!(
        hyper_liquid.hyperliquid_ws(&shared_state),
        bybit.bybit_ws(&common_tickers, &shared_state)
    );
}
