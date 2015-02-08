use hyper;
use hyper::header;

pub fn get_info() -> String {
    let mut client = hyper::Client::new();
    let mut request = client.get("https://www.circle.com/api/v2/customers/1234567");
    let mut circle_headers = header::Headers::new();
    let mime: hyper::mime::Mime = "application/json".parse().unwrap();

    circle_headers.set(header::UserAgent("rustlang 1.0.0-nightly 2015-02-04 / hyper 0.1.11 / gabe@durazo.us".to_string()));
    circle_headers.set(header::Accept(vec![header::QualityItem::new(mime, 0.0)]));
    circle_headers.set_raw("x-app-id", vec!["angularjs".to_string().into_bytes()]);
    circle_headers.set_raw("x-app-version", vec!["0.0.1".to_string().into_bytes()]);
    circle_headers.set_raw("x-customer-id", vec!["1234567".to_string().into_bytes()]);
    circle_headers.set_raw("x-customer-session-token", vec!["XYZ".to_string().into_bytes()]);
    request = request.headers(circle_headers);

    match request.send() {
        Ok(mut response) => { response.read_to_string().unwrap_or("Oops".to_string()) },
        _ => { "Oops".to_string() }
    }
}
