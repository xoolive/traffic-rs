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
pub struct Route {
    pub identifier: String,
    pub prefix: Option<String>,
    pub second_letter: Option<String>,
    pub number: Option<String>,
    pub multiple_identifier: Option<String>,
    pub begin_position: Option<String>,
    pub end_position: Option<String>,
}

pub fn parse_route_zip_file<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Route>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut routes = HashMap::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().ends_with(".BASELINE") {
            let mut reader = Reader::from_reader(BufReader::new(file));

            while let Ok(_node) = find_node(&mut reader, vec![QName(b"aixm:Route")], None) {
                let route = parse_route(&mut reader)?;
                routes.insert(route.identifier.clone(), route);
            }
        }
    }

    Ok(routes)
}

fn parse_route<R: std::io::BufRead>(reader: &mut Reader<R>) -> Result<Route, Box<dyn std::error::Error>> {
    let mut route = Route::default();

    while let Ok(node) = find_node(
        reader,
        vec![
            QName(b"gml:identifier"),
            QName(b"aixm:designatorPrefix"),
            QName(b"aixm:designatorSecondLetter"),
            QName(b"aixm:designatorNumber"),
            QName(b"aixm:multipleIdentifier"),
            QName(b"gml:beginPosition"),
            QName(b"gml:endPosition"),
        ],
        Some(QName(b"aixm:Route")),
    ) {
        match node {
            QName(b"gml:identifier") => {
                route.identifier = read_text(reader, node)?;
            }
            QName(b"aixm:designatorPrefix") => {
                route.prefix = Some(read_text(reader, node)?);
            }
            QName(b"aixm:designatorSecondLetter") => {
                route.second_letter = Some(read_text(reader, node)?);
            }
            QName(b"aixm:designatorNumber") => {
                route.number = Some(read_text(reader, node)?);
            }
            QName(b"aixm:multipleIdentifier") => {
                route.multiple_identifier = Some(read_text(reader, node)?);
            }
            QName(b"gml:beginPosition") => {
                route.begin_position = Some(read_text(reader, node)?);
            }
            QName(b"gml:endPosition") => {
                route.end_position = Some(read_text(reader, node)?);
            }
            _ => (),
        }
    }
    Ok(route)
}
