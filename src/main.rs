mod parse;
use parse::WeatherData;
use parse::IpAddr;

use get_if_addrs::{get_if_addrs, IfAddr, Ifv4Addr};
use reqwest;
use std::net::Ipv4Addr;
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;


lazy_static! {
    static ref API_KEY: String = {
        dotenv().ok(); 
        env::var("API_KEY_WEATHER").expect("API_KEY_WEATHER not found")
    };
}

#[tokio::main]
async fn main() {

    let city = match generate_ipv4().await {
        Ok(if_addr) => parser_ip_result(if_addr).await,
        Err(e) => {
            eprintln!("Erro: {}", e);
            "Unknown region".to_string() 
        }
    };

    let api_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, API_KEY.as_str());

    parser_weather_result(api_url).await;
}

async fn parser_weather_result(link:String){
    match WeatherData::parse_weather(link).await{
        Ok(weather_data) => {
            println!("Weather condition: {}", weather_data.weather[0].main);
            println!("Temperature: {:.2}°C", weather_data.main.temp - 273.15); // Convertendo de Kelvin para Celsius
            println!("Humidity: {}%", weather_data.main.humidity);
            println!("Wind Speed: {:.2} m/s", weather_data.wind.speed);
        }
        Err(e) => {
            eprintln!("Failed to fetch weather data: {}", e);
        }
    }
}

async fn parser_ip_result(ip: IfAddr) -> String {
    match IpAddr::parse_geolocation(ip).await {
        Ok(ip_data) => ip_data.region,
        Err(e) => {
            eprintln!("Failed to fetch IP data: {}", e);
            "Unknown region".to_string() 
        }
    }
}

async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.ipify.org?format=text")
        .await?
        .text()
        .await?;
    Ok(response)
}

async fn generate_ipv4() -> Result<IfAddr, String> {
    let client_ip = match get_public_ip().await {
        Ok(public_ip) => public_ip,
        Err(e) => return Err(format!("Erro ao obter IP público: {}", e)),
    };

    let ip_v4 = match client_ip.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(e) => return Err(format!("Erro ao converter IP para Ipv4Addr: {}", e)),
    };

    Ok(IfAddr::V4(Ifv4Addr {
        ip: ip_v4,
        netmask: Ipv4Addr::new(255, 255, 255, 0),
        broadcast: None, 
    }))
}