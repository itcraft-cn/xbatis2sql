//! `xbatis2sql`，通过解析 `iBATIS` 的 `sqlmap` 文件或 `MyBatis` 的 `mapper` 文件，收集散落的 `sql` 语句，输出到 `result.sql` 中

/// 解析参数
mod args;
/// 日志处置
mod logit;
/// 保存
mod save;
/// 扫描器
mod scan;
/// 解析器
mod xbatis;

use crate::{
    args::args_parser::{self, DbType, XBatisMode},
    logit::log_initializer,
    save::sql_saver,
    scan::xml_scanner,
    xbatis::{def::DialectType, ibatis_parser, mybatis_parser, xbatis_parser::Parser},
};
use log::info;

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = args_parser::check_args();
    if args.fast_fail {
        args_parser::print_usage(&args);
    } else if args.show_version {
        args_parser::print_version();
    } else {
        parse_xbatis_xml(args.mode, args.db_type, &args.src_dir, &args.output_dir);
    }
}

/// 选择并执行对应的解析器
fn parse_xbatis_xml(mode: XBatisMode, db_type: DbType, src_dir: &String, output_dir: &String) {
    log_initializer::init_logger();
    info!(
        "try to parse files in {:?}, fetch sql to {:?}",
        src_dir, output_dir
    );
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, src_dir);
    let parser = choose_parser(mode, convert(db_type));
    let sql_store = parser.parse(&files);
    sql_saver::save(output_dir, sql_store);
}

fn choose_parser(mode: XBatisMode, dialect_type: DialectType) -> Box<dyn Parser> {
    match mode {
        XBatisMode::IBatis => Box::new(ibatis_parser::create_ibatis_parser(dialect_type)),
        XBatisMode::MyBatis => Box::new(mybatis_parser::create_mybatis_parser(dialect_type)),
        _ => panic!("not supported mode"),
    }
}

fn convert(db_type: DbType) -> DialectType {
    match db_type {
        DbType::Oracle => DialectType::Oracle,
        DbType::MySQL => DialectType::MySQL,
        _ => panic!("unknown dialect type"),
    }
}
