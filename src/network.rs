use crate::location;
use crate::weather::WeatherCurrent;

pub struct OpenWeatherClient {
    pub token: String,
    lang: Option<String>,
    units: Option<String>,
    client: reqwest::Client,
}

impl OpenWeatherClient {
    pub fn new(token: &str) -> OpenWeatherClient {
        OpenWeatherClient {
            token: token.to_string(),
            lang: None,
            units: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn new_with_custom_client(token: &str, client: reqwest::Client) -> OpenWeatherClient {
        OpenWeatherClient {
            token: token.to_string(),
            lang: None,
            units: None,
            client,
        }
    }

    pub async fn get_current_weather(&self, location: location::Location) -> Option<WeatherCurrent> {
        unimplemented!()
    }
}