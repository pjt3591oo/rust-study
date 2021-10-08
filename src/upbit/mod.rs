use reqwest;
use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use reqwest::Error;
// use reqwest::Response;
// use serde_json;

pub mod upbit {
  pub fn bar() {
    println!("Hello, world!");
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockByUpbit<'a, 'b, 'c> {
  pub candleAccTradePrice: f64,
  pub candleAccTradeVolume:  f64,
  pub candleDateTime: &'a str,
  pub candleDateTimeKst:  &'b str,
  pub code:  &'c str,
  pub highPrice:  f64,
  pub lowPrice:  f64,
  pub openingPrice:  f64,
  pub timestamp: i128,
  pub tradePrice:  f64,
  pub unit:  i128,
}

pub fn get_raw_by_upbit() -> Result<String, reqwest::Error>{
  let url = "https://crix-api-cdn.upbit.com/v1/crix/candles/minutes/30?code=CRIX.UPBIT.KRW-BTC&count=20&ciqrandom=1633651016830";
  let client = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()?;
    
  let resp: reqwest::blocking::Response = client.get(url).send()?;
  let raw_text = resp.text()?;
 
  Ok(raw_text)
}
