use polars::prelude::*;
use std::{env, path::Path};
use trafficrs::data::eurocontrol::aixm::airport_heliport::parse_airport_heliport_zip_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let path = path.join("AirportHeliport.BASELINE.zip");

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
