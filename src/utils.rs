pub mod validation {

    use std::path::{Path, PathBuf};

    fn is_existing_directory(directory_path: &str) -> bool {
        return Path::new(directory_path).is_dir();
    }

    pub fn validate_directory(directory_path: &str) {
        if !is_existing_directory(directory_path) { 
            panic!("{} is not a valid directory path!", directory_path);
        }
    }

    fn is_valid_parent_path(path: &Path) -> bool {
        return path.parent().unwrap().is_dir();
    }

    pub fn validate_output_file(output_file: &str) {
        let output_file_path = Path::new(output_file);
        if output_file_path.exists() {
            if !output_file_path.is_file() { 
                panic!("{output_file} is not a file!"); 
            }
        } else if !is_valid_parent_path(output_file_path) { 
            panic!("{output_file} is not a valid file path!"); 
        }
    }

    pub fn validate_input_file(input_file: &str) {
        let input_file_path = Path::new(input_file);
        if input_file_path.exists() {
            if !input_file_path.is_file() { 
                panic!("{input_file} is not a file!"); 
            }
        } else { 
            panic!("{input_file} does not exists!"); 
        }
    }

}

pub mod path {
    use std::fs::{read_dir, DirEntry, FileType};
    use core::convert::AsRef;

    pub fn directory_entries<'a>(path: &'a str) -> impl Iterator<Item = DirEntry> + 'a{
        return match read_dir(path) {
            Ok(reader) => reader.map(|result| match result {
                Ok(entry) => entry,
                Err(e) => panic!("Error reading directory entry : {e:?}")
            }),
            Err(e) => panic!("Error reading directory entries from {path:?} : {e:?}"),
        }
    }

    pub fn entry_path(entry: &DirEntry) -> String {
        let entry_path_buf = entry.path();
        let entry_path_os = entry_path_buf.as_os_str();
        return match entry_path_os.to_str() {
            Some(entry_path) => entry_path.to_string(),
            None => panic!("Cannot convert path {entry_path_os:?} to str")
        };
    }
    
    pub fn file_type(entry: &DirEntry, entry_path: &str) -> FileType {
        return match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => panic!("Error reading {entry_path:?} file type : {e:?}")
        }
    }
}

pub mod file {
    use std::io::{Result, BufRead, BufReader};
    use std::fs::File;
    use std::path::Path;
    use substring::Substring;

    pub fn file_reader(path: &str) -> BufReader<File> {
        return match File::open(path) {
            Ok(file) => BufReader::new(file),
            Err(e) => panic!("Could not open File Reader for {path}!"),
        }
    }

    pub fn file_name(path_str: &str) -> &str {
        let file_path = Path::new(path_str);
        return match file_path.file_name() {
            Some(os_path) => match os_path.to_str() {
                Some(file_name) => file_name,
                None => panic!("Cannot get file name from OS path '{path_str}'")
            },
            None => panic!("Cannot get file name from path '{path_str}'")
        }
    }

    pub fn file_directory(file_path: &str) -> &str {
        return match Path::new(file_path).parent() {
            Some(directory) => match directory.to_str() {
                Some(directory_path) => directory_path,
                None => panic!("Cannot convert directory path to String for {file_path}")
            },
            None => panic!("Cannot access directory for file {file_path}!")
        }
    }

    pub fn without_ext(file_name: &str) -> &str {
        return match file_name.rfind('.') {
            Some(dot_index) => file_name.substring(0, dot_index),
            None => file_name
        }
    }
    
    fn to_line(result: Result<String>) -> String {
        return match result {
            Ok(line) => line,
            Err(e) => panic!("Error occured reading line from file : {e:?}"),
        }
    }
    
    pub fn read_lines(file: &str) -> impl Iterator<Item=String> {
        return file_reader(file).lines()
        .map(to_line); 
    }
}

pub mod media {

    pub fn is_media(media_path: &str) -> bool {
        return media_path.ends_with(".avi") || media_path.ends_with(".mp4") || media_path.ends_with(".mkv");
    }

    pub fn is_avi_file(media_path: &str) -> bool {
        return media_path.ends_with(".avi");
    }
    
    pub fn is_non_avi_video_file(media_path: &str) -> bool {
        return media_path.ends_with(".mp4") || media_path.ends_with(".mkv");
    }

}