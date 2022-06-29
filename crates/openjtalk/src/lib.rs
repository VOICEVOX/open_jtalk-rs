mod mecab;

pub use mecab::*;

use std::ffi::{CStr, CString};

#[repr(i32)]
pub enum Text2MecabError {
    Range = openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_RANGE_ERROR as i32,
    InvalidArgument = openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_INVALID_ARGUMENT as i32,
}

pub fn text2mecab(input: impl AsRef<str>) -> Result<String, Text2MecabError> {
    // NOTE:text2mecabのoutputに必要な必要な長さがわからないため8192決め打ちにしている
    // https://github.com/VOICEVOX/voicevox_core/issues/128#issuecomment-1168181887
    const MAX_TEXT2MECAB_SIZE: usize = 8192;
    let mut output = Vec::with_capacity(MAX_TEXT2MECAB_SIZE);
    let text = CString::new(input.as_ref()).unwrap();

    let result = unsafe {
        openjtalk_sys::text2mecab(
            output.as_mut_ptr() as *mut i8,
            MAX_TEXT2MECAB_SIZE,
            text.as_ptr(),
        )
    };
    if result == openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_SUCCESS {
        unsafe {
            output.set_len(
                CStr::from_ptr(output.as_ptr() as *const i8)
                    .to_bytes()
                    .len(),
            )
        }

        Ok(String::from_utf8(output).unwrap())
    } else {
        Err(unsafe { std::mem::transmute(result) })
    }
}

#[inline]
fn bool_number_to_bool(bool_number: i32) -> bool {
    bool_number == openjtalk_sys::TRUE as i32
}
