use std::ptr::null_mut;

pub struct JpCommon(open_jtalk_sys::JPCommon);

pub struct JpCommonFeature;

impl Default for JpCommon {
    fn default() -> Self {
        Self(open_jtalk_sys::JPCommon {
            head: null_mut(),
            tail: null_mut(),
            label: null_mut(),
        })
    }
}

impl JpCommon {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::JPCommon {
        &self.0 as *const open_jtalk_sys::JPCommon as *mut open_jtalk_sys::JPCommon
    }
    pub fn initialize(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_initialize(self.as_raw_ptr()) }
    }

    pub fn clear(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_clear(self.as_raw_ptr()) }
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
