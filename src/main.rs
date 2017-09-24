extern crate fb2parser;

use std::io::Read;

fn main() {
    let mut xml = String::new();
    match std::io::stdin().read_to_string(&mut xml) {
        Ok(_) => {
            match fb2parser::create(xml.clone()) {
                Ok(fb) => println!("{:#?}", fb),
                Err(err) => println!("ERRORCCURED: {}\n{}", err, xml),
            }
        }
        Err(_) => {}
    }
}
