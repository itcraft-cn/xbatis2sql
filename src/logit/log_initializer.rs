use log::{info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{env, env::consts, fs::File, process};

/// 日志初始化，写入 `stdout`，并写入临时文件夹下 `xbatis2sql.log`
pub fn init_logger() {
    let tmp_dir = env::temp_dir()
        .as_path()
        .to_str()
        .unwrap_or(".")
        .to_string();
    let log_file_name = tmp_dir + "/xbatis2sql.log";
    let log_file = create_log_file(&log_file_name);
    let rs = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ]);
    if rs.is_err() {
        eprintln!(
            "init logger error: {}",
            rs.err().unwrap_or_else(|| {
                eprintln!("create log file[{}] error", log_file_name);
                process::exit(-1);
            })
        );
        process::exit(-1);
    }
    info!(
        "log inited success, will output to stdout and {:?}",
        log_file_name
    );
}

fn create_log_file(log_file_name: &String) -> File {
    let rs_log_file = File::create(log_file_name.clone());
    rs_log_file.unwrap_or_else(|_e| {
        eprintln!("create log file error: {}", &log_file_name);
        let rs_log_file = match consts::OS {
            "windows" => File::create("xbatis2sql.log"),
            "linux" => File::create("/tmp/xbatis2sql.log"),
            "macos" => File::create("/tmp/xbatis2sql.log"),
            "unix" => File::create("/tmp/xbatis2sql.log"),
            _ => {
                eprintln!("Unknown OS: {}", consts::OS);
                process::exit(-1);
            }
        };
        rs_log_file.unwrap_or_else(|e| {
            eprintln!("create log file[{}] error: {}", &log_file_name, e);
            process::exit(-1);
        })
    })
}
