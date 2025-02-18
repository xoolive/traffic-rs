use quick_xml::{events::Event, name::QName, Reader};

pub mod airport_heliport;
pub mod designated_point;
pub mod navaid;

fn find_node<'a, R: std::io::BufRead>(
    reader: &mut Reader<R>,
    lookup: Vec<QName<'a>>,
    end: Option<QName>,
) -> Result<QName<'a>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                for elt in lookup.iter() {
                    if e.name() == *elt {
                        return Ok(*elt);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if let Some(end) = end {
                    if e.name() == end {
                        break;
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
        buf.clear();
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Node not found",
    )))
}

fn read_text<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    end: QName,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => text.push_str(&e.unescape()?),
            Ok(Event::End(e)) if e.name() == end => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
        buf.clear();
    }
    Ok(text)
}
