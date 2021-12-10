use regex::RegexSet;
use once_cell::sync::Lazy;

static EXTENSIONS_PATTERN : Lazy <RegexSet> = Lazy::new(|| {
    // CAUTION: order is important.
    RegexSet::new(&[
    r#"\.(pdf|epub|mobi)$"#, //book
    r#"\.(mp3|wav|m4a|aac|opus|webm)$"#, //video
    r#"\.(mp4|mkv|avi|wav|mpg|webm)$"#, //audio
    r#"\.(R|py|rs|sh)$"#, //code
    r#"\.(png|jpg|jpe?g|gif)$"#, //image
    ]).unwrap()
});

pub fn file_of_filetype(file: &str, filetype: &str) -> bool {
    let matches =  EXTENSIONS_PATTERN.matches(file);
    match filetype {
        "book" => matches.matched(0),
        "audio" => matches.matched(1),
        "video" => matches.matched(2),
        "code" => matches.matched(3),
        "image" => matches.matched(4),
        extension => file.ends_with(extension),
    }
}

#[test]
fn it_processes_pdf() {
    assert!(file_of_filetype("sample.pdf", "book"));
    assert!(!file_of_filetype("samplepdf", "book"));
}

#[test]
fn it_processes_epub() {
    assert!(file_of_filetype("sample.epub", "book"));
    assert!(!file_of_filetype("samplepub", "book"));
}

#[test]
fn it_processes_mobi() {
    assert!(file_of_filetype("sample.mobi", "book"));
    assert!(!file_of_filetype("samplemobi", "book"));
}

#[test]
fn it_processes_mp3() {
    assert!(file_of_filetype("sample.mp3", "audio"));
    assert!(!file_of_filetype("samplemp3", "audio"));
}

#[test]
fn it_processes_mp4() {
    assert!(file_of_filetype("sample.mp4", "video"));
    assert!(!file_of_filetype("samplemp4", "video"));
}

#[test]
fn it_processes_png() {
    assert!(file_of_filetype("sample.png", "image"));
    assert!(!file_of_filetype("samplepng", "image"));
}

#[test]
fn it_processes_jpegs() {
    assert!(file_of_filetype("sample.jpg", "image"));
    assert!(file_of_filetype("sample.jpeg", "image"));
    assert!(!file_of_filetype("samplejpg", "image"));
    assert!(!file_of_filetype("samplejpeg", "image"));
}

#[test]
fn it_processes_r_code_files() {
    assert!(file_of_filetype("sample.R", "code"));
    assert!(!file_of_filetype("sampleR", "code"));
}
