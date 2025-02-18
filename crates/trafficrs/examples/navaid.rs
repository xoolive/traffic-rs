use polars::prelude::*;
use std::path::Path;
use trafficrs::data::eurocontrol::aixm::navaid::parse_navaid_zip_file;

fn main() {
    let path =
        Path::new("/Users/xo/Documents/data/AIRAC_2413/Navaid.BASELINE.zip");
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
                println!("{:?}", df);
            }
        }
        Err(e) => eprintln!("Error parsing navaid file: {}", e),
    }
}
