use super::def::*;
use super::xml_parser::*;
use lazy_static::*;
use regex::Regex;
use xml::attribute::*;
use xml::name::*;

lazy_static! {
    static ref RE: Regex = Regex::new("DTD Mapper 3\\.0").unwrap();
}

lazy_static! {
    static ref RE_VEC: Vec<RegexReplacement> = create_replcements();
}

/// `MyBatis` 实现
pub const PARSER: MyBatisParser = MyBatisParser {};

fn create_replcements() -> Vec<RegexReplacement> {
    return vec![
        RegexReplacement {
            regex: Regex::new("[\t ]?--[^\n]*\n").unwrap(),
            target: String::from(""),
        },
        RegexReplacement {
            regex: Regex::new("[\r\n\t ]+").unwrap(),
            target: String::from(" "),
        },
        RegexReplacement {
            regex: Regex::new("#\\{[^#{]+\\}").unwrap(),
            target: String::from(":?"),
        },
        RegexReplacement {
            regex: Regex::new("\\$\\{[^${]+\\}").unwrap(),
            target: String::from(":?"),
        },
        RegexReplacement {
            regex: Regex::new("WHERE[ ]+AND[ ]+").unwrap(),
            target: String::from("WHERE "),
        },
        RegexReplacement {
            regex: Regex::new("WHERE[ ]+OR[ ]+").unwrap(),
            target: String::from("WHERE "),
        },
        RegexReplacement {
            regex: Regex::new(",[ ]+WHERE").unwrap(),
            target: String::from(" WHERE"),
        },
        RegexReplacement {
            regex: Regex::new(",$").unwrap(),
            target: String::from(""),
        },
    ];
}

pub struct MyBatisParser {}

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

    fn clear_and_push(&self, sql_store: &mut Vec<String>, origin_sql: &String) {
        self.loop_clear_and_push(sql_store, &RE_VEC, origin_sql)
    }
}
