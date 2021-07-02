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
        zip_code: String,
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
    pub fn get_parameter(&self) -> String {
        match self {
            Location::CityName { name, state_code, country_code } => {
                format!("q={}{}{}",
                        name,
                        state_code.as_ref().map(|x| ",".to_string() + x).unwrap_or("".to_string()),
                        country_code.as_ref().map(|x| ",".to_string() + x).unwrap_or(String::from("")))
            }
            Location::CityId { .. } => {unimplemented!()}
            Location::Coordinates { .. } => {unimplemented!()}
            Location::ZipCode { .. } => {unimplemented!()}
            Location::BoundingBox { .. } => {unimplemented!()}
            Location::Circle { .. } => {unimplemented!()}
        }
    }
}