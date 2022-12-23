use super::*;
use std::{ffi::CStr, mem::MaybeUninit};

#[derive(Default)]
pub struct JpCommon(Option<open_jtalk_sys::JPCommon>);

pub struct JpCommonLabelFeature;

pub struct JpCommonLabelFeatureIter<'a> {
    label_features: &'a JpCommonLabelFeature,
    index: i32,
    size: i32,
}

unsafe impl resources::Resource for JpCommon {
    unsafe fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("already initialized jpcommon");
        }
        let jpcommon = {
            let mut jpcommon = MaybeUninit::<open_jtalk_sys::JPCommon>::uninit();
            open_jtalk_sys::JPCommon_initialize(jpcommon.as_mut_ptr());
            jpcommon.assume_init()
        };
        self.0 = Some(jpcommon);
        true
    }
    unsafe fn clear(&mut self) -> bool {
        open_jtalk_sys::JPCommon_clear(self.as_raw_ptr());
        self.0 = None;
        true
    }
}

impl<'a> Iterator for JpCommonLabelFeatureIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let label_features_ptr = self.label_features as *const JpCommonLabelFeature as *mut *mut i8;
        unsafe {
            if self.index < self.size {
                let label_feature = *label_features_ptr.offset(self.index as isize);
                self.index += 1;
                Some(CStr::from_ptr(label_feature).to_str().unwrap())
            } else {
                None
            }
        }
    }
}

impl JpCommon {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::JPCommon {
        if self.0.is_none() {
            panic!("uninitialized jpcommon");
        }
        self.0.as_ref().unwrap() as *const open_jtalk_sys::JPCommon as *mut open_jtalk_sys::JPCommon
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

    pub fn njd2jpcommon(&mut self, njd: &Njd) {
        unsafe { open_jtalk_sys::njd2jpcommon(self.as_raw_ptr(), njd.as_raw_ptr()) }
    }

    pub fn get_label_feature_to_iter(&self) -> Option<JpCommonLabelFeatureIter> {
        self.get_label_feature_raw().map(|label_features| {
            let label_features_size = self.get_label_size();
            JpCommonLabelFeatureIter {
                label_features,
                index: 0,
                size: label_features_size,
            }
        })
    }

    pub(crate) fn get_label_feature_raw(&self) -> Option<&JpCommonLabelFeature> {
        unsafe {
            let feature = open_jtalk_sys::JPCommon_get_label_feature(self.as_raw_ptr());
            if !feature.is_null() {
                Some(&*(feature as *const JpCommonLabelFeature))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use resources::Resource as _;
    #[rstest]
    fn jpcommon_initialize_and_clear_works() {
        let mut jpcommon = JpCommon::default();
        unsafe {
            assert!(jpcommon.initialize());
            assert!(jpcommon.clear());
        }
    }

    #[rstest]
    fn jpcommon_get_label_size_before_make_labelworks() {
        let jpcommon = ManagedResource::<JpCommon>::initialize();
        assert_eq!(0, jpcommon.get_label_size());
    }

    #[rstest]
    fn jpcommon_get_label_feature_mut_before_make_label_works() {
        let jpcommon = ManagedResource::<JpCommon>::initialize();

        assert!(jpcommon.get_label_feature_raw().is_none());
    }

    #[rstest]
    fn jpcommon_refresh_works() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();
        jpcommon.refresh();
    }
}
