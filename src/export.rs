use crate::utils::validation::{validate_directory, validate_input_file};
use crate::utils::file::{file_name, read_lines};
use crate::utils::media::is_media;
use crate::terminal::{Terminal, ExportTerm};
use std::fs::copy;

fn is_valid(media: &String) -> bool {
    validate_input_file(media);
    return if is_media(media) { true } else { panic!("{media} is not a media file.") }
}

fn get_medias(medias_list: &str) -> Vec<String> {
    return read_lines(medias_list)
    .filter(is_valid)
    .collect(); 
}

fn export(media: &str, export_directory: &str) {
    let media_name = file_name(media);
    let destination = format!("{export_directory}/{media_name}");
    copy(media, destination);
}

fn process_medias_export(medias_list: &str, export_directory: &str) {
    let lines = get_medias(medias_list);
    let mut exporter_terminal: Terminal = ExportTerm::new(lines.len(), export_directory);
    exporter_terminal.export_start();
    for line in lines {
        let media = line.as_str();
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