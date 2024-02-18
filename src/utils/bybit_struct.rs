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
    start: u64,
    end: u64,
    interval: String,
    open: String,
    pub close: String,
    high: String,
    low: String,
    volume: String,
    turnover: String,
    confirm: bool,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BybitWsResponse {
    pub topic: String,
    pub data: Vec<BybitWsData>,
    ts: u64,
    #[serde(rename = "type")]
    type_field: String,
}
