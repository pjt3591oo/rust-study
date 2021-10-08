mod upbit;
use upbit::{StockByUpbit, get_raw_by_upbit};

fn main() {
  let raw_text: String = get_raw_by_upbit().unwrap();
  let socket_upbit: Vec<StockByUpbit> = serde_json::from_str(&raw_text).unwrap();

  println!("{:?}", socket_upbit);
  for i in socket_upbit {
    println!("tradePrice: {:?}", i.tradePrice);
  }

  upbit::upbit::bar();
}
