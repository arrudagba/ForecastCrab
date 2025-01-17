mod parse;
use parse::WeatherData;
use parse::IpAddr;

use get_if_addrs::{IfAddr, Ifv4Addr};
use reqwest;
use std::net::Ipv4Addr;
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

use eframe::egui;
use eframe::egui::IconData;


lazy_static! {
    static ref API_KEY: String = {
        dotenv().ok(); 
        env::var("API_KEY_WEATHER").expect("API_KEY_WEATHER not found")
    };
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([375.0, 520.0])
            .with_resizable(false)
            .with_fullscreen(false)
            .with_maximize_button(false)
            .with_icon(load_icon("./icons/logo.png")),
        ..Default::default()
    };

    eframe::run_native(
        "ForecastCrab",
        options,
        Box::new(|_cc| {egui_extras::install_image_loaders(&_cc.egui_ctx);Ok(Box::new(MyApp::default()))}),
    )
}


async fn parser_ip_result(ip: IfAddr) -> String {
    match IpAddr::parse_geolocation(ip).await {
        Ok(ip_data) => ip_data.city,
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

#[derive(Default)]
struct MyApp {
    input_text: String,
    displayed_text: String,
    paused: bool,
    city: String,
    weather_data: Option<WeatherData>, 
    error_message: Option<String>, 
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        let input = egui::TextEdit::singleline(&mut self.input_text)
                            .hint_text("Enter a location")
                            .text_color(egui::Color32::from_rgb(250, 250, 250))
                            .char_limit(30)
                            .font(egui::FontId{size: 17.0, family: egui::FontFamily::default(),})
                            .margin(egui::Margin::symmetric(5.0, 9.7))
                            .desired_width(ui.available_width() * 0.8);

                        ui.add(input);
                        ui.set_max_size(eframe::egui::vec2(300.0, 40.0));
                        ui.add_space(ui.available_width() * 0.5);
                        if ui
                        .add(
                            egui::Button::new("🔍")
                                .rounding(20.0) 
                                .min_size(egui::vec2(40.0, 40.0)),
                        )
                        .clicked()
                    {
                        self.displayed_text = self.input_text.clone();
                        self.paused = false;
                    }
                    });

                    
                });
            });
            ui.vertical(|ui| {
                ui.separator();
                ui.add_space(25.0);
                if self.displayed_text.is_empty() {
                    let city = get_city(self);
                    let api_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, API_KEY.as_str());
                    ui.label(
                        egui::RichText::new(self.city.as_str())
                            .size(20.0),
                    );
                    display_weather(self,ui, api_url,ctx);
                }else{
                    let api_url = format!(
                        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
                        self.displayed_text,
                        API_KEY.as_str()
                    );
    
                    display_weather( self, ui, api_url.clone(), ctx);
                }
            });
            
        });
    }
}

fn load_icon(path: &str) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn display_images(weather: &str) -> egui::Image {
    let image = match weather {
        "Clear" => egui::Image::new(egui::include_image!("../icons/Sun.png")),
        "Clouds" => egui::Image::new(egui::include_image!("../icons/Cloud.png")),
        "Rain" => egui::Image::new(egui::include_image!("../icons/Rain.png")),
        "Snow" => egui::Image::new(egui::include_image!("../icons/Snow.png")),
        "Thunderstorm" => egui::Image::new(egui::include_image!("../icons/Storm.png")),
        _ => egui::Image::new(egui::include_image!("../icons/NotFound.png")),
    };
    image
}

fn not_found(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add(
                display_images("NotFound")
                    .max_size(egui::vec2(170.0, 170.0)),
            );
            ui.label(
                egui::RichText::new("Oops! Error finding that location!")
                    .size(20.0),
            );
        });
    });
}


fn display_weather(app: &mut MyApp, ui: &mut egui::Ui, api_url: String, ctx: &egui::Context) {
    if !app.paused {
        app.paused = true;

        match futures::executor::block_on(WeatherData::parse_weather(api_url)) {
            Ok(weather_data) => {
                app.weather_data = Some(weather_data); 
                app.error_message = None; 
            }
            Err(e) => {
                app.weather_data = None; 
                app.error_message = Some(format!("Erro ao carregar dados: {}", e));
            }
        }
        
    }
    
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(weather_data) = &app.weather_data {
            ui.vertical_centered(|ui| {
                ui.add(
                    display_images(weather_data.weather[0].main.as_str())
                        .max_size(egui::vec2(170.0, 170.0)),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{:.2}°C",
                        weather_data.main.temp - 273.15
                    ))
                    .size(30.0),
                );
                ui.label(
                    egui::RichText::new(weather_data.weather[0].main.as_str())
                        .size(20.0),
                );
            });
            ui.add_space(70.0);
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::new(egui::include_image!("../icons/Humidity.png"))
                        .max_size(egui::vec2(40.0, 40.0)),
                );
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{}%",
                            weather_data.main.humidity
                        ))
                        .size(20.0),
                    );
                    ui.label("Humidity");
                });
                ui.add_space(120.0);
                ui.add(
                    egui::Image::new(egui::include_image!("../icons/Wind.png"))
                        .max_size(egui::vec2(20.0, 20.0)),
                );
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.2}Km/h",
                            weather_data.wind.speed
                        ))
                        .size(20.0),
                    );
                    ui.label("Wind Speed");
                });
            });
        } else if let Some(_) = &app.error_message {
            not_found(ui);
        } else {
            ui.label("Loading data...");
        }

        ctx.request_repaint();
    });
}

fn get_city(app: &mut MyApp) -> String {
    if !app.paused {
        app.paused = true;

        match futures::executor::block_on(generate_ipv4()) {
            Ok(if_addr) => {
                let city = futures::executor::block_on(parser_ip_result(if_addr));
                app.city = city.clone(); 
                app.paused = false;
                return city;
            }
            Err(e) => {
                eprintln!("Erro ao gerar IPv4: {}", e);
                app.paused = false;
                return "Unknown region".to_string();
            }
        }
    }

    app.city.clone() 
}

