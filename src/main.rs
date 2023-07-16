use std::path::Path;
use std::{env, fs};

use clap::Parser;
use ffmpeg::{calc_duration, trim_start_end};

mod ffmpeg;
mod helpers;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'd', long = "dir", default_value_t = env::current_dir().unwrap().into_os_string().into_string().unwrap(), help = "input directory path")]
    dir: String,

    #[clap(
        short = 's',
        long = "trim-start",
        help = "trim seconds from start of file",
        default_value = ""
    )]
    trim_start: String,

    #[clap(
        short = 'e',
        long = "trim-end",
        help = "trim seconds from end of file",
        default_value = ""
    )]
    trim_end: String,

    #[clap(short = 'c', long = "copy", help = "copy without encoding")]
    copy: bool,

    #[clap(
        short = 'v',
        long = "take-video",
        help = "take video from input source and map to the output (skip anything else, e.g. subtitles)"
    )]
    take_video: bool,

    #[clap(
        short = 'a',
        long = "take-audio",
        help = "take audio from input source and map to the output (skip anything else, e.g. subtitles)"
    )]
    take_audio: bool,

    #[clap(
        short = 'x',
        long = "ext",
        help = "file extension",
        default_value = "mp4"
    )]
    ext: String,

    #[clap(
        short = 'i',
        long = "includes",
        help = "process file that includes <i> in file path",
        default_value = ""
    )]
    includes: String,

    #[clap(
        short = 'l',
        long = "list",
        help = "list all file paths in current directory"
    )]
    list: bool,
}

fn main() {
    let args = Args::parse();

    if Path::new(&args.dir).is_dir() != true {
        panic!(
            "Invalid dir option provided! {} is not existing directory!",
            args.dir
        );
    }

    let entries = fs::read_dir(args.dir).expect("Unable to read input ");

    println!("Ends with \"{}\"", args.ext);

    let mut file_pathes: Vec<String> = Vec::new();
    for entry in entries {
        let path = entry.expect("Unable to cast DirEntry to PathBuf").path();
        let path_str = path.to_str().expect("Unable to cast PathBuf to &str");
        if path.is_file() {
            if args.list {
                println!("{:?}", path);
            }
            if path_str.ends_with(&args.ext)
                && (args.includes.is_empty() || path_str.contains(&args.includes))
            {
                file_pathes.push(path_str.to_owned());
            }
        }
    }

    if file_pathes.len() > 0 {
        if args.trim_start.is_empty() && args.trim_end.is_empty() {
            panic!("Either trim-start or trim-end option must be provided!");
        }
        for file_path in file_pathes {
            trim_start_end(
                &file_path,
                calc_duration(&file_path),
                &args.trim_start,
                &args.trim_end,
                args.copy,
                args.take_video,
                args.take_audio,
            )
        }
        println!("DONE");
    } else {
        println!("EMPTY");
    }
}
