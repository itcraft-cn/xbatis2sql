use globalvar::{fetch_global_var_mut, init_global_var};
use log::{info, warn};
use std::{fs::File, io::Write, process, sync::Mutex};

/// 回车
const CRLF: [u8; 1] = [0x0a];

pub fn init(output_dir: &String) {
    info!("write to {}/result.sql", output_dir,);
    let r = File::create(output_dir.to_string() + "/result.sql");
    if r.is_err() {
        warn!("try to write sql to {output_dir:?} failed");
        process::exit(-1);
    }
    let f = r.unwrap_or_else(|_e| {
        warn!("try to write sql to {output_dir:?} failed");
        process::exit(-1);
    });
    init_global_var("output_file", Mutex::new(f));
}

pub fn close() {
    let mtx_f = fetch_global_var_mut::<Mutex<File>>("output_file").unwrap();
    let f = mtx_f.get_mut().unwrap();
    let fr = f.flush();
    if fr.is_err() {
        warn!("try to flush file {f:?} failed");
        process::exit(-1);
    }
}

pub fn save(sql_store: Vec<String>) {
    let mtx_f = fetch_global_var_mut::<Mutex<File>>("output_file").unwrap();
    let f = mtx_f.get_mut().unwrap();
    for sql in sql_store {
        write2file(f, sql.as_bytes());
        write2file(f, &CRLF);
    }
    write2file(f, &CRLF);
}

fn write2file(f: &mut File, bdata: &[u8]) {
    let wr = f.write(bdata);
    if wr.is_err() {
        warn!("try to write [{bdata:?}] failed");
        process::exit(-1);
    }
}
