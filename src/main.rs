use std::io;
use std::env;
use serde::Deserialize;
use colored::*;
use dotenvy::dotenv;

// Struct to deserialized the JSON response from the openWatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent the weather description
#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64
} 

// Struct to represent the wind information
#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap
fn get_weather_info(city: &str, country_code:&str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url : String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key);

    let response = reqwest::blocking::get(url)?;
    let response_json:WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// function to display the weather information
fn display_weather_info(response: &WeatherResponse){
    let description:&String = &response.weather[0].description;
    let temperature:f64 = response.main.temp;
    let humidity:f64  = response.main.humidity;
    let pressure:f64  = response.main.pressure;
    let wind_speed:f64  = response.wind.speed;
    let weather_text:String = format!(
        "Weather in {}: {} {}
        > Temperature: {:1} °C,
        > Humidity: {:.1} %,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature, 
        humidity, 
        pressure, 
        wind_speed
    );

    //Coloring the weather text base in weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "clouds" | "scatter clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    }; 
        
    //Function to get emoji based on temperature 
    fn get_temp_emoji(temperature:f64) -> &'static str{
        if temperature < 0.0 {
            "❄️"
        }else if temperature >= 0.0 && temperature < 10.0 {
            "☁️"
        }else if temperature >= 10.0  && temperature < 20.0{
            "🌥️"
        }else if temperature >= 20.0 && temperature < 30.0 {
            "🌤️"
        } else {
            "🔥"
        }
    }

    println!("{}", weather_text_colored);
}

fn main(){
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    dotenv().ok();
    loop{
        //Reading the city
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city:String = String::new();
        io::stdin().read_line(&mut city).expect("Failed reading the city input!");
        let city:&str = city.trim();

        //Reading the country
        println!("{}", "Please enter the country:".on_bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed reading the country input!");
        let country_code:&str = country_code.trim();
        
        let api_key = &env::var("API_KEY").expect("The API_KEY from OpenWeatherMap is not defined!");
        match get_weather_info(&city, &country_code, api_key){
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(e) => {
                eprintln!("Error fetching weather data: {}", e);
            }
        }
        println!("{}", "Do you want to search for weather in another city? (yes/no):");
        let mut should_continue = String::new();
        io::stdin().read_line(&mut should_continue).expect("Failed reading if should continue!");
        let should_continue = should_continue.trim().to_lowercase();

        if should_continue != "yes" {
            println!("{}", "Thank you for using our software!");
            break;
        }
    }
}