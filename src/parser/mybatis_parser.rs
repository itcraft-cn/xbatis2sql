use super::abt_parser::*;
use super::def::*;
use lazy_static::*;
use regex::Regex;
use xml::attribute::*;
use xml::name::*;

lazy_static! {
    static ref RE: Regex = Regex::new("DTD Mapper 3\\.0").unwrap();
}

lazy_static! {
    static ref RE0: Regex = Regex::new("[\t ]?--[^\n]*\n").unwrap();
    static ref RE1: Regex = Regex::new("[\r\n\t ]+").unwrap();
    static ref RE2: Regex = Regex::new("#\\{[^#{]+\\}").unwrap();
    static ref RE3: Regex = Regex::new("\\$\\{[^${]+\\}").unwrap();
    static ref RE_FIX1: Regex = Regex::new("WHERE[ ]+AND[ ]+").unwrap();
    static ref RE_FIX2: Regex = Regex::new("WHERE[ ]+OR[ ]+").unwrap();
    static ref RE_FIX3: Regex = Regex::new(",[ ]+WHERE").unwrap();
    static ref RE_FIX4: Regex = Regex::new(",$").unwrap();
}

/// `MyBatis` 实现
const PARSER: MyBatisParser = MyBatisParser {};

/// 调用 `MyBatis` 实现进行解析
pub fn parse(output_dir: &String, files: &Vec<String>) {
    PARSER.parse(output_dir, files);
}

struct MyBatisParser {}

impl Parser for MyBatisParser {
    fn detect_match(&self, file: &String) -> bool {
        return self.detect_match_with_regex(file, &RE);
    }

    fn ex_parse_start_element(
        &self,
        _name: OwnedName,
        element_name: &String,
        _attributes: Vec<OwnedAttribute>,
        state: &mut XmlParsedState,
    ) {
        if element_name == "set" {
            state.sql_builder.append(" set ");
        }
    }

    fn clear_and_push(&self, origin_sql: &String, sql_store: &mut Vec<String>) {
        let mut sql = String::from(origin_sql.to_ascii_uppercase().trim());
        sql = RE0.replace_all(sql.as_str(), "").to_string();
        sql = RE1.replace_all(sql.as_str(), " ").to_string();
        sql = RE2.replace_all(sql.as_str(), ":?").to_string();
        sql = RE3.replace_all(sql.as_str(), ":?").to_string();
        sql = RE_FIX1.replace_all(sql.as_str(), "WHERE ").to_string();
        sql = RE_FIX2.replace_all(sql.as_str(), "WHERE ").to_string();
        sql = RE_FIX3.replace_all(sql.as_str(), " WHERE").to_string();
        sql = RE_FIX4.replace_all(sql.as_str(), "").to_string();
        sql_store.push(sql + ";");
    }
}
