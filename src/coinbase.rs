use crypto;
use crypto::mac::Mac;
use hyper;
use hyper::header;
use rustc_serialize::base64;
use rustc_serialize::base64::{FromBase64, ToBase64};
use time;
use websocket::{Message, Sender, Receiver};
use websocket::client::request::Url;
use websocket;

pub fn get_book() -> String {
    let mut client = hyper::Client::new();
    let mut request = client.get("https://api.exchange.coinbase.com/products/BTC-USD/book");
    request = request.header(header::UserAgent("rustlang 1.0.0-nightly 2015-02-04 / hyper 0.1.11 / gabe@durazo.us".to_string()));
    match request.send() {
        Ok(mut response) => { response.read_to_string().unwrap_or("Oops".to_string()) },
        _ => { "Oops".to_string() }
    }
}

pub fn get_time() -> String {
    let mut client = hyper::Client::new();
    let mut request = client.get("https://api.exchange.coinbase.com/time");
    request = request.header(header::UserAgent("rustlang 1.0.0-nightly 2015-02-04 / hyper 0.1.11 / gabe@durazo.us".to_string()));
    match request.send() {
        Ok(mut response) => { response.read_to_string().unwrap_or("Oops".to_string()) },
        _ => { "Oops".to_string() }
    }
}

pub fn get_accounts() -> String {
    let mut client = hyper::Client::new();
    let mut request = client.get("https://api.exchange.coinbase.com/accounts");
    let mut coinbase_headers = header::Headers::new();

    let now = time::get_time().sec;
    let sig = now.to_string() + "GET" + "/accounts";
    let api_secret = "XYZ";
    let key: Vec<u8> = api_secret.from_base64().unwrap();
    let mut hmac = crypto::hmac::Hmac::new(crypto::sha2::Sha256::new(), &key);
    hmac.input(&sig.into_bytes());
    let base64_config = base64::Config {
        char_set: base64::CharacterSet::Standard,
        newline: base64::Newline::LF,
        pad: true,
        line_length: None
    };
    let signature = hmac.result().code().to_base64(base64_config);
    println!("{:?}", signature);
    coinbase_headers.set_raw("CB-ACCESS-KEY", vec!["123abc".to_string().into_bytes()]);
    coinbase_headers.set_raw("CB-ACCESS-SIGN", vec![signature.into_bytes()]);
    coinbase_headers.set_raw("CB-ACCESS-TIMESTAMP", vec![now.to_string().into_bytes()]);
    coinbase_headers.set_raw("CB-ACCESS-PASSPHRASE", vec!["abc".to_string().into_bytes()]);
    coinbase_headers.set(header::UserAgent("rustlang 1.0.0-nightly 2015-02-04 / hyper 0.1.11 / gabe@durazo.us".to_string()));
    request = request.headers(coinbase_headers);

    match request.send() {
        Ok(mut response) => { response.read_to_string().unwrap_or("Oops".to_string()) },
        _ => { "Oops".to_string() }
    }
}

pub fn get_and_listen_websocket() {
    let url = Url::parse("wss://ws-feed.exchange.coinbase.com").unwrap();
    println!("Connecting to {}", url);

    let request = websocket::Client::connect(url).unwrap();

    let response = request.send().unwrap(); // Send the request and retrieve a response
    response.validate().unwrap(); // Validate the response

    println!("Successfully connected");

    let (mut sender, mut receiver) = response.begin().split();
    let a = sender.send_message(Message::Text("{\"type\": \"subscribe\", \"product_id\": \"BTC-USD\"}".to_string()));
    println!("{:?}", a);

    for message in receiver.incoming_messages() {
        let message = match message {
            Ok(m) => m,
            Err(e) => {
                println!("Receive Loop: {:?}", e);
                return;
            }
        };
        match message {
            Message::Close(_) => {
                // Got a close message, so send a close message and return
                return;
            }

            // Say what we received
            _ => println!("Receive Loop: {:?}", message),
        }
    }
}
