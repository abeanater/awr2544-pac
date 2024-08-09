#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rev: Rev,
    _reserved1: [u8; 0x04],
    vector: Vector,
    stat: Stat,
    wrap_rev: WrapRev,
    ctrl: Ctrl,
    err_ctrl1: ErrCtrl1,
    err_ctrl2: ErrCtrl2,
    err_stat1: ErrStat1,
    err_stat2: ErrStat2,
    err_stat3: ErrStat3,
    _reserved10: [u8; 0x10],
    sec_eoi_reg: SecEoiReg,
    sec_status_reg0: SecStatusReg0,
    sec_status_reg1: SecStatusReg1,
    _reserved13: [u8; 0x38],
    sec_enable_set_reg0: SecEnableSetReg0,
    sec_enable_set_reg1: SecEnableSetReg1,
    _reserved15: [u8; 0x38],
    sec_enable_clr_reg0: SecEnableClrReg0,
    sec_enable_clr_reg1: SecEnableClrReg1,
    _reserved17: [u8; 0x74],
    ded_eoi_reg: DedEoiReg,
    ded_status_reg0: DedStatusReg0,
    ded_status_reg1: DedStatusReg1,
    _reserved20: [u8; 0x38],
    ded_enable_set_reg0: DedEnableSetReg0,
    ded_enable_set_reg1: DedEnableSetReg1,
    _reserved22: [u8; 0x38],
    ded_enable_clr_reg0: DedEnableClrReg0,
    ded_enable_clr_reg1: DedEnableClrReg1,
    _reserved24: [u8; 0x38],
    aggr_enable_set: AggrEnableSet,
    aggr_enable_clr: AggrEnableClr,
    aggr_status_set: AggrStatusSet,
    aggr_status_clr: AggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn vector(&self) -> &Vector {
        &self.vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - Revision parameters"]
    #[inline(always)]
    pub const fn wrap_rev(&self) -> &WrapRev {
        &self.wrap_rev
    }
    #[doc = "0x14 - ECC Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x18 - ECC Error Control1 Register"]
    #[inline(always)]
    pub const fn err_ctrl1(&self) -> &ErrCtrl1 {
        &self.err_ctrl1
    }
    #[doc = "0x1c - ECC Error Control2 Register"]
    #[inline(always)]
    pub const fn err_ctrl2(&self) -> &ErrCtrl2 {
        &self.err_ctrl2
    }
    #[doc = "0x20 - ECC Error Status1 Register"]
    #[inline(always)]
    pub const fn err_stat1(&self) -> &ErrStat1 {
        &self.err_stat1
    }
    #[doc = "0x24 - ECC Error Status2 Register"]
    #[inline(always)]
    pub const fn err_stat2(&self) -> &ErrStat2 {
        &self.err_stat2
    }
    #[doc = "0x28 - ECC Error Status3 Register"]
    #[inline(always)]
    pub const fn err_stat3(&self) -> &ErrStat3 {
        &self.err_stat3
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn sec_eoi_reg(&self) -> &SecEoiReg {
        &self.sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn sec_status_reg0(&self) -> &SecStatusReg0 {
        &self.sec_status_reg0
    }
    #[doc = "0x44 - Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn sec_status_reg1(&self) -> &SecStatusReg1 {
        &self.sec_status_reg1
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn sec_enable_set_reg0(&self) -> &SecEnableSetReg0 {
        &self.sec_enable_set_reg0
    }
    #[doc = "0x84 - Interrupt Enable Set Register 1"]
    #[inline(always)]
    pub const fn sec_enable_set_reg1(&self) -> &SecEnableSetReg1 {
        &self.sec_enable_set_reg1
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn sec_enable_clr_reg0(&self) -> &SecEnableClrReg0 {
        &self.sec_enable_clr_reg0
    }
    #[doc = "0xc4 - Interrupt Enable Clear Register 1"]
    #[inline(always)]
    pub const fn sec_enable_clr_reg1(&self) -> &SecEnableClrReg1 {
        &self.sec_enable_clr_reg1
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ded_eoi_reg(&self) -> &DedEoiReg {
        &self.ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ded_status_reg0(&self) -> &DedStatusReg0 {
        &self.ded_status_reg0
    }
    #[doc = "0x144 - Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn ded_status_reg1(&self) -> &DedStatusReg1 {
        &self.ded_status_reg1
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ded_enable_set_reg0(&self) -> &DedEnableSetReg0 {
        &self.ded_enable_set_reg0
    }
    #[doc = "0x184 - Interrupt Enable Set Register 1"]
    #[inline(always)]
    pub const fn ded_enable_set_reg1(&self) -> &DedEnableSetReg1 {
        &self.ded_enable_set_reg1
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ded_enable_clr_reg0(&self) -> &DedEnableClrReg0 {
        &self.ded_enable_clr_reg0
    }
    #[doc = "0x1c4 - Interrupt Enable Clear Register 1"]
    #[inline(always)]
    pub const fn ded_enable_clr_reg1(&self) -> &DedEnableClrReg1 {
        &self.ded_enable_clr_reg1
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn aggr_enable_set(&self) -> &AggrEnableSet {
        &self.aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn aggr_enable_clr(&self) -> &AggrEnableClr {
        &self.aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn aggr_status_set(&self) -> &AggrStatusSet {
        &self.aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn aggr_status_clr(&self) -> &AggrStatusClr {
        &self.aggr_status_clr
    }
}
#[doc = "rev (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
#[doc(alias = "rev")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "Revision parameters"]
pub mod rev;
#[doc = "vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vector`]
module"]
#[doc(alias = "vector")]
pub type Vector = crate::Reg<vector::VectorSpec>;
#[doc = "ECC Vector Register"]
pub mod vector;
#[doc = "stat (rw) register accessor: Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "stat")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Misc Status"]
pub mod stat;
#[doc = "wrap_rev (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrap_rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrap_rev`]
module"]
#[doc(alias = "wrap_rev")]
pub type WrapRev = crate::Reg<wrap_rev::WrapRevSpec>;
#[doc = "Revision parameters"]
pub mod wrap_rev;
#[doc = "ctrl (rw) register accessor: ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "ECC Control Register"]
pub mod ctrl;
#[doc = "err_ctrl1 (rw) register accessor: ECC Error Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl1`]
module"]
#[doc(alias = "err_ctrl1")]
pub type ErrCtrl1 = crate::Reg<err_ctrl1::ErrCtrl1Spec>;
#[doc = "ECC Error Control1 Register"]
pub mod err_ctrl1;
#[doc = "err_ctrl2 (rw) register accessor: ECC Error Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl2`]
module"]
#[doc(alias = "err_ctrl2")]
pub type ErrCtrl2 = crate::Reg<err_ctrl2::ErrCtrl2Spec>;
#[doc = "ECC Error Control2 Register"]
pub mod err_ctrl2;
#[doc = "err_stat1 (rw) register accessor: ECC Error Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat1`]
module"]
#[doc(alias = "err_stat1")]
pub type ErrStat1 = crate::Reg<err_stat1::ErrStat1Spec>;
#[doc = "ECC Error Status1 Register"]
pub mod err_stat1;
#[doc = "err_stat2 (rw) register accessor: ECC Error Status2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat2`]
module"]
#[doc(alias = "err_stat2")]
pub type ErrStat2 = crate::Reg<err_stat2::ErrStat2Spec>;
#[doc = "ECC Error Status2 Register"]
pub mod err_stat2;
#[doc = "err_stat3 (rw) register accessor: ECC Error Status3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat3`]
module"]
#[doc(alias = "err_stat3")]
pub type ErrStat3 = crate::Reg<err_stat3::ErrStat3Spec>;
#[doc = "ECC Error Status3 Register"]
pub mod err_stat3;
#[doc = "sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_eoi_reg`]
module"]
#[doc(alias = "sec_eoi_reg")]
pub type SecEoiReg = crate::Reg<sec_eoi_reg::SecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod sec_eoi_reg;
#[doc = "sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_status_reg0`]
module"]
#[doc(alias = "sec_status_reg0")]
pub type SecStatusReg0 = crate::Reg<sec_status_reg0::SecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod sec_status_reg0;
#[doc = "sec_status_reg1 (rw) register accessor: Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_status_reg1`]
module"]
#[doc(alias = "sec_status_reg1")]
pub type SecStatusReg1 = crate::Reg<sec_status_reg1::SecStatusReg1Spec>;
#[doc = "Interrupt Status Register 1"]
pub mod sec_status_reg1;
#[doc = "sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_set_reg0`]
module"]
#[doc(alias = "sec_enable_set_reg0")]
pub type SecEnableSetReg0 = crate::Reg<sec_enable_set_reg0::SecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod sec_enable_set_reg0;
#[doc = "sec_enable_set_reg1 (rw) register accessor: Interrupt Enable Set Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_set_reg1`]
module"]
#[doc(alias = "sec_enable_set_reg1")]
pub type SecEnableSetReg1 = crate::Reg<sec_enable_set_reg1::SecEnableSetReg1Spec>;
#[doc = "Interrupt Enable Set Register 1"]
pub mod sec_enable_set_reg1;
#[doc = "sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_clr_reg0`]
module"]
#[doc(alias = "sec_enable_clr_reg0")]
pub type SecEnableClrReg0 = crate::Reg<sec_enable_clr_reg0::SecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod sec_enable_clr_reg0;
#[doc = "sec_enable_clr_reg1 (rw) register accessor: Interrupt Enable Clear Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_clr_reg1`]
module"]
#[doc(alias = "sec_enable_clr_reg1")]
pub type SecEnableClrReg1 = crate::Reg<sec_enable_clr_reg1::SecEnableClrReg1Spec>;
#[doc = "Interrupt Enable Clear Register 1"]
pub mod sec_enable_clr_reg1;
#[doc = "ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_eoi_reg`]
module"]
#[doc(alias = "ded_eoi_reg")]
pub type DedEoiReg = crate::Reg<ded_eoi_reg::DedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ded_eoi_reg;
#[doc = "ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_status_reg0`]
module"]
#[doc(alias = "ded_status_reg0")]
pub type DedStatusReg0 = crate::Reg<ded_status_reg0::DedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ded_status_reg0;
#[doc = "ded_status_reg1 (rw) register accessor: Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_status_reg1`]
module"]
#[doc(alias = "ded_status_reg1")]
pub type DedStatusReg1 = crate::Reg<ded_status_reg1::DedStatusReg1Spec>;
#[doc = "Interrupt Status Register 1"]
pub mod ded_status_reg1;
#[doc = "ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_set_reg0`]
module"]
#[doc(alias = "ded_enable_set_reg0")]
pub type DedEnableSetReg0 = crate::Reg<ded_enable_set_reg0::DedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ded_enable_set_reg0;
#[doc = "ded_enable_set_reg1 (rw) register accessor: Interrupt Enable Set Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_set_reg1`]
module"]
#[doc(alias = "ded_enable_set_reg1")]
pub type DedEnableSetReg1 = crate::Reg<ded_enable_set_reg1::DedEnableSetReg1Spec>;
#[doc = "Interrupt Enable Set Register 1"]
pub mod ded_enable_set_reg1;
#[doc = "ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_clr_reg0`]
module"]
#[doc(alias = "ded_enable_clr_reg0")]
pub type DedEnableClrReg0 = crate::Reg<ded_enable_clr_reg0::DedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ded_enable_clr_reg0;
#[doc = "ded_enable_clr_reg1 (rw) register accessor: Interrupt Enable Clear Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_clr_reg1`]
module"]
#[doc(alias = "ded_enable_clr_reg1")]
pub type DedEnableClrReg1 = crate::Reg<ded_enable_clr_reg1::DedEnableClrReg1Spec>;
#[doc = "Interrupt Enable Clear Register 1"]
pub mod ded_enable_clr_reg1;
#[doc = "aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_set`]
module"]
#[doc(alias = "aggr_enable_set")]
pub type AggrEnableSet = crate::Reg<aggr_enable_set::AggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod aggr_enable_set;
#[doc = "aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_clr`]
module"]
#[doc(alias = "aggr_enable_clr")]
pub type AggrEnableClr = crate::Reg<aggr_enable_clr::AggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod aggr_enable_clr;
#[doc = "aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_set`]
module"]
#[doc(alias = "aggr_status_set")]
pub type AggrStatusSet = crate::Reg<aggr_status_set::AggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod aggr_status_set;
#[doc = "aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_clr`]
module"]
#[doc(alias = "aggr_status_clr")]
pub type AggrStatusClr = crate::Reg<aggr_status_clr::AggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod aggr_status_clr;
