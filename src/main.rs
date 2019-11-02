use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::process::{Command, Stdio};
use std::str;
use std::iter::Iterator;

fn main() {
    let home = env::var("HOME").unwrap();
    let fasd_file = format!("{}/.fasd", home);

    let mut args = env::args();
    args.next();

    let filetype = args.next().unwrap();

    let mut directories: HashSet<&str> = HashSet::new();

    let mut ext_process = Command::new("dmenu").arg("-i").arg("-l").arg("3").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error opening dmenu");

    { //THIS CODE BLOCK is to localize the following borrow.
        let ext_process_stdin = ext_process.stdin.as_mut().unwrap();
        let mut ext_process_stdin = BufWriter::new(ext_process_stdin);

        let contents = fs::read_to_string(fasd_file).unwrap();
        for line in contents.lines() {
            let mut parts = line.split('|');
            match parts.next() {
                Some(file) => {
                    if file_of_filetype(file, filetype.as_ref()) && Path::new(file).exists(){
                        // println!("{}", file);
                        //FIXME: sometimes the file may be deleted but the parent directory is of interest. include that logic...
                        let file_ln = format!("{}\n", file);
                        ext_process_stdin.write_all(file_ln.as_bytes()).expect("Error sending name of file to dmenu");
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
                            // println!("{}", file);
                            let file_ln = format!("{}\n", file);
                            ext_process_stdin.write_all(file_ln.as_bytes()).expect("Error sending name of file to dmenu");
                        }
                    }
                    Err(_) =>  {eprintln!("{}", "fasd has not yet deleted inexistant files");},
                }
            }
        }
        ext_process_stdin.flush().expect("Failed to flush to stdout");
    }

    let output = ext_process.wait_with_output().expect("Error while getting chosen file from dmenu");
    println!("{}", str::from_utf8(&output.stdout).unwrap().trim());

}

fn file_has_extension(file: &str, extensions: &[&str]) -> bool{
    extensions.iter().any(|ext| file.ends_with(ext))
}

fn is_book(file: &str) -> bool {
    let book_exts = [".pdf", ".epub", ".mobi"];
    file_has_extension(file, &book_exts)
}

fn is_audio(file: &str) -> bool {
    let audio_exts = [".mp3", ".wav", ".m4a", ".aac", ".opus", ".webm"];
    file_has_extension(file, &audio_exts)
}

fn is_video(file: &str) -> bool {
    let video_exts = [".mp4",".mkv", ".avi", ".wav", ".mpg", ".webm"];
    file_has_extension(file, &video_exts)
}

fn is_image(file: &str) -> bool {
    let image_exts = [".png", ".jpg", ".gif"];
    file_has_extension(file, &image_exts)
}

fn is_code(file: &str) -> bool {
    let code_exts = [".R", ".py"];
    file_has_extension(file, &code_exts)
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

#[test]
fn it_processes_pdf() {
    assert!(is_book("sample.pdf"));
    assert!(!is_book("samplepdf"));
}

#[test]
fn it_processes_epub() {
    assert!(is_book("sample.epub"));
    assert!(!is_book("samplepub"));
}

#[test]
fn it_processes_mobi() {
    assert!(is_book("sample.mobi"));
    assert!(!is_book("samplemobi"));
}

#[test]
fn it_processes_mp3() {
    assert!(is_audio("sample.mp3"));
    assert!(!is_audio("samplemp3"));
}

#[test]
fn it_processes_mp4() {
    assert!(is_video("sample.mp4"));
    assert!(!is_video("samplemp4"));
}

#[test]
fn it_processes_png() {
    assert!(is_image("sample.png"));
    assert!(!is_image("samplepng"));
}

#[test]
fn it_processes_r_code_files() {
    assert!(is_code("sample.R"));
    assert!(!is_code("sampleR"));
}

//FIXME: case sensitive or not
