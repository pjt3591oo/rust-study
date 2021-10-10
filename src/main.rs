mod upbit;
use upbit::UpbitCrawler;

use crate::upbit::Crawler;

fn main() {
  println!("[REQUEST] count:1 ");
  let raw_text: String =UpbitCrawler::new(1).unwrap();
  let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
  stocks.show();

  println!("");
  println!("[REQUEST] count:3 ");
  let raw_text: String =UpbitCrawler::new(3).unwrap();
  let stocks: UpbitCrawler = UpbitCrawler::parse(&raw_text).unwrap();
  stocks.show();
}
