use log::{info, warn};
use std::{fs::File, io::Write, process};

/// 回车
const CRLF: [u8; 1] = [0x0a];

pub fn save(output_dir: &String, sql_store: Vec<String>) {
    info!(
        "write to {}/result.sql, size: {}",
        output_dir,
        sql_store.len()
    );
    let r = File::create(output_dir.to_string() + "/result.sql");
    if r.is_err() {
        warn!("try to write sql to {:?} failed", output_dir);
        process::exit(-1);
    }
    let mut f = r.unwrap_or_else(|_e| {
        warn!("try to write sql to {:?} failed", output_dir);
        process::exit(-1);
    });
    for sql in sql_store {
        write2file(&mut f, sql.as_bytes(), output_dir);
        write2file(&mut f, &CRLF, output_dir);
    }
    write2file(&mut f, &CRLF, output_dir);
    let fr = f.flush();
    if fr.is_err() {
        warn!("try to flush file {:?} failed", f);
        process::exit(-1);
    }
}

fn write2file(f: &mut File, bdata: &[u8], output_dir: &String) {
    let wr = f.write(bdata);
    if wr.is_err() {
        warn!("try to write [{:?}] to {:?} failed", bdata, output_dir);
        process::exit(-1);
    }
}
