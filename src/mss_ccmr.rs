#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccmsr1: Ccmsr1,
    ccmkeyr1: Ccmkeyr1,
    ccmsr2: Ccmsr2,
    ccmkeyr2: Ccmkeyr2,
    ccmsr3: Ccmsr3,
    ccmkeyr3: Ccmkeyr3,
    ccmpolcntrl: Ccmpolcntrl,
}
impl RegisterBlock {
    #[doc = "0x00 - CPU Compare Status Register"]
    #[inline(always)]
    pub const fn ccmsr1(&self) -> &Ccmsr1 {
        &self.ccmsr1
    }
    #[doc = "0x04 - CPU Compare Key Register"]
    #[inline(always)]
    pub const fn ccmkeyr1(&self) -> &Ccmkeyr1 {
        &self.ccmkeyr1
    }
    #[doc = "0x08 - VIM Compare Status Register"]
    #[inline(always)]
    pub const fn ccmsr2(&self) -> &Ccmsr2 {
        &self.ccmsr2
    }
    #[doc = "0x0c - VIM Compare Key Register"]
    #[inline(always)]
    pub const fn ccmkeyr2(&self) -> &Ccmkeyr2 {
        &self.ccmkeyr2
    }
    #[doc = "0x10 - Inactivity Monitor Status Register"]
    #[inline(always)]
    pub const fn ccmsr3(&self) -> &Ccmsr3 {
        &self.ccmsr3
    }
    #[doc = "0x14 - Inactivity Monitor Key Register"]
    #[inline(always)]
    pub const fn ccmkeyr3(&self) -> &Ccmkeyr3 {
        &self.ccmkeyr3
    }
    #[doc = "0x18 - CPU Compare Polarity Control Register"]
    #[inline(always)]
    pub const fn ccmpolcntrl(&self) -> &Ccmpolcntrl {
        &self.ccmpolcntrl
    }
}
#[doc = "CCMSR1 (rw) register accessor: CPU Compare Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmsr1`]
module"]
#[doc(alias = "CCMSR1")]
pub type Ccmsr1 = crate::Reg<ccmsr1::Ccmsr1Spec>;
#[doc = "CPU Compare Status Register"]
pub mod ccmsr1;
#[doc = "CCMKEYR1 (rw) register accessor: CPU Compare Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmkeyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmkeyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmkeyr1`]
module"]
#[doc(alias = "CCMKEYR1")]
pub type Ccmkeyr1 = crate::Reg<ccmkeyr1::Ccmkeyr1Spec>;
#[doc = "CPU Compare Key Register"]
pub mod ccmkeyr1;
#[doc = "CCMSR2 (rw) register accessor: VIM Compare Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmsr2`]
module"]
#[doc(alias = "CCMSR2")]
pub type Ccmsr2 = crate::Reg<ccmsr2::Ccmsr2Spec>;
#[doc = "VIM Compare Status Register"]
pub mod ccmsr2;
#[doc = "CCMKEYR2 (rw) register accessor: VIM Compare Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmkeyr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmkeyr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmkeyr2`]
module"]
#[doc(alias = "CCMKEYR2")]
pub type Ccmkeyr2 = crate::Reg<ccmkeyr2::Ccmkeyr2Spec>;
#[doc = "VIM Compare Key Register"]
pub mod ccmkeyr2;
#[doc = "CCMSR3 (rw) register accessor: Inactivity Monitor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmsr3`]
module"]
#[doc(alias = "CCMSR3")]
pub type Ccmsr3 = crate::Reg<ccmsr3::Ccmsr3Spec>;
#[doc = "Inactivity Monitor Status Register"]
pub mod ccmsr3;
#[doc = "CCMKEYR3 (rw) register accessor: Inactivity Monitor Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmkeyr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmkeyr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmkeyr3`]
module"]
#[doc(alias = "CCMKEYR3")]
pub type Ccmkeyr3 = crate::Reg<ccmkeyr3::Ccmkeyr3Spec>;
#[doc = "Inactivity Monitor Key Register"]
pub mod ccmkeyr3;
#[doc = "CCMPOLCNTRL (rw) register accessor: CPU Compare Polarity Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmpolcntrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmpolcntrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmpolcntrl`]
module"]
#[doc(alias = "CCMPOLCNTRL")]
pub type Ccmpolcntrl = crate::Reg<ccmpolcntrl::CcmpolcntrlSpec>;
#[doc = "CPU Compare Polarity Control Register"]
pub mod ccmpolcntrl;
