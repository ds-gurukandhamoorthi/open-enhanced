use std::fs;

fn main() {
    let fasd_file = String::from("/home/guru/.fasd");
    println!("{}", fasd_file);
    let contents = fs::read_to_string(fasd_file).unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }

}
