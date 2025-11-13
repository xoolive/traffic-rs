use polars::prelude::*;
use std::env;
use std::path::Path;
use trafficrs::data::eurocontrol::aixm::route_segment::parse_route_segment_zip_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let path = path.join("RouteSegment.BASELINE.zip");

    match parse_route_segment_zip_file(path) {
        Ok(route_segments) => {
            if let Ok(df) = df!(
                "identifier" => route_segments.values().map(|segment| segment.identifier.clone()).collect::<Vec<_>>(),
                "begin_position" => route_segments.values().map(|segment| segment.begin_position.clone()).collect::<Vec<_>>(),
                "end_position" => route_segments.values().map(|segment| segment.end_position.clone()).collect::<Vec<_>>(),
                "lower_limit" => route_segments.values().map(|segment| segment.lower_limit.clone()).collect::<Vec<_>>(),
                "upper_limit" => route_segments.values().map(|segment| segment.upper_limit.clone()).collect::<Vec<_>>(),
                "route_formed" => route_segments.values().map(|segment| segment.route_formed.clone()).collect::<Vec<_>>(),
                "start_designated_point" => route_segments.values().map(|segment| segment.start_designated_point.clone()).collect::<Vec<_>>(),
                "end_designated_point" => route_segments.values().map(|segment| segment.end_designated_point.clone()).collect::<Vec<_>>(),
                "start_navaid" => route_segments.values().map(|segment| segment.start_navaid.clone()).collect::<Vec<_>>(),
                "end_navaid" => route_segments.values().map(|segment| segment.end_navaid.clone()).collect::<Vec<_>>(),
                "direction" => route_segments.values().map(|segment| segment.direction.clone()).collect::<Vec<_>>(),
            ) {
                println!("{df:?}");
            }
        }
        Err(e) => eprintln!("Error parsing route segment file: {e}"),
    }
}
