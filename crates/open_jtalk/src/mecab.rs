use super::*;
use std::{ffi::CString, mem::MaybeUninit, path::Path};

#[derive(Default)]
pub struct Mecab(Option<open_jtalk_sys::Mecab>);

pub struct MecabFeature;

impl resources::Resource for Mecab {
    fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("already initialized mecab");
        }
        unsafe {
            #[allow(clippy::uninit_assumed_init)]
            let mut m: open_jtalk_sys::Mecab = MaybeUninit::uninit().assume_init();
            let result = bool_number_to_bool(open_jtalk_sys::Mecab_initialize(&mut m));
            self.0 = Some(m);
            result
        }
    }
    fn clear(&mut self) -> bool {
        let result = unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_clear(self.as_raw_ptr())) };
        self.0 = None;
        result
    }
}

impl Mecab {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::Mecab {
        if self.0.is_none() {
            panic!("uninitialized mecab");
        }
        self.0.as_ref().unwrap() as *const open_jtalk_sys::Mecab as *mut open_jtalk_sys::Mecab
    }

    pub fn load(&mut self, dic_dir: impl AsRef<Path>) -> bool {
        let dic_dir = CString::new(dic_dir.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            bool_number_to_bool(open_jtalk_sys::Mecab_load(
                self.as_raw_ptr(),
                dic_dir.as_ptr(),
            ))
        }
    }

    pub fn get_feature_mut(&mut self) -> &mut MecabFeature {
        unsafe { &mut *(open_jtalk_sys::Mecab_get_feature(self.as_raw_ptr()) as *mut MecabFeature) }
    }

    pub fn analysis(&mut self, str: impl AsRef<str>) -> bool {
        let str = CString::new(str.as_ref()).unwrap();
        unsafe {
            bool_number_to_bool(open_jtalk_sys::Mecab_analysis(
                self.as_raw_ptr(),
                str.as_ptr(),
            ))
        }
    }

    pub fn print(&mut self) -> bool {
        unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_print(self.as_raw_ptr())) }
    }

    pub fn get_size(&mut self) -> i32 {
        unsafe { open_jtalk_sys::Mecab_get_size(self.as_raw_ptr()) }
    }

    pub fn refresh(&mut self) -> bool {
        unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_refresh(self.as_raw_ptr())) }
    }
}

impl MecabFeature {
    pub(crate) unsafe fn as_raw_ptr(&mut self) -> *mut *mut i8 {
        std::mem::transmute(self)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, ptr::null_mut, str::FromStr};

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use resources::Resource as _;

    #[rstest]
    fn mecab_initialize_and_clear_works() {
        let mut mecab = Mecab::default();
        assert!(mecab.initialize());
        assert!(mecab.clear());
    }

    #[rstest]
    fn mecab_get_feature_mut_works() {
        let mut mecab = ManagedResource::<Mecab>::initialize();
        assert_eq!(null_mut(), mecab.get_feature_mut() as *mut MecabFeature);
    }

    #[rstest]
    fn mecab_load_works() {
        let mut mecab = ManagedResource::<Mecab>::initialize();
        assert!(mecab.load(
            PathBuf::from_str(std::env!("CARGO_MANIFEST_DIR"))
                .unwrap()
                .join("src/testdata/mecab_load"),
        ));
    }

    #[rstest]
    fn mecab_get_size_before_analysis_works() {
        let mut mecab = ManagedResource::<Mecab>::initialize();
        assert_eq!(0, mecab.get_size());
    }

    #[rstest]
    #[case("h^o-d+e=s/A:2+3+2/B:22-xx_xx/C:10_7+2/D:xx+xx_xx/E:5_5!0_xx-0/F:4_1#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:1_5/I:1-4@2+1&2-1|6+4/J:xx_xx/K:2+2-9",true)]
    fn mecab_analysis_works(#[case] input: &str, #[case] expected: bool) {
        let mut mecab = ManagedResource::<Mecab>::initialize();
        let s = text2mecab(input).unwrap();
        assert_eq!(expected, mecab.analysis(s));
    }

    #[rstest]
    fn mecab_refresh_works() {
        let mut mecab = ManagedResource::<Mecab>::initialize();
        assert!(mecab.refresh());
    }
}
