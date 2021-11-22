use std::env;

pub fn get_name() -> String {
    "Windows".to_owned()
}

pub fn get_downloads_dir() -> String {
    let home_env = "USERPROFILE";
    let home = env::var(home_env).expect(&format!("{} is not defind", home_env));
    format!("{}\\Downloads", home)
}