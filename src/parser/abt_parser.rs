use log::*;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process;
use std::*;

const CRLF: [u8; 1] = [0x0a];

pub trait Parser {
    fn parse(&self, output_dir: &String, files: &Vec<String>) {
        let mut sql_store: Vec<String> = Vec::new();
        for file in files {
            self.check_and_parse(file, &mut sql_store);
        }
        self.save(output_dir, sql_store);
    }

    fn check_and_parse(&self, file: &String, sql_store: &mut Vec<String>) {
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

    fn read_and_parse(&self, file: &String, sql_store: &mut Vec<String>) {
        self.read_xml(file, sql_store);
    }

    fn read_xml(&self, file: &String, sql_store: &mut Vec<String>);

    fn save(&self, output_dir: &String, sql_store: Vec<String>) {
        info!("write to {:?}/resut.sql, size: {:?}", output_dir, sql_store.len());
        let r = File::create(output_dir.to_string() + "/result.sql");
        if r.is_err() {
            warn!("try to write sql to {:?} failed", output_dir);
            process::exit(-1);
        }
        let mut f = r.unwrap();
        for sql in sql_store {
            write2file(&mut f, sql, output_dir);
            writeln(&mut f, output_dir);
        }
        let fr = f.flush();
        if fr.is_err() {
            warn!("try to flush file {:?} failed", f);
            process::exit(-1);
        }
    }
}

fn write2file(f: &mut File, sql: String, output_dir: &String) {
    let wr = f.write(sql.as_bytes());
    if wr.is_err() {
        warn!("try to write sql[{:?}] to {:?} failed", sql, output_dir);
        process::exit(-1);
    }
}

fn writeln(f: &mut File, output_dir: &String) {
    let wr = f.write(&CRLF);
    if wr.is_err() {
        warn!("try to write crlf to {:?} failed", output_dir);
        process::exit(-1);
    }
}
