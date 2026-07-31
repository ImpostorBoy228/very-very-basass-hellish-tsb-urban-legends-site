use num_bigint::BigUint;
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" {
    fn its_nsecs_str(buf: *mut c_char, len: usize);
}

pub fn current_nsecs() -> BigUint {
    let mut buf = vec![0u8; 512];
    unsafe {
        its_nsecs_str(buf.as_mut_ptr() as *mut c_char, buf.len());
    }
    let s = CStr::from_bytes_until_nul(&buf)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    BigUint::parse_bytes(s.as_bytes(), 10).unwrap()
}
