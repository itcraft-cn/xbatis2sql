use log::warn;
use regex::Regex;
use std::process;
use xml::attribute::OwnedAttribute;

/// 替换 `include`，用对应的 `sql` 进行合并
pub fn replace_included_sql(orig_sql: &str, id: &str, sql_part: &str) -> String {
    let replace_target = format!("{}{}{}", "__INCLUDE_ID_", id, "_END__");
    let replaced = sql_part;
    let rx = Regex::new(replace_target.as_str()).unwrap_or_else(|e| {
        warn!("build regex[{replace_target}] failed: {e}");
        process::exit(-1);
    });
    rx.replace_all(orig_sql, replaced).to_string()
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
