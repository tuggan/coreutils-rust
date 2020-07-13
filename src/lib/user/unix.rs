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

use std::mem;
use std::ptr;
use std::ffi::CStr;

extern crate libc;

use crate::lib::user::error::{UserError, UserResult};

pub fn get_username() -> UserResult<String> {
    unsafe {
        let bufsize = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n == -1 => 16384 as usize,
            n => n as usize,
        };
        
        let mut result = ptr::null_mut();
        let mut buffer = Vec::with_capacity(bufsize);
        let mut passwd: libc::passwd = mem::zeroed();
    
        match libc::getpwuid_r(libc::geteuid(), &mut passwd, buffer.as_mut_ptr(), buffer.capacity() as libc::size_t, &mut result) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_name as *const _;
                let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
                return Ok(username)
            },
            _ => return Err(UserError::new(1, "unable to get user passwd struct")),
        }
    }
}

