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

}