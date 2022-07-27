use std::ffi::CString;

pub fn str_to_local_multi_byte_string(s: &str) -> Result<CString, std::io::Error> {
    let cp = unsafe { windows::Win32::Globalization::GetACP() };
    Ok(unsafe {
        CString::from_vec_unchecked(local_encoding::windows::string_to_multibyte(cp, s, None)?)
    })
}
