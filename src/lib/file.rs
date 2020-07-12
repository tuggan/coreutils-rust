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