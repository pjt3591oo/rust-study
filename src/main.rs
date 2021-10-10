mod upbit;
use upbit::UpbitCrawler;

use crate::upbit::Crawler;

fn main() {
  let btc_symbol: String = String::from("BTC");
  println!("[REQUEST] count:1 {}", btc_symbol);
  let raw_text: String =UpbitCrawler::new(1, &btc_symbol).unwrap();
  let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
  stocks.show();

  println!("");

  let eth_symbol: String = String::from("ETH");
  println!("[REQUEST] count:1 {}", eth_symbol);
  let raw_text: String =UpbitCrawler::new(3, &eth_symbol).unwrap();
  let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
  stocks.show();
}
