use std::sync::Arc;

use crate::share_state::SharedState;

pub async fn compare_prices(shared_state: &Arc<SharedState>, symbol: &str) {
    let bybit_price = {
        let bybit_prices = shared_state.bybit_prices.read().await;
        *bybit_prices.get(symbol).unwrap_or(&0.0)
    };

    let hyperliquid_price = {
        let hyperliquid_prices = shared_state.hyperliquid_prices.read().await;
        *hyperliquid_prices.get(symbol).unwrap_or(&0.0)
    };

    if bybit_price == 0.0 || hyperliquid_price == 0.0 {
        return;
    }

    let difference = ((bybit_price - hyperliquid_price) / bybit_price).abs() * 100.0;
    println!(
        "comparing: {:#?}, bybit price: {:#?}, hyperliquid price: {:#?}, difference: {:.5}%",
        symbol, bybit_price, hyperliquid_price, difference
    );

    if difference >= 5.0 {
        println!(
            "5% difference for {}: bybit price: {}, hyperliquid price: {:.5}%",
            symbol, bybit_price, hyperliquid_price
        );
    }
}
