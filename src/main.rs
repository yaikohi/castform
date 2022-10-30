use dotenv::dotenv;
use exitfailure::ExitFailure;
use std::env;
use chrono::Utc;


use crate::utils::{get_coordinates::LocationResponse, tokens::load_token};

mod utils;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let current_time = Utc::now();
    let api_key_positionstack = load_token(String::from("API_TOKEN_POSITIONSTACK"));
    let api_key_openweathermap = load_token(String::from("API_TOKEN_OPENWEATHERMAP"));

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

    let location_response = LocationResponse::get_lat_lon(&location_query, &api_key_positionstack).await?;
    println!(
        "{}'s lat: {:?}, lon: {:?}",
        location_query,
        location_response[0].expect("Null value"),
        location_response[1].expect("Null value")
    );

    let lat = location_response[0];
    let lon = location_response[1];

    // let weather_response = WeatherResponse::get(&lat, &lon, &api_key_openweathermap).await?;

    Ok(())
}
