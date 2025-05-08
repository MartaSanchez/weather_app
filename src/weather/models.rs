use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>, 
    pub main: Main, 
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub humidity: f32,
    pub pressure: i32
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32
} 