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
        self.0 = None;
        true
    }
}

impl Njd {
    pub(crate) unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::NJD {
        if self.0.is_none() {
            panic!("uninitialized njd");
        }
        self.0.as_ref().unwrap() as *const open_jtalk_sys::NJD as *mut open_jtalk_sys::NJD
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

    pub fn set_accent_phrase(&mut self) {
        unsafe { open_jtalk_sys::njd_set_accent_phrase(self.as_raw_ptr()) }
    }

    pub fn set_unvoiced_vowel(&mut self) {
        unsafe { open_jtalk_sys::njd_set_unvoiced_vowel(self.as_raw_ptr()) }
    }

    pub fn set_long_vowel(&mut self) {
        unsafe { open_jtalk_sys::njd_set_long_vowel(self.as_raw_ptr()) }
    }

    pub fn refresh(&mut self) {
        unsafe { open_jtalk_sys::NJD_refresh(self.as_raw_ptr()) }
    }

    pub fn mecab2njd(&mut self, mecab_feature: &MecabFeature, mecab_feature_size: i32) {
        unsafe {
            open_jtalk_sys::mecab2njd(
                self.as_raw_ptr(),
                mecab_feature as *const MecabFeature as *mut *mut i8,
                mecab_feature_size,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resources::Resource as _;
    use std::path::PathBuf;
    use std::str::FromStr;
    #[rstest]
    fn njd_initialize_and_clear_works() {
        let mut njd = Njd::default();
        assert!(njd.initialize());
        assert!(njd.clear());
    }

    #[rstest]
    fn njd_set_pronunciation_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_pronunciation();
    }

    #[rstest]
    fn njd_set_digit_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_digit();
    }

    #[rstest]
    fn njd_set_accent_type_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_accent_type();
    }

    #[rstest]
    fn njd_set_accent_phrase_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_accent_phrase();
    }

    #[rstest]
    fn njd_set_unvoiced_vowel_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_unvoiced_vowel();
    }
    #[rstest]
    fn njd_set_long_vowel_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.set_long_vowel();
    }
    #[rstest]
    fn njd_refresh_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        njd.refresh();
    }

    #[rstest]
    fn njd_mecab2njd_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        let mut mecab = ManagedResource::<Mecab>::initialize();

        assert!(mecab.load(
            PathBuf::from_str(std::env!("CARGO_MANIFEST_DIR"))
                .unwrap()
                .join("src/mecab/testdata/mecab_load"),
        ));
        let s = text2mecab("h^o-d+e=s/A:2+3+2/B:22-xx_xx/C:10_7+2/D:xx+xx_xx/E:5_5!0_xx-0/F:4_1#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:1_5/I:1-4@2+1&2-1|6+4/J:xx_xx/K:2+2-9").unwrap();
        assert!(mecab.analysis(s));
        njd.mecab2njd(mecab.get_feature().unwrap(), mecab.get_size());
    }
}
