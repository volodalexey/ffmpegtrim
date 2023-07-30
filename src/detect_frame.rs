use ffmpeg_sidecar::{
    command::FfmpegCommand,
    event::{FfmpegEvent, LogLevel},
};

// let input_filepath_escaped: AsRef<OsStr> = input_filepath.as_ref();
// let mut command = Command::new("ffmpeg");
// command.args([
//     "-i",
//     input_filepath_escaped.to_str(),
//     "-r",
//     "1", // frame rate 1 frame per second
//     "-loop",
//     "1", // continue till end
//     "-i",
//     frame_filepath,
//     "-an", // no audio
//     "-filter_complex",
//     format!(
//         r#""blend=difference:shortest=1,blackframe={}:{}""#,
//         blackframe_amount, blackframe_threshold
//     )
//     .as_str(),
//     "-f",
//     "null",
//     "-",
// ]);

// let status = child.wait().unwrap();

// let stdout = child.stdout.unwrap();

// let mut reader = BufReader::new(stdout);

// let mut result = String::new();

// reader.read_to_string(&mut result).unwrap();

// if status.success() {
//     panic!("Parsed duration less then 0 (at {result})");
// }
// panic!("Unable to calc duration! Result is {result}");

pub fn detect_frame(
    input_filepath: &str,
    frame_filepath: &str,
    duration: f32,
    blackframe_amount: &str,
    blackframe_threshold: &str,
    greater_than_duration: &str,
    lower_than_duration: &str,
    first_or_last: bool,
) -> f32 {
    let greater_than_duration: f32 = greater_than_duration.parse().unwrap_or(-1.0);
    let lower_than_duration: f32 = lower_than_duration.parse().unwrap_or(-1.0);
    println!(
        "Frame analyse started... (Duration {}) gtd ({}) ltd ({})=>",
        duration, greater_than_duration, lower_than_duration
    );

    let mut first_detected_frame_duration: f32 = -1.0;
    let mut last_detected_frame_duration: f32 = -1.0;

    FfmpegCommand::new()
        .args([
            "-i",
            input_filepath,
            "-r",
            "1", // rate 1 Hz
            "-loop",
            "1", // continue till end
            "-i",
            frame_filepath,
            "-an", // no audio
            "-filter_complex",
            format!(
                "blend=difference:shortest=1,blackframe={}:{}",
                blackframe_amount, blackframe_threshold
            )
            .as_str(),
            "-f",
            "null",
            "-",
        ])
        .print_command()
        .spawn()
        .expect("Unable to spawn child process")
        .iter()
        .expect("Unable to obtain child process iterator")
        .for_each(|e| match e {
            FfmpegEvent::Log(LogLevel::Error, e) => println!("Error: {}", e),
            FfmpegEvent::Log(LogLevel::Info, msg) => {
                // e.g. [ffmpeg] [Parsed_blackframe_1 @ 0x55632ceae440] [info] frame:7711 pblack:92 pts:3948032 t:308.440000 type:B last_keyframe:7700
                if msg.contains("Parsed_blackframe_1") {
                    let parts = msg.split_whitespace();

                    for part in parts {
                        if part.starts_with("t:") {
                            let t_parts = part.split(":");

                            if let Some(t_part) = t_parts.last() {
                                // println!("t_part {}", t_part);
                                let elapsed: f32 = t_part
                                    .parse()
                                    .expect(&format!("Unable to parse time value {}", t_part));
                                if match greater_than_duration != -1.0 {
                                    true => elapsed > greater_than_duration,
                                    false => true,
                                } && match lower_than_duration != -1.0 {
                                    true => lower_than_duration > elapsed,
                                    false => true,
                                } {
                                    if first_detected_frame_duration == -1.0 {
                                        first_detected_frame_duration = elapsed;
                                    } else {
                                        last_detected_frame_duration = elapsed;
                                    }
                                }
                            }
                        }
                    }
                } else if msg.starts_with("[info]") {
                    // [info] frame=  240 fps=231 q=-0.0 size=N/A time=00:00:09.60 bitrate=N/A speed=9.25x
                    let frame_parts = msg.split("frame=");

                    if let Some(frame_str) = frame_parts.last() {
                        if let Some(frame_value) = frame_str.split_whitespace().next() {
                            if let Ok(frame) = frame_value.parse::<u32>() {
                                println!("Progress frame {}", frame);
                            }
                        }
                    }
                }
            }
            _ => {}
        });

    match first_or_last {
        true => first_detected_frame_duration,
        false => last_detected_frame_duration,
    }
}
