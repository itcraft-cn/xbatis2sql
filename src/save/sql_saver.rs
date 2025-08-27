use crate::xbatis::xbatis_parser::Parser;
use globalvar::{drop_global_var, fetch_global_var_mut, init_global_var};
use log::{info, warn};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
};

/// 回车
const CRLF: [u8; 1] = [0x0a];

struct WrappedFile {
    dir: String,
    path: PathBuf,
    file: File,
}

pub fn init(output_dir: &String) {
    info!("write to {}/result.sql", output_dir,);
    let file_name = format!("{}/{}", output_dir, "result.sql");
    let file_path = PathBuf::from(&file_name);
    let r = File::create(&file_name);
    if r.is_err() {
        warn!("try to write sql to {output_dir:?} failed");
        process::exit(-1);
    }
    let f = r.unwrap_or_else(|_e| {
        warn!("try to write sql to {output_dir:?} failed");
        process::exit(-1);
    });
    init_global_var(
        "output_file",
        Mutex::new(WrappedFile {
            dir: output_dir.clone(),
            path: file_path,
            file: f,
        }),
    );
}

pub fn close() {
    let mtx_wrapped = fetch_global_var_mut::<Mutex<WrappedFile>>("output_file").unwrap();
    let wrapped = mtx_wrapped.get_mut().unwrap();
    let f = &mut wrapped.file;
    if f.flush().is_err() {
        warn!("try to flush file {f:?} failed");
        process::exit(-1);
    }
    if f.sync_all().is_err() {
        warn!("close file {f:?} failed");
        process::exit(-1);
    }
}

pub fn save(sql_store: Vec<String>) {
    if sql_store.is_empty() {
        return;
    }
    let mtx_wrapped = fetch_global_var_mut::<Mutex<WrappedFile>>("output_file").unwrap();
    let wrapped = mtx_wrapped.get_mut().unwrap();
    let f = &mut wrapped.file;
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

pub(crate) fn rewrite(
    parser: Box<dyn Parser>,
    arc_global_inc_map: Arc<Mutex<HashMap<String, String>>>,
) {
    info!("rewrite result.sql");
    let mtx_wrapped = fetch_global_var_mut::<Mutex<WrappedFile>>("output_file").unwrap();
    let wrapped = mtx_wrapped.get_mut().unwrap();
    let new_name = format!("{}/{}", wrapped.dir, "result.tmp");
    let tmp = PathBuf::from(&new_name);
    if fs::rename(&wrapped.path, &tmp).is_ok() {
        let rf = File::open(&tmp).unwrap();
        let mut wf = File::create(&wrapped.path).unwrap();
        let buf_reader = BufReader::new(&rf);
        let mut buf_writer = BufWriter::new(&mut wf);
        buf_reader.lines().for_each(|rs| {
            let line = rs.unwrap_or_else(|_| "".to_string());
            let new_line = parser.replace_final_sql(arc_global_inc_map.clone(), &line);
            let _ = &buf_writer.write_all(new_line.as_bytes()).unwrap();
            let _ = &buf_writer.write_all(&CRLF).unwrap();
        });
        fs::remove_file(&tmp).unwrap();
    } else {
        warn!("try to rename {:?} to {tmp:?} failed", wrapped.path);
    }
    drop_global_var::<Mutex<WrappedFile>>("output_file");
    info!("rewrite result.sql done");
}
