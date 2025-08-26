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
    args::args_parser::{self, Args, DbType, XBatisMode},
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
    let (args, options) = args_parser::check_args();
    if args.fast_fail {
        args_parser::print_usage(&options);
    } else if args.show_version {
        args_parser::print_version();
    } else {
        parse_xbatis_xml(&args);
    }
}

/// 选择并执行对应的解析器
fn parse_xbatis_xml(args: &Args) {
    let src_dir = &args.src_dir;
    let output_dir = &args.output_dir;
    log_initializer::init_logger();
    info!("try to parse files in {src_dir:?}, fetch sql to {output_dir:?}");
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, src_dir);
    let arc_queue = Arc::new(ConcurrentQueue::<Vec<String>>::unbounded());
    let arc_limit = Arc::new(AtomicI16::new(0));
    let arc_active = Arc::new(AtomicBool::new(true));
    let output_dir_clone = output_dir.clone();
    let arc_queue_writer_clone = arc_queue.clone();
    let arc_active_writer_clone = arc_active.clone();
    let builder = thread::Builder::new().name("xbatis-writer".to_string());
    let handler = builder
        .spawn(move || {
            write_handle(
                output_dir_clone,
                arc_queue_writer_clone,
                arc_active_writer_clone,
            )
        })
        .unwrap();
    for file in files {
        let arc_limit_clone = arc_limit.clone();
        while arc_limit_clone.load(Ordering::SeqCst) >= 8 {
            thread::sleep(Duration::from_millis(100));
        }
        loop_parse_handle(args, &arc_queue, &arc_limit, file);
    }
    while arc_limit.load(Ordering::SeqCst) > 0 && !arc_queue.is_empty() {
        thread::sleep(Duration::from_millis(100));
    }
    arc_active.store(false, Ordering::SeqCst);
    handler.join().unwrap();
}

fn write_handle(
    output_dir: String,
    arc_queue: Arc<ConcurrentQueue<Vec<String>>>,
    arc_active: Arc<AtomicBool>,
) {
    sql_saver::init(&output_dir);
    loop {
        let arc_queue_clone = arc_queue.clone();
        let arc_active_clone = arc_active.clone();
        if !arc_active_clone.load(Ordering::SeqCst) && arc_queue_clone.is_empty() {
            info!("all sqls have been saved");
            break;
        }
        if let Ok(sql_store) = arc_queue_clone.pop() {
            sql_saver::save(sql_store);
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
    sql_saver::close();
}

fn loop_parse_handle(
    args: &Args,
    arc_queue: &Arc<ConcurrentQueue<Vec<String>>>,
    arc_limit: &Arc<AtomicI16>,
    file: String,
) {
    let arc_limit_clone = arc_limit.clone();
    let v = arc_limit_clone.fetch_add(1, Ordering::SeqCst);
    let arc_queue_clone = arc_queue.clone();
    let args_clone = args.clone();
    let builder = thread::Builder::new().name(format!("xbatis-parser-{}", v));
    let _ = builder.spawn(move || parse_handle(args_clone, file, arc_limit_clone, arc_queue_clone));
}

fn parse_handle(
    args: Args,
    file: String,
    arc_limit: Arc<AtomicI16>,
    arc_queue: Arc<ConcurrentQueue<Vec<String>>>,
) {
    let mode = args.mode;
    let db_type = args.db_type;
    let gen_explain = args.gen_explain;
    let replace_num = args.replace_num;
    let sql_limit = args.sql_limit;
    let mut parser = choose_parser(mode, convert(db_type));
    parser.setup_gen_explain(gen_explain);
    parser.setup_replace_num(replace_num);
    parser.setup_sql_limit(sql_limit);
    if let Some(sql_store) = parser.parse(&file.clone()) {
        while arc_queue.len() >= 100 {
            thread::sleep(Duration::from_millis(100));
        }
        if arc_queue.push(sql_store).is_ok() {
            //
        } else {
            warn!("push to queue failed");
        }
    }
    arc_limit.fetch_sub(1, Ordering::SeqCst);
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
