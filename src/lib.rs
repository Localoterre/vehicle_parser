use pest::Parser;
use pest_derive::Parser;
use serde_json::{json, to_string_pretty, Value};
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct VehicleParser;

#[derive(Error, Debug)]
pub enum VehicleParseError {
    #[error("Pest parsing error: {0}")]
    PestError(#[from] Box<pest::error::Error<Rule>>),

    #[error("JSON formatting error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub fn parse_vehicle(input: &str) -> Result<Value, VehicleParseError> {
    let mut parsed = VehicleParser::parse(Rule::vehicle, input)
        .map_err(|e| VehicleParseError::PestError(Box::new(e)))?;

    let mut brand = "";
    let mut model = "";
    let mut year = "";
    let mut class = "";
    let mut volume = "";
    let mut power = "";
    let mut engine_type = "";
    let mut dimensions = ("", "", "");
    let mut max_speed = "";
    let mut acceleration = "";

    for record in parsed.next().unwrap().into_inner() {
        match record.as_rule() {
            Rule::brand => brand = record.into_inner().as_str(),
            Rule::model => model = record.into_inner().as_str(),
            Rule::year => year = record.into_inner().as_str(),
            Rule::class => class = record.into_inner().as_str(),
            Rule::volume => volume = record.as_str(),
            Rule::power => power = record.as_str(),
            Rule::engine_type => engine_type = record.into_inner().as_str(),
            Rule::dimensions => {
                let mut dim_parts = record.into_inner();
                dimensions = (
                    dim_parts.next().unwrap().as_str(),
                    dim_parts.next().unwrap().as_str(),
                    dim_parts.next().unwrap().as_str(),
                );
            }
            Rule::max_speed => max_speed = record.into_inner().as_str(),
            Rule::acceleration => acceleration = record.into_inner().as_str(),
            Rule::engine => {
                let engine_parts = record.into_inner();
                for engine_record in engine_parts {
                    match engine_record.as_rule() {
                        Rule::volume => volume = engine_record.into_inner().as_str(),
                        Rule::power => power = engine_record.into_inner().as_str(),
                        Rule::engine_type => engine_type = engine_record.into_inner().as_str(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let vehicle_json = json!({
        "brand": brand,
        "model": model,
        "year": year,
        "class": class,
        "engine": {
            "volume": volume,
            "power": power,
            "type": engine_type,
        },
        "dimensions": {
            "length": dimensions.0,
            "width": dimensions.1,
            "height": dimensions.2,
        },
        "max_speed": max_speed,
        "acceleration": acceleration,
    });

    let pretty_json = to_string_pretty(&vehicle_json)?;

    println!("Parsed Vehicle JSON:\n{}", pretty_json);
    Ok(vehicle_json)
}
