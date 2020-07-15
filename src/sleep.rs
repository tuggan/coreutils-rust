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

use std::error;
use std::{process, thread, time};

extern crate clap;
use clap::{load_yaml, App};

extern crate signal_hook;
use signal_hook::{iterator::Signals, SIGINT, SIGTERM, SIGUSR1, SIGUSR2};

mod lib;
use lib::time as libtime;

fn main() -> Result<(), Box<dyn error::Error>> {
    let now = time::Instant::now();

    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let yml = load_yaml!("args/sleep.yaml");
    let matches = App::from_yaml(yml).author(AUTHORS).get_matches();

    // This will always unwrap since NUMBER is required
    let numbers = matches.values_of("NUMBER").unwrap();

    let mut sleep: f64 = 0.0;

    for number in numbers {
        if let Err(e) = libtime::parse_to_seconds(number) {
            return Err(e.into());
        }
        sleep += match libtime::parse_to_seconds(number) {
            Ok(n) => n,
            Err(e) => return Err(e.into()),
        }
    }

    let duration = time::Duration::from_secs_f64(sleep);

    // Setup signal handling
    let signals = Signals::new(&[SIGINT, SIGTERM, SIGUSR1, SIGUSR2]).unwrap();
    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == (SIGINT | SIGTERM) {
                if now.elapsed() < duration {
                    eprintln!("\ronly slept for {} seconds", now.elapsed().as_secs_f64());
                }
                // exit with signal code
                process::exit(128 + sig);
            } else if sig == SIGUSR1 {
                eprintln!("slept for {} seconds", now.elapsed().as_secs_f64());
            } else if sig == SIGUSR2 {
            } else {
                process::exit(128 + sig);
            }
        }
    });

    // println!("Removing {} secs", (duration - (duration - now.elapsed())).as_secs_f64() );
    match duration.checked_sub(now.elapsed()) {
        None => eprintln!("time already elapsed, unable to sleep"),
        Some(t) => thread::sleep(t),
    };

    // println!("Slept for {} seconds", now.elapsed().as_secs_f64());
    if !(now.elapsed() >= duration) {
        eprintln!("only slept for {} seconds", now.elapsed().as_secs_f64());
    };

    Ok(())
}
