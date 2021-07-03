pub trait LocationFormat {
    fn format(&self) -> Vec<(&str, String)>;
}

pub enum UnitLocation {
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
}

impl LocationFormat for UnitLocation {
    fn format(&self) -> Vec<(&str, String)> {
        match self {
            UnitLocation::CityName {
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
            UnitLocation::CityId { id } => {
                vec![("id", id.clone())]
            }
            UnitLocation::Coordinates { lat, lon } => {
                vec![("lat", lat.to_string()), ("lon", lon.to_string())]
            }
            UnitLocation::ZipCode {
                zip_code,
                country_code,
            } => {
                vec![("zip", format!("{},{}", zip_code, country_code))]
            }
        }
    }
}

pub enum MultiLocation {
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

impl LocationFormat for MultiLocation {
    fn format(&self) -> Vec<(&str, String)> {
        unimplemented!()
    }
}
