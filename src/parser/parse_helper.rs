use rstring_builder::StringBuilder;
use std::collections::HashMap;
use std::*;
use xml::attribute::*;

/// 替换 `include`，用对应的 `sql` 进行合并
pub fn replace_included_sql(
    builder: &mut StringBuilder,
    include_temp_sqls: &HashMap<i32, String>,
    include_temp_sqls_ids: &HashMap<String, i32>,
) -> String {
    let mut sql = builder.to_string().trim().to_ascii_uppercase();
    for e in include_temp_sqls_ids {
        let replace_target = "__INCLUDE_ID_".to_string() + e.0.as_str() + "_END__";
        let replaced = &include_temp_sqls.get(e.1).unwrap().to_ascii_uppercase();
        sql = sql.replace((replace_target).to_ascii_uppercase().as_str(), replaced);
    }
    builder.clear();
    return sql;
}

/// 检索属性，匹配情况下回调闭包
pub fn search_matched_attr(
    attributes: &Vec<OwnedAttribute>,
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
    *element_name == "select"
        || *element_name == "insert"
        || *element_name == "update"
        || *element_name == "delete"
        || *element_name == "statement"
}
