#[non_exhaustive]
pub struct Vdso {}

impl Vdso {
    #[doc = r" Parse vDSO from memory"]
    #[doc = r" # Safety"]
    #[doc = r" This is unsafe because we can't validate the given pointer so"]
    #[doc = r" use it carefully"]
    #[inline]
    pub(crate) unsafe fn from_reader(_reader: crate::VdsoReader) -> Option<Self> {
        Some(Vdso {})
    }
}
