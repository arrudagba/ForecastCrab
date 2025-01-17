# 🦀 ForecastCrab
<div style="display: inline_block">
  <img align="center" alt="Rust" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"/>
</div><br/>

![ForecastCrab Banner](icons/banner.png)

ForecastCrab is a simple weather forecasting application built in **Rust**, leveraging the power of **Egui/Eframe** for a sleek and responsive user interface. The application fetches real-time weather data from reliable APIs to display the current weather conditions for your location and any searched location. Whether you're planning your day or exploring other cities, ForecastCrab is here to provide precise and up-to-date forecasts.

---

## 🌟 Features
- **Real-Time Weather Updates**: Get instant weather data for your current location.
- **Search for Any City**: Retrieve weather information for cities worldwide with ease.
- **Modern UI with Egui/Eframe**: Enjoy a user-friendly interface powered by Rust's cutting-edge frameworks.
- **Lightweight and Fast**: Optimized for performance, delivering results quickly.

---

## 🔧 Prerequisites
To run ForecastCrab, ensure you have the following:
- **Rust** installed on your system. Download and install it from [Rust's official site](https://www.rust-lang.org/tools/install).
- An active internet connection to fetch weather data from APIs.
- API key(s) from the weather service(s) of your choice (e.g., OpenWeatherMap). [Sign up for a free API key here](https://openweathermap.org/api).
- API key(s) from the ip-lookup service(s) of your choice (e.g., Ipinfo). [Sign up for a free API key here](https://ipinfo.io/products/ip-geolocation-api).

---

## 🚀 Installation and Running
Follow these steps to set up and run ForecastCrab:

1. **Clone the Repository**  
   ```bash
   git clone https://github.com/arrudagba/ForecastCrab.git
   cd ForecastCrab
2. **Set Up Your API Key**
- Create a **.env** file in the root directory.
- Add your Weather and IPinfo API key:
    ```bash
    API_KEY_WEATHER=your_api_key_here
    API_KEY_IPLOOKUP=your_api_key_here
3. **Build the project**
    ```bash
    cargo build --release
4. **Run the Application**
    ```bash
    cargo run
5. **Enjoy the ForecastCrab**
Open the app, and start exploring weather updates for your location or search for any city worldwide!

---

## 🛠️ Built With
- [Rust](https://www.rust-lang.org/) - A fast, memory-safe programming language.
- [Egui/Eframe](https://github.com/emilk/egui) - A simple and immediate mode GUI library for Rust.

---
## 🎥 Example of usage
https://github.com/user-attachments/assets/f229754d-53ff-4ac9-8b45-723028da65f2
