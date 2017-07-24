use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = fs::copy( Path::new(&args[1]), Path::new(&args[2]));
    match result {
        Ok(_) => { },
        Err(err) => {
            println!("COPY ERROR: {} {}", args[1], err);
        },
    }
}
