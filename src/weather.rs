use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    lon: f32,
    lat: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    temp: f32,
    pressure: f32,
    humidity: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Wind {
    speed: f32,
    deq: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Clouds {
    all: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename="type")]
    sys_type: u32,
    id: u32,
    message: f32,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherCurrent {
    coord: Coordinates,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    id: u32,
    name: String,
    cod: u32,
}