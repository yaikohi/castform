use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub timezone: Option<String>,
    #[serde(rename = "timezone_offset")]
    pub timezone_offset: Option<i64>,
    pub data: Vec<Weather>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub dt: Option<i64>,
    pub sunrise: Option<i64>,
    pub sunset: Option<i64>,
    pub temp: Option<f64>,
    #[serde(rename = "feels_like")]
    pub feels_like: Option<f64>,
    pub pressure: Option<i64>,
    pub humidity: Option<i64>,
    #[serde(rename = "dew_point")]
    pub dew_point: Option<f64>,
    pub uvi: Option<f64>,
    pub clouds: Option<i64>,
    pub visibility: Option<i64>,
    #[serde(rename = "wind_speed")]
    pub wind_speed: Option<f64>,
    #[serde(rename = "wind_deg")]
    pub wind_deg: Option<i64>,
    pub weather: Vec<WeatherDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherDetails {
    pub id: Option<i64>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}
