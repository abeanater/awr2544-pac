#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    start: Start,
    _reserved1: [u8; 0x07f8],
    end: End,
}
impl RegisterBlock {
    #[doc = "0x00 - START"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x7fc - END"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
}
#[doc = "START (rw) register accessor: START\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "START"]
pub mod start;
#[doc = "END (rw) register accessor: END\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "END"]
pub mod end;
