use polars::prelude::*;
use std::path::Path;
use trafficrs::data::eurocontrol::aixm::designated_point::parse_designated_point_zip_file;

fn main() {
    let path = Path::new(
        "/Users/xo/Documents/data/AIRAC_2413/DesignatedPoint.BASELINE.zip",
    );
    match parse_designated_point_zip_file(path) {
        Ok(point) => {
            if let Ok(df) = df!(
                "identifier" => point.values().map(|point| point.identifier.clone()).collect::<Vec<_>>(),
                "designator" => point.values().map(|point| point.designator.clone()).collect::<Vec<_>>(),
                "name" => point.values().map(|point| point.name.clone()).collect::<Vec<_>>(),
                "latitude" => point.values().map(|point| point.latitude).collect::<Vec<_>>(),
                "longitude" => point.values().map(|point| point.longitude).collect::<Vec<_>>(),
                "type" => point.values().map(|point| point.r#type.clone()).collect::<Vec<_>>(),
            ) {
                println!("{:?}", df);
            }
        }
        Err(e) => eprintln!("Error parsing designated point file: {}", e),
    }
}
