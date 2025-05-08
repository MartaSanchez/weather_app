use crate::weather::models::WeatherResponse;

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

pub fn get_main_icon(input: &String) -> String {
    let weather_description = input.as_str();
    match weather_description {
        desc if desc.contains("thunderstorm") => "⛈️".to_string(),
        desc if desc.contains("cloud") => "☁️".to_string(),
        desc if desc.contains("drizzle") => "🌦️".to_string(),
        desc if desc.contains("rain") => "🌧️".to_string(),
        desc if desc.contains("snow") => "❄️".to_string(),
        "clear sky" => "☀️".to_string(),
        "partly Cloudy" | "few clouds" => "⛅".to_string(),
        _ => "🌫️".to_string()
    }
}

pub fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temp = response.main.temp;
    let temp_min = response.main.temp_min;
    let temp_max = response.main.temp_max;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;
    let name = &response.name;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {}°C, 
        > Temperature max: {}°C,
        > Temperature min: {}°C,
        > Humidity: {}%, 
        > Pressure: {} hPa, 
        > Wind Speed: {} m/s",
        name,
        capitalize_first(description),
        get_main_icon(description),
        temp,
        temp_max,
        temp_min,
        humidity,
        pressure,
        wind_speed
    );
    println!("{}", weather_text);
} 