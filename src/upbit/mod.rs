use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StockByUpbit {
  // pub candleDateTimeKst:  &'b str,
  // pub candleDateTime: &'a str,
  // pub code:  &'c str,
  pub candleAccTradeVolume:  f64,
  pub candleAccTradePrice: f64,
  pub openingPrice:  f64,
  pub tradePrice:  f64,
  pub highPrice:  f64,
  pub lowPrice:  f64,
  pub timestamp: i128,
  pub unit:  i128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpbitCrawler {
  pub stocks: Vec<StockByUpbit>,
}

pub trait Crawler {
  fn new(count: i32) -> Result<String, reqwest::Error>;
  fn parse(raw_text: &str) -> Result<UpbitCrawler, serde_json::Error>;
  fn show(&self);
}

impl Crawler for UpbitCrawler {
  fn new(count: i32) -> Result<String, reqwest::Error> {
    let url = format!("https://crix-api-cdn.upbit.com/v1/crix/candles/minutes/30?code=CRIX.UPBIT.KRW-BTC&count={}&ciqrandom=1633651016830", count);
    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build().unwrap();

    let raw_text: String = client.get(url).send().unwrap().text().unwrap();

    Ok(raw_text)
  }

  fn parse(raw_text: &str) -> Result<UpbitCrawler, serde_json::Error> {
    let socket_upbit: Vec<StockByUpbit> = serde_json::from_str(&raw_text).unwrap();

    Ok(UpbitCrawler{stocks: socket_upbit})
  }

  fn show(&self) {
      for stock in &self.stocks {
      println!("tradePrice: {:?}", stock.tradePrice);
    }
  }
}
