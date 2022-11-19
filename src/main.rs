//! `xbatis2sql`，通过解析 `iBATIS` 的 `sqlmap` 文件或 `MyBatis` 的 `mapper` 文件，收集散落的 `sql` 语句，输出到 `result.sql` 中

/// 日志处置
mod logger;
/// 解析器
mod parser;
/// 扫描器
mod scanner;

use log::*;
use logger::*;
use parser::*;
use scanner::*;
use std::env;
use std::process;

/// 主函数，解析参数并调用后续函数
fn main() {
    log_init::init_logger();
    let args: Vec<String> = env::args().collect();
    let args_len: u8 = args.len() as u8 - 1;
    if args_len == 3 {
        choose_parser(&args[1], &args[2], &args[3]);
    } else {
        warn!("just need three arguments, got {} argument(s)", args_len);
        warn!("USAGE:\txbatis2sql [ibatis|mybatis] src_dir output_dir");
        process::exit(-1);
    }
}

/// 选择并执行对应的解析器
fn choose_parser(mode: &String, src_dir: &String, output_dir: &String) {
    let match_ibatis = mode == "ibatis";
    let match_mybatis = mode == "mybatis";
    if match_ibatis || match_mybatis {
        info!(
            "try to parse files in {:?}, fetch sql to {:?}",
            src_dir, output_dir
        );
        let mut files: Vec<String> = Vec::new();
        xml_scanner::scan(&mut files, src_dir);
        if match_ibatis {
            ibatis_parser::parse(output_dir, &files);
        } else {
            mybatis_parser::parse(output_dir, &files);
        }
    } else {
        warn!("not supported: {:?}", mode);
        process::exit(-1);
    }
}
