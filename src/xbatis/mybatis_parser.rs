use super::def::*;
use super::parse_helper::search_matched_attr;
use super::xbatis_parser::*;
use lazy_static::*;
use regex::Regex;
use xml::attribute::*;
use xml::name::*;

lazy_static! {
    static ref RE: Regex = Regex::new("DTD Mapper 3\\.0").unwrap();
}

/// `MyBatis` 实现
pub fn create_mybatis_parser(dialect_type: DialectType) -> MyBatisParser {
    let re_vec;
    {
        re_vec = create_replcements(&dialect_type);
    }
    return MyBatisParser {
        dialect_type,
        re_vec,
    };
}

fn create_replcements(dialect_type: &DialectType) -> Vec<RegexReplacement> {
    let placeholder = match dialect_type {
        DialectType::Oracle => " :? ",
        DialectType::MySQL => " @1 ",
    };
    return vec![
        RegexReplacement::new("[\t ]?--[^\n]*\n", ""),
        RegexReplacement::new("[\r\n\t ]+", " "),
        RegexReplacement::new("#\\{[^#{]+\\}", placeholder),
        RegexReplacement::new("\\$\\{[^${]+\\}", placeholder),
        RegexReplacement::new("WHERE[ ]+AND[ ]+", "WHERE "),
        RegexReplacement::new("WHERE[ ]+OR[ ]+", "WHERE "),
        RegexReplacement::new(",[ ]+WHERE", " WHERE"),
        RegexReplacement::new("[ ]*,[ ]*\\)", ")"),
        RegexReplacement::new(",$", ""),
    ];
}

pub struct MyBatisParser {
    dialect_type: DialectType,
    re_vec: Vec<RegexReplacement>,
}

impl Parser for MyBatisParser {
    fn setup_dialect_type(&mut self, dialect_type: DialectType) {
        self.dialect_type = dialect_type;
    }

    fn detect_match(&self, file: &String) -> bool {
        return self.detect_match_with_regex(file, &RE);
    }

    fn ex_parse_start_element(
        &self,
        _name: OwnedName,
        element_name: &String,
        attributes: &Vec<OwnedAttribute>,
        state: &mut XmlParsedState,
    ) {
        if element_name == "set" {
            state.sql_builder += " set ";
        } else if element_name == "trim" {
            state.in_loop = true;
            search_matched_attr(attributes, "prefix", |attr| {
                self.fill_content(state, attr.value.clone());
            });
            search_matched_attr(attributes, "suffix", |attr| {
                state.loop_def.suffix = attr.value.clone();
            });
        } else if element_name == "foreach" {
            state.in_loop = true;
            search_matched_attr(attributes, "open", |attr| {
                self.fill_content(state, attr.value.clone());
            });
            search_matched_attr(attributes, "close", |attr| {
                state.loop_def.suffix = attr.value.clone();
            });
            search_matched_attr(attributes, "separator", |attr| {
                state.loop_def.separator = attr.value.clone();
            });
        }
    }

    fn ex_parse_end_element(
        &self,
        _name: OwnedName,
        element_name: &String,
        state: &mut XmlParsedState,
    ) {
        if element_name == "trim" {
            let suffix;
            {
                suffix = &state.loop_def.suffix;
            }
            self.fill_content(state, suffix.clone());
            state.in_loop = false;
            state.loop_def.reset();
        } else if element_name == "foreach" {
            let suffix;
            {
                suffix = &state.loop_def.suffix;
            }
            self.fill_content(state, suffix.clone());
            state.in_loop = false;
            state.loop_def.reset();
        }
    }

    fn clear_and_push(&self, sql_store: &mut Vec<String>, origin_sql: &String) {
        self.loop_clear_and_push(sql_store, &self.re_vec, origin_sql)
    }
}
