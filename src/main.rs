use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use vehicle_parser::parse_vehicle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let input_file_name = "input.txt";
    let output_file_name = "output.json";

    if args.len() < 2 {
        println!("Usage:\n");
        println!("cargo run <command>\n");
        println!("Commands:");
        println!("  parse                         -Parse from input.txt to output.txt");
        println!("  help                          -Show help");
        println!("  credits                       -Show credits");
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            let mut file = File::open(input_file_name)?;
            let mut input = String::new();
            file.read_to_string(&mut input)?;

            match parse_vehicle(&input) {
                Ok(vehicle_json) => {
                    let formatted_json = serde_json::to_string_pretty(&vehicle_json)?;
                    let mut output_file = File::create(output_file_name)?;
                    output_file.write_all(formatted_json.as_bytes())?;

                    println!("Vehicle data successfully written to {}", output_file_name);
                }
                Err(e) => eprintln!("Error parsing vehicle data: {}", e),
            }
        }
        "help" => {
            println!("Usage:\n");
            println!("cargo run <command>\n");
            println!("Commands:");
            println!("  parse                         -Parse from input.txt to output.txt");
            println!("  help                          -Show help");
            println!("  credits                       -Show credits");
        }
        "credits" => {
            println!("vehicle_parser");
            println!("by Artem Shylov");
        }
        _ => {
            eprintln!("Unknown command");
            println!("Run 'cargo run help' to see available commands.");
        }
    }

    Ok(())
}
