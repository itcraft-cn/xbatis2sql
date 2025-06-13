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
    static ref RE: Regex = Regex::new("DTD Mapper 3\\.0").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {e}");
        process::exit(-1);
    });
}

/// `MyBatis` 实现
pub fn create_mybatis_parser(dialect_type: DialectType) -> MyBatisParser {
    let re_vec;
    {
        re_vec = create_replcements(&dialect_type);
    }
    MyBatisParser {
        dialect_type,
        re_vec,
        gen_explain: false,
        replace_num: 0,
    }
}

fn create_replcements(dialect_type: &DialectType) -> Vec<RegexReplacement> {
    let placeholder = var_placeholder(dialect_type);
    vec![
        RegexReplacement::new("[\t ]?--[^\n]*\n", ""),
        RegexReplacement::new("[\r\n\t ]+", " "),
        RegexReplacement::new("\\$\\{[^${]+\\}\\.", "__REPLACE_SCHEMA__."),
        RegexReplacement::new("#\\{[^#{]+\\}", placeholder),
        RegexReplacement::new("\\$\\{[^${]+\\}", placeholder),
        RegexReplacement::new("WHERE[ ]+AND[ ]+", "WHERE "),
        RegexReplacement::new("WHERE[ ]+OR[ ]+", "WHERE "),
        RegexReplacement::new(",[ ]+WHERE", " WHERE"),
        RegexReplacement::new("[ ]*,[ ]*\\)", ")"),
        RegexReplacement::new("AND[ ]*$", ""),
        RegexReplacement::new("OR[ ]*$", ""),
        RegexReplacement::new(",$", ""),
    ]
}

pub struct MyBatisParser {
    dialect_type: DialectType,
    re_vec: Vec<RegexReplacement>,
    gen_explain: bool,
    replace_num: i16,
}

impl Parser for MyBatisParser {
    fn setup_gen_explain(&mut self, gen_explain: bool) {
        self.gen_explain = gen_explain;
    }

    fn is_gen_explain(&self) -> bool {
        self.gen_explain
    }

    fn setup_replace_num(&mut self, replace_num: i16) {
        self.replace_num = replace_num;
    }

    fn replace_num(&self) -> i16 {
        self.replace_num
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
        element_name: &str,
        attributes: &[OwnedAttribute],
        state: &mut XmlParsedState,
    ) {
        if element_name == "set" {
            state.sql_builder += " set ";
        } else if element_name == "trim" {
            state.in_loop = true;
            parse_helper::search_matched_attr(attributes, "prefix", |attr| {
                self.fill_content(state, attr.value.clone());
            });
            parse_helper::search_matched_attr(attributes, "suffix", |attr| {
                state.loop_def.suffix = attr.value.clone();
            });
        } else if element_name == "foreach" {
            state.in_loop = true;
            parse_helper::search_matched_attr(attributes, "open", |attr| {
                self.fill_content(state, attr.value.clone());
            });
            parse_helper::search_matched_attr(attributes, "close", |attr| {
                state.loop_def.suffix = attr.value.clone();
            });
            parse_helper::search_matched_attr(attributes, "separator", |attr| {
                state.loop_def.separator = attr.value.clone();
            });
        }
    }

    fn ex_parse_end_element(
        &self,
        _name: OwnedName,
        element_name: &str,
        state: &mut XmlParsedState,
    ) {
        if element_name == "trim" || element_name == "foreach" {
            let suffix;
            {
                suffix = &state.loop_def.suffix;
            }
            self.fill_content(state, suffix.clone());
            state.in_loop = false;
            state.loop_def.reset();
        }
    }

    fn vec_regex(&self) -> &Vec<RegexReplacement> {
        &self.re_vec
    }
}
