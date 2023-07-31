# ffmpegtrim
CLI tool to trim off some seconds from start and/or from end of file

Process all files in the directory unless .
Trim specified seconds from start.
Trim specified seconds from end.

[ffmpeg](https://ffmpeg.org/) must be installed!

See [stackoverflow](https://stackoverflow.com/a/55337279/5431545) for more details
See [sseof](https://stackoverflow.com/a/36120894/5431545) input option for simple usage

Tested on Linux.

## Options
```shell
Usage: ffmpegtrim [OPTIONS]

Options:
  -d, --dir <DIR>              input directory path [default: /mnt/mystorage/rs_projects/ffmpegtrim]
      --ts <TRIM_START>        trim seconds from start of file [default: ]
      --if <INTRO_FRAME>       intro frame file path [default: ]
      --te <TRIM_END>          trim seconds from end of file [default: ]
      --of <OUTRO_FRAME>       outro frame file path [default: ]
      --se                     skip encoding
      --tv                     take video from input source and map to the output (skip anything else, e.g. subtitles)
      --ta                     take audio from input source and map to the output (skip anything else, e.g. subtitles)
  -x, --ext <EXT>              file(s) extension [default: mp4]
  -f, --filter <FILTER>        process file that includes <f> in file path [default: ]
  -l, --list                   list all file paths in current directory
      --testi                  run test mode for images output only
      --testv                  run test mode for videos output only
      --scr <MAKE_SCREENSHOT>  make screenshot at time [default: ]
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples
Show help
```shell
./ffmpegtrim --help
```
Trim 60 seconds from start and 20 seconds from end
```shell
./ffmpegtrim --ts 60 --te 20
```
Trim 45 seconds from start & filter files with mkv extension
```shell
./ffmpegtrim --ts 45 -x mkv
```
Trim 45 seconds from start & filter files with mp4 extension & do not re-encode, try to quick split if possible
```shell
./ffmpegtrim --ts 45 -x mp4 --se
```
Trim 48 seconds from start & keep only 5 seconds after start position & filter files with mkv extension
```shell
./ffmpegtrim --ts 48dur5 -x mkv
```
Trim 32 seconds from end & keep only 5 seconds before end position & take first video stream & take first audio stream
```shell
./ffmpegtrim --te 32dur5 --ta --tv
```
Trim 33 seconds from start & keep only 5 seconds after start position & filter files with mkv extension and path contains 01
```shell
./ffmpegtrim --ts 33dur5 -f 01 -x mkv
```
Print all files in current folder and filtered files that will be processed
```shell
./ffmpegtrim -l
```

## Build
```shell
cargo build --release
```