use quick_xml::name::QName;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zip::read::ZipArchive;

use super::{find_node, read_text};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteSegment {
    pub identifier: String,
    pub begin_position: Option<String>,
    pub end_position: Option<String>,
    pub lower_limit: Option<String>,
    pub upper_limit: Option<String>,
    pub route_formed: Option<String>,
    pub start_designated_point: Option<String>,
    pub end_designated_point: Option<String>,
    pub start_navaid: Option<String>,
    pub end_navaid: Option<String>,
    pub direction: Option<String>,
}

pub fn parse_route_segment_zip_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, RouteSegment>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut route_segments = HashMap::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().ends_with(".BASELINE") {
            let mut reader = Reader::from_reader(BufReader::new(file));

            while let Ok(_node) = find_node(&mut reader, vec![QName(b"aixm:RouteSegment")], None) {
                let route_segment = parse_route_segment(&mut reader)?;
                route_segments.insert(route_segment.identifier.clone(), route_segment);
            }
        }
    }

    Ok(route_segments)
}

fn parse_route_segment<R: std::io::BufRead>(
    reader: &mut Reader<R>,
) -> Result<RouteSegment, Box<dyn std::error::Error>> {
    let mut segment = RouteSegment::default();

    while let Ok(node) = find_node(
        reader,
        vec![
            QName(b"gml:identifier"),
            QName(b"gml:beginPosition"),
            QName(b"gml:endPosition"),
            QName(b"aixm:lowerLimit"),
            QName(b"aixm:upperLimit"),
            QName(b"aixm:routeFormed"),
            QName(b"aixm:start"),
            QName(b"aixm:end"),
            QName(b"aixm:direction"),
        ],
        Some(QName(b"aixm:RouteSegment")),
    ) {
        match node {
            QName(b"gml:identifier") => {
                segment.identifier = read_text(reader, node)?;
            }
            QName(b"aixm:beginPosition") => {
                segment.begin_position = Some(read_text(reader, node)?);
            }
            QName(b"aixm:endPosition") => {
                segment.end_position = Some(read_text(reader, node)?);
            }
            QName(b"aixm:lowerLimit") => {
                segment.lower_limit = Some(read_text(reader, node)?);
            }
            QName(b"aixm:upperLimit") => {
                segment.upper_limit = Some(read_text(reader, node)?);
            }
            QName(b"aixm:routeFormed") => {
                segment.route_formed = Some(read_text(reader, node)?);
            }
            QName(b"aixm:direction") => {
                // TODO that's wrong for the moment
                segment.direction = Some(read_text(reader, node)?);
            }
            QName(b"aixm:start") => {}
            QName(b"aixm:end") => {}
            _ => (),
        }
    }
    Ok(segment)
}
