mod jpcommon;
mod mecab;
mod njd;
mod resource;
mod text2mecab;

#[cfg(target_os = "windows")]
mod win_api_helper;

pub use jpcommon::*;
pub use mecab::*;
pub use njd::*;
pub use resource::*;
pub use text2mecab::*;

#[cfg(test)]
use rstest::rstest;

#[inline]
fn bool_number_to_bool(bool_number: i32) -> bool {
    bool_number == 1
}
