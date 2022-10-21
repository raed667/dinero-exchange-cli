use dinero::{
    api::Rate,
    currencies::{
        Currency, AED, AFN, ALL, AMD, ANG, AOA, ARS, AUD, AWG, AZN, BAM, BBD, BDT, BGN, BHD, BIF,
        BMD, BND, BOB, BOV, BRL, BSD, BTN, BWP, BYN, BZD, CAD, CDF, CHE, CHF, CHW, CLF, CLP, CNY,
        COP, COU, CRC, CUC, CUP, CVE, CZK, DJF, DKK, DOP, DZD, EGP, ERN, ETB, EUR, FJD, FKP, GBP,
        GEL, GHS, GIP, GMD, GNF, GTQ, GYD, HKD, HNL, HRK, HTG, HUF, IDR, ILS, INR, IQD, IRR, ISK,
        JMD, JOD, JPY, KES, KGS, KHR, KMF, KPW, KRW, KWD, KYD, KZT, LAK, LBP, LKR, LRD, LSL, LYD,
        MAD, MDL, MGA, MKD, MMK, MNT, MOP, MRU, MUR, MVR, MWK, MXN, MXV, MYR, MZN, NAD, NGN, NIO,
        NOK, NPR, NZD, OMR, PAB, PEN, PGK, PHP, PKR, PLN, PYG, QAR, RON, RSD, RUB, RWF, SAR, SBD,
        SCR, SDG, SEK, SGD, SHP, SLL, SOS, SRD, SSP, STN, SVC, SYP, SZL, THB, TJS, TMT, TND, TOP,
        TRY, TTD, TWD, TZS, UAH, UGX, USD, USN, UYI, UYU, UYW, UZS, VES, VND, VUV, WST, XAF, XCD,
        XOF, XPF, YER, ZAR, ZMW, ZWL,
    },
};
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
        get_currency(&to.to_string()),
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

pub fn get_currency(currency_string: &String) -> Currency {
    match currency_string.as_str() {
        "AED" => AED,
        "BOV" => BOV,
        "CUP" => CUP,
        "GTQ" => GTQ,
        "KHR" => KHR,
        "MOP" => MOP,
        "PHP" => PHP,
        "SSP" => SSP,
        "UYU" => UYU,
        "AFN" => AFN,
        "BRL" => BRL,
        "CVE" => CVE,
        "GYD" => GYD,
        "KMF" => KMF,
        "MRU" => MRU,
        "PKR" => PKR,
        "STN" => STN,
        "UYW" => UYW,
        "ALL" => ALL,
        "BSD" => BSD,
        "CZK" => CZK,
        "HKD" => HKD,
        "KPW" => KPW,
        "MUR" => MUR,
        "PLN" => PLN,
        "SVC" => SVC,
        "UZS" => UZS,
        "AMD" => AMD,
        "BTN" => BTN,
        "DJF" => DJF,
        "HNL" => HNL,
        "KRW" => KRW,
        "MVR" => MVR,
        "PYG" => PYG,
        "SYP" => SYP,
        "VES" => VES,
        "ANG" => ANG,
        "BWP" => BWP,
        "DKK" => DKK,
        "HRK" => HRK,
        "KWD" => KWD,
        "MWK" => MWK,
        "QAR" => QAR,
        "SZL" => SZL,
        "VND" => VND,
        "AOA" => AOA,
        "BYN" => BYN,
        "DOP" => DOP,
        "HTG" => HTG,
        "KYD" => KYD,
        "MXN" => MXN,
        "RON" => RON,
        "THB" => THB,
        "VUV" => VUV,
        "ARS" => ARS,
        "BZD" => BZD,
        "DZD" => DZD,
        "HUF" => HUF,
        "KZT" => KZT,
        "MXV" => MXV,
        "RSD" => RSD,
        "TJS" => TJS,
        "WST" => WST,
        "AUD" => AUD,
        "CAD" => CAD,
        "EGP" => EGP,
        "IDR" => IDR,
        "LAK" => LAK,
        "MYR" => MYR,
        "RUB" => RUB,
        "TMT" => TMT,
        "XAF" => XAF,
        "AWG" => AWG,
        "CDF" => CDF,
        "ERN" => ERN,
        "ILS" => ILS,
        "LBP" => LBP,
        "MZN" => MZN,
        "RWF" => RWF,
        "TND" => TND,
        "XCD" => XCD,
        "AZN" => AZN,
        "CHE" => CHE,
        "ETB" => ETB,
        "LKR" => LKR,
        "NAD" => NAD,
        "SAR" => SAR,
        "TOP" => TOP,
        "XOF" => XOF,
        "BAM" => BAM,
        "CHF" => CHF,
        "EUR" => EUR,
        "INR" => INR,
        "LRD" => LRD,
        "NGN" => NGN,
        "SBD" => SBD,
        "TRY" => TRY,
        "XPF" => XPF,
        "BBD" => BBD,
        "CHW" => CHW,
        "FJD" => FJD,
        "IQD" => IQD,
        "LSL" => LSL,
        "NIO" => NIO,
        "SCR" => SCR,
        "TTD" => TTD,
        "YER" => YER,
        "BDT" => BDT,
        "CLF" => CLF,
        "FKP" => FKP,
        "IRR" => IRR,
        "LYD" => LYD,
        "NOK" => NOK,
        "SDG" => SDG,
        "TWD" => TWD,
        "ZAR" => ZAR,
        "BGN" => BGN,
        "CLP" => CLP,
        "GBP" => GBP,
        "ISK" => ISK,
        "MAD" => MAD,
        "NPR" => NPR,
        "SEK" => SEK,
        "TZS" => TZS,
        "ZMW" => ZMW,
        "BHD" => BHD,
        "CNY" => CNY,
        "GEL" => GEL,
        "JMD" => JMD,
        "MDL" => MDL,
        "NZD" => NZD,
        "SGD" => SGD,
        "UAH" => UAH,
        "ZWL" => ZWL,
        "BIF" => BIF,
        "COP" => COP,
        "GHS" => GHS,
        "JOD" => JOD,
        "MGA" => MGA,
        "OMR" => OMR,
        "SHP" => SHP,
        "UGX" => UGX,
        "BMD" => BMD,
        "COU" => COU,
        "GIP" => GIP,
        "JPY" => JPY,
        "MKD" => MKD,
        "PAB" => PAB,
        "SLL" => SLL,
        "USD" => USD,
        "BND" => BND,
        "CRC" => CRC,
        "GMD" => GMD,
        "KES" => KES,
        "MMK" => MMK,
        "PEN" => PEN,
        "SOS" => SOS,
        "USN" => USN,
        "BOB" => BOB,
        "CUC" => CUC,
        "GNF" => GNF,
        "KGS" => KGS,
        "MNT" => MNT,
        "PGK" => PGK,
        "SRD" => SRD,
        "UYI" => UYI,
        _ => panic!("Unknown currency {}", currency_string),
    }
}
