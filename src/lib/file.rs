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
use std::fs;
use std::io::Error;
use std::path::Path;

//#[cfg(unix)]
//use std::os::unix::fs::MetadataExt;

#[cfg(target_os="linux")]
use std::os::linux::fs::MetadataExt;

#[cfg(target_os="macos")]
use std::os::macos::fs::MetadataExt;


pub fn blocksize(file: &Path) -> Result<u64, Error> {
    match fs::metadata(file) {
        Ok(metadata) => {
            return Result::Ok(metadata.st_blksize());
        }
        Err(error) => return Result::Err(error),
    }
}

pub fn blocksize_as_usize(file: &Path) -> Result<usize, Error> {
    match blocksize(file) {
        Ok(blk) => {
            return Result::Ok(blk as usize);
        }
        Err(error) => return Result::Err(error),
    }
}