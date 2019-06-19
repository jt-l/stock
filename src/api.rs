extern crate requests;

use requests::ToJson;

pub fn get_stock(symbol: String, alpha_vantage_key: &String) {

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

    // print response
    let data = response.json().unwrap();
    println!("{}", data);
}
