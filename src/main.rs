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

use args::args_parser::XBatisMode::*;
use args::args_parser::*;
use log::*;
use logit::log_initializer::*;
use save::sql_saver::*;
use scan::xml_scanner::*;
use xbatis::def::*;
use xbatis::ibatis_parser::*;
use xbatis::mybatis_parser::*;
use xbatis::xbatis_parser::*;

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = check_args();
    if args.fast_fail {
        print_usage(&args);
    } else if args.show_version {
        print_version();
    } else {
        parse_xbatis_xml(args.mode, args.db_type, &args.src_dir, &args.output_dir);
    }
}

/// 选择并执行对应的解析器
fn parse_xbatis_xml(mode: XBatisMode, db_type: DbType, src_dir: &String, output_dir: &String) {
    init_logger();
    info!(
        "try to parse files in {:?}, fetch sql to {:?}",
        src_dir, output_dir
    );
    let mut files: Vec<String> = Vec::new();
    scan(&mut files, src_dir);
    let parser = choose_parser(mode, convert(db_type));
    let sql_store = parser.parse(&files);
    save(output_dir, sql_store);
}

fn choose_parser(mode: XBatisMode, dialect_type: DialectType) -> Box<dyn Parser> {
    match mode {
        IBatis => {
            return Box::new(create_ibatis_parser(dialect_type));
        }
        MyBatis => {
            return Box::new(create_mybatis_parser(dialect_type));
        }
        _ => {
            panic!("not supported mode");
        }
    }
}

fn convert(db_type: DbType) -> DialectType {
    return match db_type {
        DbType::Oracle => DialectType::Oracle,
        DbType::MySQL => DialectType::MySQL,
        _ => panic!("unknown dialect type"),
    };
}
