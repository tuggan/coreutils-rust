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

use std::ffi::CStr;
use std::mem;

extern crate libc;

use crate::lib::env::error::{EnvError, Result};

pub fn get_cwd<'a>() -> Result<String> {
    let path = unsafe {
        let mut buf = Vec::with_capacity(libc::PATH_MAX as usize);

        if libc::getcwd(buf.as_mut_ptr(), buf.capacity()).is_null() {
            return Err(EnvError::new(1, "unable to get current working directory"));
        }

        // Make sure to set size of buf to get the correct string.
        let s = CStr::from_ptr(buf.as_ptr());
        buf.set_len(s.to_bytes().len());

        String::from_utf8(mem::transmute(buf)).unwrap()
    };

    return Ok(path.into());
}
