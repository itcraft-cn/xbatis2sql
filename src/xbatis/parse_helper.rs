use xml::attribute::OwnedAttribute;

/// 替换 `include`，用对应的 `sql` 进行合并
pub fn replace_included_sql(orig_sql: &str, id: &str, sql_part: &str) -> String {
    let replace_target = "__INCLUDE_ID_".to_string() + id + "_END__";
    let replaced = sql_part;
    return orig_sql.replace(replace_target.as_str(), replaced);
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
