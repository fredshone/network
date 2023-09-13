use std::path::PathBuf;

use network::Links;

pub fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("fixtures/network_big.xml");
    let links = Links::from_xml(path, false).unwrap();
    println!("{}", links.lengths.len());
}