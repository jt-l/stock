extern crate requests;

use requests::ToJson;

#[derive(Debug)]
pub struct Response {
    pub symbol: String,
    pub change_percent: String,
    pub price: String,
}

pub fn get_stock(symbol: String, api_key: &String) -> Response {
    
    // build string
    let mut url = "https://api.worldtradingdata.com/api/v1/stock?".to_string();
    let mut symb = "symbol=".to_string();
    let mut key = "&api_token=".to_string();

    symb.push_str(&symbol);
    key.push_str(api_key);

    url.push_str(&symb);
    url.push_str(&key);

    // make request
    let response = requests::get(url).unwrap();
    assert_eq!(response.reason(), "OK");
    assert_eq!(response.status_code(), requests::StatusCode::Ok);

    // parse response
    let data = response.json().unwrap();

    let r = Response {
        symbol: data["data"][0]["symbol"].to_string(), 
        change_percent: data["data"][0]["change_pct"].to_string(),
        price: data["data"][0]["price"].to_string(),
    }; 

    r
}
