use pbr::ProgressBar;
use std::{
    io::{stdout, BufRead, BufReader, Read, Stdout, Write},
    path::Path,
    process::{Command, Stdio},
    str::Split,
};

use crate::helpers::parse_float;

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

pub struct CommandResult {
    pub command: Command,
    pub command_str: String,
    pub input_filename: String,
    pub output_filename: String,
    pub duration: f32,
    pub seconds_from_start: f32,
}

pub fn calc_command_result(
    input_filepath: &str,
    duration: f32,
    trim_start: &str,
    trim_end: &str,
    copy: bool,
    take_video: bool,
    take_audio: bool,
) -> CommandResult {
    if trim_start.contains("dur") && trim_end.contains("dur") {
        panic!("Both trim-start and trim-end options use duration value, only one allowed");
    }

    let mut new_duration: f32 = 0.0;
    let mut seconds_from_start: f32 = trim_start.parse().unwrap_or(0.0);
    let mut from_start_duration: f32 = 0.0;
    if trim_start.contains("dur") {
        let parsed = parse_float(trim_start, "dur");
        seconds_from_start = parsed.before_float;
        from_start_duration = parsed.after_float;
    }

    let mut seconds_from_end: f32 = trim_end.parse().unwrap_or(0.0);
    let mut from_end_duration: f32 = 0.0;
    if trim_end.contains("dur") {
        let parsed = parse_float(trim_end, "dur");
        seconds_from_end = parsed.before_float;
        from_end_duration = parsed.after_float;
    }

    if seconds_from_end > 0.0 && from_start_duration > 0.0 {
        panic!("Trim-start contains duration that conflicts with trim-end");
    }
    if seconds_from_start > 0.0 && from_end_duration > 0.0 {
        panic!("Trim-end contains duration that conflicts with trim-start");
    }

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
    let output_filepath = input_path.with_file_name(format!("{file_stem}_tr.{extension}"));
    let output_filename = output_filepath.file_name().unwrap().to_str().unwrap();

    let mut command = Command::new("ffmpeg");
    if from_start_duration > 0.0 {
        new_duration = seconds_from_start + from_start_duration;
    }
    if from_end_duration > 0.0 {
        seconds_from_start = new_duration - from_end_duration;
    }
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
        // command.args(["-flags", "+ildct+ilme"]); // keep interlace frame
        command.args(["-vf", "yadif"]); // remove interlacing
    }
    command.arg(output_filepath.clone());
    let command_str = command
        .get_args()
        .map(|arg| arg.to_str().unwrap())
        .collect::<Vec<&str>>()
        .join(" ");

    return CommandResult {
        command,
        command_str,
        input_filename: input_filename.to_owned(),
        output_filename: output_filename.to_owned(),
        duration: (if new_duration > 0.0 {
            new_duration
        } else {
            duration
        }) - (if seconds_from_start > 0.0 {
            seconds_from_start
        } else {
            0.0
        }),
        seconds_from_start,
    };
}

#[cfg(test)]
mod tests {
    use super::calc_command_result;

    #[test]
    fn calc_command_result_basic() {
        let command_result = calc_command_result("/f.f", 10.0, "", "", false, false, false);

        assert_eq!(command_result.input_filename, "f.f");
        assert_eq!(command_result.output_filename, "f_tr.f");
        assert_eq!(command_result.duration, 10.0);
        assert_eq!(command_result.seconds_from_start, 0.0);
        assert_eq!(
            command_result.command_str,
            "-i /f.f -progress pipe:2 -vf yadif /f_tr.f"
        );
    }

    #[test]
    fn calc_command_result_trim_start() {
        let command_result = calc_command_result("/a.mp4", 10.0, "4.5", "", true, false, false);

        assert_eq!(command_result.input_filename, "a.mp4");
        assert_eq!(command_result.output_filename, "a_tr.mp4");
        assert_eq!(command_result.duration, 5.5);
        assert_eq!(command_result.seconds_from_start, 4.5);
        assert_eq!(
            command_result.command_str,
            "-ss 4.5 -i /a.mp4 -progress pipe:2 -c copy /a_tr.mp4"
        );
    }

    #[test]
    fn calc_command_result_trim_end() {
        let command_result = calc_command_result("/b.mp4", 10.0, "", "5.46", false, true, true);

        assert_eq!(command_result.input_filename, "b.mp4");
        assert_eq!(command_result.output_filename, "b_tr.mp4");
        assert_eq!(command_result.duration, 4.54);
        assert_eq!(command_result.seconds_from_start, 0.0);
        assert_eq!(
            command_result.command_str,
            "-to 4.54 -i /b.mp4 -progress pipe:2 -map 0:v:0 -map 0:a:0 -vf yadif /b_tr.mp4"
        );
    }

    #[test]
    fn calc_command_result_trim_both() {
        let command_result =
            calc_command_result("/some/c.mp4", 10.0, "1.52", "3.33", true, false, true);

        assert_eq!(command_result.input_filename, "c.mp4");
        assert_eq!(command_result.output_filename, "c_tr.mp4");
        assert_eq!(command_result.duration, 5.15);
        assert_eq!(command_result.seconds_from_start, 1.52);
        assert_eq!(
            command_result.command_str,
            "-ss 1.52 -to 6.67 -i /some/c.mp4 -progress pipe:2 -map 0:a:0 -c copy /some/c_tr.mp4"
        );
    }

    #[test]
    #[should_panic(
        expected = "Both trim-start and trim-end options use duration value, only one allowed"
    )]
    fn calc_command_result_panic_both_dur() {
        calc_command_result("/some/c.mp4", 0.0, "1dur1", "1dur1", false, false, false);
    }

    #[test]
    fn calc_command_result_trim_start_dur() {
        let command_result =
            calc_command_result("/s/d.mp4", 10.0, "1.52dur4.5", "", false, false, true);

        assert_eq!(command_result.input_filename, "d.mp4");
        assert_eq!(command_result.output_filename, "d_tr.mp4");
        assert_eq!(command_result.duration, 4.5);
        assert_eq!(command_result.seconds_from_start, 1.52);
        assert_eq!(
            command_result.command_str,
            "-ss 1.52 -to 6.02 -i /s/d.mp4 -progress pipe:2 -map 0:a:0 -vf yadif /s/d_tr.mp4"
        );
    }
}

pub fn trim_start_end(
    input_filepath: &str,
    duration: f32,
    trim_start: &str,
    trim_end: &str,
    copy: bool,
    take_video: bool,
    take_audio: bool,
) {
    let mut command_result = calc_command_result(
        input_filepath,
        duration,
        trim_start,
        trim_end,
        copy,
        take_video,
        take_audio,
    );

    println!(
        "Started trim for => {}\nffmpeg {:?}\noutput => {}",
        command_result.input_filename, command_result.command_str, command_result.output_filename,
    );
    let mut child = command_result
        .command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let progress_duration = command_result.duration.round() as i32;
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
