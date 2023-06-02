#[non_exhaustive]
pub struct Vdso {}

impl Vdso {
    #[inline]
    pub(crate) unsafe fn from_reader(_reader: crate::VdsoReader) -> Option<Self> {
        Some(Vdso {})
    }
}
