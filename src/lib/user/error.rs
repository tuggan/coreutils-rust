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

use std::fmt;

pub type UserResult<T> = std::result::Result<T, UserError>;

#[derive(Debug, Clone)]
pub struct UserError {
    pub code: usize,
    pub message: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserError {{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl UserError {
    pub fn new(code: usize, message: &str) -> UserError {
        UserError{ code: code, message: String::from(message) }
    }
}
