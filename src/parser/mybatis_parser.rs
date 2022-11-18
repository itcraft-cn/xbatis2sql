use log::*;

use super::abt_parser::Parser;

const PARSER: MyBatisParser = MyBatisParser {};

pub fn parse(files: &Vec<String>) {
    PARSER.parse(files);
}

struct MyBatisParser {}

impl Parser for MyBatisParser {
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
