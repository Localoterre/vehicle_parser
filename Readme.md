# Car Characteristics Parser

## Overview
This parser is designed to extract and interpret technical characteristics of cars from structured text input using the Rust pest library. It parses key attributes related to a car's make, model, manufacturing year, class, engine specifications, dimensions, maximum speed, and acceleration. These characteristics are parsed into discrete elements, allowing the data to be programmatically accessed and analyzed for further processing.

## Grammar Rules
The grammar rules define how each characteristic should be recognized in the input text, ensuring a structured and consistent parsing process. Below is a description of each rule, which specifies both the attribute and its expected format:

1. car
The main rule that defines a valid car entry, encompassing each characteristic separated by commas.

2. brand
Matches the car’s brand with the format:

Pattern: Brand: [name]
Example: Brand: Toyota

3. model
Matches the car’s model with the format:

Pattern: Model: [name]
Example: Model: Corolla

4. year
Matches the car’s manufacturing year with the format:

Pattern: Year: [4-digit number]
Example: Year: 2021

5. class
Matches the car’s class with the format:

Pattern: Class: [name]
Example: Class: Sedan

6. engine
Matches the engine specifications with sub-rules for volume, power, and type, encapsulated within parentheses:

Pattern: Engine: (Volume: [number]L, Power: [number]kW, Type: [engine type])
Example: Engine: (Volume: 2L, Power: 150kW, Type: Petrol)

Sub-rules within engine:

volume: Matches engine volume (e.g., Volume: 2L).
power: Matches engine power output (e.g., Power: 150kW).
engine_type_inner: Recognizes engine types, allowing only Petrol, Diesel, or Electric.

7. dimensions
Matches the car’s dimensions in the format length x width x height:

Pattern: Dimensions: [number]x[number]x[number]mm
Example: Dimensions: 4500x1800x1400mm

8. max_speed
Matches the car’s maximum speed in km/h:

Pattern: Max speed: [number]km/h
Example: Max speed: 220km/h

9. acceleration
Matches the car’s acceleration time from 0 to 100 km/h in seconds:

Pattern: Acceleration: [number]s to 100 km/h
Example: Acceleration: 7.5s to 100 km/h

Utility Rules

WHITESPACE: Handles whitespace, tabs, and newlines.
name: Matches alphanumeric strings for flexible input of car brands, models, and classes.
NUMBER: Matches digit-only strings for years, speeds, and measurements.
Parsing Process
The parser processes a block of text containing structured car characteristics and matches each characteristic to the predefined grammar rules. Each field is extracted based on strict patterns that prevent mismatched data, ensuring each parsed attribute corresponds to a specific car feature.

```pest
WHITESPACE = _{ " " | "\t" | NEWLINE }

name = @{ ASCII_ALPHANUMERIC+ }
number = @{ (ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+) | ASCII_DIGIT+ }

vehicle = ${ brand ~  ", " ~ model ~  ", " ~ year ~  ", " ~ class ~  ", " ~ engine ~  ", " ~ dimensions ~  ", " ~ max_speed ~  ", " ~ acceleration }

brand = ${ "Brand:" ~ WHITESPACE ~ name }
model = ${ "Model:" ~ WHITESPACE ~ name }
year = ${ "Year:" ~ WHITESPACE ~ number }
class = ${ "Class:" ~ WHITESPACE ~ name }

engine = ${ "Engine: (" ~ volume ~  ", " ~ power ~  ", " ~ engine_type ~ ")" }
volume = ${ "Volume:" ~ WHITESPACE ~ number ~ "L" }
power = ${ "Power:" ~ WHITESPACE ~ number ~ "kW" }
engine_type = ${ "Type:" ~ WHITESPACE ~ engine_type_inner }
engine_type_inner = @{("Petrol" | "Diesel" | "Electric")}

dimensions = ${ "Dimensions:" ~ WHITESPACE ~ number ~ "x" ~ number ~ "x" ~ number ~ "mm" }

max_speed = ${ "Max speed:" ~ WHITESPACE ~ number ~ "km/h" }
acceleration = ${ "Acceleration:" ~ WHITESPACE ~ number ~ "s to 100 km/h" }

```

The output of this parsing process is a structured data object containing individual attributes of the car, which can be further analyzed, stored in a database, or used in other applications, such as comparison engines or technical databases.

## Usage
This parser is intended for technical applications where structured car data is required. To use the parser:

Create an input string in the correct format.
Pass the input string to the parser using Rust's pest framework.
Retrieve parsed data from the resulting structured object.