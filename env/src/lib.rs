pub fn url() -> String {
  let u = dotenvy::var("CMC_URL")
    .unwrap_or(String::from("https://sandbox-api.coinmarketcap.com/v2/cryptocurrency/ohlcv/historical"));
  u
}

pub fn key() -> String {
  let k = dotenvy::var("CMC_KEY")
    .unwrap_or(String::from("X"));
  k
}

pub fn token() -> String {
  let t = dotenvy::var("TOKEN_SYMBOL")
    .unwrap_or(String::from("BTC"));
  t
}
