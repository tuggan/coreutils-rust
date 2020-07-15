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
use std::fmt;

pub type Result<T> = std::result::Result<T, TimeError>;

#[derive(Debug, Clone)]
pub struct TimeError {
    pub code: usize,
    pub error_type: String,
    pub message: String,
}

impl error::Error for TimeError {}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserError {{ code: {} ({}), message: {} }}",
            self.error_type, self.code, self.message
        )
    }
}

impl TimeError {
    /// Returns a new TimeError object
    ///
    /// Code is used to create a readable error type therefore only implemented codes are supported
    ///
    /// # Codes
    /// - 1: Parse error
    pub fn new(code: usize, message: &str) -> TimeError {
        let code_str: [&str; 1] = ["ParseError"];
        TimeError {
            code: code,
            error_type: String::from(code_str[code - 1]),
            message: String::from(message),
        }
    }
}
