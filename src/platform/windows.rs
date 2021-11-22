use std::env;

pub fn get_name() -> String {
    "Windows".to_owned()
}

pub fn get_downloads_dir() -> String {
    let home = env::var("USERPROFILE").expect("HOME is not defind");
    format!("{}\\Downloads", home)
}