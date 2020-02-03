#[macro_use] extern crate lazy_static;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::process::{Command, Stdio};
use std::str;
use std::iter::Iterator;

mod fileutils;

fn main() {
    let home = env::var("HOME").unwrap();
    let fasd_file = format!("{}/.fasd", home);

    let mut args = env::args();
    let called_by = args.next().unwrap();

    let filetype = args.next().unwrap();

    let mut directories: HashSet<&str> = HashSet::new();
    let current_folders = get_current_dirs_of_tmux();

    for f in &current_folders{
        directories.insert(f);
    }

    let mut ext_process = if called_by.contains("list") {
         Command::new("cat").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error opening dmenu")
    } else {
        let dmenu_args = ["-i", "-l", "3"];
        Command::new("dmenu").args(&dmenu_args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error opening dmenu")
    };

    { //THIS CODE BLOCK is to localize the following borrow.
        let ext_process_stdin = ext_process.stdin.as_mut().unwrap();
        let mut ext_process_stdin = BufWriter::new(ext_process_stdin);

        let contents = fs::read_to_string(fasd_file).unwrap();
        for line in contents.lines() {
            let mut parts = line.split('|');
            match parts.next() {
                Some(file) => {
                    if fileutils::file_of_filetype(file, filetype.as_ref()) && Path::new(file).exists(){
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

        show_related_files_from_directories(&filetype, directories, &mut ext_process_stdin);
        // println!("{:?}", directories);
        ext_process_stdin.flush().expect("Failed to flush to stdout");
    }

    let output = ext_process.wait_with_output().expect("Error while getting chosen file from dmenu");
    println!("{}", str::from_utf8(&output.stdout).unwrap().trim());

}


//FIXME: case sensitive or not

fn get_current_dirs_of_tmux() -> Vec<String>{
    let tmux_args = ["list-panes", "-s", "-F #{pane_current_path}"];
    let output = Command::new("tmux").args(&tmux_args).output();
    let output = output.unwrap();
    let current_folders = String::from_utf8_lossy(output.stdout.as_slice());
    let current_folders: Vec<String> = current_folders.lines().map(|f| f.trim().to_owned()).collect();
    //NOTE: does not return unique values. May not be problem as we use a set later.
    //FIXME: return current folder of current pane first
    current_folders
}

fn show_related_files_from_directories(filetype: &str, directories: HashSet<&str>, ext_process_stdin: &mut BufWriter<impl Write>) -> (){
        for dir in directories {
            let files = fs::read_dir(dir).unwrap();
            for file in files {
                match file {
                    Ok(file) => {
                        let file = format!("{}", file.path().display());
                        if fileutils::file_of_filetype(file.as_ref(), filetype.as_ref()) {
                            // println!("{}", file);
                            let file_ln = format!("{}\n", file);
                            ext_process_stdin.write_all(file_ln.as_bytes()).expect("Error sending name of file to dmenu");
                        }
                    }
                    Err(_) =>  {eprintln!("{}", "fasd has not yet deleted inexistant files");},
                }
            }
        }
}
