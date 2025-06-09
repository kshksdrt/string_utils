use clap::{Parser, ValueEnum};

mod color_utils;
mod date_time_utils;
mod utils;

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
pub enum CliFunction {
    ColorHex,
    ColorHexLightBg,
    WordCount,
    BeBackInString,
    BeBackInStringWithTimestamp,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input_string: String,

    #[arg(long, value_name = "FUNCTION_NAME")]
    invoke: Option<CliFunction>,
}

fn main() {
    // Parse the command line arguments using the struct defined above
    let cli = Cli::parse();

    match cli.invoke {
        Some(function_name) => match function_name {
            CliFunction::ColorHex => {
                let color = color_utils::string_to_hex_color(&cli.input_string);
                println!("{}", color);
            }
            CliFunction::ColorHexLightBg => {
                const SATURATION_PERCENTAGE: f64 = 0.8;
                const LIGHTNESS_PERCENTAGE: f64 = 0.8;
                let color = color_utils::string_to_hex_color_sl(
                    &cli.input_string,
                    SATURATION_PERCENTAGE,
                    LIGHTNESS_PERCENTAGE,
                );
                println!("{}", color);
            }
            CliFunction::WordCount => {
                println!("Coming soon!");
            }
            CliFunction::BeBackInString => match cli.input_string.parse::<u32>() {
                Ok(seconds) => {
                    let formatted = date_time_utils::format_duration(seconds);
                    println!("{}", formatted);
                }
                Err(_) => {
                    println!("Error: Please provide a valid number of seconds");
                }
            },
            CliFunction::BeBackInStringWithTimestamp => match cli.input_string.parse::<u32>() {
                Ok(seconds) => {
                    let formatted = date_time_utils::format_duration_with_timestamp(seconds);
                    println!("{}", formatted);
                }
                Err(_) => {
                    println!("Error: Please provide a valid number of seconds");
                }
            },
        },
        _ => {
            println!("I don't know what to do with it.");
        }
    }
}
