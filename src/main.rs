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

use args::args_parser::Mode::*;
use args::args_parser::*;
use log::*;
use logit::log_initializer::*;
use save::sql_saver::*;
use scan::xml_scanner::*;
use xbatis::ibatis_parser::*;
use xbatis::mybatis_parser::*;
use xbatis::xml_parser::*;

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = check_args();
    if args.fast_fail || args.show_version {
        print_usage(&args);
    } else {
        choose_parser(args.mode, &args.src_dir, &args.output_dir);
    }
}

/// 选择并执行对应的解析器
fn choose_parser(mode: Mode, src_dir: &String, output_dir: &String) {
    init_logger();
    info!(
        "try to parse files in {:?}, fetch sql to {:?}",
        src_dir, output_dir
    );
    let mut files: Vec<String> = Vec::new();
    scan(&mut files, src_dir);
    let parser = fetch_parser(mode);
    let sql_store = parser.parse(&files);
    save(output_dir, sql_store);
}

fn fetch_parser(mode: Mode) -> Box<dyn Parser> {
    match mode {
        IBatis => {
            return Box::new(IBATIS_PARSER);
        }
        MyBatis => {
            return Box::new(MYBATIS_PARSER);
        }
        _ => {
            panic!("not supported mode");
        }
    }
}
