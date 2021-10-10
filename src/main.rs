mod upbit;

use crate::upbit as other_upbit;
use other_upbit::{ Crawler, UpbitCrawler };

fn main() {
  let mut symbol: String = std::env::args().nth(1).expect("symbol must exist");
  let count: String = std::env::args().nth(2).expect("count must exist");
  let count: i32 = count.parse::<i32>().expect("count must be integer");
  
  symbol = symbol.to_uppercase();

  println!("[REQUEST] count:{}, symbol: {}", count, &symbol);
  let raw_text: String = UpbitCrawler::new(count, &symbol).unwrap();
  let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
  stocks.show();
}
