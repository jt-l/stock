extern crate requests;

use requests::ToJson;

#[derive(Debug)]
pub struct Response {
    pub symbol: String,
    pub change_percent: String,
    pub price: String,
}

pub fn get_stock(symbol: String, alpha_vantage_key: &String) -> Response {
    
    // build string
    let mut url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE".to_string();
    let mut symb = "&symbol=".to_string();
    let mut key = "&apikey=".to_string();

    symb.push_str(&symbol);
    key.push_str(alpha_vantage_key);

    url.push_str(&symb);
    url.push_str(&key);

    // make request
    let response = requests::get(url).unwrap();
    assert_eq!(response.reason(), "OK");
    assert_eq!(response.status_code(), requests::StatusCode::Ok);

    // parse response
    let data = response.json().unwrap();

    let r = Response {
        symbol: data["Global Quote"]["01. symbol"].to_string(), 
        change_percent: data["Global Quote"]["10. change percent"].to_string(),
        price: data["Global Quote"]["05. price"].to_string(),
    }; 

    r
}
