#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;
use std::io::{self, Read};

use serde_xml_rs::deserialize;

// <Project name="my_project">
//     <Item name="hello" source="world.rs" />
// </Project>
#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

fn main() {
    let mut reader = io::stdin();
    let mut xml = String::new();
    match reader.read_to_string(&mut xml) {
        Ok(_) => {
            let project: Project = deserialize(xml.as_bytes()).unwrap();
            println!("{:#?}", project);
        },
        Err(_) => {}
    }
}
