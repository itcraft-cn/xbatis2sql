use log::*;
use simplelog::*;
use std::env;
use std::fs::File;

pub fn init_logger() {
    let tmp_dir = env::temp_dir().as_path().to_str().unwrap().to_string();
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
            File::create(tmp_dir + "/tosql.log").unwrap(),
        ),
    ])
    .unwrap();
}
