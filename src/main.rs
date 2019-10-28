use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let home = env::var("HOME").unwrap();
    let fasd_file = String::from(format!("{}/.fasd", home));

    let mut args = env::args();
    args.next();

    let filetype = args.next().unwrap();

    let mut directories: HashSet<&str> = HashSet::new();

    let contents = fs::read_to_string(fasd_file).unwrap();
    for line in contents.lines() {
        let mut parts = line.split('|');
        match parts.next() {
            Some(file) => {
                if file_of_filetype(file, filetype.as_ref()) {
                    println!("{}", file);
                    let dir = Path::new(file).parent().unwrap();
                    match dir.as_os_str().to_str() {
                        Some(direc) => {
                            directories.insert(direc);
                        }
                        None => eprintln!("{}, {:?}", "Unable to insert", dir),
                    }
                }
            }
            None => eprintln!("{}", "Some error occurred at parsing the .fasd file"),
        }
    }
    // println!("{:?}", directories);
    for dir in directories {
        let files = fs::read_dir(dir).unwrap();
        for file in files {
            match file {
                Ok(file) => {
                    let file = format!("{}", file.path().display());
                    if file_of_filetype(file.as_ref(), filetype.as_ref()) {
                        println!("{}", file);
                    }
                }
                Err(_) => (), //fasd has not yet deleted inexistant files. FIXME: more elaborate error message
            }
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

fn file_of_filetype(file: &str, filetype: &str) -> bool {
    match filetype {
        "book" => is_book(file),
        "audio" => is_audio(file),
        "video" => is_video(file),
        "code" => is_code(file),
        "image" => is_image(file),
        _ => false,
    }
}
