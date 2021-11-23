extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

mod platform;

fn main() {
    println!("used {}", platform::get_name());
    let watch_dir = platform::get_downloads_dir();

    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(&format!("{}", watch_dir), RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
           Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
               println!("{:?} {:?} ({:?})", op, path, cookie)
           },
           Ok(event) => println!("broken event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
