use super::def::*;
use super::parse_helper::*;
use log::*;
use regex::Regex;
use std::collections::HashMap;
use std::*;
use xml::attribute::*;
use xml::name::*;
use xml::reader::*;

/// 解析器
pub trait Parser {
    fn setup_dialect_type(&mut self, dialect_type: DialectType);

    fn parse(&self, files: &Vec<String>) -> Vec<String> {
        let mut sql_store: Vec<String> = Vec::new();
        for file in files {
            self.check_and_parse(file, &mut sql_store);
        }
        return sql_store;
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
                Ok(XmlEvent::CData(content)) => {
                    self.fill_xml_content(&mut state, content);
                }
                Ok(XmlEvent::Characters(content)) => {
                    self.fill_xml_content(&mut state, content);
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

    fn fill_xml_content(&self, state: &mut XmlParsedState, content: String) {
        self.fill_content(state, content);
        if state.in_loop {
            let separator;
            {
                separator = state.loop_def.separator.clone();
            }
            self.fill_content(state, separator);
        }
    }

    fn fill_content(&self, state: &mut XmlParsedState, content: String) {
        if state.in_statement {
            if state.in_sql_key {
                state.key_sql_builder += content.as_str();
            } else {
                state.sql_builder += content.as_str();
            }
        }
    }

    fn parse_start_element(
        &self,
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        state: &mut XmlParsedState,
    ) {
        let element_name = name.local_name.as_str().to_ascii_lowercase();
        if match_statement(&element_name) {
            state.in_statement = true;
            search_matched_attr(&attributes, "id", |attr| {
                state.current_id = attr.value.clone();
            });
        } else if element_name == "selectkey" {
            state.in_sql_key = true;
            state.has_sql_key = true;
            state.current_key_id = state.current_id.as_str().to_string() + ".selectKey";
        } else if element_name == "where" {
            state.sql_builder += " where ";
        } else if element_name == "include" {
            search_matched_attr(&attributes, "refid", |attr| {
                state.sql_builder += " __INCLUDE_ID_";
                let refid = attr.value.clone();
                state.sql_builder += refid.as_str();
                state.sql_builder += "_END__";
                state.has_include = true;
                state.include_keys.push(refid);
            });
        } else {
            self.ex_parse_start_element(name, &element_name, &attributes, state);
        }
    }

    fn ex_parse_start_element(
        &self,
        name: OwnedName,
        element_name: &String,
        attributes: &Vec<OwnedAttribute>,
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
        } else {
            self.ex_parse_end_element(name, &element_name, state);
        }
    }

    fn ex_parse_end_element(
        &self,
        name: OwnedName,
        element_name: &String,
        state: &mut XmlParsedState,
    );

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
                self.clear_and_push(sql_store, &sql);
            } else {
                self.clear_and_push(sql_store, &stat.sql);
            }
            if stat.has_sql_key {
                sql_store.push("--- ".to_string() + &stat.sql_key.key);
                self.clear_and_push(sql_store, &stat.sql_key.sql);
            }
        }
    }

    fn clear_and_push(&self, sql_store: &mut Vec<String>, origin_sql: &String);

    fn loop_clear_and_push(
        &self,
        sql_store: &mut Vec<String>,
        regex_replacements: &Vec<RegexReplacement>,
        origin_sql: &String,
    ) {
        let mut sql = String::from(origin_sql.to_ascii_uppercase().trim());
        for regex_replacement in regex_replacements.iter() {
            sql = self.regex_clear_and_push(&sql, regex_replacement);
        }
        sql_store.push(sql + ";");
    }

    fn regex_clear_and_push(
        &self,
        origin_sql: &String,
        regex_replacement: &RegexReplacement,
    ) -> String {
        return regex_replacement
            .regex
            .replace_all(origin_sql, regex_replacement.target.as_str())
            .to_string();
    }
}
