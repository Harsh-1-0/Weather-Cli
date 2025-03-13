use clap::Parser;
use dotenvy::dotenv;
use reqwest;
use serde::Deserialize;
use tokio;
use colored::*;

#[derive(Parser)]
#[command(version="1.0.0", about="Weather CLI",author="Harsh Kumar Sinha",about="Fetch Weather Information from OpenWeatherApi by passing city ")]
struct Args {
    city: String,
}

fn get_api_key() -> String {
    dotenv().ok();
    std::env::var("API_KEY").expect("API KEY NOT FOUND")
}

#[derive(Deserialize)]
struct ApiResponse {
    main: Main,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
}

struct Weather {
    city: String,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    feel_like: f32,
    temp_min: f32,
    temp_max: f32,
    description: String,
}

async fn get_weather(city: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let res = reqwest::get(&url).await?;
    let body: ApiResponse = res.json().await?;

    let weather = Weather {
        city: city.to_string(),
        temperature: body.main.temp,
        humidity: body.main.humidity,
        pressure: body.main.pressure,
        feel_like: body.main.feels_like,
        temp_min: body.main.temp_min,
        temp_max: body.main.temp_max,
        description: body.weather[0].description.clone(),
    };

    Ok(weather)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let city = args.city;

    match get_weather(&city).await {
        Ok(weather) => {
            println!(
                "{} {}: {}°C, {}",
                "Weather in".blue().bold(),
                weather.city.green().to_uppercase(),
                weather.temperature.to_string().bright_blue().on_bright_white(),
                weather.description.cyan()
            );
            println!(
                "{}: {}°C  {}: {}°C  {}: {}°C",
                "Feels Like".magenta(),
                weather.feel_like.to_string().yellow(),
                "Min Temp".blue(),
                weather.temp_min.to_string().yellow(),
                "Max Temp".red(),
                weather.temp_max.to_string().yellow()
            );

            println!(
                "{}: {}  {}: {} hPa",
                "Humidity".green(),
                format!("{}%", weather.humidity).yellow(),
                "Pressure".purple(),
                weather.pressure.to_string().yellow()
            );
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
