use crate::ffprobe::streams::Stream;
use crate::ffprobe::parsers::to_stream;
use std::process::{Command, Stdio, Child, ChildStdout};
use std::io::{Result, BufReader, BufRead, Lines, Error, ErrorKind};

fn get_stream(option: Option<Stream>) -> Stream {
    return match option {
        Some(stream) => stream,
        None => panic!("Found empty stream after filtering!")
    }
}

fn to_line(result: Result<String>) -> String {
    return match result {
        Ok(line) => line,
        Err(e) => panic!("Error reading line : {e:?}")
    }
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
                .map(to_line)
                .map(to_stream)
                .filter(Option::is_some)
                .map(get_stream)
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

pub fn ffprobe<'a>(media_location: &str) -> Result<impl Iterator<Item = Stream>> {
    let mut ffprobe_result = Command::new("ffprobe")
    .args(["-show_streams","-loglevel","quiet","-print_format","compact",media_location])
    .stdout(Stdio::piped())
    .spawn();
    return ffprobe_output(ffprobe_result);
}
  
pub mod parsers {
    use crate::ffprobe::streams::{Stream, from};
    use core::str::Split;
    
    fn next_or_fail<'a>(pair: &'a mut Split<char>, chunck: &str) -> &'a str {
        return match pair.next() {
            Some(value) => value,
            None => panic!("Invalid stream chunck: {chunck}"),
        }
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
            let mut pair = chunck.split('=');
            let key = next_or_fail(&mut pair, chunck);
            match key {
                "codec_type" => {
                    let codec = next_or_fail(&mut pair, chunck);
                    if(is_valid(codec)) {
                        stream.set_codec(from(codec));
                    } else {
                        break;
                    }
                },
                "codec_name" => {
                    let name = next_or_fail(&mut pair, chunck);
                    stream.set_name(name);
                },
                "tag:language" | "TAG:language" | "tag:LANGUAGE" | "TAG:LANGUAGE" => {
                    let language = next_or_fail(&mut pair, chunck);
                    stream.set_language(language);
                },
                _ => (),
            };
            if stream.is_filled() { return Some(stream); }
        };
        return None;
    }
    
    pub fn to_stream(line: String) -> Option<Stream> {
        let mut chuncks = line.split("|");
        return match chuncks.next() {
            Some(first_chunck) => if("stream".eq(first_chunck)) { build_stream_from(chuncks) } else { None },
            None => None,
        };
    }

}

pub mod streams {
    use std::borrow::Borrow;
    use std::cmp::Eq;
    use std::fmt::{self, Debug};

    #[derive(PartialEq, Eq)]
    pub enum Codec {
        Video,
        Audio,
        Subtitle,
    }

    impl Debug for Codec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Video => write!(f, "Video"),
                Self::Audio => write!(f, "Audio"),
                Self::Subtitle => write!(f, "Subtitle"),
            }
        }
    }

    pub fn from(codec_type: &str) -> Codec {
        return match codec_type {
            "video" => Codec::Video,
            "audio" => Codec::Audio,
            "subtitle" => Codec::Subtitle,
            _ => panic!("Unrecognize codec type : '{codec_type:?}'"),
        }
    }

    pub struct Stream {
        codec: Option<Codec>,
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

        pub fn get_codec(&self) -> Option<&Codec> {
            return self.codec.as_ref();
        }

        pub fn set_codec(&mut self, codec: Codec) {
            self.codec = Some(codec);
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
                Some(codec) => Codec::Video.eq(codec),
                None => false,
            };
        }

        pub fn is_audio(&self) -> bool {
            return match self.get_codec() {
                Some(codec) => Codec::Audio.eq(codec),
                None => false,
            };
        }

        pub fn is_subtitle(&self) -> bool {
            return match self.get_codec() {
                Some(codec) => Codec::Subtitle.eq(codec),
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