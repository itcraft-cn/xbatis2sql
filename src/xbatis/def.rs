use regex::Regex;
use rstring_builder::StringBuilder;
use std::collections::HashMap;
use std::*;

pub enum Mode {
    Statement,
    Select,
    Insert,
    Update,
    Delete,
    SelectKey,
    SqlPart,
}

impl Mode {
    pub fn from(name: &str) -> Self {
        match name {
            "statement" => Mode::Statement,
            "select" => Mode::Select,
            "insert" => Mode::Insert,
            "update" => Mode::Update,
            "delete" => Mode::Delete,
            "selectkey" => Mode::SelectKey,
            "sql" => Mode::SqlPart,
            _ => panic!("unkown mode"),
        }
    }
}

pub struct SqlKey {
    /// 键名
    pub key: String,
    /// 键语句
    pub sql: String,
}

impl SqlKey {
    pub fn empty() -> SqlKey {
        return SqlKey {
            key: String::from(""),
            sql: String::from(""),
        };
    }
}

pub struct SqlStatement {
    pub mode: Mode,
    pub id: String,
    pub sql: String,
    pub has_include: bool,
    pub include_keys: Vec<String>,
    pub has_sql_key: bool,
    pub sql_key: SqlKey,
}

impl SqlStatement {
    pub fn new(
        mode: Mode,
        id: String,
        sql: String,
        has_include: bool,
        include_keys: Vec<String>,
        has_sql_key: bool,
        sql_key: SqlKey,
    ) -> Self {
        return SqlStatement {
            mode,
            id,
            sql,
            has_include,
            include_keys,
            has_sql_key,
            sql_key,
        };
    }
}

/// 解析过程中数据
pub struct XmlParsedState {
    /// 过程中变化

    /// 是否在语句中
    pub in_statement: bool,
    /// 是否在key语句中
    pub in_sql_key: bool,
    /// 是否在loop语句中
    pub in_loop: bool,
    /// 是否有子句
    pub has_include: bool,
    /// 是否有取键语句
    pub has_sql_key: bool,
    /// 当前ID
    pub current_id: String,
    /// 取键语句ID
    pub current_key_id: String,
    /// 子集key
    pub include_keys: Vec<String>,
    /// 循环定义
    pub loop_def: LoopDef,

    /// 过程中累计

    /// 主连接器
    pub sql_builder: StringBuilder,
    /// 取键语句连接器
    pub key_sql_builder: StringBuilder,
    /// 语句集
    pub statements: Vec<SqlStatement>,
    /// 语句集
    pub sql_part_map: HashMap<String, SqlStatement>,

    /// 过程中不再变化

    /// 文件名
    pub filename: String,
}

impl XmlParsedState {
    /// 构建器，构造工厂
    pub fn new() -> Self {
        return XmlParsedState {
            in_statement: false,
            in_sql_key: false,
            in_loop: false,
            has_include: false,
            has_sql_key: false,
            sql_builder: StringBuilder::new(),
            key_sql_builder: StringBuilder::new(),
            current_id: String::from(""),
            current_key_id: String::from(""),
            include_keys: Vec::new(),
            loop_def: LoopDef {
                suffix: String::from(""),
                separator: String::from(""),
            },
            statements: Vec::new(),
            sql_part_map: HashMap::new(),
            filename: String::from(""),
        };
    }

    pub fn reset(&mut self) {
        self.in_statement = false;
        self.in_sql_key = false;
        self.in_loop = false;
        self.has_include = false;
        self.has_sql_key = false;
        self.current_id = String::from("");
        self.current_key_id = String::from("");
        self.include_keys = Vec::new();
        self.loop_def = LoopDef {
            suffix: String::from(""),
            separator: String::from(""),
        };
        self.sql_builder.clear();
        self.key_sql_builder.clear();
    }
}

pub struct RegexReplacement {
    pub regex: Regex,
    pub target: String,
}

impl RegexReplacement {
    pub fn new(regex: &str, target: &str) -> Self {
        return RegexReplacement {
            regex: Regex::new(regex).unwrap(),
            target: String::from(target),
        };
    }
}

pub struct LoopDef {
    pub suffix: String,
    pub separator: String,
}

impl LoopDef {
    pub fn reset(&mut self) {
        self.suffix = String::from("");
        self.separator = String::from("");
    }
}
