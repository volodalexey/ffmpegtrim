use std::path::{Path, PathBuf};
use std::{env, fs};

use calc_duration::calc_duration;
use clap::Parser;
use detect_frame::detect_frame;
use make_screenshot::make_screenshot;
use trim_start_end::trim_start_end;

mod calc_duration;
mod detect_frame;
mod helpers;
mod make_screenshot;
mod trim_start_end;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short = 'd',
        long = "dir",
        help = "directory", 
        default_value_t = env::current_dir().unwrap().into_os_string().into_string().unwrap(), help = "input directory path"
    )]
    dir: String,

    #[clap(
        long = "ts",
        help = "trim seconds from start of file",
        default_value = ""
    )]
    trim_start: String,

    #[clap(long = "if", help = "intro frame file path", default_value = "")]
    intro_frame: String,

    #[clap(
        long = "ifgt",
        help = "intro frame greater than duration",
        default_value = ""
    )]
    intro_gt_duration: String,

    #[clap(
        long = "iflt",
        help = "intro frame less than duration",
        default_value = ""
    )]
    intro_lt_duration: String,

    #[clap(
        long = "ifba",
        help = "intro frame blackframe amount",
        default_value = "95"
    )]
    intro_blackframe_amount: String,

    #[clap(
        long = "ifbt",
        help = "intro frame blackframe threshold",
        default_value = "15"
    )]
    intro_blackframe_threshold: String,

    #[clap(
        long = "te",
        help = "trim seconds from end of file",
        default_value = ""
    )]
    trim_end: String,

    #[clap(long = "of", help = "outro frame file path", default_value = "")]
    outro_frame: String,

    #[clap(
        long = "ofgt",
        help = "outro frame greater than duration",
        default_value = ""
    )]
    outro_gt_duration: String,

    #[clap(
        long = "oflt",
        help = "outro frame less than duration",
        default_value = ""
    )]
    outro_lt_duration: String,

    #[clap(
        long = "ofba",
        help = "outro frame blackframe amount",
        default_value = "95"
    )]
    outro_blackframe_amount: String,

    #[clap(
        long = "ofbt",
        help = "outro frame blackframe threshold",
        default_value = "15"
    )]
    outro_blackframe_threshold: String,

    #[clap(long = "se", help = "skip encoding")]
    skip_encoding: bool,

    #[clap(
        long = "tv",
        help = "take video from input source and map to the output (skip anything else, e.g. subtitles)"
    )]
    take_video: bool,

    #[clap(
        long = "ta",
        help = "take audio from input source and map to the output (skip anything else, e.g. subtitles)"
    )]
    take_audio: bool,

    #[clap(
        short = 'x',
        long = "ext",
        help = "file(s) extension",
        default_value = "mp4"
    )]
    ext: String,

    #[clap(
        short = 'f',
        long = "filter",
        help = "process file that includes <f> in file path",
        default_value = ""
    )]
    filter: String,

    #[clap(
        short = 'l',
        long = "list",
        help = "list all file paths in current directory"
    )]
    list: bool,

    #[clap(long = "testi", help = "run test mode for images output only")]
    test_images: bool,

    #[clap(long = "testv", help = "run test mode for videos output only")]
    test_videos: bool,

    #[clap(long = "scr", help = "make screenshot at time", default_value = "")]
    make_screenshot: String,
}

fn main() {
    let args = Args::parse();

    let dir_path = Path::new(&args.dir);
    if dir_path.is_dir() != true {
        panic!(
            "Invalid dir option provided! {} is not existing directory!",
            args.dir
        );
    }

    let entries = fs::read_dir(dir_path).expect("Unable to read input ");

    let mut file_pathes: Vec<(String, String)> = Vec::new();
    for entry in entries {
        let dir_entry = entry.expect("Unable to cast DirEntry to PathBuf");
        let path = dir_entry.path();
        let path_str = path.to_str().expect("Unable to cast PathBuf to &str");
        if path.is_file() {
            let file_name = dir_entry.file_name();
            let file_name_str = file_name.to_str().expect("Unable to get file name");
            if args.list {
                println!("raw {:?}", path);
            }
            if path_str.ends_with(&args.ext)
                && (args.filter.is_empty() || path_str.contains(&args.filter))
            {
                file_pathes.push((file_name_str.to_owned(), path_str.to_owned()));
            }
        }
    }

    if file_pathes.len() == 0 {
        println!("No files to process!");
        return;
    }
    if args.list {
        println!("filtered {:?}", file_pathes);
    }

    let intro_add: f32 = 0.0;
    let outro_add: f32 = 1.0;

    for (file_name, file_path) in file_pathes {
        if !args.make_screenshot.is_empty() {
            let mut path_buf = PathBuf::from(&file_path);
            path_buf.set_extension("jpg");
            let os_string = path_buf.into_os_string();
            let screenshot_filepath = os_string
                .to_str()
                .expect("Unable to cast path buffer to string");
            make_screenshot(&file_path, &screenshot_filepath, &args.make_screenshot);
        }
        if args.trim_start.is_empty()
            && args.intro_frame.is_empty()
            && args.trim_end.is_empty()
            && args.outro_frame.is_empty()
        {
            continue;
        }
        let duration = calc_duration(&file_path);
        let mut last_intro_frame_time = -1.0;
        let mut first_outro_frame_time = -1.0;
        if !args.intro_frame.is_empty() {
            if let Some(frame_filepath) = dir_path.join(args.intro_frame.clone()).to_str() {
                last_intro_frame_time = detect_frame(
                    &file_path,
                    frame_filepath,
                    duration,
                    &args.intro_blackframe_amount,
                    &args.intro_blackframe_threshold,
                    &args.intro_gt_duration,
                    &args.intro_lt_duration,
                    false,
                ) + intro_add;
                println!(
                    "\nDetected last intro frame {} (add {})",
                    last_intro_frame_time, intro_add
                );
            }
        }
        let last_intro_frame_time_str = last_intro_frame_time.to_string();
        if args.test_images && last_intro_frame_time >= 0.0 {
            let screenshot_filename = format!("{}_intro.jpg", file_name);
            let mut path_buf = PathBuf::from(&file_path);
            path_buf.set_file_name(screenshot_filename);
            let os_string = path_buf.into_os_string();
            let screenshot_filepath = os_string
                .to_str()
                .expect("Unable to cast path buffer to string");
            make_screenshot(&file_path, &screenshot_filepath, &last_intro_frame_time_str);
        }
        if !args.outro_frame.is_empty() {
            if let Some(frame_filepath) = dir_path.join(args.outro_frame.clone()).to_str() {
                first_outro_frame_time = detect_frame(
                    &file_path,
                    frame_filepath,
                    duration,
                    &args.outro_blackframe_amount,
                    &args.outro_blackframe_threshold,
                    &args.outro_gt_duration,
                    &args.outro_lt_duration,
                    true,
                ) + outro_add;
                println!(
                    "\nDetected first outro frame {} (add {})",
                    first_outro_frame_time, outro_add
                );
            }
        }
        let outro_frame_from_end_str = (duration - first_outro_frame_time).to_string();
        let first_outro_frame_time_str = first_outro_frame_time.to_string();
        if args.test_images && first_outro_frame_time >= 0.0 {
            let screenshot_filename = format!("{}_outro.jpg", file_name);
            let mut path_buf = PathBuf::from(&file_path);
            path_buf.set_file_name(screenshot_filename);
            let os_string = path_buf.into_os_string();
            let screenshot_filepath = os_string
                .to_str()
                .expect("Unable to cast path buffer to string");
            make_screenshot(
                &file_path,
                &screenshot_filepath,
                &first_outro_frame_time_str,
            );
        }
        if args.test_images {
            continue;
        }
        trim_start_end(
            &file_path,
            duration,
            match last_intro_frame_time >= 0.0 {
                true => &last_intro_frame_time_str,
                false => &args.trim_start,
            },
            match first_outro_frame_time >= 0.0 {
                true => &outro_frame_from_end_str,
                false => &args.trim_end,
            },
            args.skip_encoding,
            args.take_video,
            args.take_audio,
        )
    }
    println!("DONE");
}
