#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        mem,
        os::unix::prelude::OsStrExt,
        path::PathBuf,
    };

    #[test]
    fn test_whisper_init() {
        let path = PathBuf::from("./test.db");

        unsafe {
            let whisper_context =
                whisper_init(CString::new(path.as_os_str().as_bytes()).unwrap().as_ptr());
        }
    }
}
