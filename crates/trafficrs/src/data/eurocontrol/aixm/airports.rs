use quick_xml::name::QName;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zip::read::ZipArchive;

use super::{find_node, read_text};

#[derive(Debug, Serialize, Deserialize)]
pub struct Airport {
    pub identifier: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub iata: Option<String>,
    pub icao: String,
    pub name: String,
    pub city: Option<String>,
    pub r#type: String,
}

pub fn parse_airport_zip_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Airport>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut airports = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().ends_with(".BASELINE") {
            let mut reader = Reader::from_reader(BufReader::new(file));
            while let Ok(_node) =
                find_node(&mut reader, vec![QName(b"adrmsg:hasMember")], None)
            {
                airports.push(parse_airport(&mut reader)?);
            }
        }
    }
    Ok(airports)
}

fn parse_airport<R: std::io::BufRead>(
    reader: &mut Reader<R>,
) -> Result<Airport, Box<dyn std::error::Error>> {
    let mut airport = Airport {
        identifier: String::new(),
        latitude: 0.0,
        longitude: 0.0,
        altitude: 0.0,
        iata: None,
        icao: String::new(),
        name: String::new(),
        city: None,
        r#type: String::new(),
    };

    while let Ok(node) = find_node(
        reader,
        vec![
            QName(b"gml:identifier"),
            QName(b"aixm:locationIndicatorICAO"),
            QName(b"aixm:designatorIATA"),
            QName(b"aixm:name"),
            QName(b"aixm:servedCity"),
            QName(b"aixm:controlType"),
            QName(b"aixm:ARP"),
        ],
        Some(QName(b"adrmsg:hasMember")),
    ) {
        match node {
            QName(b"gml:identifier") => {
                airport.identifier = read_text(reader, node)?;
            }
            QName(b"aixm:locationIndicatorICAO") => {
                airport.icao = read_text(reader, node)?;
            }
            QName(b"aixm:designatorIATA") => {
                airport.iata = Some(read_text(reader, node)?);
            }
            QName(b"aixm:name") => {
                airport.name = read_text(reader, node)?;
            }

            QName(b"aixm:servedCity") => {
                find_node(reader, vec![QName(b"aixm:City")], Some(node))?;
                find_node(
                    reader,
                    vec![QName(b"aixm:name")],
                    Some(QName(b"aixm:City")),
                )?;
                airport.city = Some(read_text(reader, QName(b"aixm:name"))?);
            }
            QName(b"aixm:controlType") => {
                airport.r#type = read_text(reader, node)?;
            }
            QName(b"aixm:ARP") => {
                find_node(
                    reader,
                    vec![QName(b"aixm:ElevatedPoint")],
                    Some(node),
                )?;

                while let Ok(node) = find_node(
                    reader,
                    vec![QName(b"gml:pos"), QName(b"aixm:elevation")],
                    Some(QName(b"aixm:ElevatedPoint")),
                ) {
                    match node {
                        QName(b"gml:pos") => {
                            let coords: Vec<f64> = read_text(reader, node)?
                                .split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
                            airport.latitude = coords[0];
                            airport.longitude = coords[1];
                        }
                        QName(b"aixm:elevation") => {
                            airport.altitude =
                                read_text(reader, node)?.parse()?;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    Ok(airport)
}
