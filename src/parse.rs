use serde::Deserialize;
use get_if_addrs::IfAddr;
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    static ref API_KEY: String = {
        dotenv().ok(); 
        env::var("API_KEY_IPLOOKUP").expect("API_KEY_IPLOOKUP not found")
    };
}

#[derive(Debug, Deserialize)]
pub struct WeatherData { 
    pub weather: Vec<Weather>, 
    pub main: Main,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct Weather { 
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct Main { 
    pub temp: f64,
    pub humidity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Wind { 
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct IpAddr{
    pub city: String,
}

impl WeatherData {
    pub async fn parse_weather(link: String) -> Result<Self, reqwest::Error> {
        let response = reqwest::Client::new()
            .get(link)
            .send()
            .await?
            .json::<WeatherData>()
            .await?;
    
        Ok(response)
    }
    
}

impl IpAddr{
    pub async fn parse_geolocation(ip: IfAddr) -> Result<Self, reqwest::Error> {
        let link = format!("https://ipinfo.io/{}?token={}", ip.ip(), API_KEY.as_str());
        let response = reqwest::Client::new()
            .get(link)
            .send()
            .await?
            .json::<IpAddr>()
            .await?;

        Ok(response)
    }
}