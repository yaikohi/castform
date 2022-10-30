use dotenv::dotenv;
use exitfailure::ExitFailure;
use std::env;

use crate::utils::get_coordinates::Response;

mod utils;

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
