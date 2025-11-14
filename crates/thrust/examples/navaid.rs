use polars::prelude::*;
use std::{env, path::Path};
use thrust::data::eurocontrol::aixm::navaid::parse_navaid_zip_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let path = path.join("Navaid.BASELINE.zip");

    match parse_navaid_zip_file(path) {
        Ok(navaids) => {
            if let Ok(df) = df!(
                "identifier" => navaids.values().map(|navaid| navaid.identifier.clone()).collect::<Vec<_>>(),
                "name" => navaids.values().map(|navaid| navaid.name.clone()).collect::<Vec<_>>(),
                "description" => navaids.values().map(|navaid| navaid.description.clone()).collect::<Vec<_>>(),
                "latitude" => navaids.values().map(|navaid| navaid.latitude).collect::<Vec<_>>(),
                "longitude" => navaids.values().map(|navaid| navaid.longitude).collect::<Vec<_>>(),
                "type" => navaids.values().map(|navaid| navaid.r#type.clone()).collect::<Vec<_>>(),
            ) {
                println!("{df:?}");
            }
        }
        Err(e) => eprintln!("Error parsing navaid file: {e}"),
    }
}
