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

extern crate clap;
use clap::{load_yaml, App};

mod lib;
use self::lib::user;

fn main() {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/whoami.yaml");
    let _matches = App::from_yaml(yml).author(AUTHORS).get_matches();

    match user::native::get_username() {
        Err(e) => {
            eprintln!("error getting username: {}", e);
        }
        Ok(s) => {
            println!("{}", s);
        }
    }
}
