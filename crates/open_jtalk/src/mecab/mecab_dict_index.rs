use std::ffi::CString;

pub fn mecab_dict_index(argv: &[&str]) {
    let argv = argv
        .iter()
        .map(|&s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    let mut argv = argv
        .iter()
        .map(|cs| cs.as_ptr() as *mut i8)
        .collect::<Vec<_>>();
    unsafe { open_jtalk_sys::mecab_dict_index(argv.len() as i32, argv.as_mut_ptr()) };
}
