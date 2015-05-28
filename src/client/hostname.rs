//! Wrapper around the hostname function from libc

use std::iter::repeat;
use std::io::{Error,ErrorKind,Result};

use libc::{c_char,c_int,size_t};

extern {
    pub fn gethostname(name: *mut c_char, size: size_t) -> c_int;
}

/// Calls `gethostname`
pub fn hostname() -> Result<String> {
    // Create a buffer for the hostname to be copied into
    let buffer_len: usize = 255;
    let mut buffer: Vec<u8> = repeat(0).take(buffer_len).collect();

    let error = unsafe {
        gethostname(buffer.as_mut_ptr() as *mut c_char, buffer_len as size_t)
    };

    if error != 0 {
        return Err(Error::last_os_error());
    }

    // Find the end of the string and truncate the vector to that length
    let len = buffer.iter().position(|b| *b == 0).unwrap_or(buffer_len);
    buffer.truncate(len);

    // Create an owned string from the buffer, transforming UTF-8 errors into IO errors
    String::from_utf8(buffer).map_err(|e| Error::new(ErrorKind::Other, e))
}

#[cfg(test)]
#[test]
fn test_hostname() {
    hostname().unwrap();
}
