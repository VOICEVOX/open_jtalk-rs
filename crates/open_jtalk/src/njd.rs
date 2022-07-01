use super::*;
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct Njd(Option<open_jtalk_sys::NJD>);

impl resources::Resource for Njd {
    fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("njd already initialized");
        }
        unsafe {
            #[allow(clippy::uninit_assumed_init)]
            let mut njd: open_jtalk_sys::NJD = MaybeUninit::uninit().assume_init();
            open_jtalk_sys::NJD_initialize(&mut njd);
            self.0 = Some(njd);
        }
        true
    }
    fn clear(&mut self) -> bool {
        unsafe { open_jtalk_sys::NJD_clear(self.as_raw_ptr()) };
        true
    }
}

impl Njd {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::NJD {
        if self.0.is_none() {
            panic!("uninitialized njd");
        }
        &mut self.0.unwrap() as *mut open_jtalk_sys::NJD
    }

    pub fn set_pronunciation(&mut self) {
        unsafe { open_jtalk_sys::njd_set_pronunciation(self.as_raw_ptr()) }
    }

    pub fn set_digit(&mut self) {
        unsafe { open_jtalk_sys::njd_set_digit(self.as_raw_ptr()) }
    }

    pub fn set_accent_type(&mut self) {
        unsafe { open_jtalk_sys::njd_set_accent_type(self.as_raw_ptr()) }
    }

    pub fn set_unvoiced_vowel(&mut self) {
        unsafe { open_jtalk_sys::njd_set_unvoiced_vowel(self.as_raw_ptr()) }
    }

    pub fn set_long_vowel(&mut self) {
        unsafe { open_jtalk_sys::njd_set_long_vowel(self.as_raw_ptr()) }
    }
}
