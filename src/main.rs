use clap::{Parser, ValueEnum};

mod color_utils;

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
pub enum CliFunction {
    HexColor,
    WordCount,
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
            CliFunction::HexColor => {
                let color = color_utils::string_to_hex_color(&cli.input_string);
                println!("{}", color);
            }
            CliFunction::WordCount => {
                println!("Coming soon!");
            }
        },
        _ => {
            println!("I don't know what to do with it.");
        }
    }
}
