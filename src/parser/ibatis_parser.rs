use log::*;

use super::abt_parser::Parser;

const PARSER: IBatisParser = IBatisParser {};

pub fn parse(files: &Vec<String>) {
    PARSER.parse(files);
}

struct IBatisParser {}

impl Parser for IBatisParser {
    fn detect_match(&self, file: &String) -> bool {
        info!(">>{:?}", file);
        return false;
    }

    fn read_and_parse(&self, file: &String, sql_store: &Vec<String>) -> Vec<String> {
        info!(">>{:?}", file);
        for sql in sql_store {
            info!("{:?}", sql);
        }
        return Vec::new();
    }
}
