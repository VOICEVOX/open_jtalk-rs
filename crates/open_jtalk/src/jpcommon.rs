use super::*;
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct JpCommon(Option<open_jtalk_sys::JPCommon>);

pub struct JpCommonFeature;

impl resources::Resource for JpCommon {
    fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("already initialized jpcommon");
        }
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
        self.0 = None;
        true
    }
}

impl JpCommon {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::JPCommon {
        if self.0.is_none() {
            panic!("uninitialized jpcommon");
        }
        &mut self.0.unwrap() as *mut open_jtalk_sys::JPCommon
    }

    pub fn refresh(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_refresh(self.as_raw_ptr()) }
    }

    pub fn make_label(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_make_label(self.as_raw_ptr()) }
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

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use resources::Resource as _;
    #[rstest]
    pub fn jpcommon_initialize_and_clear_works() {
        let mut jpcommon = JpCommon::default();
        assert!(jpcommon.initialize());
        assert!(jpcommon.clear());
    }

    #[rstest]
    pub fn jpcommon_get_label_size_before_make_labelworks() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();
        assert_eq!(jpcommon.get_label_size(), 0);
    }

    #[rstest]
    pub fn jpcommon_get_label_feature_mut_before_make_label_works() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();
        assert_eq!(
            jpcommon.get_label_feature_mut() as *mut JpCommonFeature,
            null_mut()
        );
    }

    #[rstest]
    pub fn jpcommon_refresh_works() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();
        jpcommon.refresh();
    }
}
