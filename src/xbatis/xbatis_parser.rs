use super::{
    def::{DialectType, Mode, RegexReplacement, SqlKey, SqlStatement, XmlParsedState},
    parse_helper::{match_statement, replace_included_sql, search_matched_attr},
};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;
use std::{collections::HashMap, fs, io::BufReader, process};
use xml::{attribute::OwnedAttribute, name::OwnedName, reader::XmlEvent, EventReader};

lazy_static! {
    static ref XML_REGEX: Regex = Regex::new("XML-FILE:").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {}", e);
        process::exit(-1);
    });
    static ref STAT_REGEX: Regex = Regex::new("STAT-ID:").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {}", e);
        process::exit(-1);
    });
    static ref ORA_QUERY_PLAN_REGEX: Regex = Regex::new("DBMS_XPLAN").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {}", e);
        process::exit(-1);
    });
    static ref INC_REGEX: Regex = Regex::new("__INCLUDE_ID_").unwrap_or_else(|e| {
        warn!("Unable to parse the regex: {}", e);
        process::exit(-1);
    });
}

/// 解析器
pub trait Parser {
    fn setup_gen_explain(&mut self, gen_explain: bool);

    fn is_gen_explain(&self) -> bool;

    fn setup_replace_num(&mut self, replace_num: i16);

    fn replace_num(&self) -> i16;

    fn dialect_type(&self) -> &DialectType;

    fn parse(&self, files: &Vec<String>) -> Vec<String> {
        let mut sql_store: Vec<String> = Vec::new();
        let mut global_inc_map = HashMap::new();
        for file in files {
            self.check_and_parse(file, &mut sql_store, &mut global_inc_map);
        }
        let mut replaced_sql_store = Vec::new();
        for sql in sql_store {
            replaced_sql_store.push(self.replace_inc_between_xml(&sql, &global_inc_map));
        }
        let mut final_sql_store = Vec::new();
        for sql in replaced_sql_store {
            self.loop_clear_and_push(&mut final_sql_store, self.vec_regex(), &sql, false, false);
        }
        final_sql_store
    }

    fn replace_inc_between_xml(
        &self,
        sql: &String,
        global_inc_map: &HashMap<String, String>,
    ) -> String {
        debug!("--------------------------------");
        debug!("{}", sql);
        let mut new_sql = sql.clone();
        for key in global_inc_map.keys() {
            let target = format!("{}{}{}", "__INCLUDE_ID_", key, "_END__").to_ascii_uppercase();
            debug!("{}", target);
            new_sql = replace_included_sql(
                &new_sql,
                key.to_ascii_uppercase().as_str(),
                global_inc_map.get(key).unwrap_or(&target).as_str(),
            )
        }
        debug!("{}", new_sql);
        debug!("--------------------------------");
        new_sql
    }

    fn check_and_parse(
        &self,
        file: &String,
        sql_store: &mut Vec<String>,
        global_inc_map: &mut HashMap<String, String>,
    ) {
        if self.detect_match(file) {
            info!("try to parse [{}]", file);
            self.read_and_parse(file, sql_store, global_inc_map);
        }
    }

    fn detect_match(&self, file: &str) -> bool;

    fn detect_match_with_regex(&self, file: &str, re: &Regex) -> bool {
        let result = fs::read_to_string(file);
        if let Ok(content) = result {
            re.is_match(content.as_str())
        } else {
            false
        }
    }

    fn read_and_parse(
        &self,
        file: &String,
        sql_store: &mut Vec<String>,
        global_inc_map: &mut HashMap<String, String>,
    ) {
        let mut sql_parsed = Vec::new();
        self.read_xml(file, &mut sql_parsed, global_inc_map);
        for sql in sql_parsed {
            sql_store.push(sql);
        }
    }

    fn read_xml(
        &self,
        filename: &String,
        sql_store: &mut Vec<String>,
        global_inc_map: &mut HashMap<String, String>,
    ) {
        let mut file_inc_map = HashMap::new();
        sql_store.push(compose_comment(
            &comment_leading(self.dialect_type()),
            &filename.to_string(),
            &comment_tailing(self.dialect_type()),
        ));
        let file = fs::File::open(filename).unwrap_or_else(|e| {
            warn!("open file [{}] failed: {}", filename, e);
            process::exit(-1);
        });
        let buf = BufReader::new(file);
        let parser = EventReader::new(buf);
        let mut state = XmlParsedState::new();
        state.filename = filename.clone();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => self.parse_start_element(name, attributes, &mut state),
                Ok(XmlEvent::EndElement { name }) => {
                    self.parse_end_element(name, &mut state, global_inc_map, &mut file_inc_map)
                }
                Ok(XmlEvent::CData(content)) => self.fill_xml_content(&mut state, content),
                Ok(XmlEvent::Characters(content)) => self.fill_xml_content(&mut state, content),
                Err(e) => {
                    warn!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        self.replace_and_fill(sql_store, &state.statements, &file_inc_map);
    }

    fn fill_xml_content(&self, state: &mut XmlParsedState, content: String) {
        self.fill_content(state, content);
        if state.in_loop {
            self.fill_content(state, state.loop_def.separator.clone());
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
        if element_name == "mapper" || element_name == "sqlmap" {
            search_matched_attr(&attributes, "namespace", |attr| {
                state.namespace = attr.value.clone();
                debug!("namespace: {}", state.namespace);
            });
        } else if match_statement(&element_name) {
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
            debug!("{}, {}", state.filename, state.current_id);
            search_matched_attr(&attributes, "refid", |attr| {
                state.sql_builder += " __INCLUDE_ID_";
                let refid = attr.value.clone();
                state.sql_builder += refid.as_str();
                state.sql_builder += "_END__";
                state.has_include = true;
            });
        } else {
            self.ex_parse_start_element(name, &element_name, &attributes, state);
        }
    }

    fn ex_parse_start_element(
        &self,
        name: OwnedName,
        element_name: &str,
        attributes: &[OwnedAttribute],
        state: &mut XmlParsedState,
    );

    fn parse_end_element(
        &self,
        name: OwnedName,
        state: &mut XmlParsedState,
        global_inc_map: &mut HashMap<String, String>,
        file_inc_map: &mut HashMap<String, String>,
    ) {
        let element_name = name.local_name.as_str().to_ascii_lowercase();
        if match_statement(&element_name) {
            let mode = Mode::from(element_name.as_str());
            match mode {
                Mode::SqlPart => self.handle_end_sql_part(state, global_inc_map, file_inc_map),
                _ => self.handle_end_statement(mode, state),
            }
        } else if element_name == "selectkey" {
            state.in_sql_key = false;
        } else {
            self.ex_parse_end_element(name, &element_name, state);
        }
    }

    fn ex_parse_end_element(&self, name: OwnedName, element_name: &str, state: &mut XmlParsedState);

    fn handle_end_sql_part(
        &self,
        state: &mut XmlParsedState,
        global_inc_map: &mut HashMap<String, String>,
        file_inc_map: &mut HashMap<String, String>,
    ) {
        file_inc_map.insert(state.current_id.clone(), state.sql_builder.to_string());
        global_inc_map.insert(
            format!("{}.{}", state.namespace, state.current_id.clone()),
            state.sql_builder.to_string(),
        );
        state.reset();
    }

    fn handle_end_statement(&self, mode: Mode, state: &mut XmlParsedState) {
        let sql_stat = SqlStatement::new(
            mode,
            state.current_id.clone(),
            state.sql_builder.to_string(),
            state.has_include,
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
        file_inc_map: &HashMap<String, String>,
    ) {
        let comment_leading = comment_leading2(self.dialect_type());
        let comment_tailing = comment_tailing2(self.dialect_type());
        for stat in statements {
            self.replace_single_statement(
                sql_store,
                &comment_leading,
                stat,
                &comment_tailing,
                file_inc_map,
            );
        }
    }

    fn replace_single_statement(
        &self,
        sql_store: &mut Vec<String>,
        comment_leading: &String,
        stat: &SqlStatement,
        comment_tailing: &String,
        file_inc_map: &HashMap<String, String>,
    ) {
        debug!("----------------------------------------------------------------");
        sql_store.push(compose_comment(
            &comment_leading.to_string(),
            &String::from(&stat.id),
            &comment_tailing.to_string(),
        ));
        if stat.has_include {
            self.clear_and_push(
                sql_store,
                &loop_replace_include_part(stat, file_inc_map, self.replace_num()),
                self.is_gen_explain(),
            );
        } else {
            self.clear_and_push(sql_store, &stat.sql, self.is_gen_explain());
        }
        if stat.has_sql_key {
            sql_store.push(compose_comment(
                &comment_leading.to_string(),
                &stat.sql_key.key,
                &comment_tailing.to_string(),
            ));
            self.clear_and_push(sql_store, &stat.sql_key.sql, self.is_gen_explain());
        }
    }

    fn clear_and_push(&self, sql_store: &mut Vec<String>, origin_sql: &str, gen_explain: bool) {
        self.loop_clear_and_push(sql_store, self.vec_regex(), origin_sql, gen_explain, true);
    }

    fn loop_clear_and_push(
        &self,
        sql_store: &mut Vec<String>,
        regex_replacements: &[RegexReplacement],
        origin_sql: &str,
        gen_explain: bool,
        append_semicolon: bool,
    ) {
        let mut sql;
        if XML_REGEX.is_match(origin_sql)
            || STAT_REGEX.is_match(origin_sql)
            || ORA_QUERY_PLAN_REGEX.is_match(origin_sql)
        {
            sql = String::from(origin_sql);
        } else {
            sql = String::from(origin_sql.to_ascii_uppercase().trim());
            for regex_replacement in regex_replacements.iter() {
                sql = self.regex_clear_and_push(&sql, regex_replacement);
            }
        }
        if gen_explain && append_semicolon {
            sql_store.push(format!(
                "{}{}{}",
                explain_dialect(self.dialect_type()),
                sql,
                ";"
            ));
            self.append_oracle_list_plan(sql_store);
        } else if !gen_explain && append_semicolon {
            sql_store.push(sql + ";");
        } else if !append_semicolon && gen_explain {
            sql_store.push(format!("{}{}", explain_dialect(self.dialect_type()), sql));
            self.append_oracle_list_plan(sql_store);
        } else {
            sql_store.push(sql);
        }
    }

    fn regex_clear_and_push(
        &self,
        origin_sql: &str,
        regex_replacement: &RegexReplacement,
    ) -> String {
        regex_replacement
            .regex
            .replace_all(origin_sql, regex_replacement.target.as_str())
            .to_string()
    }

    fn vec_regex(&self) -> &Vec<RegexReplacement>;

    fn append_oracle_list_plan(&self, sql_store: &mut Vec<String>) {
        if let DialectType::Oracle = self.dialect_type() {
            sql_store.push(String::from("SELECT * FROM TABLE(DBMS_XPLAN.DISPLAY);"))
        }
    }
}

fn loop_replace_include_part(
    stat: &SqlStatement,
    file_inc_map: &HashMap<String, String>,
    replace_num: i16,
) -> String {
    let mut sql = stat.sql.clone();
    for _i in 0..replace_num {
        for key in file_inc_map.keys() {
            let (new_sql, replace) = replace_included_sql_by_key(&sql, stat, file_inc_map, key);
            if replace {
                sql = new_sql;
            }
        }
        if !INC_REGEX.is_match(&sql) {
            break;
        }
    }
    sql
}

fn replace_included_sql_by_key(
    sql: &str,
    stat: &SqlStatement,
    file_inc_map: &HashMap<String, String>,
    key: &String,
) -> (String, bool) {
    debug!("key:::{}", key);
    let key_opt = file_inc_map.get(key);
    if let Some(sql_part) = key_opt {
        debug!("{}:::-->{}", key, sql_part);
        let new_sql = replace_included_sql(sql, key, sql_part);
        debug!("{}", new_sql);
        (new_sql, true)
    } else {
        warn!(
            "can not find include_key[{}] in statement[{}]",
            key, stat.id
        );
        (String::from(""), false)
    }
}

fn comment_leading(dialet_type: &DialectType) -> String {
    match dialet_type {
        DialectType::Oracle => "SELECT \"XML-FILE: ".to_string(),
        DialectType::MySQL => "SELECT \"XML-FILE: ".to_string(),
    }
}

fn comment_leading2(dialet_type: &DialectType) -> String {
    match dialet_type {
        DialectType::Oracle => "SELECT \"STAT-ID: ".to_string(),
        DialectType::MySQL => "SELECT \"STAT-ID: ".to_string(),
    }
}

fn comment_tailing(dialet_type: &DialectType) -> String {
    match dialet_type {
        DialectType::Oracle => "\" AS XML_FILE FROM DUAL;".to_string(),
        DialectType::MySQL => "\" AS XML_FILE;".to_string(),
    }
}

fn comment_tailing2(dialet_type: &DialectType) -> String {
    match dialet_type {
        DialectType::Oracle => "\" AS STAT_ID FROM DUAL;".to_string(),
        DialectType::MySQL => "\" AS STAT_ID;".to_string(),
    }
}

fn compose_comment(leading: &String, line: &String, trailing: &String) -> String {
    format!("{leading}{line}{trailing}")
}

pub(crate) fn var_placeholder(dialect_type: &DialectType) -> &str {
    match dialect_type {
        DialectType::Oracle => ":?",
        DialectType::MySQL => "@1",
    }
}

fn explain_dialect(dialect_type: &DialectType) -> &str {
    match dialect_type {
        DialectType::Oracle => "explain plan for ",
        DialectType::MySQL => "explain ",
    }
}
