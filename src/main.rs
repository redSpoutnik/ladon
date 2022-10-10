#![allow(unused)]

mod path;
mod ffprobe;
mod search;

use clap::Parser;
use search::search_medias_to_transcode;

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    action: Actions,
}

#[derive(clap::Subcommand)]
enum Actions {
    Search {
        #[clap(short, long, value_parser)]
        media_directory: String,
        #[clap(short, long, value_parser)]
        output_file: String
    },
}

fn main() {
    let args = Args::parse();
    match &args.action {
        Actions::Search { media_directory, output_file } => {
            search_medias_to_transcode(media_directory, output_file)
        }
    }
}
