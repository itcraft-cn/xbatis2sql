use log::*;
use std::{str::FromStr, *};

pub fn scan(files: &mut Vec<String>, dir: &String) {
    for d in walkdir::WalkDir::new(dir).into_iter() {
        if d.is_ok() {
            check_if_xml_file(files, d);
        }
    }
}

fn check_if_xml_file(files: &mut Vec<String>, d: Result<walkdir::DirEntry, walkdir::Error>) {
    let entry = d.unwrap();
    let file_type = entry.file_type();
    if file_type.is_file() {
        let ext = path::Path::new(entry.file_name()).extension();
        if ext.is_some() && ext.unwrap().eq("xml") {
            debug!("file: {:?}", entry);
            files.push(String::from_str(entry.path().to_str().unwrap()).unwrap());
        }
    }
}
