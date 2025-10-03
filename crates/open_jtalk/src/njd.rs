use std::{
    ffi::{c_int, c_void},
    mem::{self, MaybeUninit},
    os::raw::c_char,
    ptr::{self, NonNull},
};

use libc::size_t;

use super::*;

pub use self::string::Utf8LibcString;

#[cfg(target_env = "msvc")]
const MAX_ALIGN: usize = mem::align_of::<size_t>();

#[cfg(not(target_env = "msvc"))]
const MAX_ALIGN: usize = mem::align_of::<libc::max_align_t>();

#[derive(Default)]
pub struct Njd(Option<open_jtalk_sys::NJD>);

unsafe impl resources::Resource for Njd {
    unsafe fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("njd already initialized");
        }
        let njd = {
            let mut njd = MaybeUninit::<open_jtalk_sys::NJD>::uninit();
            open_jtalk_sys::NJD_initialize(njd.as_mut_ptr());
            njd.assume_init()
        };
        self.0 = Some(njd);
        true
    }
    unsafe fn clear(&mut self) -> bool {
        open_jtalk_sys::NJD_clear(self.as_raw_ptr());
        self.0 = None;
        true
    }
}

// SAFETY: `Send`と対立する性質はないはず。
unsafe impl Send for Njd {}

impl Njd {
    // TODO: この関数自体は事前条件を持たないはずなので、`unsafe`じゃなくてもいいはず
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
                mecab_feature as *const MecabFeature as *mut *mut c_char,
                mecab_feature_size,
            )
        }
    }

    pub fn update(&mut self, f: impl FnOnce(Vec<NjdNode>) -> Vec<NjdNode>) {
        // SAFETY: ?
        let this = unsafe { self.as_raw_ptr() };
        let mut this = NonNull::new(this).expect("should have been checked");

        let nodes = {
            // SAFETY: `this` is valid at this point.
            let mut nodes =
                Vec::with_capacity(unsafe { open_jtalk_sys::NJD_get_size(this.as_ptr()) } as _);

            let this = unsafe {
                // SAFETY: `raw` should be valid for read/write since `&mut self` is held and all
                // other functions should not leave `this` broken. It should be also aligned
                // because it comes from `malloc`.
                const _: () = assert!(mem::align_of::<open_jtalk_sys::NJD>() <= MAX_ALIGN);
                this.as_mut()
            };

            while let Some(head) = NonNull::new(this.head) {
                const _: () = assert!(mem::align_of::<open_jtalk_sys::NJDNode>() <= MAX_ALIGN);

                // SAFETY: Open JTalk should have allocated `head` with `malloc` and initialize,
                // therefore the `*NJDNode` should be valid for read and aligned.
                let &open_jtalk_sys::NJDNode { next, .. } = unsafe { head.as_ref() };

                // SAFETY:
                // - As stated above, Open JTalk does allocate and initialize `head`.
                // - We don't use `head` later in this block.
                unsafe { nodes.push(NjdNode::from_raw(head)) };

                this.head = next;
            }

            nodes
        };

        let nodes = f(nodes);
        for node in nodes {
            let node = node.into_raw();
            // SAFETY:
            // - At beginning of the loop, `this` is a empty list that `NJD_push_node` can
            //   safely push first `node`. `this->tail` is dangling, however `NJD_push_node` does
            //   not see it. Even if `f` panics, no other functions should see the `tail`. For
            //   second or later time, `this` is a valid bidirectional linked list.
            // - `NjdNode::into_raw` returns a valid `NJDNode` that Open JTalk can handle.
            unsafe { open_jtalk_sys::NJD_push_node(this.as_ptr(), node.as_ptr()) };
        }
    }
}

#[derive(Debug)]
pub struct NjdNode {
    pub string: Option<Utf8LibcString>,
    pub pos: Option<Utf8LibcString>,
    pub pos_group1: Option<Utf8LibcString>,
    pub pos_group2: Option<Utf8LibcString>,
    pub pos_group3: Option<Utf8LibcString>,
    pub ctype: Option<Utf8LibcString>,
    pub cform: Option<Utf8LibcString>,
    pub orig: Option<Utf8LibcString>,
    pub read: Option<Utf8LibcString>,
    pub pron: Option<Utf8LibcString>,
    pub acc: c_int,
    pub mora_size: c_int,
    pub chain_rule: Option<Utf8LibcString>,
    pub chain_flag: c_int,
}

impl NjdNode {
    /// # Safety
    ///
    /// - `raw` must be come from Open JTalk.
    /// - You must not use `raw` after calling this function.
    unsafe fn from_raw(raw: NonNull<open_jtalk_sys::NJDNode>) -> Self {
        let open_jtalk_sys::NJDNode {
            string,
            pos,
            pos_group1,
            pos_group2,
            pos_group3,
            ctype,
            cform,
            orig,
            read,
            pron,
            acc,
            mora_size,
            chain_rule,
            chain_flag,
            prev: _,
            next: _,
        } = unsafe {
            // SAFETY: Open JTalk should have allocated this with `malloc`, thus this is valid for
            // read and aligned.
            raw.read()
        };

        let this = Self {
            string: from_raw(string),
            pos: from_raw(pos),
            pos_group1: from_raw(pos_group1),
            pos_group2: from_raw(pos_group2),
            pos_group3: from_raw(pos_group3),
            ctype: from_raw(ctype),
            cform: from_raw(cform),
            orig: from_raw(orig),
            read: from_raw(read),
            pron: from_raw(pron),
            acc,
            mora_size,
            chain_rule: from_raw(chain_rule),
            chain_flag,
        };

        // SAFETY: Open JTalk should have allocated this with `malloc`.
        unsafe { libc::free(raw.as_ptr() as *mut _) };

        return this;

        fn from_raw(s: *mut c_char) -> Option<Utf8LibcString> {
            NonNull::new(s).map(Utf8LibcString::from_raw)
        }
    }

    fn into_raw(self) -> NonNull<open_jtalk_sys::NJDNode> {
        let Self {
            string,
            pos,
            pos_group1,
            pos_group2,
            pos_group3,
            ctype,
            cform,
            orig,
            read,
            pron,
            acc,
            mora_size,
            chain_rule,
            chain_flag,
        } = self;

        let raw = open_jtalk_sys::NJDNode {
            string: into_raw(string),
            pos: into_raw(pos),
            pos_group1: into_raw(pos_group1),
            pos_group2: into_raw(pos_group2),
            pos_group3: into_raw(pos_group3),
            ctype: into_raw(ctype),
            cform: into_raw(cform),
            orig: into_raw(orig),
            read: into_raw(read),
            pron: into_raw(pron),
            acc,
            mora_size,
            chain_rule: into_raw(chain_rule),
            chain_flag,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        };

        let buf = malloc(mem::size_of::<open_jtalk_sys::NJDNode>()).cast();
        unsafe {
            // SAFETY: `malloc` correctly allocates enough aligned memory.
            const _: () = assert!(mem::align_of::<open_jtalk_sys::NJDNode>() <= MAX_ALIGN);
            buf.write(raw);
        }
        return buf;

        fn into_raw(s: Option<Utf8LibcString>) -> *mut c_char {
            s.map(Utf8LibcString::into_raw).unwrap_or_default()
        }
    }
}

fn malloc(size: size_t) -> NonNull<c_void> {
    // SAFETY: `malloc` itself does require nothing.
    let buf = unsafe { libc::malloc(size) };

    NonNull::new(buf).unwrap_or_else(|| panic!("`malloc` failed"))
}

mod string {
    use std::{
        ffi::{c_char, CStr},
        fmt::{self, Debug, Formatter},
        mem,
        ptr::NonNull,
    };

    pub struct Utf8LibcString(NonNull<c_char>);

    impl Utf8LibcString {
        /// Creates a new `Utf8LibcString`.
        ///
        /// # Panics
        ///
        /// Panics if `s` contains nul bytes.
        pub fn new(s: &str) -> Self {
            if s.as_bytes().contains(&b'\0') {
                panic!("must not contain nul bytes");
            }
            let buf = super::malloc(s.len() + 1).cast::<u8>();
            unsafe {
                // SAFETY: `malloc` allocates `s.len() + 1` bytes correctly.
                buf.copy_from_nonoverlapping(as_non_null_ptr(s), s.len());
                buf.add(s.len()).write(b'\0');
            }
            return Self(buf.cast());

            fn as_non_null_ptr(s: &str) -> NonNull<u8> {
                NonNull::new(s.as_ptr() as *mut _).expect("should be always non-null")
            }
        }

        pub(super) fn from_raw(raw: NonNull<c_char>) -> Self {
            Self(raw)
        }

        pub(super) fn into_raw(self) -> *mut c_char {
            let Self(raw) = self;
            mem::forget(self);
            raw.as_ptr()
        }

        fn as_str(&self) -> &str {
            // SAFETY: `self.0` is valid until `self` is dropped.
            unsafe { CStr::from_ptr(self.0.as_ptr()) }.to_str().unwrap()
        }
    }

    impl Drop for Utf8LibcString {
        fn drop(&mut self) {
            // SAFETY: `self.0` is valid, and is exposed only in `Self::as_str`, `Self::into_raw`,
            // and this `Drop` implementation.
            unsafe { libc::free(self.0.as_ptr() as *mut _) }
        }
    }

    impl AsRef<str> for Utf8LibcString {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl PartialEq<str> for Utf8LibcString {
        fn eq(&self, other: &str) -> bool {
            self.as_str() == other
        }
    }

    impl Debug for Utf8LibcString {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8Path;
    use resources::Resource as _;
    #[rstest]
    fn njd_initialize_and_clear_works() {
        let mut njd = Njd::default();
        unsafe {
            assert!(njd.initialize());
            assert!(njd.clear());
        }
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

        mecab
            .load(
                Utf8Path::new(std::env!("CARGO_MANIFEST_DIR"))
                    .join("src/mecab/testdata/mecab_load"),
            )
            .unwrap();
        let s = text2mecab("h^o-d+e=s/A:2+3+2/B:22-xx_xx/C:10_7+2/D:xx+xx_xx/E:5_5!0_xx-0/F:4_1#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:1_5/I:1-4@2+1&2-1|6+4/J:xx_xx/K:2+2-9").unwrap();
        assert!(mecab.analysis(s));
        njd.mecab2njd(mecab.get_feature().unwrap(), mecab.get_size());
    }

    #[rstest]
    fn njd_update_works() {
        let mut njd = ManagedResource::<Njd>::initialize();
        let mut mecab = ManagedResource::<Mecab>::initialize();

        mecab
            .load(
                Utf8Path::new(std::env!("CARGO_MANIFEST_DIR"))
                    .join("src/mecab/testdata/mecab_load"),
            )
            .unwrap();
        let s = text2mecab("foo bar baz").unwrap();
        assert!(mecab.analysis(s));
        njd.mecab2njd(mecab.get_feature().unwrap(), mecab.get_size());
        njd.update(|nodes| {
            let [mut node1, node2, mut node3, node4, mut node5] = nodes.try_into().unwrap();

            assert_eq!("ｆｏｏ", node1.string.as_ref().unwrap().as_ref());
            assert_eq!("　", node2.string.as_ref().unwrap().as_ref());
            assert_eq!("ｂａｒ", node3.string.as_ref().unwrap().as_ref());
            assert_eq!("　", node4.string.as_ref().unwrap().as_ref());
            assert_eq!("ｂａｚ", node5.string.as_ref().unwrap().as_ref());

            node1.pron = Some(Utf8LibcString::new("フウ"));
            node3.pron = Some(Utf8LibcString::new("バア"));
            node5.pron = Some(Utf8LibcString::new("バズ"));

            vec![node1, node3, node5]
        });
        njd.update(|nodes| {
            let [node1, node2, node3] = <&[_; _]>::try_from(&*nodes).unwrap();

            assert_eq!("フウ", node1.pron.as_ref().unwrap().as_ref());
            assert_eq!("バア", node2.pron.as_ref().unwrap().as_ref());
            assert_eq!("バズ", node3.pron.as_ref().unwrap().as_ref());

            nodes
        });
        drop(njd);
    }

    #[rstest]
    fn utf8_libc_string_works() {
        const TEXT: &str = "こんにちは";
        let s = Utf8LibcString::new(TEXT);
        assert_eq!(TEXT, s.as_ref());
    }
}
