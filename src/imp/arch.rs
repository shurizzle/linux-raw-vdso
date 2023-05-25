#[repr(u8)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Vdso {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

impl Vdso {
    #[inline]
    pub(crate) unsafe fn from_reader(_reader: crate::VdsoReader) -> Option<Self> {
        unimplemented!("Unsupported platform")
    }
}
