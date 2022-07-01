use super::*;
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct JpCommon(Option<open_jtalk_sys::JPCommon>);

pub struct JpCommonFeature;

impl resources::Resource for JpCommon {
    fn initialize(&mut self) -> bool {
        unsafe {
            #[allow(clippy::uninit_assumed_init)]
            let mut jpcommon: open_jtalk_sys::JPCommon = MaybeUninit::uninit().assume_init();
            open_jtalk_sys::JPCommon_initialize(&mut jpcommon);
            self.0 = Some(jpcommon);
        }
        true
    }
    fn clear(&mut self) -> bool {
        unsafe { open_jtalk_sys::JPCommon_clear(self.as_raw_ptr()) };
        true
    }
}

impl JpCommon {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::JPCommon {
        &mut self.0.unwrap() as *mut open_jtalk_sys::JPCommon
    }

    pub fn refresh(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_refresh(self.as_raw_ptr()) }
    }

    pub fn make_label(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_refresh(self.as_raw_ptr()) }
    }

    pub fn get_label_size(&self) -> i32 {
        unsafe { open_jtalk_sys::JPCommon_get_label_size(self.as_raw_ptr()) }
    }

    pub fn get_label_feature_mut(&mut self) -> &mut JpCommonFeature {
        unsafe {
            &mut *(open_jtalk_sys::JPCommon_get_label_feature(self.as_raw_ptr())
                as *mut JpCommonFeature)
        }
    }
}
