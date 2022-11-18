use log::*;
use simplelog::*;
use std::env;
use std::fs::File;

pub fn init_logger() {
    let tmp_dir = env::temp_dir().as_path().to_str().unwrap().to_string();
    let log_file_name = tmp_dir + "/xbatis2sql.log";
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
            File::create(&log_file_name).unwrap(),
        ),
    ])
    .unwrap();
    info!(
        "log inited success, will output to stdout and {:?}",
        log_file_name
    );
}
