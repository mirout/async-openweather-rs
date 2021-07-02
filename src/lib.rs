pub mod location;
pub mod network;
pub mod weather;
pub mod language;

#[cfg(test)]
mod tests {
    use crate::network::OpenWeatherClient;
    use std::env;
    use crate::location::Location;

    #[tokio::test]
    async fn test_current_weather_by_city() {
        let client = OpenWeatherClient::new(&env::var("OWT").unwrap());
        let city = Location::CityName {
            name: "London".to_string(),
            state_code: None,
            country_code: Some("GB".to_string()),
        };
        let result = client.get_current_weather(city).await;
        assert_eq!(result.unwrap().name, "London")
    }

    #[tokio::test]
    async fn test_current_weather_by_city_id() {
        let client = OpenWeatherClient::new(&env::var("OWT").unwrap());
        let city = Location::CityId {
            id: "498817".to_string()
        };
        let result = client.get_current_weather(city).await;
        assert_eq!(result.unwrap().name, "Saint Petersburg")
    }

    #[tokio::test]
    async fn test_current_weather_by_coordinates() {
        let client = OpenWeatherClient::new(&env::var("OWT").unwrap());
        let city = Location::Coordinates {
            lat: 55.751244,
            lon: 37.618423,
        };
        let result = client.get_current_weather(city).await;
        assert_eq!(result.unwrap().name, "Moscow")
    }

    #[tokio::test]
    async fn test_current_weather_by_zipcode() {
        let client = OpenWeatherClient::new(&env::var("OWT").unwrap());
        let city = Location::ZipCode {
            zip_code: "10001".to_string(),
            country_code: "US".to_string(),
        };
        let result = client.get_current_weather(city).await;
        assert_eq!(result.unwrap().name, "New York")
    }
}
