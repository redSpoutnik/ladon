use crate::ffprobe::ffprobe;
use crate::ffprobe::streams::Stream;
use crate::utils::validation::{validate_directory, validate_output_file};
use crate::utils::path::{directory_entries, location_of, type_of};
use crate::utils::media::{is_avi_file, is_non_avi_video_file};
use std::borrow::Borrow;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{Result, BufWriter, Write};

fn is_invalid_media_stream(stream: Stream) -> bool {
    return match stream.get_codec() {
        Some(codec) => match codec.as_str() {
            "video" => return !stream.is_valid_video_stream(),
            "audio" => return !stream.is_valid_audio_stream(),
            "subtitle" => return !stream.is_valid_subtitle_stream(),
            _ => panic!("Invalid stream processing : codec type '{codec:?}'"),
        },
        None => panic!("Invalid stream processing : no codec type")
    };
}

fn need_transcoding(media_location: &str) -> bool {
    return match ffprobe(media_location) {
        Ok(mut ffprobe) => ffprobe.any(is_invalid_media_stream),
        Err(e) => panic!("ffprobe error {media_location}: {e:?}"),
    };
}

fn should_record_file(location: &str) -> bool {
    return is_avi_file(location) || (is_non_avi_video_file(location) && need_transcoding(location));
}

fn search_recursively(directory_location: &str, output_writer_ref: Rc<RefCell<BufWriter<File>>>) {
    for entry in directory_entries(directory_location) {
        let entry_location = location_of(&entry);
        let entry_type = type_of(&entry, &entry_location);
        if entry_type.is_dir() {
            search_recursively(&entry_location, output_writer_ref.clone())
        } else if should_record_file(&entry_location) {
            let mut output_writer = output_writer_ref.borrow_mut();
            writeln!(output_writer, "{entry_location}");
        }
    }
}

fn output_file(output_file_location: &str) -> File {
    return match File::create(output_file_location) {
        Ok(output_file) => output_file,
        Err(e) => panic!("Error while creating output file {output_file_location:?} : {e:?}")
    }
}

fn start_searching(directory_location: &str, output_file_location: &str) {
    let output_file = output_file(output_file_location);
    let mut output_writer = BufWriter::new(output_file);
    let output_writer_ref = Rc::new(RefCell::new(output_writer));
    search_recursively(directory_location, output_writer_ref);
}

pub fn search_medias_to_transcode(directory_location: &str, output_file_location: &str) {
    println!("search directory: {directory_location:?}, write output: {output_file_location:?}");
    validate_directory(directory_location);
    validate_output_file(output_file_location);
    start_searching(directory_location, output_file_location);
}