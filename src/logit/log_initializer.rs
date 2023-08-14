use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::{process, sync::Once};

const LOG_FORMAT: &str = "[{l}] {m}{n}";

/// 日志初始化，写入 `stdout`，并写入临时文件夹下 `xbatis2sql.log`
pub(crate) fn init_logger() {
    static INIT: Once = Once::new();
    INIT.call_once(init_log4rs);
}

fn init_log4rs() {
    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT)))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap_or_else(|e| {
            eprintln!("hit error: {}", e.to_string());
            process::exit(-1);
        });
    let rs = log4rs::init_config(config);
    if rs.is_err() {
        eprintln!(
            "hit error: {}",
            rs.err().unwrap_or_else(|| {
                eprintln!("hit error, failed to parse log4rs config then create the logger");
                process::exit(-1);
            })
        );
        process::exit(-1);
    }
}
