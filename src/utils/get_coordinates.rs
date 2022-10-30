use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::vec;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationResponse {
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

impl LocationResponse {
    pub async fn get(location_query: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.positionstack.com/v1/forward?access_key={}&query={}",
            api_key, location_query
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json().await?;

        Ok(res)
    }
    pub async fn get_lat_lon(
        location_query: &String,
        api_key: &String,
    ) -> Result<Vec<Option<f64>>, ExitFailure> {
        let res: LocationResponse = Self::get(location_query, api_key).await?;
        let data = &res.data[0];

        let lat = data.latitude;
        let lon = data.longitude;

        let coordinates = vec![lat, lon];

        Ok(coordinates)
    }
}
