use std::env;
use std::fs;
use std::process;

fn main() {
    let home = env::var("HOME").unwrap();
    let fasd_file = String::from(format!("{}/.fasd", home));

    let mut args = env::args();
    args.next();

    let filetype = args.next().unwrap();

    let contents = fs::read_to_string(fasd_file).unwrap();
    for line in contents.lines() {
        let mut parts = line.split('|');
        match parts.next() {
            Some(file) => match filetype.as_ref() {
                "book" => {
                    if is_book(file) {
                        println!("{}", file);
                    }
                }
                "audio" => {
                    if is_audio(file) {
                        println!("{}", file);
                    }
                }
                "video" => {
                    if is_video(file) {
                        println!("{}", file);
                    }
                }
                "code" => {
                    if is_code(file) {
                        println!("{}", file);
                    }
                }
                "image" => {
                    if is_image(file) {
                        println!("{}", file);
                    }
                }
                _ => {
                    eprintln!("{}: {}", "Unknown type", filetype);
                    process::exit(1);
                }
            },
            None => eprintln!("{}", "Some error occurred at parsing the .fasd file"),
        }
    }
}

fn is_book(file: &str) -> bool {
    file.ends_with(".pdf") || file.ends_with(".epub") || file.ends_with(".mobi")
}

fn is_audio(file: &str) -> bool {
    file.ends_with(".mp3")
        || file.ends_with(".wav")
        || file.ends_with(".m4a")
        || file.ends_with(".aac")
        || file.ends_with(".opus")
        || file.ends_with(".webm")
}

fn is_video(file: &str) -> bool {
    file.ends_with(".mp4")
        || file.ends_with(".mkv")
        || file.ends_with(".avi")
        || file.ends_with(".wav")
        || file.ends_with(".mpg")
        || file.ends_with(".webm")
}

fn is_image(file: &str) -> bool {
    file.ends_with(".png") || file.ends_with(".jpg") || file.ends_with(".gif")
}

fn is_code(file: &str) -> bool {
    file.ends_with(".R") || file.ends_with(".py")
}
