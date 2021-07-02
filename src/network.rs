use crate::location;
use crate::weather::WeatherCurrent;
use std::error::Error;
use crate::language::Lang;
use crate::unit::Unit;

const PREFIX: &str = "https://api.openweathermap.org/data/2.5/";

pub struct OpenWeatherClient {
    pub token: String,
    pub lang: Lang,
    pub units: Unit,
    client: reqwest::Client,
}

impl OpenWeatherClient {
    pub fn builder(token: &str) -> OpenWeatherClientBuilder {
        OpenWeatherClientBuilder::new(token)
    }

    pub fn new(token: &str) -> OpenWeatherClient {
        OpenWeatherClient {
            token: token.to_string(),
            lang: Lang::default(),
            units: Unit::default(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_current_weather(&self, location: location::Location) -> Result<WeatherCurrent, Box<dyn Error>> {
        let url = format!("{}weather?{}&lang={}&units={}&appid={}",
                          PREFIX,
                          location.get_parameter(),
                          self.lang.to_string(),
                          self.units.to_string(),
                          self.token);

        let result = self.client.get(url).send().await?;
        let json = result.json().await?;
        Ok(json)
    }
}

pub struct OpenWeatherClientBuilder {
    token: String,
    lang: Lang,
    units: Unit,
    client: reqwest::Client,
}

impl OpenWeatherClientBuilder {

    pub fn new(token: &str) -> OpenWeatherClientBuilder {
        OpenWeatherClientBuilder {
            token: token.to_string(),
            lang: Lang::default(),
            units: Unit::default(),
            client: reqwest::Client::new(),
        }
    }

    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    pub fn unit(mut self, unit: Unit) -> Self {
        self.units = unit;
        self
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub fn build(self) -> OpenWeatherClient {
        OpenWeatherClient {
            token: self.token,
            lang: self.lang,
            units: self.units,
            client: self.client,
        }
    }
}