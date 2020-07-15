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

use regex::Regex;

pub mod error;

/// Apply suffix for conversion to seconds.
///
/// Valid suffixes:
/// - s: seconds
/// - m: minutes
/// - h: hours
/// - d: days
///
/// # Return
/// Calculated new value without suffix or 0.0 if unrecognised suffix
fn apply_suffix(v: f64, c: char) -> f64 {
    let multiplier: f64 = match c {
        's' => 1.0,
        'm' => 60.0,
        'h' => 60.0 * 60.0,
        'd' => 60.0 * 60.0 * 24.0,
        _ => 0.0,
    };

    return v * multiplier;
}

/// Parse human readable time intervals and return u64 representing interval.
///
/// Example 2m becomes 120
///
/// Input must match regex `^(\d+\.)?\d+[smhd]?$`
///
/// # Error
/// If input does not match `^(\d+\.)?\d+[smhd]?$`
pub fn parse_to_seconds(s: &str) -> error::Result<f64> {
    let re = Regex::new(r"^(\d+\.)?\d+[smhd]?$").unwrap();
    let re_suffix = Regex::new(r"[smhd]$").unwrap();

    let ret: f64;

    if !re.is_match(s) {
        return Err(error::TimeError::new(
            1,
            format!("not a valid input string: {}", s).as_str(),
        ));
    }

    if re_suffix.is_match(s) {
        // Split suffix
        let time: f64 = s[..s.len() - 1].parse().unwrap();
        let suffix: char = s.chars().last().unwrap();

        ret = apply_suffix(time, suffix);
    } else {
        // No suffix
        ret = s.parse().unwrap();
    }

    return Ok(ret);
}
