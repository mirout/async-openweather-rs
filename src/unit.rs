pub enum Unit {
    Standard,
    Metric,
    Imperial,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Standard
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Standard => "standard".to_string(),
            Unit::Metric => "metric".to_string(),
            Unit::Imperial => "imperial".to_string(),
        }
    }
}
