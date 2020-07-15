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

use std::{env, error};

extern crate clap;
use clap::{load_yaml, App};

mod lib;

fn main() -> Result<(), Box<dyn error::Error>> {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/pwd.yaml");
    let matches = App::from_yaml(yml).author(AUTHORS).get_matches();

    let p = matches.is_present("physical");

    let mut path = String::new();
    if let Ok(s) = env::var("PWD") {
        path = s;
    };
    if p || path.is_empty() {
        path = lib::string_from_os(env::current_dir()?.into_os_string());
    };
    println!("{}", path);

    Ok(())
}
