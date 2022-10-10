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