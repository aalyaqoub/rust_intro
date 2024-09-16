use clap::{Parser, ValueEnum};
use std::fmt;

/// Program to convert temperatures 
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The temperature value 
    #[arg(short, long)]
    temp: f64,

    /// The temperature unit
    #[arg(short, long, value_enum, default_value_t = TemperatureUnit::Fahrenheit)]
    unit: TemperatureUnit,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "Celsius"),
            TemperatureUnit::Fahrenheit => write!(f, "Fahrenheit"),
        }
    }
}

fn main() {
    let args:Args = Args::parse();
    
    if args.unit == TemperatureUnit::Fahrenheit {
        let temp_in_c:f64 = (args.temp - 32.0) * 5.0 / 9.0;
        println!("Temp in Celsius: {:.3}°C", temp_in_c)
    } else if  args.unit == TemperatureUnit::Celsius {
        let temp_in_f:f64 = (args.temp * 9.0 / 5.0) + 32.0;
        println!("Temp in Celsius: {:.3}°F", temp_in_f)
    } else {
        println!("Unknown unit {} was entered", args.unit)
    }
}