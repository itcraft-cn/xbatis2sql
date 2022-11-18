use lazy_static::*;
use log::*;
use regex::Regex;
use std::*;

use super::abt_parser::Parser;

use xml::reader::{EventReader, XmlEvent};

const PARSER: IBatisParser = IBatisParser {};

pub fn parse(output_dir: &String, files: &Vec<String>) {
    PARSER.parse(output_dir, files);
}

struct IBatisParser {}

impl Parser for IBatisParser {
    fn detect_match(&self, file: &String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("DTD SQL Map 2\\.0").unwrap();
        }
        return self.detect_match_with_regex(file, &RE);
    }

    fn read_and_parse(&self, file: &String, sql_store: &Vec<String>) {
        read_xml(file, sql_store);
    }
}

fn read_xml(filename: &String, sql_store: &Vec<String>) {
    let file = fs::File::open(filename).unwrap();
    let buf = io::BufReader::new(file);

    let parser = EventReader::new(buf);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                info!("+{}", name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                info!("-{}", name);
            }
            Err(e) => {
                info!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    for sql in sql_store {
        info!("{:?}", sql);
    }
}
