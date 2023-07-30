use std::{
    io::{BufReader, Read},
    process::{Command, Stdio},
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
        let duration: f32 = result.parse().expect("Unable to parse duration as f32");
        if duration > 0.0 {
            return duration;
        }
        panic!("Parsed duration {duration} less then 0 (at {result})");
    }
    panic!("Unable to calc {filepath} duration! Result is {result}");
}
