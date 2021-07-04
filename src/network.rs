use crate::language::Lang;
use crate::location::{LocationFormat, MultiLocation, UnitLocation};
use crate::unit::Unit;
use crate::weather::{MultiCurrentWeather, WeatherCurrent};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::error::Error;

const PREFIX: &str = "https://api.openweathermap.org/data/2.5/";

pub struct OpenWeatherClient {
    pub token: String,
    pub lang: Lang,
    pub units: Unit,
    client: reqwest::Client,
}

impl OpenWeatherClient {
    pub fn builder(token: String) -> OpenWeatherClientBuilder {
        OpenWeatherClientBuilder::new(token)
    }

    pub fn new(token: String) -> OpenWeatherClient {
        OpenWeatherClient {
            token,
            lang: Lang::default(),
            units: Unit::default(),
            client: reqwest::Client::new(),
        }
    }

    pub fn params<'a>(&self, mut location: Vec<(&'a str, String)>) -> Vec<(&'a str, String)> {
        let mut client = vec![
            ("lang", self.lang.to_string()),
            ("units", self.units.to_string()),
            ("appid", self.token.clone()),
        ];
        location.append(&mut client);
        location
    }
}

pub struct OpenWeatherClientBuilder {
    token: String,
    lang: Lang,
    units: Unit,
    client: reqwest::Client,
}

impl OpenWeatherClientBuilder {
    pub fn new(token: String) -> OpenWeatherClientBuilder {
        OpenWeatherClientBuilder {
            token,
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

#[async_trait]
pub trait Request {
    async fn get<T: DeserializeOwned>(
        &self,
        url: String,
        params: Vec<(&str, String)>,
    ) -> Result<T, Box<dyn Error>>;
}

#[async_trait]
pub trait CurrentWeather<T>: Request {
    type CurWeather;
    async fn get_current_weather(&self, location: T) -> Result<Self::CurWeather, Box<dyn Error>>;
}

#[async_trait]
impl Request for OpenWeatherClient {
    async fn get<T: DeserializeOwned>(
        &self,
        url: String,
        params: Vec<(&str, String)>,
    ) -> Result<T, Box<dyn Error>> {
        let response = self.client.get(url).query(&params).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
}

#[async_trait]
impl CurrentWeather<UnitLocation> for OpenWeatherClient {
    type CurWeather = WeatherCurrent;

    async fn get_current_weather(
        &self,
        location: UnitLocation,
    ) -> Result<Self::CurWeather, Box<dyn Error>> {
        let url = format!("{}weather", PREFIX);
        let params = self.params(location.format());
        self.get(url, params).await
    }
}

#[async_trait()]
impl CurrentWeather<MultiLocation> for OpenWeatherClient {
    type CurWeather = MultiCurrentWeather;

    async fn get_current_weather(
        &self,
        location: MultiLocation,
    ) -> Result<Self::CurWeather, Box<dyn Error>> {
        let url = match location {
            MultiLocation::BoundingBox { .. } => {
                format!("{}box/city", PREFIX)
            }
            MultiLocation::Circle { .. } => {
                format!("{}find", PREFIX)
            }
        };
        let params = self.params(location.format());
        self.get(url, params).await
    }
}
