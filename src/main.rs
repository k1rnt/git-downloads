mod platform;

fn main() {
    println!("used {}", platform::get_name());
    println!("{}", platform::get_downloads_dir());
}
