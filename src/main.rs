use clap::{Parser, ValueEnum};

mod modname;

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
    out: Option<CliFunction>,
}

fn main() {
    // Parse the command line arguments using the struct defined above
    let cli = Cli::parse();

    match cli.out {
        Some(function_name) => match function_name {
            CliFunction::HexColor => {
                let color = modname::string_to_hex_color(&cli.input_string);
                println!("{}", color);
            }
            CliFunction::WordCount => {
                println!("Coming soon!");
            }
        },
        // When --out was not provided.
        _ => {
            println!("I don't know what to do with it. Try specifying --out hex_color");
        }
    }
}
