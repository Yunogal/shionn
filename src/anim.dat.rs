use crate::anim_gax;
pub type Dat = anim_gax::Gax;
pub use anim_gax::parce;

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(size_of::<Dat>(), 20);
    assert_eq!(align_of::<Dat>(), 4)
}
