use std::env;

pub fn get_name() -> String {
    "MacOS".to_owned()
}

pub fn get_downloads_dir() -> String {
    let home = env::var("HOME").expect("HOME is not defind");
    format!("{}/Downloads", home)
}
