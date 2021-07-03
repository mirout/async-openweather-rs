pub mod language;
pub mod location;
pub mod network;
pub mod unit;
pub mod weather;

#[cfg(test)]
mod tests {
    use crate::location::UnitLocation;
    use crate::network::{CurrentWeather, OpenWeatherClient};
    use std::env;

    #[tokio::test]
    async fn test_current_weather_by_city() {
        let token = env::var("OWT").unwrap();
        let client = OpenWeatherClient::new(token);
        let city = UnitLocation::CityName {
            name: "London".to_string(),
            state_code: None,
            country_code: Some("GB".to_string()),
        };
        let result = client.get_current_weather(city).await.unwrap();
        assert_eq!(result.name, "London")
    }

    #[tokio::test]
    async fn test_current_weather_by_city_id() {
        let token = env::var("OWT").unwrap();
        let client = OpenWeatherClient::new(token);
        let city = UnitLocation::CityId {
            id: "498817".to_string(),
        };
        let result = client.get_current_weather(city).await.unwrap();
        assert_eq!(result.name, "Saint Petersburg")
    }

    #[tokio::test]
    async fn test_current_weather_by_coordinates() {
        let token = env::var("OWT").unwrap();
        let client = OpenWeatherClient::new(token);
        let city = UnitLocation::Coordinates {
            lat: 55.751244,
            lon: 37.618423,
        };
        let result = client.get_current_weather(city).await.unwrap();
        assert_eq!(result.name, "Moscow")
    }

    #[tokio::test]
    async fn test_current_weather_by_zipcode() {
        let token = env::var("OWT").unwrap();
        let client = OpenWeatherClient::new(token);
        let city = UnitLocation::ZipCode {
            zip_code: 10001,
            country_code: "US".to_string(),
        };
        let result = client.get_current_weather(city).await.unwrap();
        assert_eq!(result.name, "New York")
    }
}
