use std::env;
use std::fs::File;
use std::process;

use log::*;
use simplelog::*;

mod parser;
mod scanner;

use parser::*;
use scanner::*;

fn main() {
    init_logger();
    let args: Vec<String> = env::args().collect();
    let args_len: u8 = args.len() as u8 - 1;
    if args_len == 2 {
        choose_parser(&args[1], &args[2]);
    } else {
        warn!("just need two arguments, got {} argument(s)", args_len);
        process::exit(-1);
    }
}

fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/tmp/tosql.log").unwrap(),
        ),
    ])
    .unwrap();
}

fn choose_parser(mode: &String, dir: &String) {
    if mode == "ibatis" {
        info!("try to parse ibatis sqlmap files in {:?}", dir);
        parse_ibatis(dir);
    } else if mode == "mybatis" {
        info!("try to parse mybatis mapper files in {:?}", dir);
        parse_mybatis(dir);
    } else {
        warn!("not supported: {:?}", mode);
        process::exit(-1);
    }
}

fn parse_ibatis(dir: &String) {
    debug!("{:?}", dir);
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, dir);
    ibatis_parser::parse(&files);
}

fn parse_mybatis(dir: &String) {
    debug!("{:?}", dir);
    let mut files: Vec<String> = Vec::new();
    xml_scanner::scan(&mut files, dir);
    mybatis_parser::parse(&files);
}
