use crate::utils::validation::validate_directory;
use crate::utils::path::{directory_entries, location_of, type_of};
use crate::utils::file::{file_name, file_directory, without_ext, delete, create_backup};
use crate::utils::media::is_media;
use crate::terminal::{Terminal, ImportTerm};
use std::collections::HashMap;
use std::fs::copy;
use itertools::Itertools;

fn origin(media: &String, medias_map: &mut HashMap<String, String>) -> String {
    return match medias_map.get(media) {
        Some(location) => location.to_string(),
        None => panic!("No path for media '{media}'!")
    };
}

fn destination(origin: &str, target: &str) -> String {
    let target_directory = file_directory(target);
    let origin_media_name = file_name(&origin);
    return format!("{target_directory}/{origin_media_name}");
}

fn import(origin: &str, target: &str) {
    let destination = &destination(origin, target);
    let target_backup = create_backup(target);
    match copy(origin, destination){
        Ok(_) => delete(&target_backup),
        Err(e) => panic!("Error moving {origin} to {destination}: {e:?}")
    }
}

fn process_media(existing_media_location: &str, medias_map: &mut HashMap<String, String>, importer_terminal: &mut Terminal) {
    let existing_media_name = file_name(existing_media_location);
    let existing_media = without_ext(existing_media_name).to_string();
    if medias_map.contains_key(&existing_media) {
        let origin = &origin(&existing_media, medias_map);
        importer_terminal.update_import(origin);
        import(origin, existing_media_location);
        medias_map.remove(&existing_media);
        importer_terminal.searching();
    }
}

fn search_recursively(location: &str, medias_map: &mut HashMap<String, String>, importer_terminal: &mut Terminal) {
    for entry in directory_entries(location) {
        if(medias_map.is_empty()) { break }
        let entry_location = location_of(&entry);
        let entry_type = type_of(&entry, &entry_location);
        if entry_type.is_dir() {
            search_recursively(&entry_location, medias_map, importer_terminal)
        } else if is_media(&entry_location) {
            process_media(&entry_location, medias_map, importer_terminal);
        }
    }
}

fn get_medias_map(input_directory_location: &str) -> HashMap<String, String> {
    return directory_entries(input_directory_location)
    .map(|entry| location_of(&entry))
    .filter(|file_location| is_media(file_location))
    .map(|media_location| (without_ext(file_name(&media_location)).to_string(), media_location))
    .collect();
}

fn fail_import(medias_map: &HashMap<String, String>) {
    let non_imported_medias = medias_map.values().join("\n");
    panic!("Import failed!\nRemaining medias not imported :\n{non_imported_medias}");
}

fn process_medias_import(input_directory_location: &str, target_directory_location: &str) {
    let mut medias_map: HashMap<String, String> = get_medias_map(input_directory_location);
    let mut importer_terminal: Terminal = ImportTerm::new(medias_map.len(), target_directory_location);
    importer_terminal.import_start();
    search_recursively(target_directory_location, &mut medias_map, &mut importer_terminal);
    if(!medias_map.is_empty()) { fail_import(&medias_map) }
    importer_terminal.import_done();
}

pub fn import_medias(input_directory_location: &str, target_directory_location: &str) {
    validate_directory(input_directory_location);
    validate_directory(target_directory_location);
    process_medias_import(input_directory_location, target_directory_location);
}