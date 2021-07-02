use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub lon: f32,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename="type")]
    pub sys_type: u32,
    pub id: u32,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherCurrent {
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: u64,
    pub name: String,
    pub cod: u32,
}
// {"coord":{"lon":-0.1257,"lat":51.5085},"weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04d"}],"base":"stations","main":{"temp":291.77,"feels_like":291.75,"temp_min":288.76,"temp_max":294.11,"pressure":1018,"humidity":79},"visibility":9000,"wind":{"speed":2.57,"deg":260},"clouds":{"all":75},"dt":1625218116,"sys":{"type":2,"id":268730,"country":"GB","sunrise":1625197696,"sunset":1625257232},"timezone":3600,"id":2643743,"name":"London","cod":200}