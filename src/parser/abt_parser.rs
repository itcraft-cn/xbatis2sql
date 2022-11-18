use log::*;
use regex::Regex;
use std::*;

pub trait Parser {
    fn parse(&self, files: &Vec<String>) {
        let sql_store: Vec<String> = Vec::new();
        for file in files {
            self.check_and_parse(file, &sql_store);
        }
        self.save(sql_store);
    }

    fn check_and_parse(&self, file: &String, sql_store: &Vec<String>) {
        if self.detect_match(file) {
            info!("{:?}", file);
            self.read_and_parse(file, sql_store);
        }
    }

    fn detect_match(&self, file: &String) -> bool;

    fn detect_match_with_regex(&self, file: &String, re: &Regex) -> bool {
        debug!(">>{:?}", file);
        let result = fs::read_to_string(file);
        if result.is_ok() {
            let text = result.unwrap();
            let is_match = re.is_match(text.as_str());
            if is_match {
                debug!("{:?}", text);
            }
            return is_match;
        } else {
            return false;
        }
    }

    fn read_and_parse(&self, file: &String, sql_store: &Vec<String>) -> Vec<String>;

    fn save(&self, sql_store: Vec<String>) {
        for sql in sql_store {
            info!("{:?}", sql);
        }
    }
}
