use lazy_static::*;
use log::*;
use regex::Regex;
use std::collections::HashMap;
use std::*;

use super::abt_parser::Parser;

use xml::reader::{EventReader, XmlEvent};

use rstring_builder::StringBuilder;

const PARSER: MyBatisParser = MyBatisParser {};

pub fn parse(output_dir: &String, files: &Vec<String>) {
    PARSER.parse(output_dir, files);
}

struct MyBatisParser {}

impl Parser for MyBatisParser {
    fn detect_match(&self, file: &String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("DTD Mapper 3\\.0").unwrap();
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
    let mut in_sql: bool = false;
    let mut sql_idx: i32 = 0;
    let mut builder = StringBuilder::new();
    sql_store.push("-- ".to_string() + filename);
    let mut include_temp_sqls: HashMap<i32, String> = HashMap::new();
    let mut include_temp_sqls_ids: HashMap<String, i32> = HashMap::new();
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
                            break;
                        }
                    }
                } else if in_statement && element_name == "where" {
                    builder.append("where ");
                } else if in_statement && element_name == "set" {
                    builder.append("set ");
                } else if in_statement && element_name == "include" {
                    for attr in attributes {
                        if attr.name.local_name.as_str() == "refid" {
                            builder.append("__INCLUDE_ID_");
                            builder.append(attr.value);
                            builder.append("_END__");
                            break;
                        }
                    }
                } else if in_statement {
                    for attr in attributes {
                        if attr.name.local_name.as_str() == "prepend" {
                            builder.append(attr.value.as_str());
                            break;
                        }
                    }
                } else if element_name == "sql" {
                    in_sql = true;
                    for attr in attributes {
                        if attr.name.local_name.as_str() == "id" {
                            include_temp_sqls_ids.insert(attr.value, sql_idx);
                            break;
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
                    let sql = replace_included_sql(
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

fn replace_included_sql(
    builder: &mut StringBuilder,
    include_temp_sqls: &HashMap<i32, String>,
    include_temp_sqls_ids: &HashMap<String, i32>,
) -> String {
    let mut sql = builder.to_string().trim().to_ascii_uppercase();
    for e in include_temp_sqls_ids {
        let replaced = &include_temp_sqls.get(e.1).unwrap().to_ascii_uppercase();
        sql = sql.replace(
            ("__INCLUDE_ID_".to_string() + e.0.as_str() + "_END__")
                .to_ascii_uppercase()
                .as_str(),
            replaced,
        );
    }
    builder.clear();
    return sql;
}

fn clear_and_push(origin_sql: &String, sql_store: &mut Vec<String>) {
    lazy_static! {
        static ref RE0: Regex = Regex::new("[\n\t ]+").unwrap();
        static ref RE1: Regex = Regex::new("#\\{[^#{]+\\}").unwrap();
        static ref RE2: Regex = Regex::new("\\$\\{[^${]+\\}").unwrap();
        static ref RE_FIX1: Regex = Regex::new("WHERE[ ]+AND").unwrap();
        static ref RE_FIX2: Regex = Regex::new("WHERE[ ]+OR").unwrap();
    }
    let mut sql = String::from(origin_sql);
    sql = RE0.replace_all(sql.as_str(), " ").to_string();
    sql = RE1.replace_all(sql.as_str(), ":?").to_string();
    sql = RE2.replace_all(sql.as_str(), ":?").to_string();
    sql = RE_FIX1.replace_all(sql.as_str(), "WHERE").to_string();
    sql = RE_FIX2.replace_all(sql.as_str(), "WHERE").to_string();
    sql_store.push(sql + ";");
}
