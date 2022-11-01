use dotenv::dotenv;
use exitfailure::ExitFailure;
use std::env;

use crate::utils::{
    get_coordinates::LocationResponse, get_weather::WeatherResponse, tokens::load_token,
};

mod utils;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    // let timestamp = chrono::offset::Utc::now().timestamp();
    // dbg!(timestamp);

    let api_key_positionstack = load_token(String::from("API_TOKEN_POSITIONSTACK"));
    let api_key_openweathermap = load_token(String::from("API_TOKEN_OPENWEATHERMAP"));

    let args: Vec<String> = env::args().collect();
    let mut location_query: String = "Utrecht".to_string();

    // TODO: Refactor this part.
    // - [ ] refactor to be more readable
    // - [ ] refactor to be more intuitive for the CLI
    if args.len() < 2 {
        println!("Since you didn't specify a location, it has defaulted to Utrecht.");
    } else if args.len() > 2 {
        location_query = args[0].clone();
    } else {
        location_query = args[0].clone();
    }

    let location_response =
        LocationResponse::get_lat_lon(&location_query, &api_key_positionstack).await?;
        // println!(
        //     "{}'s lat: {:?}, lon: {:?}",
        //     location_query,
        //     location_response[0].expect("Null value"),
        //     location_response[1].expect("Null value")
        // );
    
    let lat = location_response[0].expect("No lat found");
    let lon = location_response[1].expect("No lon found");

    let weather_response =
        WeatherResponse::get(&lat.to_string(), &lon.to_string(), &api_key_openweathermap).await?;

    let city = &weather_response.city.name;
    // dbg!(city);

    println!("\n\nIn {} it's {:?} degrees celsius\n\n",city, weather_response.list[0].main.temp);
    Ok(())
}
