mod jpcommon;
mod mecab;
mod njd;
mod resource;
mod text2mecab;

pub use jpcommon::*;
pub use mecab::*;
pub use njd::*;
pub use resource::*;
pub use text2mecab::*;

#[inline]
fn bool_number_to_bool(bool_number: i32) -> bool {
    bool_number == open_jtalk_sys::TRUE as i32
}
