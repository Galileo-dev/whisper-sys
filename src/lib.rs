#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::CStr, mem};

    #[test]
    fn whisper_call_context() {
        let path = "test.db".as_bytes();
        unsafe {
            let whisper_context = whisper_init(path.as_ptr() as *const char);
        }
    }
}
