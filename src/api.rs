use dinero::{api::Rate, currencies::Currency};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct APIResponse {
    success: bool,
    // timestamp: u32,
    // historical: bool,
    // base: String,
    // date: String,
    rates: HashMap<String, f32>,
}

pub async fn get_rate_from_api(date: &str, from: &str, to: &str, key: &str) -> Rate {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://api.apilayer.com/fixer/{}?symbols={}&base={}",
            date, to, from
        ))
        .header("apikey", key)
        .send()
        .await
        .unwrap();

    let status_code = response.status();

    let api_response = match status_code {
        reqwest::StatusCode::OK => match response.json::<APIResponse>().await {
            Ok(parsed) => parsed,
            Err(e) => panic!("The response didn't match the expected shape. Error={}", e),
        },

        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("UNAUTHORIZED: Need to grab a different API key.");
        }
        _ => panic!("Uh oh! Something unexpected happened"),
    };

    if !api_response.success {
        panic!("API response error {:?}", api_response)
    }

    let rate_hashmap = &api_response.rates;

    let amount_float = rate_hashmap.values().next().unwrap();

    let precision = precision_digits(&amount_float);

    let amount = (amount_float * 10.0_f32.powi(precision)) as i128;
    let rate_dinero = Rate::new(
        Currency::from_country_code_str(to.to_string()).unwrap(),
        amount,
        Some(precision.unsigned_abs()),
    );

    rate_dinero
}

fn precision_digits(x: &f32) -> i32 {
    let input = x.to_owned();
    let mut precision = 0;
    let mut rounded = input;
    while rounded % 1.0 != 0.0 {
        precision += 1;
        rounded = input * 10.0_f32.powi(precision);
    }
    precision
}
