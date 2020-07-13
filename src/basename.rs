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

use std::path::Path;
use std::ffi::OsStr;

extern crate clap;
use clap::{App, load_yaml};

fn main() {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/basename.yaml");
    let matches = App::from_yaml(yml)
        .author(AUTHORS)
        .get_matches();

    // Process all if either -a or -s is set
    let multiple = matches.is_present("multiple") || matches.is_present("suffix");
    let zero = matches.is_present("zero");
    let mut names = matches.values_of("NAME").unwrap(); // required
    let mut suffix = matches.value_of("suffix").unwrap_or_default(); // Always preset, has default value

    // If multiple and suffix is not set and there is more than 1 argument, treat the 2nd argument as suffix
    if multiple && suffix.len() < 1 && names.len() > 1 {
        suffix = names.nth(1).unwrap_or_default();
    }

    for name in names {
        let path = Path::new(name);

        // Get basename
        let basename = match path.file_name() {
            None => {
                // Special case for . and .. since file_name returns None
                let s = if name.ends_with("..") {
                    OsStr::new("..")
                } else if name.ends_with(".") {
                    OsStr::new(".")
                } else {
                    OsStr::new("")
                };
                s
            },
            Some(s) => s,
        };

        // Convert to str
        let mut basestr = match basename.to_str() {
            None => "",
            Some(s) => s,
        };

        // Remove suffix if detected
        if suffix.len() > 0 && basestr.ends_with(suffix) {
            basestr = &basestr[..basestr.len()-suffix.len()]
        }

        // Print basename
        print!("{}", basestr);

        // If -z not specified print newline
        if !zero {
            print!("\n");
        }

        // If multiline not set break after first loop
        if !multiple {
            break;
        }
    }
}