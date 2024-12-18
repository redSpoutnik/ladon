#![allow(unused)]

mod utils;
mod terminal;
mod ffprobe;
mod search;
mod export;
mod import;

use clap::Parser;
use search::search_medias_to_transcode;
use export::export_medias;
use import::import_medias;

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
    Export {
        #[clap(short, long, value_parser)]
        medias_list: String,
        #[clap(short, long, value_parser)]
        export_directory: String
    },
    Import {
        #[clap(short, long, value_parser)]
        input_directory: String,
        #[clap(short, long, value_parser)]
        target_directory: String
    }
}

fn main() {
    let args = Args::parse();
    match &args.action {
        Actions::Search { media_directory, output_file } => {
            search_medias_to_transcode(media_directory, output_file)
        },
        Actions::Export { medias_list, export_directory } => {
            export_medias(medias_list, export_directory)
        },
        Actions::Import { input_directory, target_directory } => {
            import_medias(input_directory, target_directory)
        },
    }
}
