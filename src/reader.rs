use std::{path::Path, io::{BufRead, BufReader, self}, fs::File};
use anyhow::{Context, Result};
use flate2::bufread::GzDecoder;
use quick_xml::events::attributes::AttrError;
use thiserror::Error;

pub type XMLReader = quick_xml::Reader<Box<dyn BufRead>>;

/// Create a new XML file parser for the given file path.
pub fn from_path(path: impl AsRef<Path>, compressed: bool) -> Result<XMLReader, XMLReaderError> {
    let file = File::open(&path).context("Could not open file")?;
    let reader = BufReader::new(file);

    let mut xml_reader = if compressed {
        let gz_decoder = GzDecoder::new(reader);
        let reader: Box<dyn BufRead> = Box::new(BufReader::new(gz_decoder));
        quick_xml::Reader::from_reader(reader)
    } else {
        let reader: Box<dyn BufRead> = Box::new(reader);
        quick_xml::Reader::from_reader(reader)
    };

    // Disable checking to increase performance - we already know the file (should) be well-formed
    xml_reader.check_comments(false).check_end_names(false);

    Ok(xml_reader)
}

/// An error that can occur when processing XML event files.
// todo: split out parser specific errors later
#[derive(Debug, Error)]
pub enum XMLReaderError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("An error occurred whilst parsing the XML contents")]
    XmlParse(#[from] quick_xml::Error),
    #[error("An error occurred whilst reading an XML event element")]
    InvalidEvent(#[from] anyhow::Error),
    #[error(transparent)]
    AttrError(#[from] AttrError),
}