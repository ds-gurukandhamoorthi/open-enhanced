use std::env;
use std::fs;

fn main() {
    let home = env::var("HOME").unwrap();
    let fasd_file = String::from(format!("{}/.fasd", home));
    let contents = fs::read_to_string(fasd_file).unwrap();
    for line in contents.lines() {
        let mut parts = line.split('|');
        match parts.next() {
            Some(file) => {
                if is_book(file) {
                    println!("{}", file);
                }
            }
            None => eprintln!("{}", "Some error occurred at parsing the .fasd file"),
        }
    }
}

fn is_book(file: &str) -> bool {
    file.ends_with(".pdf") || file.ends_with(".epub") || file.ends_with(".mobi")
}
