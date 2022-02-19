# prune

[![ci](https://github.com/axetroy/prune.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/axetroy/prune.rs/actions/workflows/ci.yml)
![Latest Version](https://img.shields.io/github/v/release/axetroy/prune.rs.svg)
![License](https://img.shields.io/github/license/axetroy/prune.rs.svg)
![Repo Size](https://img.shields.io/github/repo-size/axetroy/prune.rs.svg)

An extremely fast tool for prune your file-system written in Rust

The tool will traverse the target directory, look for files/directories that can be deleted (eg. `node_modules`/`bower_components`/`.temp`/`.dist`) and delete them to free up your hard disk space.

### Install

1.  Shell (Mac/Linux)

    ```bash
    curl -fsSL https://github.com/release-lab/install/raw/v1/install.sh | bash -s -- -r=axetroy/prune.rs -e=prune
    ```

2.  PowerShell (Windows):

    ```powershell
    $r="axetroy/prune.rs";$e="prune";iwr https://github.com/release-lab/install/raw/v1/install.ps1 -useb | iex
    ```

3.  [Github release page](https://github.com/axetroy/prune.rs/releases) (All platforms)

    download the executable file and put the executable file to `$PATH`

## Usage

```sh
$ prune --help
prune v0.1.0

Axetroy <axetroy.dev@gmail.com>

Prune everything

USAGE:
    prune [OPTIONS] <ROOT>

ARGS:
    <ROOT>    prune dir

OPTIONS:
    -h, --help       Print help information
    -r, --remove     remove file, defaults check only
    -V, --version    Print version information
```

## LICENSE

The [MIT License](LICENSE)
