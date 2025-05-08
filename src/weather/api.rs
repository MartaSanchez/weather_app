use crate::weather::models::WeatherResponse;

pub fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) 
} 