use std::str;
use std::iter::repeat;
use std::io::{Error,Result};

use libc::{c_char,c_int,size_t};

extern {
    pub fn gethostname(name: *mut c_char, size: size_t) -> c_int;
}

pub fn hostname() -> Result<String> {
    let buffer_len: usize = 255;
    let mut buffer: Vec<u8> = repeat(0).take(buffer_len).collect();

    let error = unsafe {
        gethostname (buffer.as_mut_ptr() as *mut c_char, buffer_len as size_t)
    };

    if error != 0 {
        return Err(Error::last_os_error());
    }

    let len = buffer.iter().position(|b| *b == 0).unwrap_or(buffer_len);
    return Ok(str::from_utf8(&buffer[..len]).unwrap().to_string());
}

#[cfg(test)]
mod test {
    #[test]
    fn hostname() {
        super::hostname().unwrap();
    }
}
