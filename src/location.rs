pub enum Location<'a> {
    CityName {
        name: &'a str,
        state_code: Option<&'a str>,
        country_code: Option<&'a str>,
    },
    CityId {
        id: &'a str,
    },
    Coordinates {
        lat: f64,
        lon: f64,
    },
    ZipCode {
        zip_code: &'a str,
        country_code: Option<&'a str>,
    },
    BoundingBox {
        lon_left: f64,
        lat_bottom: f64,
        lon_right: f64,
        lat_top: f64,
        zoom: f64,
    },
    Circle {
        lat: f64,
        lon: f64,
        count: u32,
    },
}

impl Location {}