use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub data: Vec<LocationCoordinates>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCoordinates {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub name: Option<String>,
    pub number: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub street: Option<String>,
    pub confidence: Option<i64>,
    pub region: Option<String>,
    #[serde(rename = "region_code")]
    pub region_code: Option<String>,
    pub county: Option<String>,
    pub locality: Option<String>,
    #[serde(rename = "administrative_area")]
    pub administrative_area: Option<Value>,
    pub neighbourhood: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "country_code")]
    pub country_code: Option<String>,
    pub continent: Option<String>,
    pub label: Option<String>,
}

impl Response {
    async fn get(location_query: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.positionstack.com/v1/forward?access_key={}&query={}",
            api_key, location_query
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();
    let api_key_positionstack =
        std::env::var("API_TOKEN_POSITIONSTACK").expect("API_TOKEN_POSITIONSTACK must be set.");

    let args: Vec<String> = env::args().collect();
    let mut location_query: String = "Utrecht".to_string();

    if args.len() < 2 {
        dbg!("args.len() < 2: {}", &args);
        println!("Since you didn't specify a location, it has defaulted to Utrecht.");
    } else if args.len() > 2 {
        dbg!("else if: {}", &args);
        location_query = args[0].clone();
    } else {
        dbg!("else: {}", &args);

        location_query = args[0].clone();
    }

    let res = Response::get(&location_query, &api_key_positionstack).await?;
    println!(
        "{}'s lat: {:?}, lon: {:?}",
        location_query,
        res.data[0].latitude.expect("Null value"),
        res.data[0].longitude.expect("Null value")
    );

    Ok(())
}
