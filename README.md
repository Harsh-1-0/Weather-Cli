# Weather CLI

A simple and colorful command-line tool to fetch weather information using the OpenWeatherMap API. Built with Rust.

## Features

✅ Fetch current weather details for any city  
✅ Display results in a colorful format  
✅ Works on Windows, macOS, and Linux  
✅ Easily export as a standalone executable

## Installation

### **1. Clone the Repository**

```sh
git clone https://github.com/Harsh-1-0/weather-cli.git
cd weather-cli
```

### **2. Install Dependencies**

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```sh
cargo build
```

### **3. Set Up API Key**

1. Get a free API key from [OpenWeatherMap](https://openweathermap.org/api).
2. Create a `.env` file in the project directory:
   ```sh
   echo "API_KEY=your_api_key_here" > .env
   ```

### **4. Run the CLI**

```sh
cargo run -- "Patna"
```

## Exporting as Executable

### **Linux/macOS:**

```sh
cargo build --release
sudo mv target/release/weather-cli /usr/local/bin/weather-cli
```

Now, run:

```sh
weather-cli "Berlin"
```

### **Windows:**

```sh
cargo build --release
copy target\release\weather-cli.exe C:\weather-cli\
```

Add `C:\weather-cli\` to the system **PATH** to run from anywhere.

## Example Output

```
Weather in Patna: 26.96°C, Haze
Feels Like: 28.1°C  Min Temp: 23°C  Max Temp: 28°C
Humidity: 61%  Pressure: 1005 hPa
```

## Dependencies

- [clap](https://docs.rs/clap) - Command-line argument parsing
- [dotenvy](https://docs.rs/dotenvy) - Environment variable loading
- [reqwest](https://docs.rs/reqwest) - HTTP requests
- [serde](https://docs.rs/serde) - JSON parsing
- [colored](https://docs.rs/colored) - Colorful terminal output
- [tokio](https://docs.rs/tokio) - Async runtime

## License

This project is licensed under the MIT License.
