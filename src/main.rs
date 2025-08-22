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
use concurrent_queue::ConcurrentQueue;
use log::{info, warn};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI16, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

/// 主函数，解析参数并调用后续函数
fn main() {
    let args = args_parser::check_args();
    if args.fast_fail {
        args_parser::print_usage(&args);
    } else if args.show_version {
        args_parser::print_version();
    } else {
        parse_xbatis_xml(
            args.mode,
            args.db_type,
            &args.src_dir,
            &args.output_dir,
            args.gen_explain,
            args.replace_num,
        );
    }
}

/// 选择并执行对应的解析器
fn parse_xbatis_xml(
    mode: XBatisMode,
    db_type: DbType,
    src_dir: &String,
    output_dir: &String,
    gen_explain: bool,
    replace_num: i16,
) {
    log_initializer::init_logger();
    info!("try to parse files in {src_dir:?}, fetch sql to {output_dir:?}");
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, src_dir);
    let arc_queue = Arc::new(ConcurrentQueue::<Vec<String>>::unbounded());
    let arc_limit = Arc::new(AtomicI16::new(0));
    let arc_active = Arc::new(AtomicBool::new(true));
    let output_dir_clone = output_dir.clone();
    let arc_queue_writer_clone = arc_queue.clone();
    let active_writer_clone = arc_active.clone();
    let handler = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        sql_saver::init(&output_dir_clone);
        loop {
            let arc_queue_clone = arc_queue_writer_clone.clone();
            let arc_active_clone = active_writer_clone.clone();
            if let Ok(sql_store) = arc_queue_clone.pop() {
                sql_saver::save(sql_store);
            } else {
                thread::sleep(Duration::from_millis(100));
            }
            if !arc_active_clone.load(Ordering::SeqCst) && arc_queue_clone.is_empty() {
                break;
            }
        }
        sql_saver::close();
    });
    for file in files {
        let limit_clone = arc_limit.clone();
        if limit_clone.load(Ordering::SeqCst) >= 8 {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        limit_clone.fetch_add(1, Ordering::SeqCst);
        let arc_queue_clone = arc_queue.clone();
        thread::spawn(move || {
            let mut parser = choose_parser(mode, convert(db_type));
            parser.setup_gen_explain(gen_explain);
            parser.setup_replace_num(replace_num);
            if let Some(sql_store) = parser.parse(&file.clone()) {
                let rs = arc_queue_clone.push(sql_store);
                if rs.is_err() {
                    warn!("push to queue failed");
                }
            }
            limit_clone.fetch_sub(1, Ordering::SeqCst);
        });
    }
    arc_active.store(false, Ordering::SeqCst);
    handler.join().unwrap();
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
