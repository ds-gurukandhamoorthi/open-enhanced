use regex::Regex;

fn is_book(file: &str) -> bool {
    lazy_static! {
        static ref BOOK_EXTENSIONS_PATTERN: Regex = Regex::new(r#"\.(pdf|epub|mobi)$"#).unwrap();
    }
    BOOK_EXTENSIONS_PATTERN.is_match(file)
}

fn is_audio(file: &str) -> bool {
    lazy_static! {
        static ref AUDIO_EXTENSIONS_PATTERN: Regex = Regex::new(r#"\.(mp3|wav|m4a|aac|opus|webm)$"#).unwrap();
    }
    AUDIO_EXTENSIONS_PATTERN.is_match(file)
}

fn is_video(file: &str) -> bool {
    lazy_static! {
        static ref VIDEO_EXTENSIONS_PATTERN: Regex = Regex::new(r#"\.(mp4|mkv|avi|wav|mpg|webm)$"#).unwrap();
    }
    VIDEO_EXTENSIONS_PATTERN.is_match(file)
}

fn is_image(file: &str) -> bool {
    lazy_static! {
        static ref IMAGE_EXTENSIONS_PATTERN: Regex = Regex::new(r#"\.(png|jpg|jpe?g|gif)$"#).unwrap();
    }
    IMAGE_EXTENSIONS_PATTERN.is_match(file)
}

fn is_code(file: &str) -> bool {
    lazy_static! {
        static ref CODE_EXTENSIONS_PATTERN: Regex = Regex::new(r#"\.(R|py|rs|sh)$"#).unwrap();
    }
    CODE_EXTENSIONS_PATTERN.is_match(file)
}

pub fn file_of_filetype(file: &str, filetype: &str) -> bool {
    match filetype {
        "book" => is_book(file),
        "audio" => is_audio(file),
        "video" => is_video(file),
        "code" => is_code(file),
        "image" => is_image(file),
        extension => file.ends_with(extension),
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
    fn it_processes_jpegs() {
        assert!(is_image("sample.jpg"));
        assert!(is_image("sample.jpeg"));
        assert!(!is_book("samplejpg"));
        assert!(!is_book("samplejpeg"));
    }

#[test]
    fn it_processes_r_code_files() {
        assert!(is_code("sample.R"));
        assert!(!is_code("sampleR"));
    }
