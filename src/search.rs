use crate::ffprobe::ffprobe;
use crate::ffprobe::streams::Stream;
use crate::utils::validation::{validate_directory, validate_output_file};
use crate::utils::path::{directory_entries, entry_path, file_type};
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

fn need_transcoding(media_path: &str) -> bool {
    return match ffprobe(media_path) {
        Ok(mut ffprobe) => ffprobe.any(is_invalid_media_stream),
        Err(e) => panic!("ffprobe error {media_path}: {e:?}"),
    };
}

fn should_record_file(path: &str) -> bool {
    return is_avi_file(&path) || (is_non_avi_video_file(&path) && need_transcoding(&path));
}

fn search_recursively(path: &str, output_writer_ref: Rc<RefCell<BufWriter<File>>>) {
    for entry in directory_entries(path) {
        let entry_path = entry_path(&entry);
        let entry_type = file_type(&entry, &entry_path);
        if entry_type.is_dir() {
            search_recursively(&entry_path, output_writer_ref.clone())
        } else if should_record_file(&entry_path) {
            let mut output_writer = output_writer_ref.borrow_mut();
            writeln!(output_writer, "{entry_path}");
        }
    }
}

fn output_file(output_path: &str) -> File {
    let result = File::create(output_path);
    return match result {
        Ok(output_file) => output_file,
        Err(e) => panic!("Error while creating output file {output_path:?} : {e:?}")
    }
}

fn start_searching(path: &str, output_path: &str) {
    let output_file = output_file(output_path);
    let mut output_writer = BufWriter::new(output_file);
    let output_writer_ref = Rc::new(RefCell::new(output_writer));
    search_recursively(path, output_writer_ref);
}

pub fn search_medias_to_transcode(directory_path: &str, output_path: &str) {
    println!("search directory: {:?}, write output: {:?}", directory_path, output_path);
    validate_directory(directory_path);
    validate_output_file(output_path);
    start_searching(directory_path, output_path);
}