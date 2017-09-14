#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;
use std::io::{self, Read};
use serde_xml_rs::deserialize;


#[derive(Debug, Deserialize)]
pub struct Sequence {
    name: String,
    #[serde(default)]
    number: String,
    #[serde(rename = "xml:lang", default)]
    lang: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
     #[serde(rename = "first-name", default)]
     first_name: String,
     #[serde(rename = "middle-name", default)]
     middle_name: String,
     #[serde(rename = "last-name", default)]
     last_name: String,
     #[serde(rename = "nickname", default)]
     nick_name: String,
     #[serde(rename = "home-page", default)]
     home_page: String,
     #[serde(rename = "email", default)]
     email: String,
}

#[derive(Debug, Deserialize)]
pub struct TitleInfo {
//     genre: Vec<Genre>,
    #[serde(rename = "author", default)]
    author: Vec<Author>,
    #[serde(rename = "book-title", default)]
    book_title: String,
//     // annotation: OptionalAnnotation,
//     // keywords
//     // date
//     //  coverpage
    #[serde(rename = "lang", default)]
    lang: String,
    #[serde(rename = "src-lang", default)]
    src_lang: String,
    #[serde(rename = "translator", default)]
    translator: Vec<Author>,
    #[serde(rename = "sequence", default)]
    sequence: Vec<Sequence>,
}

#[derive(Debug, Deserialize)]
pub struct Description {
    #[serde(rename = "title-info")]
    title_info: TitleInfo,
    //src_title_info: OptionalTitleInfo,
    //document_info: DocumentInfo,
    //publish_info: Vec<PublishInfo>,
}

#[derive(Debug, Deserialize)]
pub struct FictionBook {
    #[serde(rename = "description")]
    description: Description,
}

fn main() {
    let mut reader = io::stdin();
    let mut xml = String::new();
    match reader.read_to_string(&mut xml) {
        Ok(_) => {
            let obj: FictionBook = deserialize(xml.as_bytes()).unwrap();
            println!("{:#?}", obj);
        },
        Err(_) => {}
    }
}
