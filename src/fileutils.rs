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

    pub fn file_of_filetype(file: &str, filetype: &str) -> bool {
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
