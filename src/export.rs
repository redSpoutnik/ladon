use crate::path::validation::{validate_directory, validate_input_file};
use crate::path::media::is_media;
use std::fs::{copy, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn print_export_start(lines_number: usize) {
    print!("Export-- : 0/{lines_number}");
}

fn print_current_export(media: &str, index: i32, lines_number: usize) {
    print!("\rExport {media} : {index}/{lines_number}");
}

fn print_export_done(export_directory: &str) {
    println!();
    println!("Export to {export_directory} ended");
}

fn file_reader(medias_list: &str) -> BufReader<File> {
    return match File::open(medias_list) {
        Ok(medias_list_file) => BufReader::new(medias_list_file),
        Err(e) => panic!("Could not open File Reader for {medias_list}!"),
    }
}

fn is_valid(media: &String) -> bool {
    validate_input_file(media);
    return if is_media(media) { true } else { panic!("{media} is not a media file.") }
}

fn read_lines(medias_list: &str) -> Vec<String> {
    return file_reader(medias_list).lines()
    .map(|result| match result {
        Ok(line) => line,
        Err(e) => panic!("Error occured reading line from {medias_list} : {e:?}"),
    })
    .filter(is_valid)
    .collect(); 
}

fn media_name(media: &str) -> &str {
    let media_path = Path::new(media);
    return match media_path.file_name() {
        Some(os_path) => match os_path.to_str() {
            Some(file_name) => file_name,
            None => panic!("Cannot get file name from OS path '{media}'")
        },
        None => panic!("Cannot get file name from '{media}'")
    }
}

fn export(media: &str, export_directory: &str) {
    let media_name = media_name(media);
    let destination = format!("{export_directory}/{media_name}");
    copy(media, destination);
}

fn process_medias_export(medias_list: &str, export_directory: &str) {
    let lines = read_lines(medias_list);
    let lines_number = lines.len();
    let mut index = 1;
    print_export_start(lines_number);
    for line in lines {
        let media = &line;
        print_current_export(media, index, lines_number);
        export(media, export_directory);
        index += 1;
    }
    print_export_done(export_directory);
}

pub fn export_medias(medias_list: &str, export_directory: &str) {
    validate_input_file(medias_list);
    validate_directory(export_directory);
    process_medias_export(medias_list, export_directory);
}