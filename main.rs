use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let trimmed = args[1].trim_end();
    let string_url = trimmed.to_string();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(string_url)
        .header("testerheader", "test")
        .send()
        .unwrap();
    println!("Headers:\n{:#?}", response.headers());
}
