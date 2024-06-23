use std::ffi::CString;


pub trait UnsafeIntoCString {
    fn c_string(self) -> CString;
}

impl UnsafeIntoCString for String {
    fn c_string(self) -> CString {
        return CString::new(self).unwrap()
    }
}
impl UnsafeIntoCString for &str {
    fn c_string(self) -> CString {
        return CString::new(self).unwrap()
    }
}
