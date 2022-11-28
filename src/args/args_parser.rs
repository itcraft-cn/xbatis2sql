use getopts::*;
use std::env;

macro_rules! fail {
    ($f:tt, $o:tt) => {{
        eprintln!("Error: {}", $f);
        eprintln!();
        return Args::fail($o);
    }};
}

pub enum XBatisMode {
    NotSupported,
    IBatis,
    MyBatis,
}

pub enum DbType {
    Unknown,
    Oracle,
    MySQL,
}

impl DbType {
    fn from(name: &str) -> Self {
        match name {
            "oracle" => DbType::Oracle,
            "mysql" => DbType::MySQL,
            _ => DbType::Unknown,
        }
    }
}

pub struct Args {
    pub mode: XBatisMode,
    pub db_type: DbType,
    pub src_dir: String,
    pub output_dir: String,
    pub fast_fail: bool,
    pub show_version: bool,
    opts: Options,
}

impl Args {
    fn new(
        mode: XBatisMode,
        db_type: DbType,
        src_dir: &String,
        output_dir: &String,
        opts: Options,
    ) -> Self {
        return Args {
            mode,
            db_type,
            src_dir: src_dir.clone(),
            output_dir: output_dir.clone(),
            fast_fail: false,
            show_version: false,
            opts,
        };
    }

    fn fail(opts: Options) -> Self {
        return Args {
            mode: XBatisMode::NotSupported,
            db_type: DbType::Unknown,
            src_dir: String::from(""),
            output_dir: String::from(""),
            fast_fail: true,
            show_version: false,
            opts,
        };
    }

    fn help(opts: Options) -> Self {
        return Args {
            mode: XBatisMode::NotSupported,
            db_type: DbType::Unknown,
            src_dir: String::from(""),
            output_dir: String::from(""),
            fast_fail: false,
            show_version: true,
            opts,
        };
    }
}

/// 检查参数
pub fn check_args() -> Args {
    let opts = build_opts();
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            fail!(f, opts);
        }
    };
    let help = matches.opt_present("h");
    let version = matches.opt_present("v");
    let mode_ibatis = matches.opt_present("i");
    let mode_mybatis = matches.opt_present("m");
    let o_db_type = matches.opt_str("t");
    let src_dir = matches.opt_str("s");
    let output_dir = matches.opt_str("o");
    if help {
        return Args::fail(opts);
    } else if version {
        return Args::help(opts);
    } else if mode_ibatis && mode_mybatis {
        fail!("just support in mode: iBATIS or MyBatis, not both", opts);
    } else if !mode_ibatis && !mode_mybatis {
        fail!("must choose in iBATIS mode or MyBatis mode", opts);
    } else if o_db_type.is_none() {
        fail!("must define the db type", opts);
    } else if src_dir.is_none() {
        fail!("must define the source directory", opts);
    } else if output_dir.is_none() {
        fail!("must define the output directory", opts);
    }
    let db_type = DbType::from(o_db_type.unwrap().to_ascii_lowercase().as_str());
    match db_type {
        DbType::Unknown => {
            fail!("must choose db type in oracle or mysql", opts);
        }
        _ => {}
    }
    if mode_ibatis {
        return Args::new(
            XBatisMode::IBatis,
            db_type,
            &src_dir.unwrap(),
            &output_dir.unwrap(),
            opts,
        );
    } else {
        return Args::new(
            XBatisMode::MyBatis,
            db_type,
            &src_dir.unwrap(),
            &output_dir.unwrap(),
            opts,
        );
    }
}

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("i", "ibatis", "try to parse iBATIS sqlmap files");
    opts.optflag("m", "mybatis", "try to parse MyBatis mapper files");
    opts.optopt("t", "type", "db type", "DB");
    opts.optopt("s", "src", "source directory", "SRC");
    opts.optopt("o", "output", "output directory", "OUTPUT");
    opts.optflag("v", "version", "show version information");
    opts.optflag("h", "help", "print this help menu");
    return opts;
}

/// 打印使用方法
pub fn print_usage(args: &Args) {
    print!(
        "{}",
        args.opts
            .usage("Usage: xbatis2sql [-i|-m] -t [Oracle/MySQL] -s ... -o ...")
    );
}

pub fn print_version() {
    println!("xbatis2sql");
    println!();
    println!("\tcollect sql statements from iBATIS sqlmap files/MyBatis mapper files.");
    println!();
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!();
}
