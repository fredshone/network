use std::{collections::HashMap, path::Path, str::{self, Utf8Error}, string::FromUtf8Error, num::{ParseFloatError, ParseIntError}, ops::Deref};
use quick_xml::events::attributes::Attribute;
use thiserror::Error;

mod reader;
use reader::{XMLReader, XMLReaderError};
mod index;
use index::Indexer;

pub type Key = String;
pub type Value = f32;
pub type LinkLengths = Vec<Value>;

#[derive(Debug, Default)]
pub struct Links {
    pub lengths: LinkLengths,
    pub indexer: Indexer,
}

impl Deref for Links {
    type Target = LinkLengths;

    fn deref(&self) -> &Self::Target {
        &self.lengths
    }
}

impl Links {

    pub fn get(&self, index: usize) -> Value {
        self.lengths[index]
    }

    pub fn from_xml(path: impl AsRef<Path>, compressed: bool) -> Result<Self, LinkParserError> {
        let reader = reader::from_path(path, compressed)?;
        Self::from_reader(reader)
    }

    pub fn from_reader(mut reader: XMLReader) -> Result<Self, LinkParserError> {
        let mut lengths = LinkLengths::new();
        let mut indexer = Indexer::new();
        
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(& mut buf)? {
                quick_xml::events::Event::Start(e) => {
                    if b"link" == e.name().as_ref() {
                        let mut attributes = e.attributes();
                        let mut attributes = attributes.with_checks(false).flatten();

                        // get id (we know it will be first)
                        let id_att = attributes.next().ok_or(LinkParserError::LinkIdReadFailure())?.value;
                        let key = Key::from_utf8(id_att.into_owned())?;
                        indexer.add(key);

                        // get length (this could be anywhere later)
                        let length = attributes.find_map( Self::parse_length )
                        .ok_or(LinkParserError::FindLengthError())??;

                        // add to hashmap
                        lengths.push(length);
                }
            },
                quick_xml::events::Event::Eof => break,
                _ => (),
            }
        }
        Ok(Links {
            lengths,
            indexer,
        })
    }

    fn parse_length(a: Attribute) -> Option<Result<Value, LinkParserError>> {
        match a.key.into_inner() {
            b"length" => Some(Self::parse_length_value(&a.value)),
            _ => None
        } 
    }

    fn parse_length_value(v: &[u8]) -> Result<Value, LinkParserError> {
        let l = str::from_utf8(v)?
        .parse::<f32>()?;
        Ok(l as Value)
    }
    
}


/// An error that can occur when parsing.
#[derive(Debug, Error)]
pub enum LinkParserError {
    #[error("An error occurred trying to parse link id")]
    LinkIdReadFailure(),
    #[error("An error occurred trying to parse utf8 to String")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("An error occurred trying to parse utf8 to str")]
    Utf8Error(#[from] Utf8Error),
    #[error("An error occurred trying to parse utf8 to str")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Failed to find link lenth attribute")]
    ParseIntError(#[from] ParseIntError),
    #[error("Failed to find link lenth attribute")]
    FindLengthError(),
    #[error(transparent)]
    XMLReaderError(#[from] XMLReaderError),
    #[error("An error occurred whilst parsing the XML contents")]
    XmlParse(#[from] quick_xml::Error),
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Test network
    #[test]
    fn parse_test_network_xml() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/network_test.xml");
        let links = Links::from_xml(path, false).unwrap();
        assert_eq!(links.lengths.len(), 2);
        // assert_eq!(links.lengths.get("linkE"), Some(&100.5));
        // assert_eq!(links.lengths.get("linkW"), Some(&102.));
        assert_eq!(links.get(0), 100.5);
        assert_eq!(links.get(1), 102.);
    }
    
    #[test]
    fn parse_test_network_zipped() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/network_test.xml.gz");
        let links = Links::from_xml(path, true).unwrap();
        assert_eq!(links.lengths.len(), 2);
        // assert_eq!(links.lengths.get("linkE"), Some(&100.5));
        // assert_eq!(links.lengths.get("linkW"), Some(&102.));
        assert_eq!(links.get(0), 100.5);
        assert_eq!(links.get(1), 102.);
    }

    // // Small network
    // #[test]
    // fn parse_small_network_xml() {
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("fixtures/network_small.xml");
    //     let links = Links::from_xml(path, false).unwrap();
    //     assert_eq!(links.lengths.len(), 8);
    //     assert_eq!(links.lengths.get("1-2"), Some(&1000.));
    //     assert_eq!(links.lengths.get("1-5"), Some(&20000.));
    // }
    
    // #[test]
    // fn parse_small_network_zipped() {
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("fixtures/network_small.xml.gz");
    //     let links = Links::from_xml(path, true).unwrap();
    //     assert_eq!(links.lengths.len(), 8);
    //     assert_eq!(links.lengths.get("1-2"), Some(&1000.));
    //     assert_eq!(links.lengths.get("1-5"), Some(&20000.));
    // }

    // #[test]
    // fn parse_big_network_xml() {
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("fixtures/network_big.xml");
    //     let links = Links::from_xml(path, false).unwrap();
    //     assert_eq!(links.lengths.len(), 15899);
    //     assert_eq!(links.lengths.get("5221366235007855731_5221366234271588645"), Some(&36.999683));
    // }

}
