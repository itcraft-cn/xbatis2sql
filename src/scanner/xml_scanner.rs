use log::*;
use std::*;

/// 扫描给定的文件夹，过滤出 `xml` 文件
pub fn scan(files: &mut Vec<String>, dir: &String) {
    for d in walkdir::WalkDir::new(dir).into_iter() {
        if d.is_ok() {
            check_if_xml_file(files, d);
        }
    }
}

/// 判断是否是 `xml` 文件。是，攒入 `files`。
fn check_if_xml_file(files: &mut Vec<String>, d: Result<walkdir::DirEntry, walkdir::Error>) {
    let entry = d.unwrap();
    let file_type = entry.file_type();
    if file_type.is_file() {
        let ext = path::Path::new(entry.file_name()).extension();
        if ext.is_some() && ext.unwrap().eq("xml") {
            debug!("file: {:?}", entry);
            files.push(entry.path().to_string_lossy().to_string());
        }
    }
}
