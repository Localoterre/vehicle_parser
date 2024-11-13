use anyhow::Result;
use pest::Parser;
use vehicle_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() -> Result<()> {
        let input = " \t\n";
        assert!(VehicleParser::parse(Rule::WHITESPACE, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_name() -> Result<()> {
        let input = "Toyota";
        assert!(VehicleParser::parse(Rule::name, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let input = "30";
        assert!(VehicleParser::parse(Rule::number, input).is_ok());

        let input = "30.5";
        assert!(VehicleParser::parse(Rule::number, input).is_ok());

        Ok(())
    }

    #[test]
    fn test_brand() -> Result<()> {
        let input = "Brand: Toyota";
        assert!(VehicleParser::parse(Rule::brand, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_model() -> Result<()> {
        let input = "Model: Corolla";
        assert!(VehicleParser::parse(Rule::model, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_year() -> Result<()> {
        let input = "Year: 2021";
        assert!(VehicleParser::parse(Rule::year, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_class() -> Result<()> {
        let input = "Class: Sedan";
        assert!(VehicleParser::parse(Rule::class, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_engine() -> Result<()> {
        let input = "Engine: (Volume: 2L, Power: 150kW, Type: Petrol)";
        assert!(VehicleParser::parse(Rule::engine, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_volume() -> Result<()> {
        let input = "Volume: 2L";
        assert!(VehicleParser::parse(Rule::volume, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_power() -> Result<()> {
        let input = "Power: 150kW";
        assert!(VehicleParser::parse(Rule::power, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_engine_type() -> Result<()> {
        let input = "Type: Petrol";
        assert!(VehicleParser::parse(Rule::engine_type, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_engine_type_inner() -> Result<()> {
        let input = "Petrol";
        assert!(VehicleParser::parse(Rule::engine_type_inner, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_dimensions() -> Result<()> {
        let input = "Dimensions: 4500x1800x1400mm";
        assert!(VehicleParser::parse(Rule::dimensions, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_max_speed() -> Result<()> {
        let input = "Max speed: 220km/h";
        assert!(VehicleParser::parse(Rule::max_speed, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_acceleration() -> Result<()> {
        let input = "Acceleration: 7.5s to 100 km/h";
        assert!(VehicleParser::parse(Rule::acceleration, input).is_ok());
        Ok(())
    }

    #[test]
    fn test_vehicle() -> Result<()> {
        let input = "Brand: Toyota, Model: Corolla, Year: 2021, Class: Sedan, Engine: (Volume: 2L, Power: 150kW, Type: Petrol), Dimensions: 4500x1800x1400mm, Max speed: 220km/h, Acceleration: 7.5s to 100 km/h";
        assert!(VehicleParser::parse(Rule::vehicle, input).is_ok());
        Ok(())
    }
}
