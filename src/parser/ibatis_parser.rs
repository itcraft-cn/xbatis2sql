use lazy_static::*;
use log::*;
use regex::Regex;
use std::*;

use super::abt_parser::Parser;

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

    fn read_and_parse(&self, file: &String, sql_store: &Vec<String>) -> Vec<String> {
        info!(">>{:?}", file);
        for sql in sql_store {
            info!("{:?}", sql);
        }
        return Vec::new();
    }
}
