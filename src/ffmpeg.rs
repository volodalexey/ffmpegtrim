use pbr::ProgressBar;
use std::{
    ffi::OsStr,
    io::{stdout, BufRead, BufReader, Read, Stdout, Write},
    path::Path,
    process::{Command, Stdio},
    str::Split,
};

pub fn calc_duration(filepath: &str) -> f32 {
    let mut child = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "csv=p=0",
        ])
        .arg(filepath)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let status = child.wait().unwrap();

    let stdout = child.stdout.unwrap();

    let mut reader = BufReader::new(stdout);

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    if status.success() {
        result.pop(); // remove en of line for parsing
        let duration: f32 = result.parse().unwrap();
        if duration > 0.0 {
            return duration;
        }
        panic!("Parsed duration {duration} less then 0 (at {result})");
    }
    panic!("Unable to calc {filepath} duration! Result is {result}");
}

pub fn trim_start_end(
    input_filepath: &str,
    trim_start: &str,
    trim_end: &str,
    copy: bool,
    take_video: bool,
    take_audio: bool,
) {
    let duration = calc_duration(input_filepath);

    let mut new_duration: f32 = 0.0;
    let seconds_from_start: f32 = trim_start.parse().unwrap_or(0.0);
    let seconds_from_end: f32 = trim_end.parse().unwrap_or(0.0);
    if seconds_from_end > 0.0 {
        new_duration = duration - seconds_from_end;
        if new_duration <= 0.0 {
            panic!(
                "Required duration {new_duration} is not real ({duration} - {seconds_from_end})!",
            );
        }
    }

    let input_path = Path::new(input_filepath);
    let input_filename = input_path.file_name().unwrap().to_str().unwrap();
    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let extension = input_path.extension().unwrap().to_str().unwrap();
    let binding = input_path.with_file_name(format!("{file_stem}_tr.{extension}"));
    let output_filepath = binding.to_str().unwrap();

    let output_filename = Path::new(&output_filepath)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let mut command = Command::new("ffmpeg");
    if seconds_from_start > 0.0 {
        command.args(["-ss", &seconds_from_start.to_string()]);
    }
    if new_duration > 0.0 {
        command.args(["-to", &new_duration.to_string()]);
    }
    command.args(["-i", input_filepath, "-progress", "pipe:2"]);
    if take_video {
        command.args(["-map", "0:v:0"]);
    }
    if take_audio {
        command.args(["-map", "0:a:0"]);
    }
    if copy {
        command.args(["-c", "copy"]);
    } else {
        command.args(["-async", "1"]);
    }
    command.arg(output_filepath);

    let args: Vec<&OsStr> = command.get_args().collect();
    println!(
        "Started trim for {input_filename} {:?} => {output_filename}",
        args
    );
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let progress_duration = ((if new_duration > 0.0 {
        new_duration
    } else {
        duration
    }) - (if seconds_from_start > 0.0 {
        seconds_from_start
    } else {
        0.0
    }))
    .round() as i32;
    let mut pb: ProgressBar<Stdout> = ProgressBar::new(progress_duration as u64);
    pb.show_counter = false;
    let mut progress_started = false;

    let child_stderr = child.stderr.as_mut().expect("Unable to pipe stderr");
    let mut reader = BufReader::new(child_stderr);
    let mut buff = String::new();
    let mut out_time_found = false;
    while reader.read_line(&mut buff).expect("Unable to read line") > 0 {
        for line in buff.lines() {
            if line.contains("out_time_ms") {
                let parts: Split<&str> = line.split("=");

                let out_time_ms: i32 = parts
                    .last()
                    .expect("Can not detect out_time_ms value")
                    .trim()
                    .parse()
                    .unwrap_or(-1);

                if out_time_ms >= 0 {
                    let current_time_ms = out_time_ms / 1000000;
                    pb.set(current_time_ms as u64);
                    if !progress_started {
                        progress_started = true;
                    }
                }
                out_time_found = true;
            }
        }
        if out_time_found {
            buff.clear();
            out_time_found = false;
        }
    }
    if progress_started {
        pb.finish();
    }

    let status = child.wait().unwrap();
    if status.success() {
        return;
    } else {
        let mut self_stdout = stdout();
        self_stdout
            .write(buff.as_bytes())
            .expect("Unable to write to stdout");
    }
    panic!("Unable to complete decode!");
}
