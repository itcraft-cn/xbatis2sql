use log::*;

pub fn parse(files: &Vec<String>) {
    for file in files {
        info!("{:?}", file);
    }
}
