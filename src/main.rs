#[macro_use]

extern crate serde_derive;
extern crate serde_xml_rs;
use std::io::{self, Read};

mod serde {
    pub use serde_xml_rs::deserialize;
    pub use serde_xml_rs::Error;

    //  Элемент <sequence>
    // Атрибуты
    //     name (обязательный) - название серии;
    //     number (опциональный) - номер книги в серии;
    //     xml:lang (опциональный) - язык.
    // Подчиненные элементы - НЕТ
    // Подчинен
    //     <title-info> - 0..n (любое число, опционально);
    //     <src-title-info> - 0..n (любое число, опционально);
    //     <publish-info> - 0..n (любое число, опционально).
    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Sequence {
        #[serde(rename = "name", default)]
        name: String,
        #[serde(rename = "number", default)]
        number: String,
        #[serde(rename = "xml:lang", default)]
        lang: String,
    }

    // Элемент <author>
    // http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_author&oldid=2752
    //  Атрибуты - НЕТ
    // Подчиненные элементы
    //     <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
    //     <middle-name> - 0..1 (один, опционально) - отчество;
    //     <last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
    //     <nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
    //     <home-page> - 0..n (любое число, опционально);
    //     <email> - 0..n (любое число, опционально);
    //     <id> - 0..1 (один, опционально) с версии 2.2 - идентификатор автора, присваивается библиотекой.
    // Подчинен
    //     <title-info> 1..n (любое число, один обязателен);
    //     <src-title-info> 1..n (любое число, один обязателен) с версии 2.1;
    //     <document-info> 1..n (любое число, один обязателен);
    #[derive(Debug, Deserialize, PartialEq)]
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

    // Элемент <genre>
    // http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_genre&oldid=2774
    // Example:
    // <genre>adv_western</genre>
    // <genre match="20">detective</genre>
    type Genre = String;

    // Элемент <title-info>
    // http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_title-info&oldid=2920
    //  Атрибуты - НЕТ
    // Подчиненные элементы
    //     <genre> - 1..n (любое число, один обязaтелен);
    //     <author> - 1..n (любое число, один обязaтелен);
    //     <book-title> - 1 (один, обязателен);
    //     <annotation> - 0..1 (один, опционально);
    //     <keywords> - 0..1 (один, опционально);
    //     <date> - 0..1 (один, опционально);
    //     <coverpage> - 0..1 (один, опционально);
    //     <lang> - 1 (один, обязателен);
    //     <src-lang> - 0..1 (один, опционально);
    //     <translator> - 0..n (любое число, опционально);
    //     <sequence> - 0..n (любое число, опционально).
    // Подчинен
    //     <description> - 1 (один, обязателен)


    #[derive(Debug, Deserialize, PartialEq)]
    pub struct TitleInfo {
        #[serde(rename = "genre", default)]
        genre: Vec<Genre>,
        #[serde(rename = "author", default)]
        author: Vec<Author>,
        #[serde(rename = "book-title", default)]
        book_title: String,
        #[serde(rename = "date", default)]
        date: String,
        #[serde(rename = "translator", default)]
        translator: Vec<Author>,
        #[serde(rename = "sequence", default)]
        sequence: Vec<Sequence>,
        #[serde(rename = "lang", default)]
        lang: String,
        #[serde(rename = "src-lang", default)]
        src_lang: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct PublishInfo {
        #[serde(rename = "book-name", default)]
        book_name: String,
        #[serde(rename = "publisher", default)]
        publisher: String,
        #[serde(rename = "city", default)]
        city: String,
        #[serde(rename = "year", default)]
        year: String,
        #[serde(rename = "isbn", default)]
        isbn: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct DocumentInfo {
        #[serde(rename = "author", default)]
        author: Vec<Author>,
        #[serde(rename = "program-used", default)]
        program_used: String,
        #[serde(rename = "date", default)]
        date: String,
        #[serde(rename = "src-url", default)]
        src_url: Vec<String>,
        #[serde(rename = "src-ocr", default)]
        src_ocr: String,
        #[serde(rename = "version", default)]
        version: String,
        #[serde(rename = "publisher", default)]
        publisher: Vec<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Description {
        #[serde(rename = "title-info")]
        title_info: TitleInfo,
        // #[serde(rename = "document-info")]      src_title_info: OptionalTitleInfo,
        #[serde(rename = "document-info")]
        document_info: DocumentInfo,
        #[serde(rename = "publish-info", default)]
        publish_info: Vec<PublishInfo>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct FictionBook {
        #[serde(rename = "description")]
        description: Description,
    }
}


pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

fn escape(xml: &String) -> String {
    if xml.find("&amp;").is_none() {
        if xml.find("&").is_some() {
            return xml.replace("&amp;", "&").replace("&", "&amp;");
        }
    }
    return xml.clone();
}

fn get_tag(content: &str, tag: &str) -> Option<String> {
    let beg = String::from("<") + tag + ">";
    let end = String::from("</") + tag + ">";
    if let Some(spos) = content.find(&beg) {
        if let Some(epos) = content.find(&end) {
            let needle: &str = &content[spos..epos + end.len()];
            return Some(String::from(needle));
        }
    }
    None
}

fn fix_double_tags(xml: &String, parent: &str, tag: &str) -> String {
    if let Some(content) = get_tag(&xml, parent) {
        if let Some(value) = get_tag(&xml, tag) {
            if let Some(first) = content.find(&value) {
                if let Some(last) = content.rfind(&value) {
                    if first != last {
                        return xml.replacen(&value, "", 1);
                    }
                }
            }
        }
    }
    return xml.clone();
}

fn fix_fb_tools_defect(xml: &String) -> String {
    if xml.find("<program-used>FB Tools</program-used>").is_some() {
        return fix_double_tags(&xml, "title-info", "lang");
    }
    return xml.clone();
}

use serde::FictionBook;
use serde::Error;
use serde::deserialize;

fn parse(xml: &String) -> Result<FictionBook, Error> {
    let mut fb = deserialize(xml.as_bytes());
    if fb.is_err() {
        let escaped: String = escape(xml);
        fb = deserialize(escaped.as_bytes());
        if fb.is_err() {
            let fb_tools_fixed = fix_fb_tools_defect(&escaped);
            fb = deserialize(fb_tools_fixed.as_bytes());
            if fb.is_err() {
                let double_tag_fixed = fix_double_tags(&fb_tools_fixed, "title-info", "last-name");
                fb = deserialize(double_tag_fixed.as_bytes());
            }
        }
    }
    return fb;
}

fn main() {
    let mut reader = io::stdin();
    let mut xml = String::new();
    match reader.read_to_string(&mut xml) {
        Ok(_) => {
            match parse(&xml) {
                Ok(fb) => println!("{:#?}", fb),
                Err(_) => println!("{}", xml),
            }
        }
        Err(_) => {}
    }
}
