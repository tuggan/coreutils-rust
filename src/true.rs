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

use std::env;

extern crate clap;
use clap::{load_yaml, App};

fn main() -> Result<(), clap::Error> {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/true.yaml");
    let mut matches = App::from_yaml(yml).author(AUTHORS);

    let args: Vec<String> = env::args().collect();

    // Only accept single help or version flag.
    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            matches.print_help()?;
        } else if args[1] == "-V" || args[1] == "--version" {
            matches.get_matches();
        }
        print!("\n");
    }

    Ok(())
}
