use getopts::{Matches, Options};
use std::env;

macro_rules! fail {
    ($f:tt, $o:tt) => {{
        eprintln!("Error: {}", $f);
        eprintln!();
        return Args::fail($o);
    }};
}

const REPLACE_NUM_STR: &str = "10";
const REPLACE_NUM: i16 = 10;

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
    pub gen_explain: bool,
    pub replace_num: i16,
    pub fast_fail: bool,
    pub show_version: bool,
    opts: Options,
}

impl Args {
    fn new(
        mode: XBatisMode,
        db_type: DbType,
        src_dir: &str,
        output_dir: &str,
        gen_explain: bool,
        replace_num: i16,
        opts: Options,
    ) -> Self {
        Args {
            mode,
            db_type,
            src_dir: src_dir.to_owned(),
            output_dir: output_dir.to_owned(),
            gen_explain,
            replace_num,
            fast_fail: false,
            show_version: false,
            opts,
        }
    }

    fn fail(opts: Options) -> Self {
        Args {
            mode: XBatisMode::NotSupported,
            db_type: DbType::Unknown,
            src_dir: String::from(""),
            output_dir: String::from(""),
            gen_explain: false,
            replace_num: 0,
            fast_fail: true,
            show_version: false,
            opts,
        }
    }

    fn help(opts: Options) -> Self {
        Args {
            mode: XBatisMode::NotSupported,
            db_type: DbType::Unknown,
            src_dir: String::from(""),
            output_dir: String::from(""),
            gen_explain: false,
            replace_num: 0,
            fast_fail: false,
            show_version: true,
            opts,
        }
    }
}

/// 检查参数
pub fn check_args() -> Args {
    let opts = build_opts();
    let args: Vec<String> = env::args().collect();
    match opts.parse(&args[1..]) {
        Ok(m) => actual_check_args(opts, m),
        Err(f) => fail!(f, opts),
    }
}

fn build_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("i", "ibatis", "try to parse iBATIS sqlmap files");
    opts.optflag("m", "mybatis", "try to parse MyBatis mapper files");
    opts.optopt("t", "type", "db type", "DB");
    opts.optopt("s", "src", "source directory", "SRC");
    opts.optopt("o", "output", "output directory", "OUTPUT");
    opts.optflag("e", "explain", "generate explain sql");
    opts.optopt(
        "n",
        "num",
        "times to replace <include> tag, default is 10",
        "TIMES",
    );
    opts.optflag("v", "version", "show version information");
    opts.optflag("h", "help", "print this help menu");
    opts
}

fn actual_check_args(opts: Options, matches: Matches) -> Args {
    let help = matches.opt_present("h");
    let version = matches.opt_present("v");
    let mode_ibatis = matches.opt_present("i");
    let mode_mybatis = matches.opt_present("m");
    let o_db_type = matches.opt_str("t");
    let src_dir = matches.opt_str("s");
    let output_dir = matches.opt_str("o");
    let gen_explain = matches.opt_present("e");
    let num = matches
        .opt_str("n")
        .unwrap_or(String::from(REPLACE_NUM_STR))
        .to_string();
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
    let db_type = DbType::from(
        o_db_type
            .unwrap_or(String::from(""))
            .to_ascii_lowercase()
            .as_str(),
    );
    match db_type {
        DbType::Unknown => fail!("must choose db type in oracle or mysql", opts),
        _ => gen_args(
            opts,
            db_type,
            mode_ibatis,
            src_dir,
            output_dir,
            gen_explain,
            &num,
        ),
    }
}

fn gen_args(
    opts: Options,
    db_type: DbType,
    mode_ibatis: bool,
    src_dir: Option<String>,
    output_dir: Option<String>,
    gen_explain: bool,
    num_str: &str,
) -> Args {
    let mode = if mode_ibatis {
        XBatisMode::IBatis
    } else {
        XBatisMode::MyBatis
    };
    Args::new(
        mode,
        db_type,
        &src_dir.unwrap_or(String::from("")),
        &output_dir.unwrap_or(String::from("")),
        gen_explain,
        num_str.parse::<i16>().unwrap_or(REPLACE_NUM),
        opts,
    )
}

/// 打印使用方法
pub fn print_usage(args: &Args) {
    print!(
        "{}",
        args.opts
            .usage("Usage: xbatis2sql [-i|-m] -t [Oracle/MySQL] -s ... -o ... [-e] [-n 10]")
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
