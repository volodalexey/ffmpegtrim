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
```shell
./ffmpegtrim --help
./ffmpegtrim -s 60 -e 20
./ffmpegtrim -s 45 -x mkv
./ffmpegtrim -s 45 -x mp4 -c
```

## Build
```shell
cargo build --release
```