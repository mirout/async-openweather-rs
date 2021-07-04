use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    #[serde(alias = "Lon")]
    pub lon: f32,
    #[serde(alias = "Lat")]
    pub lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: Option<f32>,
    pub today: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub sys_type: Option<u32>,
    pub id: Option<u32>,
    pub country: String,
    pub sunrise: Option<u64>,
    pub sunset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Precipitation {
    #[serde(rename = "1h")]
    one_h: Option<f32>,
    #[serde(rename = "3h")]
    three_h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherCurrent {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Option<Sys>,
    pub rain: Option<Precipitation>,
    pub snow: Option<Precipitation>,
    pub timezone: Option<i32>,
    pub id: u64,
    pub name: String,
    pub cod: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiCurrentWeather {
    pub message: Option<String>,
    #[serde(deserialize_with = "from_u32_or_str")]
    pub cod: u32,
    pub calctime: Option<f32>,
    #[serde(alias = "cnt", alias = "count")]
    pub count: u32,
    pub list: Vec<WeatherCurrent>,
}

fn from_u32_or_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::Number(num) => num.as_u64().unwrap() as u32,
        Value::String(s) => s.parse().unwrap(),
        _ => return Err(de::Error::custom("wrong type")),
    })
}
