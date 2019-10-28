use std::fs;

fn main() {
    let fasd_file = String::from("/home/guru/.fasd");
    println!("{}", fasd_file);
    let contents = fs::read_to_string(fasd_file).unwrap();
    for line in contents.lines() {
        let mut parts = line.split('|');
        match parts.next() {
            Some(file) => println!("{}", file),
            None => eprintln!("{}", "Some error occurred at parsing the .fasd file"),
        }
    }

}
