use std::ffi::CString;


pub trait IntoCString {
    fn c_string(self) -> CString;
}

impl IntoCString for String {
    fn c_string(self) -> CString {
        return CString::new(self).unwrap()
    }
}
impl IntoCString for &str {
    fn c_string(self) -> CString {
        return CString::new(self).unwrap()
    }
}
