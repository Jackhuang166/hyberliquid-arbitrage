use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    min_leverage: String,
    max_leverage: String,
    leverage_step: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    min_price: String,
    max_price: String,
    tick_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    max_order_qty: String,
    min_order_qty: String,
    qty_step: String,
    post_only_max_order_qty: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitList {
    pub symbol: String,
    #[serde(rename = "contractType")]
    contract_type: String,
    status: String,
    #[serde(rename = "baseCoin")]
    base_coin: String,
    #[serde(rename = "quoteCoin")]
    quote_coin: String,
    #[serde(rename = "launchTime")]
    launch_time: String,
    #[serde(rename = "deliveryTime")]
    delivery_time: String,
    #[serde(rename = "deliveryFeeRate")]
    delivery_fee_rate: String,
    #[serde(rename = "priceScale")]
    price_scale: String,
    #[serde(rename = "leverageFilter")]
    leverage_filter: LeverageFilter,
    #[serde(rename = "priceFilter")]
    price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    lot_size_filter: LotSizeFilter,
    #[serde(rename = "unifiedMarginTrade")]
    unified_margin_trade: bool,
    #[serde(rename = "fundingInterval")]
    funding_interval: i64,
    #[serde(rename = "settleCoin")]
    settle_coin: String,
    #[serde(rename = "copyTrading")]
    copy_trading: String,
    #[serde(rename = "upperFundingRate")]
    upper_funding_rate: String,
    #[serde(rename = "lowerFundingRate")]
    lower_funding_rate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitApiResult {
    category: String,
    pub list: Vec<BybitList>,
    #[serde(rename = "nextPageCursor")]
    next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitApiResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,
    #[serde(rename = "retMsg")]
    ret_msg: String,
    pub result: BybitApiResult,
    #[serde(rename = "retExtInfo")]
    ret_ext_info: HashMap<String, serde_json::Value>,
    time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitWsData {
    pub symbol: String,
    #[serde(rename = "tickDirection")]
    tick_direction: String,
    #[serde(rename = "price24hPcnt")]
    price24h_pcnt: String,
    #[serde(rename = "lastPrice")]
    last_price: String,
    #[serde(rename = "prevPrice24h")]
    prev_price24h: String,
    #[serde(rename = "highPrice24h")]
    high_price24h: String,
    #[serde(rename = "lowPrice24h")]
    low_price24h: String,
    #[serde(rename = "prevPrice1h")]
    prev_price1h: String,
    #[serde(rename = "markPrice")]
    mark_price: String,
    #[serde(rename = "indexPrice")]
    index_price: String,
    #[serde(rename = "openInterest")]
    open_interest: String,
    #[serde(rename = "openInterestValue")]
    open_interest_value: String,
    turnover24h: String,
    volume24h: String,
    #[serde(rename = "nextFundingTime")]
    next_funding_time: String,
    #[serde(rename = "fundingRate")]
    funding_rate: String,
    #[serde(rename = "bid1Price")]
    bid1_price: String,
    #[serde(rename = "bid1Size")]
    bid1_size: String,
    #[serde(rename = "ask1Price")]
    ask1_price: String,
    #[serde(rename = "ask1Size")]
    ask1_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitWsResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    data: BybitWsData,
    cs: u64,
    ts: u64,
}
