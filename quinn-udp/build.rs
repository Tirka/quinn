use std::env;

fn main() {
    for (key, value) in env::vars() {
        if key.starts_with("CARGO_CFG_") {
            println!("{}: {:?}", key, value);
        }
    }
    panic!("stop and dump stdout");
}