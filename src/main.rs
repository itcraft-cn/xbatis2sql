//! `xbatis2sql`，通过解析 `iBATIS` 的 `sqlmap` 文件或 `MyBatis` 的 `mapper` 文件，收集散落的 `sql` 语句，输出到 `result.sql` 中

/// 解析参数
mod args;
/// 日志处置
mod logger;
/// 解析器
mod parser;
/// 保存
mod saver;
/// 扫描器
mod scanner;

use args::parse_args::Mode;
use args::*;
use log::*;
use logger::*;
use parser::*;
use parser::parser::*;
use scanner::*;

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = parse_args::check_args();
    if args.fast_fail || args.show_version {
        parse_args::print_usage(&args);
    } else {
        choose_parser(args.mode, &args.src_dir, &args.output_dir);
    }
}

/// 选择并执行对应的解析器
fn choose_parser(mode: parse_args::Mode, src_dir: &String, output_dir: &String) {
    log_init::init_logger();
    info!(
        "try to parse files in {:?}, fetch sql to {:?}",
        src_dir, output_dir
    );
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, src_dir);
    let parser = fetch_parser(mode);
    let sql_store = parser.parse(&files);
    saver::save::save(output_dir, sql_store);
}

fn fetch_parser(mode: Mode) -> Box<dyn Parser> {
    match mode {
        Mode::IBatis => {
            return Box::new(ibatis_parser::PARSER);
        }
        Mode::MyBatis => {
            return Box::new(mybatis_parser::PARSER);
        }
        _ => {
            panic!("not supported mode");
        }
    }
}
