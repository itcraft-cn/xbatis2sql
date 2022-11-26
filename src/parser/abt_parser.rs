use super::def::*;
use super::parse_helper::*;
use log::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process;
use std::*;
use xml::attribute::*;
use xml::name::*;
use xml::reader::*;

/// 回车
const CRLF: [u8; 1] = [0x0a];

/// 解析器
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
            info!("try to parse [{}]", file);
            self.read_and_parse(file, sql_store);
        }
    }

    fn detect_match(&self, file: &String) -> bool;

    fn detect_match_with_regex(&self, file: &String, re: &Regex) -> bool {
        let result = fs::read_to_string(file);
        if result.is_ok() {
            return re.is_match(result.unwrap().as_str());
        } else {
            return false;
        }
    }

    fn read_and_parse(&self, file: &String, sql_store: &mut Vec<String>) {
        self.read_xml(file, sql_store);
    }

    fn read_xml(&self, filename: &String, sql_store: &mut Vec<String>) {
        sql_store.push("-- ".to_string() + filename);
        let file = fs::File::open(filename).unwrap();
        let buf = io::BufReader::new(file);
        let parser = EventReader::new(buf);
        let mut state = XmlParsedState::new();
        state.filename = filename.clone();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    self.parse_start_element(name, attributes, &mut state);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    self.parse_end_element(name, &mut state);
                }
                Ok(XmlEvent::CData(s)) => {
                    self.fill_content(&mut state, s);
                }
                Ok(XmlEvent::Characters(s)) => {
                    self.fill_content(&mut state, s);
                }
                Err(e) => {
                    warn!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        self.replace_and_fill(sql_store, &state.statements, &state.sql_part_map);
    }

    fn fill_content(&self, state: &mut XmlParsedState, s: String) {
        if state.in_statement {
            if state.in_sql_key {
                state.key_sql_builder.append(s);
            } else {
                state.sql_builder.append(s);
            }
        }
    }

    fn parse_start_element(
        &self,
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        state: &mut XmlParsedState,
    );

    fn parse_end_element(&self, name: OwnedName, state: &mut XmlParsedState) {
        let element_name = name.local_name.as_str().to_ascii_lowercase();
        if match_statement(&element_name) {
            let mode = Mode::from(element_name.as_str());
            match mode {
                Mode::SqlPart => self.handle_end_sql_part(mode, state),
                _ => self.handle_end_statement(mode, state),
            }
        } else if element_name == "selectkey" {
            state.in_sql_key = false;
        }
    }

    fn handle_end_sql_part(&self, mode: Mode, state: &mut XmlParsedState) {
        let sql_stat = SqlStatement::new(
            mode,
            state.current_id.clone(),
            state.sql_builder.to_string(),
            false,
            Vec::new(),
            false,
            SqlKey::empty(),
        );
        state
            .sql_part_map
            .insert(state.current_id.clone(), sql_stat);
        state.reset();
    }

    fn handle_end_statement(&self, mode: Mode, state: &mut XmlParsedState) {
        let sql_stat = SqlStatement::new(
            mode,
            state.current_id.clone(),
            state.sql_builder.to_string(),
            state.has_include,
            state.include_keys.clone(),
            state.has_sql_key,
            SqlKey {
                key: state.current_key_id.clone(),
                sql: state.key_sql_builder.to_string(),
            },
        );
        state.statements.push(sql_stat);
        state.reset();
    }

    fn replace_and_fill(
        &self,
        sql_store: &mut Vec<String>,
        statements: &Vec<SqlStatement>,
        sql_part_map: &HashMap<String, SqlStatement>,
    ) {
        for stat in statements {
            sql_store.push("--- ".to_string() + &stat.id);
            if stat.has_include {
                let mut sql = stat.sql.clone();
                for key in &stat.include_keys {
                    let sql_part = sql_part_map.get_key_value(key).unwrap();
                    sql = replace_included_sql(&sql, &sql_part.0, &sql_part.1.sql);
                }
                self.clear_and_push(&sql, sql_store);
            } else {
                self.clear_and_push(&stat.sql, sql_store);
            }
            if stat.has_sql_key {
                sql_store.push("--- ".to_string() + &stat.sql_key.key);
                self.clear_and_push(&stat.sql_key.sql, sql_store);
            }
        }
    }

    fn clear_and_push(&self, origin_sql: &String, sql_store: &mut Vec<String>);

    fn save(&self, output_dir: &String, sql_store: Vec<String>) {
        info!(
            "write to {:?}/resut.sql, size: {:?}",
            output_dir,
            sql_store.len()
        );
        let r = File::create(output_dir.to_string() + "/result.sql");
        if r.is_err() {
            warn!("try to write sql to {:?} failed", output_dir);
            process::exit(-1);
        }
        let mut f = r.unwrap();
        for sql in sql_store {
            self.write2file(&mut f, &sql.as_bytes(), output_dir);
            self.write2file(&mut f, &CRLF, output_dir);
        }
        self.write2file(&mut f, &CRLF, output_dir);
        let fr = f.flush();
        if fr.is_err() {
            warn!("try to flush file {:?} failed", f);
            process::exit(-1);
        }
    }

    fn write2file(&self, f: &mut File, bdata: &[u8], output_dir: &String) {
        let wr = f.write(bdata);
        if wr.is_err() {
            warn!("try to write [{:?}] to {:?} failed", bdata, output_dir);
            process::exit(-1);
        }
    }
}
