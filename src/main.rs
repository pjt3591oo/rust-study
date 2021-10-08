mod upbit;
use upbit::UpbitCrawler;

// #[derive(Serialize, Deserialize, Debug)]
// struct StockByUpbit {
//   pub candleAccTradeVolume:  f64,
//   pub candleAccTradePrice: f64,
//   pub openingPrice:  f64,
//   pub tradePrice:  f64,
//   pub highPrice:  f64,
//   pub lowPrice:  f64,
//   pub timestamp: i128,
//   pub unit:  i128,
// }

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
