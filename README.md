# Markdown Table of Contents Generator

This is an small cli utility when given a markdown file, the cli will parse it and create table of contents.

## Table of Contents

- [Usage](#usage)
- [Installation](#installation)
  - [Requirements](#requirements)
  - [Steps to install](#steps-to-install)
- [Planned Features](#planned-features)

## Usage

Usage is pretty straight forward. Call the utility with the location of the markdown you wish to add table of contents to. If you ever get stuck or forget which commands are available `--help` will print them out.

```bash
Generate table of contents in a markdown file

USAGE:
    mdtoc --input-file <INPUT_FILE>

OPTIONS:
    -h, --help                       Print help information
    -i, --input-file <INPUT_FILE>    Location of the markdown file
    -V, --version                    Print version information

```

Example

```bash
mdtoc --input-file README.md

# Shorthand
mdtoc -i README.md
```

## Installation

As of the latest changes to the README, there isn't an official release candidate binary available for download. In order to get this working on your system, follow the instruction below.

### Requirements

- Rust 1.60.X

### Steps to install

1. Clone this repository to a local hard drive `git clone git@github.com:VeprUA/mdtoc.git`.
2. Step into the repository `cd mdtoc`.
3. Run `cargo build --release`. This will create a binary in the `target/release/` directory.

## Planned Features

|       Status       | Description                                                           |                     Issue                      |
| :----------------: | --------------------------------------------------------------------- | :--------------------------------------------: |
| :heavy_check_mark: | Generates table of contents at the top of the markdown file.          |                                                |
|                    | Generating table of contents in a specific location of the documents. | [#1](https://github.com/VeprUA/mdtoc/issues/1) |
|                    | Escaping unsupported characters and input validation.                 | [#2](https://github.com/VeprUA/mdtoc/issues/2) |
|                    | Support for `stdin`.                                                  | [#3](https://github.com/VeprUA/mdtoc/issues/3) |
|                    | CI/CD Configuration.                                                  | [#4](https://github.com/VeprUA/mdtoc/issues/4) |
