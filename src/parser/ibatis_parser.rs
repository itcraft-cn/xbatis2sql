use super::abt_parser::Parser;
use super::parse_helper;
use lazy_static::*;
use log::*;
use regex::Regex;
use rstring_builder::StringBuilder;
use std::collections::HashMap;
use std::*;
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

    fn read_xml(&self, filename: &String, sql_store: &mut Vec<String>) {
        sql_store.push("-- ".to_string() + filename);
        let file = fs::File::open(filename).unwrap();
        let buf = io::BufReader::new(file);
        let parser = EventReader::new(buf);
        let mut in_statement: bool = false;
        let mut in_sql: bool = false;
        let mut sql_idx: i32 = 0;
        let mut builder = StringBuilder::new();
        let mut include_temp_sqls: HashMap<i32, String> = HashMap::new();
        let mut include_temp_sqls_ids: HashMap<String, i32> = HashMap::new();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let element_name = name.local_name.as_str().to_ascii_lowercase();
                    if parse_helper::match_statement(&element_name) {
                        in_statement = true;
                        parse_helper::search_matched_attr(&attributes, "id", |attr| {
                            sql_store.push("-- ".to_string() + attr.value.as_str());
                        });
                    } else if in_statement && element_name == "where" {
                        builder.append("where ");
                    } else if in_statement && element_name == "include" {
                        parse_helper::search_matched_attr(&attributes, "refid", |attr| {
                            builder.append("__INCLUDE_ID_");
                            builder.append(attr.value.as_str());
                            builder.append("_END__");
                        });
                    } else if in_statement {
                        parse_helper::search_matched_attr(&attributes, "prepend", |attr| {
                            builder.append(attr.value.as_str());
                        });
                    } else if element_name == "sql" {
                        in_sql = true;
                        parse_helper::search_matched_attr(&attributes, "id", |attr| {
                            include_temp_sqls_ids.insert(attr.value.as_str().to_string(), sql_idx);
                        });
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let element_name = name.local_name.as_str().to_ascii_lowercase();
                    if parse_helper::match_statement(&element_name) {
                        let sql = parse_helper::replace_included_sql(
                            &mut builder,
                            &include_temp_sqls,
                            &include_temp_sqls_ids,
                        );
                        clear_and_push(&sql, sql_store);
                        in_statement = false;
                    } else if element_name == "sql" {
                        include_temp_sqls.insert(sql_idx, builder.to_string());
                        sql_idx += 1;
                        builder.clear();
                        in_sql = false;
                    }
                }
                Ok(XmlEvent::CData(s)) => {
                    if in_statement || in_sql {
                        builder.append(s);
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if in_statement || in_sql {
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
}

fn clear_and_push(origin_sql: &String, sql_store: &mut Vec<String>) {
    lazy_static! {
        static ref RE0: Regex = Regex::new("[\r\n\t ]+").unwrap();
        static ref RE1: Regex = Regex::new("#[^#{]+#").unwrap();
        static ref RE2: Regex = Regex::new("\\$[^${]+\\$").unwrap();
        static ref RE3: Regex = Regex::new("\\$\\{[^${]+\\}").unwrap();
        static ref RE_FIX1: Regex = Regex::new("WHERE[ ]+AND").unwrap();
        static ref RE_FIX2: Regex = Regex::new("WHERE[ ]+OR").unwrap();
    }
    let mut sql = String::from(origin_sql);
    sql = RE0.replace_all(sql.as_str(), " ").to_string();
    sql = RE1.replace_all(sql.as_str(), ":?").to_string();
    sql = RE2.replace_all(sql.as_str(), ":?").to_string();
    sql = RE3
        .replace_all(sql.as_str(), "__REPLACE_SCHEMA__")
        .to_string();
    sql = RE_FIX1.replace_all(sql.as_str(), "WHERE").to_string();
    sql = RE_FIX2.replace_all(sql.as_str(), "WHERE").to_string();
    sql_store.push(sql + ";");
}
