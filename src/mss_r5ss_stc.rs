#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stcgcr0: Stcgcr0,
    stcgcr1: Stcgcr1,
    stctpr: Stctpr,
    stc_caddr: StcCaddr,
    stccicr: Stccicr,
    stcgstat: Stcgstat,
    stcfstat: Stcfstat,
    stcscscr: Stcscscr,
    stc_caddr2: StcCaddr2,
    stc_clkdiv: StcClkdiv,
    stc_segplr: StcSegplr,
    seg0_start_addr: Seg0StartAddr,
    seg1_start_addr: Seg1StartAddr,
    seg2_start_addr: Seg2StartAddr,
    seg3_start_addr: Seg3StartAddr,
    core1_curmisr_0: Core1Curmisr0,
    core1_curmisr_1: Core1Curmisr1,
    core1_curmisr_2: Core1Curmisr2,
    core1_curmisr_3: Core1Curmisr3,
    core1_curmisr_4: Core1Curmisr4,
    core1_curmisr_5: Core1Curmisr5,
    core1_curmisr_6: Core1Curmisr6,
    core1_curmisr_7: Core1Curmisr7,
    core1_curmisr_8: Core1Curmisr8,
    core1_curmisr_9: Core1Curmisr9,
    core1_curmisr_10: Core1Curmisr10,
    core1_curmisr_11: Core1Curmisr11,
    core1_curmisr_12: Core1Curmisr12,
    core1_curmisr_13: Core1Curmisr13,
    core1_curmisr_14: Core1Curmisr14,
    core1_curmisr_15: Core1Curmisr15,
    core1_curmisr_16: Core1Curmisr16,
    core1_curmisr_17: Core1Curmisr17,
    core1_curmisr_18: Core1Curmisr18,
    core1_curmisr_19: Core1Curmisr19,
    core1_curmisr_20: Core1Curmisr20,
    core1_curmisr_21: Core1Curmisr21,
    core1_curmisr_22: Core1Curmisr22,
    core1_curmisr_23: Core1Curmisr23,
    core1_curmisr_24: Core1Curmisr24,
    core1_curmisr_25: Core1Curmisr25,
    core1_curmisr_26: Core1Curmisr26,
    core1_curmisr_27: Core1Curmisr27,
    core2_curmisr_0: Core2Curmisr0,
    core2_curmisr_1: Core2Curmisr1,
    core2_curmisr_2: Core2Curmisr2,
    core2_curmisr_3: Core2Curmisr3,
    core2_curmisr_4: Core2Curmisr4,
    core2_curmisr_5: Core2Curmisr5,
    core2_curmisr_6: Core2Curmisr6,
    core2_curmisr_7: Core2Curmisr7,
    core2_curmisr_8: Core2Curmisr8,
    core2_curmisr_9: Core2Curmisr9,
    core2_curmisr_10: Core2Curmisr10,
    core2_curmisr_11: Core2Curmisr11,
    core2_curmisr_12: Core2Curmisr12,
    core2_curmisr_13: Core2Curmisr13,
    core2_curmisr_14: Core2Curmisr14,
    core2_curmisr_15: Core2Curmisr15,
    core2_curmisr_16: Core2Curmisr16,
    core2_curmisr_17: Core2Curmisr17,
    core2_curmisr_18: Core2Curmisr18,
    core2_curmisr_19: Core2Curmisr19,
    core2_curmisr_20: Core2Curmisr20,
    core2_curmisr_21: Core2Curmisr21,
    core2_curmisr_22: Core2Curmisr22,
    core2_curmisr_23: Core2Curmisr23,
    core2_curmisr_24: Core2Curmisr24,
    core2_curmisr_25: Core2Curmisr25,
    core2_curmisr_26: Core2Curmisr26,
    core2_curmisr_27: Core2Curmisr27,
}
impl RegisterBlock {
    #[doc = "0x00 - Self test Global control Reg0. *NOT BYTE ACCESSIBLE"]
    #[inline(always)]
    pub const fn stcgcr0(&self) -> &Stcgcr0 {
        &self.stcgcr0
    }
    #[doc = "0x04 - Self test Global control Reg1"]
    #[inline(always)]
    pub const fn stcgcr1(&self) -> &Stcgcr1 {
        &self.stcgcr1
    }
    #[doc = "0x08 - Time out counter preload register"]
    #[inline(always)]
    pub const fn stctpr(&self) -> &Stctpr {
        &self.stctpr
    }
    #[doc = "0x0c - Current Address register for CORE1"]
    #[inline(always)]
    pub const fn stc_caddr(&self) -> &StcCaddr {
        &self.stc_caddr
    }
    #[doc = "0x10 - Current Interval count register"]
    #[inline(always)]
    pub const fn stccicr(&self) -> &Stccicr {
        &self.stccicr
    }
    #[doc = "0x14 - Global Status Register"]
    #[inline(always)]
    pub const fn stcgstat(&self) -> &Stcgstat {
        &self.stcgstat
    }
    #[doc = "0x18 - Fail Status Register"]
    #[inline(always)]
    pub const fn stcfstat(&self) -> &Stcfstat {
        &self.stcfstat
    }
    #[doc = "0x1c - Signature compare Self Check Register"]
    #[inline(always)]
    pub const fn stcscscr(&self) -> &Stcscscr {
        &self.stcscscr
    }
    #[doc = "0x20 - Current Address register for CORE2"]
    #[inline(always)]
    pub const fn stc_caddr2(&self) -> &StcCaddr2 {
        &self.stc_caddr2
    }
    #[doc = "0x24 - Clock Divider Register"]
    #[inline(always)]
    pub const fn stc_clkdiv(&self) -> &StcClkdiv {
        &self.stc_clkdiv
    }
    #[doc = "0x28 - Segment 1st interval Preload Register"]
    #[inline(always)]
    pub const fn stc_segplr(&self) -> &StcSegplr {
        &self.stc_segplr
    }
    #[doc = "0x2c - ROM Start address for Segment0"]
    #[inline(always)]
    pub const fn seg0_start_addr(&self) -> &Seg0StartAddr {
        &self.seg0_start_addr
    }
    #[doc = "0x30 - ROM Start address for Segment1"]
    #[inline(always)]
    pub const fn seg1_start_addr(&self) -> &Seg1StartAddr {
        &self.seg1_start_addr
    }
    #[doc = "0x34 - ROM Start address for Segment2"]
    #[inline(always)]
    pub const fn seg2_start_addr(&self) -> &Seg2StartAddr {
        &self.seg2_start_addr
    }
    #[doc = "0x38 - ROM Start address for Segment3"]
    #[inline(always)]
    pub const fn seg3_start_addr(&self) -> &Seg3StartAddr {
        &self.seg3_start_addr
    }
    #[doc = "0x3c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_0(&self) -> &Core1Curmisr0 {
        &self.core1_curmisr_0
    }
    #[doc = "0x40 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_1(&self) -> &Core1Curmisr1 {
        &self.core1_curmisr_1
    }
    #[doc = "0x44 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_2(&self) -> &Core1Curmisr2 {
        &self.core1_curmisr_2
    }
    #[doc = "0x48 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_3(&self) -> &Core1Curmisr3 {
        &self.core1_curmisr_3
    }
    #[doc = "0x4c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_4(&self) -> &Core1Curmisr4 {
        &self.core1_curmisr_4
    }
    #[doc = "0x50 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_5(&self) -> &Core1Curmisr5 {
        &self.core1_curmisr_5
    }
    #[doc = "0x54 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_6(&self) -> &Core1Curmisr6 {
        &self.core1_curmisr_6
    }
    #[doc = "0x58 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_7(&self) -> &Core1Curmisr7 {
        &self.core1_curmisr_7
    }
    #[doc = "0x5c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_8(&self) -> &Core1Curmisr8 {
        &self.core1_curmisr_8
    }
    #[doc = "0x60 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_9(&self) -> &Core1Curmisr9 {
        &self.core1_curmisr_9
    }
    #[doc = "0x64 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_10(&self) -> &Core1Curmisr10 {
        &self.core1_curmisr_10
    }
    #[doc = "0x68 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_11(&self) -> &Core1Curmisr11 {
        &self.core1_curmisr_11
    }
    #[doc = "0x6c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_12(&self) -> &Core1Curmisr12 {
        &self.core1_curmisr_12
    }
    #[doc = "0x70 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_13(&self) -> &Core1Curmisr13 {
        &self.core1_curmisr_13
    }
    #[doc = "0x74 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_14(&self) -> &Core1Curmisr14 {
        &self.core1_curmisr_14
    }
    #[doc = "0x78 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_15(&self) -> &Core1Curmisr15 {
        &self.core1_curmisr_15
    }
    #[doc = "0x7c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_16(&self) -> &Core1Curmisr16 {
        &self.core1_curmisr_16
    }
    #[doc = "0x80 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_17(&self) -> &Core1Curmisr17 {
        &self.core1_curmisr_17
    }
    #[doc = "0x84 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_18(&self) -> &Core1Curmisr18 {
        &self.core1_curmisr_18
    }
    #[doc = "0x88 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_19(&self) -> &Core1Curmisr19 {
        &self.core1_curmisr_19
    }
    #[doc = "0x8c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_20(&self) -> &Core1Curmisr20 {
        &self.core1_curmisr_20
    }
    #[doc = "0x90 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_21(&self) -> &Core1Curmisr21 {
        &self.core1_curmisr_21
    }
    #[doc = "0x94 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_22(&self) -> &Core1Curmisr22 {
        &self.core1_curmisr_22
    }
    #[doc = "0x98 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_23(&self) -> &Core1Curmisr23 {
        &self.core1_curmisr_23
    }
    #[doc = "0x9c - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_24(&self) -> &Core1Curmisr24 {
        &self.core1_curmisr_24
    }
    #[doc = "0xa0 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_25(&self) -> &Core1Curmisr25 {
        &self.core1_curmisr_25
    }
    #[doc = "0xa4 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_26(&self) -> &Core1Curmisr26 {
        &self.core1_curmisr_26
    }
    #[doc = "0xa8 - Holds the MISR signature for CORE1"]
    #[inline(always)]
    pub const fn core1_curmisr_27(&self) -> &Core1Curmisr27 {
        &self.core1_curmisr_27
    }
    #[doc = "0xac - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_0(&self) -> &Core2Curmisr0 {
        &self.core2_curmisr_0
    }
    #[doc = "0xb0 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_1(&self) -> &Core2Curmisr1 {
        &self.core2_curmisr_1
    }
    #[doc = "0xb4 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_2(&self) -> &Core2Curmisr2 {
        &self.core2_curmisr_2
    }
    #[doc = "0xb8 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_3(&self) -> &Core2Curmisr3 {
        &self.core2_curmisr_3
    }
    #[doc = "0xbc - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_4(&self) -> &Core2Curmisr4 {
        &self.core2_curmisr_4
    }
    #[doc = "0xc0 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_5(&self) -> &Core2Curmisr5 {
        &self.core2_curmisr_5
    }
    #[doc = "0xc4 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_6(&self) -> &Core2Curmisr6 {
        &self.core2_curmisr_6
    }
    #[doc = "0xc8 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_7(&self) -> &Core2Curmisr7 {
        &self.core2_curmisr_7
    }
    #[doc = "0xcc - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_8(&self) -> &Core2Curmisr8 {
        &self.core2_curmisr_8
    }
    #[doc = "0xd0 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_9(&self) -> &Core2Curmisr9 {
        &self.core2_curmisr_9
    }
    #[doc = "0xd4 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_10(&self) -> &Core2Curmisr10 {
        &self.core2_curmisr_10
    }
    #[doc = "0xd8 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_11(&self) -> &Core2Curmisr11 {
        &self.core2_curmisr_11
    }
    #[doc = "0xdc - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_12(&self) -> &Core2Curmisr12 {
        &self.core2_curmisr_12
    }
    #[doc = "0xe0 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_13(&self) -> &Core2Curmisr13 {
        &self.core2_curmisr_13
    }
    #[doc = "0xe4 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_14(&self) -> &Core2Curmisr14 {
        &self.core2_curmisr_14
    }
    #[doc = "0xe8 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_15(&self) -> &Core2Curmisr15 {
        &self.core2_curmisr_15
    }
    #[doc = "0xec - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_16(&self) -> &Core2Curmisr16 {
        &self.core2_curmisr_16
    }
    #[doc = "0xf0 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_17(&self) -> &Core2Curmisr17 {
        &self.core2_curmisr_17
    }
    #[doc = "0xf4 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_18(&self) -> &Core2Curmisr18 {
        &self.core2_curmisr_18
    }
    #[doc = "0xf8 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_19(&self) -> &Core2Curmisr19 {
        &self.core2_curmisr_19
    }
    #[doc = "0xfc - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_20(&self) -> &Core2Curmisr20 {
        &self.core2_curmisr_20
    }
    #[doc = "0x100 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_21(&self) -> &Core2Curmisr21 {
        &self.core2_curmisr_21
    }
    #[doc = "0x104 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_22(&self) -> &Core2Curmisr22 {
        &self.core2_curmisr_22
    }
    #[doc = "0x108 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_23(&self) -> &Core2Curmisr23 {
        &self.core2_curmisr_23
    }
    #[doc = "0x10c - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_24(&self) -> &Core2Curmisr24 {
        &self.core2_curmisr_24
    }
    #[doc = "0x110 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_25(&self) -> &Core2Curmisr25 {
        &self.core2_curmisr_25
    }
    #[doc = "0x114 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_26(&self) -> &Core2Curmisr26 {
        &self.core2_curmisr_26
    }
    #[doc = "0x118 - Holds the MISR signature for CORE2"]
    #[inline(always)]
    pub const fn core2_curmisr_27(&self) -> &Core2Curmisr27 {
        &self.core2_curmisr_27
    }
}
#[doc = "STCGCR0 (rw) register accessor: Self test Global control Reg0. *NOT BYTE ACCESSIBLE\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcgcr0`]
module"]
#[doc(alias = "STCGCR0")]
pub type Stcgcr0 = crate::Reg<stcgcr0::Stcgcr0Spec>;
#[doc = "Self test Global control Reg0. *NOT BYTE ACCESSIBLE"]
pub mod stcgcr0;
#[doc = "STCGCR1 (rw) register accessor: Self test Global control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcgcr1`]
module"]
#[doc(alias = "STCGCR1")]
pub type Stcgcr1 = crate::Reg<stcgcr1::Stcgcr1Spec>;
#[doc = "Self test Global control Reg1"]
pub mod stcgcr1;
#[doc = "STCTPR (rw) register accessor: Time out counter preload register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctpr`]
module"]
#[doc(alias = "STCTPR")]
pub type Stctpr = crate::Reg<stctpr::StctprSpec>;
#[doc = "Time out counter preload register"]
pub mod stctpr;
#[doc = "STC_CADDR (rw) register accessor: Current Address register for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_caddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_caddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stc_caddr`]
module"]
#[doc(alias = "STC_CADDR")]
pub type StcCaddr = crate::Reg<stc_caddr::StcCaddrSpec>;
#[doc = "Current Address register for CORE1"]
pub mod stc_caddr;
#[doc = "STCCICR (rw) register accessor: Current Interval count register\n\nYou can [`read`](crate::Reg::read) this register and get [`stccicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stccicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stccicr`]
module"]
#[doc(alias = "STCCICR")]
pub type Stccicr = crate::Reg<stccicr::StccicrSpec>;
#[doc = "Current Interval count register"]
pub mod stccicr;
#[doc = "STCGSTAT (rw) register accessor: Global Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcgstat`]
module"]
#[doc(alias = "STCGSTAT")]
pub type Stcgstat = crate::Reg<stcgstat::StcgstatSpec>;
#[doc = "Global Status Register"]
pub mod stcgstat;
#[doc = "STCFSTAT (rw) register accessor: Fail Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcfstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcfstat`]
module"]
#[doc(alias = "STCFSTAT")]
pub type Stcfstat = crate::Reg<stcfstat::StcfstatSpec>;
#[doc = "Fail Status Register"]
pub mod stcfstat;
#[doc = "STCSCSCR (rw) register accessor: Signature compare Self Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcscscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcscscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcscscr`]
module"]
#[doc(alias = "STCSCSCR")]
pub type Stcscscr = crate::Reg<stcscscr::StcscscrSpec>;
#[doc = "Signature compare Self Check Register"]
pub mod stcscscr;
#[doc = "STC_CADDR2 (rw) register accessor: Current Address register for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_caddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_caddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stc_caddr2`]
module"]
#[doc(alias = "STC_CADDR2")]
pub type StcCaddr2 = crate::Reg<stc_caddr2::StcCaddr2Spec>;
#[doc = "Current Address register for CORE2"]
pub mod stc_caddr2;
#[doc = "STC_CLKDIV (rw) register accessor: Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stc_clkdiv`]
module"]
#[doc(alias = "STC_CLKDIV")]
pub type StcClkdiv = crate::Reg<stc_clkdiv::StcClkdivSpec>;
#[doc = "Clock Divider Register"]
pub mod stc_clkdiv;
#[doc = "STC_SEGPLR (rw) register accessor: Segment 1st interval Preload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_segplr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_segplr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stc_segplr`]
module"]
#[doc(alias = "STC_SEGPLR")]
pub type StcSegplr = crate::Reg<stc_segplr::StcSegplrSpec>;
#[doc = "Segment 1st interval Preload Register"]
pub mod stc_segplr;
#[doc = "SEG0_START_ADDR (rw) register accessor: ROM Start address for Segment0\n\nYou can [`read`](crate::Reg::read) this register and get [`seg0_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg0_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg0_start_addr`]
module"]
#[doc(alias = "SEG0_START_ADDR")]
pub type Seg0StartAddr = crate::Reg<seg0_start_addr::Seg0StartAddrSpec>;
#[doc = "ROM Start address for Segment0"]
pub mod seg0_start_addr;
#[doc = "SEG1_START_ADDR (rw) register accessor: ROM Start address for Segment1\n\nYou can [`read`](crate::Reg::read) this register and get [`seg1_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg1_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg1_start_addr`]
module"]
#[doc(alias = "SEG1_START_ADDR")]
pub type Seg1StartAddr = crate::Reg<seg1_start_addr::Seg1StartAddrSpec>;
#[doc = "ROM Start address for Segment1"]
pub mod seg1_start_addr;
#[doc = "SEG2_START_ADDR (rw) register accessor: ROM Start address for Segment2\n\nYou can [`read`](crate::Reg::read) this register and get [`seg2_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg2_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg2_start_addr`]
module"]
#[doc(alias = "SEG2_START_ADDR")]
pub type Seg2StartAddr = crate::Reg<seg2_start_addr::Seg2StartAddrSpec>;
#[doc = "ROM Start address for Segment2"]
pub mod seg2_start_addr;
#[doc = "SEG3_START_ADDR (rw) register accessor: ROM Start address for Segment3\n\nYou can [`read`](crate::Reg::read) this register and get [`seg3_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg3_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg3_start_addr`]
module"]
#[doc(alias = "SEG3_START_ADDR")]
pub type Seg3StartAddr = crate::Reg<seg3_start_addr::Seg3StartAddrSpec>;
#[doc = "ROM Start address for Segment3"]
pub mod seg3_start_addr;
#[doc = "CORE1_CURMISR_0 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_0`]
module"]
#[doc(alias = "CORE1_CURMISR_0")]
pub type Core1Curmisr0 = crate::Reg<core1_curmisr_0::Core1Curmisr0Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_0;
#[doc = "CORE1_CURMISR_1 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_1`]
module"]
#[doc(alias = "CORE1_CURMISR_1")]
pub type Core1Curmisr1 = crate::Reg<core1_curmisr_1::Core1Curmisr1Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_1;
#[doc = "CORE1_CURMISR_2 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_2`]
module"]
#[doc(alias = "CORE1_CURMISR_2")]
pub type Core1Curmisr2 = crate::Reg<core1_curmisr_2::Core1Curmisr2Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_2;
#[doc = "CORE1_CURMISR_3 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_3`]
module"]
#[doc(alias = "CORE1_CURMISR_3")]
pub type Core1Curmisr3 = crate::Reg<core1_curmisr_3::Core1Curmisr3Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_3;
#[doc = "CORE1_CURMISR_4 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_4`]
module"]
#[doc(alias = "CORE1_CURMISR_4")]
pub type Core1Curmisr4 = crate::Reg<core1_curmisr_4::Core1Curmisr4Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_4;
#[doc = "CORE1_CURMISR_5 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_5`]
module"]
#[doc(alias = "CORE1_CURMISR_5")]
pub type Core1Curmisr5 = crate::Reg<core1_curmisr_5::Core1Curmisr5Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_5;
#[doc = "CORE1_CURMISR_6 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_6`]
module"]
#[doc(alias = "CORE1_CURMISR_6")]
pub type Core1Curmisr6 = crate::Reg<core1_curmisr_6::Core1Curmisr6Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_6;
#[doc = "CORE1_CURMISR_7 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_7`]
module"]
#[doc(alias = "CORE1_CURMISR_7")]
pub type Core1Curmisr7 = crate::Reg<core1_curmisr_7::Core1Curmisr7Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_7;
#[doc = "CORE1_CURMISR_8 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_8`]
module"]
#[doc(alias = "CORE1_CURMISR_8")]
pub type Core1Curmisr8 = crate::Reg<core1_curmisr_8::Core1Curmisr8Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_8;
#[doc = "CORE1_CURMISR_9 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_9`]
module"]
#[doc(alias = "CORE1_CURMISR_9")]
pub type Core1Curmisr9 = crate::Reg<core1_curmisr_9::Core1Curmisr9Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_9;
#[doc = "CORE1_CURMISR_10 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_10`]
module"]
#[doc(alias = "CORE1_CURMISR_10")]
pub type Core1Curmisr10 = crate::Reg<core1_curmisr_10::Core1Curmisr10Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_10;
#[doc = "CORE1_CURMISR_11 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_11`]
module"]
#[doc(alias = "CORE1_CURMISR_11")]
pub type Core1Curmisr11 = crate::Reg<core1_curmisr_11::Core1Curmisr11Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_11;
#[doc = "CORE1_CURMISR_12 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_12`]
module"]
#[doc(alias = "CORE1_CURMISR_12")]
pub type Core1Curmisr12 = crate::Reg<core1_curmisr_12::Core1Curmisr12Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_12;
#[doc = "CORE1_CURMISR_13 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_13`]
module"]
#[doc(alias = "CORE1_CURMISR_13")]
pub type Core1Curmisr13 = crate::Reg<core1_curmisr_13::Core1Curmisr13Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_13;
#[doc = "CORE1_CURMISR_14 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_14`]
module"]
#[doc(alias = "CORE1_CURMISR_14")]
pub type Core1Curmisr14 = crate::Reg<core1_curmisr_14::Core1Curmisr14Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_14;
#[doc = "CORE1_CURMISR_15 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_15`]
module"]
#[doc(alias = "CORE1_CURMISR_15")]
pub type Core1Curmisr15 = crate::Reg<core1_curmisr_15::Core1Curmisr15Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_15;
#[doc = "CORE1_CURMISR_16 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_16`]
module"]
#[doc(alias = "CORE1_CURMISR_16")]
pub type Core1Curmisr16 = crate::Reg<core1_curmisr_16::Core1Curmisr16Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_16;
#[doc = "CORE1_CURMISR_17 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_17`]
module"]
#[doc(alias = "CORE1_CURMISR_17")]
pub type Core1Curmisr17 = crate::Reg<core1_curmisr_17::Core1Curmisr17Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_17;
#[doc = "CORE1_CURMISR_18 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_18`]
module"]
#[doc(alias = "CORE1_CURMISR_18")]
pub type Core1Curmisr18 = crate::Reg<core1_curmisr_18::Core1Curmisr18Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_18;
#[doc = "CORE1_CURMISR_19 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_19`]
module"]
#[doc(alias = "CORE1_CURMISR_19")]
pub type Core1Curmisr19 = crate::Reg<core1_curmisr_19::Core1Curmisr19Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_19;
#[doc = "CORE1_CURMISR_20 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_20`]
module"]
#[doc(alias = "CORE1_CURMISR_20")]
pub type Core1Curmisr20 = crate::Reg<core1_curmisr_20::Core1Curmisr20Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_20;
#[doc = "CORE1_CURMISR_21 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_21`]
module"]
#[doc(alias = "CORE1_CURMISR_21")]
pub type Core1Curmisr21 = crate::Reg<core1_curmisr_21::Core1Curmisr21Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_21;
#[doc = "CORE1_CURMISR_22 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_22`]
module"]
#[doc(alias = "CORE1_CURMISR_22")]
pub type Core1Curmisr22 = crate::Reg<core1_curmisr_22::Core1Curmisr22Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_22;
#[doc = "CORE1_CURMISR_23 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_23`]
module"]
#[doc(alias = "CORE1_CURMISR_23")]
pub type Core1Curmisr23 = crate::Reg<core1_curmisr_23::Core1Curmisr23Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_23;
#[doc = "CORE1_CURMISR_24 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_24`]
module"]
#[doc(alias = "CORE1_CURMISR_24")]
pub type Core1Curmisr24 = crate::Reg<core1_curmisr_24::Core1Curmisr24Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_24;
#[doc = "CORE1_CURMISR_25 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_25`]
module"]
#[doc(alias = "CORE1_CURMISR_25")]
pub type Core1Curmisr25 = crate::Reg<core1_curmisr_25::Core1Curmisr25Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_25;
#[doc = "CORE1_CURMISR_26 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_26`]
module"]
#[doc(alias = "CORE1_CURMISR_26")]
pub type Core1Curmisr26 = crate::Reg<core1_curmisr_26::Core1Curmisr26Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_26;
#[doc = "CORE1_CURMISR_27 (rw) register accessor: Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_curmisr_27`]
module"]
#[doc(alias = "CORE1_CURMISR_27")]
pub type Core1Curmisr27 = crate::Reg<core1_curmisr_27::Core1Curmisr27Spec>;
#[doc = "Holds the MISR signature for CORE1"]
pub mod core1_curmisr_27;
#[doc = "CORE2_CURMISR_0 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_0`]
module"]
#[doc(alias = "CORE2_CURMISR_0")]
pub type Core2Curmisr0 = crate::Reg<core2_curmisr_0::Core2Curmisr0Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_0;
#[doc = "CORE2_CURMISR_1 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_1`]
module"]
#[doc(alias = "CORE2_CURMISR_1")]
pub type Core2Curmisr1 = crate::Reg<core2_curmisr_1::Core2Curmisr1Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_1;
#[doc = "CORE2_CURMISR_2 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_2`]
module"]
#[doc(alias = "CORE2_CURMISR_2")]
pub type Core2Curmisr2 = crate::Reg<core2_curmisr_2::Core2Curmisr2Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_2;
#[doc = "CORE2_CURMISR_3 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_3`]
module"]
#[doc(alias = "CORE2_CURMISR_3")]
pub type Core2Curmisr3 = crate::Reg<core2_curmisr_3::Core2Curmisr3Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_3;
#[doc = "CORE2_CURMISR_4 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_4`]
module"]
#[doc(alias = "CORE2_CURMISR_4")]
pub type Core2Curmisr4 = crate::Reg<core2_curmisr_4::Core2Curmisr4Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_4;
#[doc = "CORE2_CURMISR_5 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_5`]
module"]
#[doc(alias = "CORE2_CURMISR_5")]
pub type Core2Curmisr5 = crate::Reg<core2_curmisr_5::Core2Curmisr5Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_5;
#[doc = "CORE2_CURMISR_6 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_6`]
module"]
#[doc(alias = "CORE2_CURMISR_6")]
pub type Core2Curmisr6 = crate::Reg<core2_curmisr_6::Core2Curmisr6Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_6;
#[doc = "CORE2_CURMISR_7 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_7`]
module"]
#[doc(alias = "CORE2_CURMISR_7")]
pub type Core2Curmisr7 = crate::Reg<core2_curmisr_7::Core2Curmisr7Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_7;
#[doc = "CORE2_CURMISR_8 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_8`]
module"]
#[doc(alias = "CORE2_CURMISR_8")]
pub type Core2Curmisr8 = crate::Reg<core2_curmisr_8::Core2Curmisr8Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_8;
#[doc = "CORE2_CURMISR_9 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_9`]
module"]
#[doc(alias = "CORE2_CURMISR_9")]
pub type Core2Curmisr9 = crate::Reg<core2_curmisr_9::Core2Curmisr9Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_9;
#[doc = "CORE2_CURMISR_10 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_10`]
module"]
#[doc(alias = "CORE2_CURMISR_10")]
pub type Core2Curmisr10 = crate::Reg<core2_curmisr_10::Core2Curmisr10Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_10;
#[doc = "CORE2_CURMISR_11 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_11`]
module"]
#[doc(alias = "CORE2_CURMISR_11")]
pub type Core2Curmisr11 = crate::Reg<core2_curmisr_11::Core2Curmisr11Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_11;
#[doc = "CORE2_CURMISR_12 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_12`]
module"]
#[doc(alias = "CORE2_CURMISR_12")]
pub type Core2Curmisr12 = crate::Reg<core2_curmisr_12::Core2Curmisr12Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_12;
#[doc = "CORE2_CURMISR_13 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_13`]
module"]
#[doc(alias = "CORE2_CURMISR_13")]
pub type Core2Curmisr13 = crate::Reg<core2_curmisr_13::Core2Curmisr13Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_13;
#[doc = "CORE2_CURMISR_14 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_14`]
module"]
#[doc(alias = "CORE2_CURMISR_14")]
pub type Core2Curmisr14 = crate::Reg<core2_curmisr_14::Core2Curmisr14Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_14;
#[doc = "CORE2_CURMISR_15 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_15`]
module"]
#[doc(alias = "CORE2_CURMISR_15")]
pub type Core2Curmisr15 = crate::Reg<core2_curmisr_15::Core2Curmisr15Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_15;
#[doc = "CORE2_CURMISR_16 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_16`]
module"]
#[doc(alias = "CORE2_CURMISR_16")]
pub type Core2Curmisr16 = crate::Reg<core2_curmisr_16::Core2Curmisr16Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_16;
#[doc = "CORE2_CURMISR_17 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_17`]
module"]
#[doc(alias = "CORE2_CURMISR_17")]
pub type Core2Curmisr17 = crate::Reg<core2_curmisr_17::Core2Curmisr17Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_17;
#[doc = "CORE2_CURMISR_18 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_18`]
module"]
#[doc(alias = "CORE2_CURMISR_18")]
pub type Core2Curmisr18 = crate::Reg<core2_curmisr_18::Core2Curmisr18Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_18;
#[doc = "CORE2_CURMISR_19 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_19`]
module"]
#[doc(alias = "CORE2_CURMISR_19")]
pub type Core2Curmisr19 = crate::Reg<core2_curmisr_19::Core2Curmisr19Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_19;
#[doc = "CORE2_CURMISR_20 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_20`]
module"]
#[doc(alias = "CORE2_CURMISR_20")]
pub type Core2Curmisr20 = crate::Reg<core2_curmisr_20::Core2Curmisr20Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_20;
#[doc = "CORE2_CURMISR_21 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_21`]
module"]
#[doc(alias = "CORE2_CURMISR_21")]
pub type Core2Curmisr21 = crate::Reg<core2_curmisr_21::Core2Curmisr21Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_21;
#[doc = "CORE2_CURMISR_22 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_22`]
module"]
#[doc(alias = "CORE2_CURMISR_22")]
pub type Core2Curmisr22 = crate::Reg<core2_curmisr_22::Core2Curmisr22Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_22;
#[doc = "CORE2_CURMISR_23 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_23`]
module"]
#[doc(alias = "CORE2_CURMISR_23")]
pub type Core2Curmisr23 = crate::Reg<core2_curmisr_23::Core2Curmisr23Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_23;
#[doc = "CORE2_CURMISR_24 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_24`]
module"]
#[doc(alias = "CORE2_CURMISR_24")]
pub type Core2Curmisr24 = crate::Reg<core2_curmisr_24::Core2Curmisr24Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_24;
#[doc = "CORE2_CURMISR_25 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_25`]
module"]
#[doc(alias = "CORE2_CURMISR_25")]
pub type Core2Curmisr25 = crate::Reg<core2_curmisr_25::Core2Curmisr25Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_25;
#[doc = "CORE2_CURMISR_26 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_26`]
module"]
#[doc(alias = "CORE2_CURMISR_26")]
pub type Core2Curmisr26 = crate::Reg<core2_curmisr_26::Core2Curmisr26Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_26;
#[doc = "CORE2_CURMISR_27 (rw) register accessor: Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_curmisr_27`]
module"]
#[doc(alias = "CORE2_CURMISR_27")]
pub type Core2Curmisr27 = crate::Reg<core2_curmisr_27::Core2Curmisr27Spec>;
#[doc = "Holds the MISR signature for CORE2"]
pub mod core2_curmisr_27;
