use super::abt_parser::*;
use super::parse_helper;
use lazy_static::*;
use regex::Regex;
use rstring_builder::StringBuilder;
use xml::attribute::*;
use xml::name::*;

lazy_static! {
    static ref RE: Regex = Regex::new("DTD SQL Map 2\\.0").unwrap();
}

lazy_static! {
    static ref RE0: Regex = Regex::new("[\t ]?--[^\n]*\n").unwrap();
    static ref RE1: Regex = Regex::new("[\r\n\t ]+").unwrap();
    static ref RE2: Regex = Regex::new("\\$\\{[^${]+\\}").unwrap();
    static ref RE3: Regex = Regex::new("#[^#]+#").unwrap();
    static ref RE4: Regex = Regex::new("\\$[^$]+\\$").unwrap();
    static ref RE_FIX1: Regex = Regex::new("WHERE[ ]+AND").unwrap();
    static ref RE_FIX2: Regex = Regex::new("WHERE[ ]+OR").unwrap();
}

/// `iBATIS` 实现
const PARSER: IBatisParser = IBatisParser {};

/// 调用 `iBATIS` 实现进行解析
pub fn parse(output_dir: &String, files: &Vec<String>) {
    PARSER.parse(output_dir, files);
}

struct IBatisParser {}

impl Parser for IBatisParser {
    fn detect_match(&self, file: &String) -> bool {
        return self.detect_match_with_regex(file, &RE);
    }

    fn parse_start_element(
        &self,
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        builder: &mut StringBuilder,
        state: &mut XmlParsedState,
    ) {
        let element_name = name.local_name.as_str().to_ascii_lowercase();
        if parse_helper::match_statement(&element_name) {
            state.in_statement = true;
            parse_helper::search_matched_attr(&attributes, "id", |attr| {
                state.current_id = attr.value.clone();
            });
        } else if element_name == "where" {
            builder.append(" where ");
        } else if element_name == "include" {
            parse_helper::search_matched_attr(&attributes, "refid", |attr| {
                builder.append(" __INCLUDE_ID_");
                builder.append(attr.value.to_ascii_uppercase().as_str());
                builder.append("_END__");
            });
        } else if state.in_statement {
            parse_helper::search_matched_attr(&attributes, "prepend", |attr| {
                builder.append(" ").append(attr.value.as_str()).append(" ");
            });
        }
    }

    fn clear_and_push(&self, origin_sql: &String, sql_store: &mut Vec<String>) {
        let mut sql = String::from(origin_sql.to_ascii_uppercase().trim());
        sql = RE0.replace_all(sql.as_str(), " ").to_string();
        sql = RE1.replace_all(sql.as_str(), " ").to_string();
        sql = RE2
            .replace_all(sql.as_str(), "__REPLACE_SCHEMA__")
            .to_string();
        sql = RE3.replace_all(sql.as_str(), ":?").to_string();
        sql = RE4.replace_all(sql.as_str(), ":?").to_string();
        sql = RE_FIX1.replace_all(sql.as_str(), "WHERE").to_string();
        sql = RE_FIX2.replace_all(sql.as_str(), "WHERE").to_string();
        sql_store.push(sql + ";");
    }
}
