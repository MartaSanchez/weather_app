
# 🌤️ WeatherApp

**WeatherApp** is a command-line application written in Rust that lets you check the current weather and forecast right from your terminal. It uses a free weather API to fetch real-time data.

## 🚀 Features

- Get current weather for any city
- Displays temperature, humidity, wind speed, and more

## 📦 Requirements

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- ☁️ A free API key from [OpenWeatherMap](https://openweathermap.org/api)

## 🛠️ Installation

Clone the repository and build the project:

```bash
cargo build 
cargo run
```

## 🖥️ Example of use: 

```Welcome to the Marta weather app!
Let's see if we need ☂️ today!
Please enter the name of the city:
Barcelona
Please enter the country code:
ES
Weather in Barcelona: Clear sky ☀️
        > Temperature: 17.72°C, 
        > Temperature max: 18.69°C,
        > Temperature min: 16.63°C,
        > Humidity: 61%, 
        > Pressure: 1012 hPa, 
        > Wind Speed: 1.54 m/s
Do you want to search for weather in another city? (yes/no):
```
## 📌 Notes: 
- If you press 'no' it will exit the app.
- For the country code, you must use a valid ISO 3166 code.