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

use args::args_parser::Mode;
use args::*;
use log::*;
use logit::*;
use scan::*;
use xbatis::xml_parser::*;
use xbatis::*;

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = args_parser::check_args();
    if args.fast_fail || args.show_version {
        args_parser::print_usage(&args);
    } else {
        choose_parser(args.mode, &args.src_dir, &args.output_dir);
    }
}

/// 选择并执行对应的解析器
fn choose_parser(mode: args_parser::Mode, src_dir: &String, output_dir: &String) {
    log_initializer::init_logger();
    info!(
        "try to parse files in {:?}, fetch sql to {:?}",
        src_dir, output_dir
    );
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, src_dir);
    let parser = fetch_parser(mode);
    let sql_store = parser.parse(&files);
    save::sql_saver::save(output_dir, sql_store);
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
