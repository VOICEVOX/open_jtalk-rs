use super::*;
use std::ptr::null_mut;

pub struct Njd(open_jtalk_sys::NJD);

impl Default for Njd {
    fn default() -> Self {
        Self(open_jtalk_sys::NJD {
            head: null_mut(),
            tail: null_mut(),
        })
    }
}

impl resources::Resource for Njd {
    fn initialize(&mut self) -> bool {
        unsafe { open_jtalk_sys::NJD_initialize(self.as_raw_ptr()) };
        true
    }
    fn clear(&mut self) -> bool {
        unsafe { open_jtalk_sys::NJD_clear(self.as_raw_ptr()) };
        true
    }
}

impl Njd {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::NJD {
        &self.0 as *const open_jtalk_sys::NJD as *mut open_jtalk_sys::NJD
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
