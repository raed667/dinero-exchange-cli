#[macro_use]
extern crate log;
use chrono::{NaiveDate, Utc};
use clap::Parser;
use dinero::api::convert;
extern crate strum;
use dinero::currencies::Currency;
use dinero::format::to_unit;
use dinero::Dinero;
use serde_json::json;
use std::env;
use std::error::Error;
use std::time::Duration;

use crate::api::get_rate_from_api;

mod api;

#[derive(Parser, Debug, Clone)]
#[command(name = "Dinero ")]
#[command(author = "Raed667 <github@raed.email>")]
#[command(version = "1.0")]
#[command(about = "Apply exchange rates. Simple CLI project to demonstrate the `Dinero` crate capabilities.", long_about = None)]
struct Cli {
    /// Date of exchange rate for historical values
    #[arg(short, long)]
    date: Option<String>,
    /// Base currency (Example: EUR,USD,CAD)
    #[arg(short, long, default_value = "EUR")]
    from: String,
    /// Target currency (Example: EUR,USD,CAD)
    #[arg(short, long, default_value = "TND")]
    to: String,
    /// API key for Fixer API
    #[arg(short, long)]
    key: Option<String>,
    /// Amount to apply the exchange rate to
    #[arg(value_parser = clap::value_parser!(u32).range(1..),default_value_t = 1)]
    value: u32,
    /// Output JSON instead of human readable messages
    #[arg(long = "json")]
    json: bool,
}

fn parse_args(args: Cli) -> (String, Currency, Currency, String, i128, bool) {
    // Date
    let date = match args.date {
        Some(value) => {
            let date_time_res = NaiveDate::parse_from_str(&value, "%Y-%m-%d");
            match date_time_res {
                Ok(value) => value,
                Err(e) => panic!("Error date format. Expected YYYY-MM-DD. Error={}", e),
            };
            value
        }
        None => {
            let dt = Utc::now();
            let formatted = format!("{}", dt.format("%Y-%m-%d"));
            formatted
        }
    };

    // Key
    let key = match args.key {
        Some(value) => value,
        None => {
            let env_key_res = env::var("API_LAYER_KEY");

            match env_key_res {
                Ok(value)=> value,
                Err(_e)=> panic!("API key not provided. Either pass -k param, or set \"API_LAYER_KEY\" environnement variable.")
            }
        }
    };

    let value = args.value as i128;

    let from = Currency::from_country_code_str(args.from).unwrap();

    let to = Currency::from_country_code_str(args.to).unwrap();

    (date, from, to, key, value, args.json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Cli::parse();
    let from_str = args.from.clone();
    let to_str = args.to.clone();
    let (date, from, to, key, value, is_json_output) = parse_args(args);

    trace!(
        "parameters: date={date}, from={:?}, to={:?}, value={value}, key={key}",
        from,
        to
    );

    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));

    let rate = get_rate_from_api(&date, &from_str, &to_str, &key).await;
    pb.finish();

    let base_value = crate::Dinero::new(value, from, Some(0));

    let target_value = convert(&base_value, &rate);
    let from_value = to_unit(base_value, None, None);
    let to_value = to_unit(target_value, None, None);

    trace!("rate: {:?}", rate);
    trace!("base_value: {:?}", base_value);
    trace!("target_value: {:?}", target_value);

    if is_json_output {
        // JSON Message
        println!(
            "{}",
            json!({
              "date": date,
                "from": {
                    "currency":from_str,
                    "amount":from_value
                },
                "to":{
                    "currency":to_str,
                    "amount":to_value
                }
            })
        );
    } else {
        // Human readable message
        println!("[{date}] {from_value} {from_str} = {to_value} {to_str}");
    }

    Ok(())
}
