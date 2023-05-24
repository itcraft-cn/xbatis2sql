use super::def::*;
use super::parse_helper::*;
use super::xbatis_parser::*;
use lazy_static::*;
use regex::Regex;
use xml::attribute::*;
use xml::name::*;

lazy_static! {
    static ref RE: Regex = Regex::new("DTD SQL Map 2\\.0").unwrap();
}

/// `iBATIS` 实现
pub fn create_ibatis_parser(dialect_type: DialectType) -> IBatisParser {
    let re_vec;
    {
        re_vec = create_replcements(&dialect_type);
    }
    return IBatisParser {
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
    ];
}

pub struct IBatisParser {
    dialect_type: DialectType,
    re_vec: Vec<RegexReplacement>,
}

impl Parser for IBatisParser {
    fn setup_dialect_type(&mut self, dialect_type: DialectType) {
        self.dialect_type = dialect_type;
    }

    fn detect_match(&self, file: &String) -> bool {
        return self.detect_match_with_regex(file, &RE);
    }

    fn ex_parse_start_element(
        &self,
        _name: OwnedName,
        _element_name: &String,
        attributes: &Vec<OwnedAttribute>,
        state: &mut XmlParsedState,
    ) {
        if state.in_statement {
            search_matched_attr(attributes, "prepend", |attr| {
                state.sql_builder += " ";
                state.sql_builder += attr.value.as_str();
                state.sql_builder += " ";
            });
        }
    }

    fn ex_parse_end_element(
        &self,
        _name: OwnedName,
        _element_name: &String,
        _state: &mut XmlParsedState,
    ) {
    }

    fn clear_and_push(&self, sql_store: &mut Vec<String>, origin_sql: &String) {
        self.loop_clear_and_push(sql_store, &self.re_vec, origin_sql)
    }
}
