# uptobox-dl (Only Premium)

## Overview

This project is inspired by [Aerion](https://github.com/Aerion/uptobox-dl).

With `uptobox-dl`, you're able to download multiple uptobox/uptostream links without any other action other than grabbing the links and running `uptobox-dl`.

*Note: Uptobox is a file hosting provider*.


## Example

With multiple links
```
$ ./uptobox-dl -t <my_user_token> -l "https://uptobox.com/<filecode_1> https://uptostream.com/iframe/<filecode_2>"
```

## Usage

### CLI

`./uptobox-dl -t <my_user_token> -l [my_links...]`

```
./uptobox-dl --help

USAGE:
    uptobox-dl --token <TOKEN> --links <LINKS>

OPTIONS:
    -h, --help             Print help information
    -l, --links <LINKS>    Uptobox links
    -t, --token <TOKEN>    Uptobox api token
```

### Build from source

```
cargo build --release
cd target/release
./uptobox-dl --help
```