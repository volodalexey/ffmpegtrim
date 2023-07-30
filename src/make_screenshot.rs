use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};

pub fn make_screenshot(input_filepath: &str, output_filepath: &str, frame_time: &str) {
    println!("Make screenshot started... =>");

    // ffmpeg -ss 00:00:19.32 -i 01.\ Хорошие\ манеры.mp4 -vframes 1 -q:v 2 output2.jpg
    FfmpegCommand::new()
        .args([
            "-ss",
            frame_time,
            "-i",
            input_filepath,
            "-vframes",
            "1",
            "-q:v",
            "2",
        ])
        .output(output_filepath)
        .print_command()
        .spawn()
        .expect("Unable to spawn child process")
        .iter()
        .expect("Unable to obtain child process iterator")
        .for_each(|e| match e {
            FfmpegEvent::Error(err) => println!("Error making screenshot\n{}", err),
            _ => {}
        });
}
