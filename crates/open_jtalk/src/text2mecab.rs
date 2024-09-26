use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[repr(i32)]
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Text2MecabError {
    #[error("text2mecab range error")]
    Range = open_jtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_RANGE_ERROR as i32,
    #[error("text2mecab invalid argument")]
    InvalidArgument =
        open_jtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_INVALID_ARGUMENT as i32,
}

pub fn text2mecab(input: impl AsRef<str>) -> Result<String, Text2MecabError> {
    // NOTE:text2mecabのoutputに必要な必要な長さがわからないため8192決め打ちにしている
    // https://github.com/VOICEVOX/voicevox_core/issues/128#issuecomment-1168181887
    const MAX_TEXT2MECAB_SIZE: usize = 8192;
    let mut output = Vec::with_capacity(MAX_TEXT2MECAB_SIZE);
    let text = CString::new(input.as_ref()).unwrap();

    let result = unsafe {
        open_jtalk_sys::text2mecab(
            output.as_mut_ptr() as *mut c_char,
            MAX_TEXT2MECAB_SIZE,
            text.as_ptr(),
        )
    };
    if result == open_jtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_SUCCESS {
        unsafe {
            output.set_len(
                CStr::from_ptr(output.as_ptr() as *const c_char)
                    .to_bytes()
                    .len(),
            )
        }

        Ok(String::from_utf8(output).unwrap())
    } else {
        Err(unsafe {
            std::mem::transmute::<open_jtalk_sys::text2mecab_result_t, Text2MecabError>(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[rstest]
    #[case("", Ok("".into()))]
    #[case("あいうえお", Ok("あいうえお".into()))]
    #[case("あいう\nえお", Ok("あいうえお".into()))]
    fn text2mecab_works(
        #[case] input: impl AsRef<str>,
        #[case] expected: Result<String, Text2MecabError>,
    ) {
        let result = text2mecab(input);
        assert_eq!(expected, result);
    }
}
