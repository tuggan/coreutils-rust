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

use std::io::{self, Write};

extern crate clap;
use clap::{load_yaml, App};

fn main() {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/yes.yaml");
    let matches = App::from_yaml(yml).author(AUTHORS).get_matches();

    let mut strings: Vec<_> = matches.values_of("STRING").unwrap().collect();
    strings.push("\n");
    let output = strings.join(" ");

    let output_str = output.as_bytes();

    while io::stdout().write_all(output_str).is_ok() {
        continue;
    }
}
