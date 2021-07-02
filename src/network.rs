use crate::location;
use crate::weather::{WeatherCurrent, Weather};

const PREFIX: &str = "https://api.openweathermap.org/data/2.5/";

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
        let url = format!("{}weather?{}&appid={}", PREFIX, location.get_parameter(), self.token);
        let result = self.client.get(url).send().await.ok()?;
        let json = result.json().await.ok()?;
        Some(json)
    }
}