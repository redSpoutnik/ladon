use crate::path::validation::{validate_directory, validate_input_file};
use crate::path::media::is_media;
use crate::terminal::{Terminal, Exporter};
use std::fs::{copy, File};
use std::io::{Result, BufRead, BufReader};
use std::path::Path;

fn file_reader(medias_list: &str) -> BufReader<File> {
    return match File::open(medias_list) {
        Ok(medias_list_file) => BufReader::new(medias_list_file),
        Err(e) => panic!("Could not open File Reader for {medias_list}!"),
    }
}

fn to_line(result: Result<String>) -> String {
    return match result {
        Ok(line) => line,
        Err(e) => panic!("Error occured reading line from file : {e:?}"),
    }
}

fn is_valid(media: &String) -> bool {
    validate_input_file(media);
    return if is_media(media) { true } else { panic!("{media} is not a media file.") }
}

fn read_lines(medias_list: &str) -> Vec<String> {
    return file_reader(medias_list).lines()
    .map(to_line)
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
    let mut exporter_terminal: Terminal = Exporter::new(lines.len(), export_directory);
    exporter_terminal.export_start();
    for line in lines {
        let media = &line;
        exporter_terminal.update_export(media);
        export(media, export_directory);
    }
    exporter_terminal.export_done();
}

pub fn export_medias(medias_list: &str, export_directory: &str) {
    validate_input_file(medias_list);
    validate_directory(export_directory);
    process_medias_export(medias_list, export_directory);
}