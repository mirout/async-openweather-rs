use crate::location;
use crate::weather::WeatherCurrent;
use std::error::Error;
use crate::language::Lang;

const PREFIX: &str = "https://api.openweathermap.org/data/2.5/";

pub struct OpenWeatherClient {
    pub token: String,
    lang: Lang,
    units: Option<String>,
    client: reqwest::Client,
}

impl OpenWeatherClient {
    pub fn new(token: &str) -> OpenWeatherClient {
        OpenWeatherClient {
            token: token.to_string(),
            lang: Lang::default(),
            units: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn new_with_custom_client(token: &str, client: reqwest::Client) -> OpenWeatherClient {
        OpenWeatherClient {
            token: token.to_string(),
            lang: Lang::default(),
            units: None,
            client,
        }
    }

    pub async fn get_current_weather(&self, location: location::Location) -> Result<WeatherCurrent, Box<dyn Error>> {
        let url = format!("{}weather?{}&lang={}&appid={}", PREFIX, location.get_parameter(), self.lang.to_string(), self.token);
        let result = self.client.get(url).send().await?;
        let json = result.json().await?;
        Ok(json)
    }
}