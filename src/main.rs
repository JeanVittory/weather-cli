use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialized the JSON response from the openWatherMap API
#[Derive(deserialized, debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent the weather description
#[derive(deserialized, debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather parameters
#[derive(deserialized, debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64
} 

// Struct to represent the wind information
#[derive(deserialized, debug)]
struct Wind{
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap
fn get_weather_info(city: &str, country_code:&str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url : String = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}", city, country_code, api_key);

    let response = reqwest::blocking::get(url)?;
    let response_json:WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// function to display the weather information
fn display_weather_info(response: &WeatherResponse){
    let description:&string = &response.weather[0].description;
    let temprature:f64 = response.main.temp;
    let humidity:f64  = response.main.humidity;
    let pressure:f64  = response.main.pressure;
    let wind_speed:f64  = response.wind.speed;
    let weather_text:string = formt(
        "Weather in {}: {} {}
        > Temperature: {:1} °C,
        > Humidity: {:.1} %,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        temperature, 
        humidity, 
        pressure, 
        wind_speed
    );  
}