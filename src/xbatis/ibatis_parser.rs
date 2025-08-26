use super::{
    def::{DialectType, RegexReplacement, XmlParsedState},
    parse_helper,
    xbatis_parser::{var_placeholder, Parser},
};
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;
use std::process;
use xml::{attribute::OwnedAttribute, name::OwnedName};

lazy_static! {
    static ref RE: Regex = Regex::new("DTD SQL Map 2\\.0").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {e}");
        process::exit(-1);
    });
}

/// `iBATIS` 实现
pub fn create_ibatis_parser(dialect_type: DialectType) -> IBatisParser {
    let re_vec;
    {
        re_vec = create_replcements(&dialect_type);
    }
    IBatisParser {
        dialect_type,
        re_vec,
        gen_explain: false,
        replace_num: 0,
        sql_limit: 0,
    }
}

fn create_replcements(dialect_type: &DialectType) -> Vec<RegexReplacement> {
    let placeholder = var_placeholder(dialect_type);
    vec![
        RegexReplacement::new("[\t ]?--[^\n]*\n", " "),
        RegexReplacement::new("[\r\n\t ]+", " "),
        RegexReplacement::new("\\$\\{[^${]+\\}", "__REPLACE_SCHEMA__"),
        RegexReplacement::new("#[^#]+#", placeholder),
        RegexReplacement::new("\\$[^$]+\\$", placeholder),
        RegexReplacement::new("WHERE[ ]+AND[ ]+", "WHERE "),
        RegexReplacement::new("WHERE[ ]+OR[ ]+", "WHERE "),
        RegexReplacement::new(",[ ]+WHERE", " WHERE"),
        RegexReplacement::new("[ ]*,[ ]*\\)", ")"),
        RegexReplacement::new("AND[ ]*$", ""),
        RegexReplacement::new("OR[ ]*$", ""),
        RegexReplacement::new(",$", ""),
    ]
}

pub struct IBatisParser {
    dialect_type: DialectType,
    re_vec: Vec<RegexReplacement>,
    gen_explain: bool,
    replace_num: i16,
    sql_limit: i16,
}

impl Parser for IBatisParser {
    fn setup_gen_explain(&mut self, gen_explain: bool) {
        self.gen_explain = gen_explain;
    }

    fn is_gen_explain(&self) -> bool {
        self.gen_explain
    }

    fn setup_replace_num(&mut self, replace_num: i16) {
        self.replace_num = replace_num;
    }

    fn setup_sql_limit(&mut self, sql_limit: i16) {
        self.sql_limit = sql_limit;
    }

    fn replace_num(&self) -> i16 {
        self.replace_num
    }

    fn is_sql_limit(&self) -> bool{
        self.sql_limit > 0
    }

    fn sql_limit(&self) -> i16{
        self.sql_limit
    }

    fn dialect_type(&self) -> &DialectType {
        &self.dialect_type
    }

    fn detect_match(&self, file: &str) -> bool {
        self.detect_match_with_regex(file, &RE)
    }

    fn ex_parse_start_element(
        &self,
        _name: OwnedName,
        _element_name: &str,
        attributes: &[OwnedAttribute],
        state: &mut XmlParsedState,
    ) {
        if state.in_statement {
            parse_helper::search_matched_attr(attributes, "prepend", |attr| {
                state.sql_builder += " ";
                state.sql_builder += attr.value.as_str();
                state.sql_builder += " ";
            });
        }
    }

    fn ex_parse_end_element(
        &self,
        _name: OwnedName,
        _element_name: &str,
        _state: &mut XmlParsedState,
    ) {
    }

    fn vec_regex(&self) -> &Vec<RegexReplacement> {
        &self.re_vec
    }
}
