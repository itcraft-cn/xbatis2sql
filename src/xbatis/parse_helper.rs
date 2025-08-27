use globalvar::{fetch_global_var_mut, init_global_var};
use log::warn;
use regex::Regex;
use std::{
    collections::HashMap,
    process,
    sync::atomic::{AtomicBool, Ordering},
};
use xml::attribute::OwnedAttribute;

static REGEX_INC_MAP_INIT: AtomicBool = AtomicBool::new(false);

/// 替换 `include`，用对应的 `sql` 进行合并
pub fn replace_included_sql(orig_sql: &str, id: &str, sql_part: &str) -> String {
    let rx = gen_regex_by_id(id);
    let replaced = sql_part;
    rx.replace_all(orig_sql, replaced).to_string()
}

fn gen_regex_by_id(id: &str) -> Regex {
    if !REGEX_INC_MAP_INIT.load(Ordering::SeqCst) {
        init_global_var::<HashMap<String, Regex>>("regex_inc_map", HashMap::new());
        REGEX_INC_MAP_INIT.store(true, Ordering::SeqCst);
    }
    let regex_inc_map = fetch_global_var_mut::<HashMap<String, Regex>>("regex_inc_map").unwrap();
    let replace_target = format!("{}{}{}", "__INCLUDE_ID_", id, "_END__");
    regex_inc_map
        .entry(replace_target.clone())
        .or_insert_with(|| {
            Regex::new(replace_target.as_str()).unwrap_or_else(|e| {
                warn!("build regex[{replace_target}] failed: {e}");
                process::exit(-1);
            })
        })
        .clone()
}

/// 检索属性，匹配情况下回调闭包
pub fn search_matched_attr(
    attributes: &[OwnedAttribute],
    matched_name: &str,
    mut f: impl FnMut(&OwnedAttribute),
) {
    for attr in attributes {
        if attr.name.local_name.as_str() == matched_name {
            f(attr);
            break;
        }
    }
}

/// 是否匹配语句块
pub fn match_statement(element_name: &String) -> bool {
    *element_name == "statement"
        || *element_name == "select"
        || *element_name == "insert"
        || *element_name == "update"
        || *element_name == "delete"
        || *element_name == "sql"
}
