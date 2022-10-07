# ffmpegtrim
Rust cli tool to trim off last seconds from file

Process all files in the directory.
Trim specified seconds from start.
Trim specified seconds from end.

[ffmpeg](https://ffmpeg.org/) must be installed!

See [stackoverflow](https://stackoverflow.com/a/55337279/5431545) for more details

Tested on Linux.

## Options
```shell
USAGE:
    ffmpegtrim [OPTIONS]

OPTIONS:
    -a, --take-audio                 take audio from input source and map to the output (skip
                                     anything else, e.g. subtitles)
    -c, --copy                       copy without encoding
    -d, --dir <DIR>                  input directory path [default: /mnt/mystorage/Video]
    -e, --trim-end <TRIM_END>        trim seconds from end of file [default: ]
    -h, --help                       Print help information
    -s, --trim-start <TRIM_START>    trim seconds from start of file [default: ]
    -v, --take-video                 take video from input source and map to the output (skip
                                     anything else, e.g. subtitles)
    -V, --version                    Print version information
    -x, --ext <EXT>                  file extension [default: mp4]
```

## Examples
Show help
```shell
./ffmpegtrim --help
```
Trim 60 seconds from start and 20 seconds from end
```shell
./ffmpegtrim -s 60 -e 20
```
Trim 45 seconds from start & filter files with mkv extension
```shell
./ffmpegtrim -s 45 -x mkv
```
Trim 45 seconds from start & filter files with mp4 extension & do not re-encode, try to quick split if possible
```shell
./ffmpegtrim -s 45 -x mp4 -c
```
Trim 48 seconds from start & keep only 5 seconds after start position & filter files with mkv extension
```shell
./ffmpegtrim -s 48dur5 -x mkv
```
Trim 32 seconds from end & keep only 5 seconds before end position & take first video stream & take first audio stream
```shell
./ffmpegtrim -e 32dur5 -a -v
```

## Build
```shell
cargo build --release
```