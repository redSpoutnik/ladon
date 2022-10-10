use std::process::{Command, Stdio, Child, ChildStdout};
use std::io::{Result, BufReader, BufRead, Lines, Error, ErrorKind};
use core::str::Split;
use crate::ffprobe::streams::Stream;
use substring::Substring;

fn split_index(chunck: &str) -> usize {
    return match chunck.find('=') {
        Some(index) => index,
        None => panic!("Invalid stream chunck: {}", chunck),
    };
}

fn key(chunck: &str, split_index: usize) -> &str {
    return chunck.substring(0, split_index);
}

fn value(chunck: &str, split_index: usize) -> &str {
    return chunck.substring(split_index + 1, chunck.len());
}

fn is_valid(codec: &str) -> bool {
    return match codec {
        "video" | "audio" | "subtitle" => true,
        _ => false,
    };
}

fn build_stream_from(mut chuncks: Split<&str>) -> Option<Stream> {
    let mut stream = Stream::new();
    while let Some(chunck) = chuncks.next() {
        let split_index = split_index(chunck);
        let key = key(chunck, split_index);
        match key {
            "codec_type" => {
                let codec = value(chunck, split_index);
                if(is_valid(codec)) {
                    stream.set_codec(codec);
                } else {
                    println!("Invalid codec: {:?}", codec);
                    break;
                }
            },
            "codec_name" => {
                let name = value(chunck, split_index);
                stream.set_name(name);
            },
            "tag:language" | "TAG:language" | "tag:LANGUAGE" | "TAG:LANGUAGE" => {
                let language = value(chunck, split_index);
                stream.set_language(language);
            },
            _ => (),
        };
        if stream.is_filled() { return Some(stream); }
    };
    return None;
}

fn to_stream(line: String) -> Option<Stream> {
    let mut chuncks = line.split("|");
    return match chuncks.next() {
        Some(first_chunck) => if("stream".eq(first_chunck)) { build_stream_from(chuncks) } else { None },
        None => None,
    };
}

fn stdout_reader(mut ffprobe_process: Child) -> Result<BufReader<ChildStdout>> {
    let opt = ffprobe_process.stdout.take();
    return match opt {
        Some(stdout) => Ok(BufReader::new(stdout)),
        None => Err(Error::new(ErrorKind::Other, "Cannot get stdout from process")),
    };
}

fn stdout_streams(mut ffprobe_process: Child) -> Result<impl Iterator<Item = Stream>> {
    return match stdout_reader(ffprobe_process) {
        Ok(stdout_reader) => Ok(
                stdout_reader.lines()
                .map(Result::unwrap)
                .map(to_stream)
                .filter(Option::is_some)
                .map(Option::unwrap)
            ),
        Err(e) => Err(e),
    };
}

fn ffprobe_output(result: Result<Child>) -> Result<impl Iterator<Item = Stream>> {
    return match result {
        Ok(ffprobe_process) => stdout_streams(ffprobe_process),
        Err(e) => Err(e),
    };
}

pub fn ffprobe<'a>(media_path: &str) -> Result<impl Iterator<Item = Stream>> {
    let mut ffprobe_result = Command::new("ffprobe")
    .args(["-show_streams","-loglevel","quiet","-print_format","compact",media_path])
    .stdout(Stdio::piped())
    .spawn();
    return ffprobe_output(ffprobe_result);
}
    

pub mod streams {
    use std::borrow::Borrow;

    pub struct Stream {
        codec: Option<String>,
        name: Option<String>,
        language: Option<String>,
    }
    
    impl Stream {
    
        pub fn new() -> Stream {
            Stream {
                codec: None,
                name: None,
                language: None,
            }
        }

        pub fn get_codec(&self) -> Option<&String> {
            return self.codec.as_ref();
        }

        pub fn set_codec(&mut self, codec: &str) {
            self.codec = Some(codec.to_string());
        }

        pub fn get_name(&self) -> Option<&String> {
            return self.name.as_ref();
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }

        pub fn get_language(&self) -> Option<&String> {
            return self.language.as_ref();
        }

        pub fn set_language(&mut self, language: &str) {
            self.language = Some(language.to_string());
        }
    
        pub fn is_video(&self) -> bool {
            return match self.get_codec() {
                Some(codec) => "video".eq(codec),
                None => false,
            };
        }

        pub fn is_audio(&self) -> bool {
            return match self.get_codec() {
                Some(codec) => "audio".eq(codec),
                None => false,
            };
        }

        pub fn is_subtitle(&self) -> bool {
            return match self.get_codec() {
                Some(codec) => "subtitle".eq(codec),
                None => false,
            };
        }
    
        pub fn is_filled(&self) -> bool {
            return self.codec.is_some() && self.name.is_some() && (self.is_video() || self.language.is_some());
        }
    
        pub fn is_valid_video_stream(&self) -> bool {
            return self.is_video() && match self.get_name() {
                Some(name) => "h264".eq(name),
                None => false,
            };
        }
    
        pub fn is_valid_audio_stream(&self) -> bool {
            return self.is_audio() && match self.get_name() {
                Some(name) => "aac".eq(name),
                None => false,
            };
        }
    
        pub fn is_valid_subtitle_stream(&self) -> bool {
            return self.is_subtitle() && match self.get_language() {
                Some(language) => match language.as_str() {
                    "fra" | "fre" | "eng" | "und" => true,
                    _ => false,
                },
                None => false
            };
        }
    
    }

}