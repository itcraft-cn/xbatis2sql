use log::{debug, warn};
use std::{path::Path, process};
use walkdir::{DirEntry, Error, WalkDir};

/// 扫描给定的文件夹，过滤出 `xml` 文件
pub fn scan(files: &mut Vec<String>, dir: &String) {
    for d in WalkDir::new(dir).into_iter() {
        check_if_xml_file(files, d);
    }
}

/// 判断是否是 `xml` 文件。是，攒入 `files`。
fn check_if_xml_file(files: &mut Vec<String>, d: Result<DirEntry, Error>) {
    if d.is_err() {
        return;
    }
    let entry = d.unwrap_or_else(|e| {
        warn!("error while walking in directory: {}", e.to_string());
        process::exit(-1);
    });
    let file_type = entry.file_type();
    if !file_type.is_file() {
        return;
    }
    let opt_ext = Path::new(entry.file_name()).extension();
    if let Some(ext) = opt_ext {
        if "xml".eq(ext) {
            debug!("file: {:?}", entry);
            files.push(entry.path().to_string_lossy().to_string());
        }
    }
}
