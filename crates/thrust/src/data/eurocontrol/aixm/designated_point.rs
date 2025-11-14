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
pub struct DesignatedPoint {
    pub identifier: String,
    pub latitude: f64,
    pub longitude: f64,
    pub designator: String,
    pub name: Option<String>,
    pub r#type: String,
}

pub fn parse_designated_point_zip_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, DesignatedPoint>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut points = HashMap::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().ends_with(".BASELINE") {
            let mut reader = Reader::from_reader(BufReader::new(file));

            while let Ok(_nome) = find_node(
                &mut reader,
                vec![QName(b"aixm:DesignatedPoint")],
                None,
            ) {
                let point = parse_designated_point(&mut reader)?;
                points.insert(point.identifier.clone(), point);
            }
        }
    }

    Ok(points)
}

fn parse_designated_point<R: std::io::BufRead>(
    reader: &mut Reader<R>,
) -> Result<DesignatedPoint, Box<dyn std::error::Error>> {
    let mut point = DesignatedPoint::default();

    while let Ok(node) = find_node(
        reader,
        vec![
            QName(b"gml:identifier"),
            QName(b"aixm:name"),
            QName(b"aixm:designator"),
            QName(b"aixm:type"),
            QName(b"aixm:Point"),
        ],
        Some(QName(b"aixm:DesignatedPoint")),
    ) {
        match node {
            QName(b"gml:identifier") => {
                point.identifier = read_text(reader, node)?;
            }
            QName(b"aixm:name") => {
                point.name = Some(read_text(reader, node)?);
            }
            QName(b"aixm:designator") => {
                point.designator = read_text(reader, node)?;
            }
            QName(b"aixm:type") => {
                point.r#type = read_text(reader, node)?;
            }
            QName(b"aixm:Point") => {
                while let Ok(node) =
                    find_node(reader, vec![QName(b"gml:pos")], Some(node))
                {
                    let coords: Vec<f64> = read_text(reader, node)?
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    point.latitude = coords[0];
                    point.longitude = coords[1];
                }
            }
            _ => (),
        }
    }

    Ok(point)
}
