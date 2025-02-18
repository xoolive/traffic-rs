use polars::prelude::*;
use std::path::Path;
use trafficrs::data::eurocontrol::aixm::airport_heliport::parse_airport_heliport_zip_file;

fn main() {
    let path = Path::new(
        "/Users/xo/Documents/data/AIRAC_2413/AirportHeliport.BASELINE.zip",
    );
    match parse_airport_heliport_zip_file(path) {
        Ok(airports) => {
            if let Ok(df) = df!(
                "identifier" => airports.values().map(|airport| airport.identifier.clone()).collect::<Vec<_>>(),
                "icao" => airports.values().map(|airport| airport.icao.clone()).collect::<Vec<_>>(),
                "iata" => airports.values().map(|airport| airport.iata.clone()).collect::<Vec<_>>(),
                "name" => airports.values().map(|airport| airport.name.clone()).collect::<Vec<_>>(),
                "latitude" => airports.values().map(|airport| airport.latitude).collect::<Vec<_>>(),
                "longitude" => airports.values().map(|airport| airport.longitude).collect::<Vec<_>>(),
                "altitude" => airports.values().map(|airport| airport.altitude).collect::<Vec<_>>(),
                "city" => airports.values().map(|airport| airport.city.clone()).collect::<Vec<_>>(),
                "type" => airports.values().map(|airport| airport.r#type.clone()).collect::<Vec<_>>(),
            ) {
                println!("{:?}", df);
            }
        }
        Err(e) => eprintln!("Error parsing airport file: {}", e),
    }
}
