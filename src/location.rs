pub enum Location {
    CityName {
        name: String,
        state_code: Option<String>,
        country_code: Option<String>,
    },
    CityId {
        id: String,
    },
    Coordinates {
        lat: f32,
        lon: f32,
    },
    ZipCode {
        zip_code: u32,
        country_code: String,
    },
    BoundingBox {
        lon_left: f32,
        lat_bottom: f32,
        lon_right: f32,
        lat_top: f32,
        zoom: f32,
    },
    Circle {
        lat: f32,
        lon: f32,
        count: u32,
    },
}

impl Location {
    pub fn format(&self) -> Vec<(&str, String)> {
        match self {
            Location::CityName {
                name,
                state_code,
                country_code,
            } => {
                let val = format!(
                    "{},{},{}",
                    name,
                    state_code.as_ref().unwrap_or(&"".to_string()),
                    country_code.as_ref().unwrap_or(&"".to_string())
                );
                vec![("q", val)]
            }
            Location::CityId { id } => {
                vec![("id", id.clone())]
            }
            Location::Coordinates { lat, lon } => {
                vec![("lat", lat.to_string()), ("lon", lon.to_string())]
            }
            Location::ZipCode {
                zip_code,
                country_code,
            } => {
                vec![("zip", format!("{},{}", zip_code, country_code))]
            }
            Location::BoundingBox { .. } => {
                unimplemented!()
            }
            Location::Circle { .. } => {
                unimplemented!()
            }
        }
    }
}
