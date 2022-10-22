use crate::utils::validation::{validate_directory, validate_input_file};
use crate::utils::file::{file_name, read_lines};
use crate::utils::media::is_media;
use crate::terminal::{Terminal, ExportTerm};
use std::fs::copy;

fn is_valid(media_location: &String) -> bool {
    validate_input_file(media_location);
    return if is_media(media_location) { true } else { panic!("{media_location} is not a media file.") }
}

fn get_medias(medias_list_location: &str) -> Vec<String> {
    return read_lines(medias_list_location)
    .filter(is_valid)
    .collect(); 
}

fn export(media_location: &str, export_directory_location: &str) {
    let media_name = file_name(media_location);
    let destination = format!("{export_directory_location}/{media_name}");
    copy(media_location, destination);
}

fn process_medias_export(medias_list_location: &str, export_directory_location: &str) {
    let lines = get_medias(medias_list_location);
    let mut exporter_terminal: Terminal = ExportTerm::new(lines.len(), export_directory_location);
    exporter_terminal.export_start();
    for line in lines {
        let media_location = line.as_str();
        exporter_terminal.update_export(media_location);
        export(media_location, export_directory_location);
    }
    exporter_terminal.export_done();
}

pub fn export_medias(medias_list_location: &str, export_directory_location: &str) {
    validate_input_file(medias_list_location);
    validate_directory(export_directory_location);
    process_medias_export(medias_list_location, export_directory_location);
}