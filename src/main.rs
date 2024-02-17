use bybit::Bybit;
use hyperliquid::HyperLiquidStruct;

mod bybit;
mod hyperliquid;
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
    // let hyper_liquid = HyperLiquidStruct::new().await;
    let bybit = Bybit::new();

    // let hyperliquid_tickers = hyper_liquid.get_tickers().await;

    // let bybit_tickers = bybit
    //     .get_tickers()
    //     .await
    //     .expect("Error calling bybit get tickers");

    // let common_tickers = get_common_tickers(bybit_tickers, hyperliquid_tickers);
    bybit.bybit_ws().await;

    // tokio::join!(
    //     hyper_liquid.hyperliquid_ws(),
    //     bybit.bybit_ws(&common_tickers)
    // );
}
