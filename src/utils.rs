pub mod validation {

    use std::path::{Path, PathBuf};

    fn is_existing_directory(directory_location: &str) -> bool {
        return Path::new(directory_location).is_dir();
    }

    pub fn validate_directory(directory_location: &str) {
        if !is_existing_directory(directory_location) { 
            panic!("{} is not a valid directory path!", directory_location);
        }
    }

    fn is_valid_parent_path(path: &Path) -> bool {
        return path.parent().unwrap().is_dir();
    }

    pub fn validate_output_file(output_file_location: &str) {
        let output_file_path = Path::new(output_file_location);
        if output_file_path.exists() {
            if !output_file_path.is_file() { 
                panic!("{output_file_location} is not a file!"); 
            }
        } else if !is_valid_parent_path(output_file_path) { 
            panic!("{output_file_location} is not a valid file path!"); 
        }
    }

    pub fn validate_input_file(input_file_location: &str) {
        let input_file_path = Path::new(input_file_location);
        if input_file_path.exists() {
            if !input_file_path.is_file() { 
                panic!("{input_file_location} is not a file!"); 
            }
        } else { 
            panic!("{input_file_location} does not exists!"); 
        }
    }

}

pub mod path {
    use std::fs::{read_dir, DirEntry, FileType};
    use core::convert::AsRef;

    pub fn directory_entries<'a>(directory_location: &'a str) -> impl Iterator<Item = DirEntry> + 'a{
        return match read_dir(directory_location) {
            Ok(reader) => reader.map(|result| match result {
                Ok(entry) => entry,
                Err(e) => panic!("Error reading directory entry : {e:?}")
            }),
            Err(e) => panic!("Error reading directory entries from {directory_location:?} : {e:?}"),
        }
    }

    pub fn location_of(entry: &DirEntry) -> String {
        let entry_path_buf = entry.path();
        let entry_path_os = entry_path_buf.as_os_str();
        return match entry_path_os.to_str() {
            Some(entry_location) => entry_location.to_string(),
            None => panic!("Cannot convert path {entry_path_os:?} to str")
        };
    }
    
    pub fn type_of(entry: &DirEntry, entry_location: &str) -> FileType {
        return match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => panic!("Error reading {entry_location:?} file type : {e:?}")
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

    pub fn file_name(location: &str) -> &str {
        let file_path = Path::new(location);
        return match file_path.file_name() {
            Some(os_path) => match os_path.to_str() {
                Some(file_name) => file_name,
                None => panic!("Cannot get file name from OS path '{location}'")
            },
            None => panic!("Cannot get file name from path '{location}'")
        }
    }

    pub fn file_directory(file_location: &str) -> &str {
        return match Path::new(file_location).parent() {
            Some(directory) => match directory.to_str() {
                Some(directory_path) => directory_path,
                None => panic!("Cannot convert directory path to String for {file_location}")
            },
            None => panic!("Cannot access directory for file {file_location}!")
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
    
    pub fn read_lines(file_location: &str) -> impl Iterator<Item=String> {
        return file_reader(file_location).lines()
        .map(to_line); 
    }
}

pub mod media {

    pub fn is_media(media_location: &str) -> bool {
        return media_location.ends_with(".avi") || media_location.ends_with(".mp4") || media_location.ends_with(".mkv");
    }

    pub fn is_avi_file(media_location: &str) -> bool {
        return media_location.ends_with(".avi");
    }
    
    pub fn is_non_avi_video_file(media_location: &str) -> bool {
        return media_location.ends_with(".mp4") || media_location.ends_with(".mkv");
    }

}