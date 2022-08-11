use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn Hello(name: GoString) -> *const c_char;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

pub fn hello(name: &str) -> String {
    let c_name = CString::new(name).unwrap();
    let ptr_name = c_name.as_ptr();

    let go_string_name = GoString {
        a: ptr_name,
        b: c_name.as_bytes().len() as i64,
    };

    let res = unsafe { Hello(go_string_name) };
    let c_str = unsafe { CStr::from_ptr(res) };

    c_str.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn it_works() {
        assert_eq!("Hello, mate!", hello("mate!"));
    }
}
