/*
    Copyright 2020 Dennis Vesterlund <dennisvesterlund@gmail.com>

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/
use std::fs::File;
use std::io::{self, BufWriter, BufReader, BufRead, Write, Read};
use std::path::Path;

extern crate clap;
use clap::{App, load_yaml};

#[allow(dead_code)]
mod lib;

#[derive(Debug)]
struct CopyOptions {
    nnempty: bool, // Number non empty lines
    sends: bool, // Add $ at end of each line
    nline: bool, // Number lines
    sblank: bool, // Squeeze blank lines
}

fn main() -> io::Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let yml = load_yaml!("args/cat.yaml");
    let matches = App::from_yaml(yml)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .get_matches();


    let options = CopyOptions {
        nnempty: matches.is_present("numberNonBlank"),
        sends: matches.is_present("showEnds"),
        nline: matches.is_present("numberLines"),
        sblank: matches.is_present("squeezeBlanks"),
    };

    let number = matches.is_present("number");

    if number {
        print!("Will print linenumbers!");
    }

    let files = matches.values_of("INPUT").unwrap();

    for file in files {
        let mut reader = match get_file_reader(file) {
            Err(why) => {
                eprintln!("Failed to open file: {}", why);
                continue;
            },
            Ok(reader) => reader,
        };
        if !(options.nnempty || options.sends || options.nline || options.sblank) {
            copy_to_stdout(&mut reader);
        } else {
            special_reader_writer(&mut reader, &options)
        }
    }
    Ok(())
}


// Get a reader for file or stdin
fn get_file_reader(path: &str) -> Result<Box<dyn Read>, io::Error> {
    let reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        let path = Path::new(path);
        let fd = match File::open(&path) {
            Err(why) => {
                return Err(why)
            },
            Ok(fd) => fd,
        };
        Box::new(fd)
    };
    return Ok(reader);
}


// Copy reader directly to stdout
fn copy_to_stdout(reader: &mut Box<dyn Read>) {
    let writer: Box<dyn Write> = Box::new(io::stdout());
    let mut buffered = BufWriter::new(writer);
    if let Err(e) = io::copy(reader, &mut buffered) {
        panic!("Unable to copy file to stdout: {}", e);
    };
    if buffered.buffer().last().unwrap() != &b'\n' {
        if let Err(e) = buffered.write(b"\n") {
            panic!("Unable to write to stdout: {}", e);
        };
    };
}


// Copy reader to stdout with special options
fn special_reader_writer(reader: &mut Box<dyn Read>, options: &CopyOptions) {
    let mut buf: Vec<u8> = Vec::new();
    let mut bufreader = BufReader::new(reader);
    let mut line: u64 = 0;
    let mut last_line_blank = false;
    while let Ok(n) = bufreader.read_until(b'\n', &mut buf) {
        if n == 0 {
            break;
        } else if options.sblank && last_line_blank && n == 1 {
            buf.clear();
            continue;
        } else if n == 1 {
            last_line_blank = true;
        } else {
            last_line_blank = false;
        };
        if ((options.nline && !options.nnempty) || (options.nnempty && buf.len() > 1)) && buf.last().unwrap() == &b'\n' {
            line += 1;
            print!("{:6}  ", line);
        };
        if let Err(e) = io::stdout().write_all(&buf) {
            panic!("Unable to write to stdout: {}", e);
        }
        buf.clear();
    }
}
