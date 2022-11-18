use lazy_static::*;
use log::*;
use regex::Regex;
use std::*;

use super::abt_parser::Parser;

use xml::reader::{EventReader, XmlEvent};

use rstring_builder::StringBuilder;

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

    fn read_and_parse(&self, file: &String, sql_store: &mut Vec<String>) {
        read_xml(file, sql_store);
    }
}

fn read_xml(filename: &String, sql_store: &mut Vec<String>) {
    let file = fs::File::open(filename).unwrap();
    let buf = io::BufReader::new(file);
    let parser = EventReader::new(buf);
    let mut in_statement: bool = false;
    let mut builder = StringBuilder::new();
    sql_store.push("-- ".to_string() + filename);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let element_name = name.local_name.as_str().to_ascii_lowercase();
                if element_name == "select"
                    || element_name == "insert"
                    || element_name == "update"
                    || element_name == "delete"
                    || element_name == "statement"
                {
                    in_statement = true;
                    for attr in attributes {
                        if attr.name.local_name.as_str() == "id" {
                            sql_store.push("-- ".to_string() + attr.value.as_str());
                        }
                    }
                } else if in_statement && element_name == "where" {
                    builder.append("where ");
                } else if in_statement {
                    for attr in attributes {
                        if attr.name.local_name.as_str() == "prepend" {
                            builder.append(attr.value.as_str());
                        }
                    }
                } else {
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                let element_name = name.local_name.as_str().to_ascii_lowercase();
                if element_name == "select"
                    || element_name == "insert"
                    || element_name == "update"
                    || element_name == "delete"
                    || element_name == "statement"
                {
                    clear_and_push(&mut builder, sql_store);
                    in_statement = false;
                } else if in_statement && (element_name == "include") {
                } else if in_statement {
                } else if element_name == "sql" {
                }
            }
            Ok(XmlEvent::CData(s)) => {
                if in_statement {
                    builder.append(s);
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                if in_statement {
                    builder.append(s);
                }
            }
            Err(e) => {
                info!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn clear_and_push(builder: &mut StringBuilder, sql_store: &mut Vec<String>) {
    lazy_static! {
        static ref RE0: Regex = Regex::new("[\n\t ]+").unwrap();
        static ref RE1: Regex = Regex::new("#[^#{]+#").unwrap();
        static ref RE2: Regex = Regex::new("\\$[^${]+\\$").unwrap();
        static ref RE3: Regex = Regex::new("\\$\\{[^${]+\\}").unwrap();
        static ref RE_FIX1: Regex = Regex::new("WHERE[ ]+AND").unwrap();
        static ref RE_FIX2: Regex = Regex::new("WHERE[ ]+OR").unwrap();
    }
    let mut sql = builder.to_string().trim().to_ascii_uppercase();
    sql = RE0.replace_all(sql.as_str(), " ").to_string();
    sql = RE1.replace_all(sql.as_str(), ":?").to_string();
    sql = RE2.replace_all(sql.as_str(), ":?").to_string();
    sql = RE3
        .replace_all(sql.as_str(), "__REPLACE_SCHEMA__")
        .to_string();
    sql = RE_FIX1.replace_all(sql.as_str(), "WHERE").to_string();
    sql = RE_FIX2.replace_all(sql.as_str(), "WHERE").to_string();
    sql_store.push(sql + ";");
    builder.clear();
}
