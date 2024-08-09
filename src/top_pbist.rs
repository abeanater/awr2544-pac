#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    pbist_a0: PbistA0,
    pbist_a1: PbistA1,
    pbist_a2: PbistA2,
    pbist_a3: PbistA3,
    pbist_l0: PbistL0,
    pbist_l1: PbistL1,
    pbist_l2: PbistL2,
    pbist_l3: PbistL3,
    pbist_dd10: PbistDd10,
    pbist_de10: PbistDe10,
    _reserved10: [u8; 0x08],
    pbist_ca0: PbistCa0,
    pbist_ca1: PbistCa1,
    pbist_ca2: PbistCa2,
    pbist_ca3: PbistCa3,
    pbist_cl0: PbistCl0,
    pbist_cl1: PbistCl1,
    pbist_cl2: PbistCl2,
    pbist_cl3: PbistCl3,
    pbist_ci0: PbistCi0,
    pbist_ci1: PbistCi1,
    pbist_ci2: PbistCi2,
    _reserved21: [u8; 0x02],
    pbist_ci3: PbistCi3,
    _reserved22: [u8; 0x02],
    pbist_ramt: PbistRamt,
    pbist_dlr: PbistDlr,
    _reserved24: [u8; 0x02],
    pbist_cms: PbistCms,
    _reserved25: [u8; 0x03],
    pbist_pc: PbistPc,
    _reserved26: [u8; 0x03],
    pbist_scr1: PbistScr1,
    pbist_scr4: PbistScr4,
    pbist_cs: PbistCs,
    pbist_fdly: PbistFdly,
    _reserved30: [u8; 0x03],
    pbist_pact: PbistPact,
    _reserved31: [u8; 0x03],
    pbist_id: PbistId,
    _reserved32: [u8; 0x03],
    pbist_ovr: PbistOvr,
    _reserved33: [u8; 0x04],
    pbist_fsfr0: PbistFsfr0,
    _reserved34: [u8; 0x03],
    pbist_fsfr1: PbistFsfr1,
    _reserved35: [u8; 0x03],
    pbist_fsrcr0: PbistFsrcr0,
    _reserved36: [u8; 0x03],
    pbist_fsrcr1: PbistFsrcr1,
    _reserved37: [u8; 0x03],
    pbist_fsra0: PbistFsra0,
    pbist_fsra1: PbistFsra1,
    _reserved39: [u8; 0x02],
    pbist_fsrdl0: PbistFsrdl0,
    _reserved40: [u8; 0x04],
    pbist_fsrdl1: PbistFsrdl1,
    pbist_margin: PbistMargin,
    pbist_wrenz: PbistWrenz,
    pbist_pgs: PbistPgs,
    pbist_rom: PbistRom,
    _reserved45: [u8; 0x03],
    pbist_algo: PbistAlgo,
    pbist_rinfol: PbistRinfol,
    pbist_rinfou: PbistRinfou,
}
impl RegisterBlock {
    #[doc = "0x100 - Variable Address Register0"]
    #[inline(always)]
    pub const fn pbist_a0(&self) -> &PbistA0 {
        &self.pbist_a0
    }
    #[doc = "0x104 - Variable Address Register1"]
    #[inline(always)]
    pub const fn pbist_a1(&self) -> &PbistA1 {
        &self.pbist_a1
    }
    #[doc = "0x108 - Variable Address Register2"]
    #[inline(always)]
    pub const fn pbist_a2(&self) -> &PbistA2 {
        &self.pbist_a2
    }
    #[doc = "0x10c - Variable Address Register3"]
    #[inline(always)]
    pub const fn pbist_a3(&self) -> &PbistA3 {
        &self.pbist_a3
    }
    #[doc = "0x110 - Variable Loop Count Register L0"]
    #[inline(always)]
    pub const fn pbist_l0(&self) -> &PbistL0 {
        &self.pbist_l0
    }
    #[doc = "0x114 - Variable Loop Count Register L1"]
    #[inline(always)]
    pub const fn pbist_l1(&self) -> &PbistL1 {
        &self.pbist_l1
    }
    #[doc = "0x118 - Variable Loop Count Register L2"]
    #[inline(always)]
    pub const fn pbist_l2(&self) -> &PbistL2 {
        &self.pbist_l2
    }
    #[doc = "0x11c - Variable Loop Count Register L3"]
    #[inline(always)]
    pub const fn pbist_l3(&self) -> &PbistL3 {
        &self.pbist_l3
    }
    #[doc = "0x120 - DD0 Data Register 16 (D0)"]
    #[inline(always)]
    pub const fn pbist_dd10(&self) -> &PbistDd10 {
        &self.pbist_dd10
    }
    #[doc = "0x124 - DE0 Data Register 16 (D0)"]
    #[inline(always)]
    pub const fn pbist_de10(&self) -> &PbistDe10 {
        &self.pbist_de10
    }
    #[doc = "0x130 - Constant Address Register0"]
    #[inline(always)]
    pub const fn pbist_ca0(&self) -> &PbistCa0 {
        &self.pbist_ca0
    }
    #[doc = "0x134 - Constant Address Register1"]
    #[inline(always)]
    pub const fn pbist_ca1(&self) -> &PbistCa1 {
        &self.pbist_ca1
    }
    #[doc = "0x138 - Constant Address Register2"]
    #[inline(always)]
    pub const fn pbist_ca2(&self) -> &PbistCa2 {
        &self.pbist_ca2
    }
    #[doc = "0x13c - Constant Address Register3"]
    #[inline(always)]
    pub const fn pbist_ca3(&self) -> &PbistCa3 {
        &self.pbist_ca3
    }
    #[doc = "0x140 - Constant Loop Count Register0"]
    #[inline(always)]
    pub const fn pbist_cl0(&self) -> &PbistCl0 {
        &self.pbist_cl0
    }
    #[doc = "0x144 - Constant Loop Count Register1"]
    #[inline(always)]
    pub const fn pbist_cl1(&self) -> &PbistCl1 {
        &self.pbist_cl1
    }
    #[doc = "0x148 - Constant Loop Count Register2"]
    #[inline(always)]
    pub const fn pbist_cl2(&self) -> &PbistCl2 {
        &self.pbist_cl2
    }
    #[doc = "0x14c - Constant Loop Count Register3"]
    #[inline(always)]
    pub const fn pbist_cl3(&self) -> &PbistCl3 {
        &self.pbist_cl3
    }
    #[doc = "0x150 - Constant Increment Register0"]
    #[inline(always)]
    pub const fn pbist_ci0(&self) -> &PbistCi0 {
        &self.pbist_ci0
    }
    #[doc = "0x154 - Constant Increment Register1"]
    #[inline(always)]
    pub const fn pbist_ci1(&self) -> &PbistCi1 {
        &self.pbist_ci1
    }
    #[doc = "0x158 - Constant Increment Register2"]
    #[inline(always)]
    pub const fn pbist_ci2(&self) -> &PbistCi2 {
        &self.pbist_ci2
    }
    #[doc = "0x15c - Constant Increment Register3"]
    #[inline(always)]
    pub const fn pbist_ci3(&self) -> &PbistCi3 {
        &self.pbist_ci3
    }
    #[doc = "0x160 - RAM Configuration (RAMT -RAM)"]
    #[inline(always)]
    pub const fn pbist_ramt(&self) -> &PbistRamt {
        &self.pbist_ramt
    }
    #[doc = "0x164 - Datalogger 0"]
    #[inline(always)]
    pub const fn pbist_dlr(&self) -> &PbistDlr {
        &self.pbist_dlr
    }
    #[doc = "0x168 - Clock mux select"]
    #[inline(always)]
    pub const fn pbist_cms(&self) -> &PbistCms {
        &self.pbist_cms
    }
    #[doc = "0x16c - Program Control"]
    #[inline(always)]
    pub const fn pbist_pc(&self) -> &PbistPc {
        &self.pbist_pc
    }
    #[doc = "0x170 - Address Scramble 0 -3"]
    #[inline(always)]
    pub const fn pbist_scr1(&self) -> &PbistScr1 {
        &self.pbist_scr1
    }
    #[doc = "0x174 - Address Scramble 4-7"]
    #[inline(always)]
    pub const fn pbist_scr4(&self) -> &PbistScr4 {
        &self.pbist_scr4
    }
    #[doc = "0x178 - Chip Select 0"]
    #[inline(always)]
    pub const fn pbist_cs(&self) -> &PbistCs {
        &self.pbist_cs
    }
    #[doc = "0x17c - Fail Delay"]
    #[inline(always)]
    pub const fn pbist_fdly(&self) -> &PbistFdly {
        &self.pbist_fdly
    }
    #[doc = "0x180 - Pbist Active"]
    #[inline(always)]
    pub const fn pbist_pact(&self) -> &PbistPact {
        &self.pbist_pact
    }
    #[doc = "0x184 - PBIST ID"]
    #[inline(always)]
    pub const fn pbist_id(&self) -> &PbistId {
        &self.pbist_id
    }
    #[doc = "0x188 - PBIST Overrides"]
    #[inline(always)]
    pub const fn pbist_ovr(&self) -> &PbistOvr {
        &self.pbist_ovr
    }
    #[doc = "0x190 - Fail status fail - port 0"]
    #[inline(always)]
    pub const fn pbist_fsfr0(&self) -> &PbistFsfr0 {
        &self.pbist_fsfr0
    }
    #[doc = "0x194 - Fail status fail - port 1"]
    #[inline(always)]
    pub const fn pbist_fsfr1(&self) -> &PbistFsfr1 {
        &self.pbist_fsfr1
    }
    #[doc = "0x198 - Fail Count fail - port 0"]
    #[inline(always)]
    pub const fn pbist_fsrcr0(&self) -> &PbistFsrcr0 {
        &self.pbist_fsrcr0
    }
    #[doc = "0x19c - Fail Count fail - port 1"]
    #[inline(always)]
    pub const fn pbist_fsrcr1(&self) -> &PbistFsrcr1 {
        &self.pbist_fsrcr1
    }
    #[doc = "0x1a0 - Fail status address - port 0"]
    #[inline(always)]
    pub const fn pbist_fsra0(&self) -> &PbistFsra0 {
        &self.pbist_fsra0
    }
    #[doc = "0x1a4 - Fail status address - port 1"]
    #[inline(always)]
    pub const fn pbist_fsra1(&self) -> &PbistFsra1 {
        &self.pbist_fsra1
    }
    #[doc = "0x1a8 - Fail status Data - port 0"]
    #[inline(always)]
    pub const fn pbist_fsrdl0(&self) -> &PbistFsrdl0 {
        &self.pbist_fsrdl0
    }
    #[doc = "0x1b0 - Fail status Data - port 1"]
    #[inline(always)]
    pub const fn pbist_fsrdl1(&self) -> &PbistFsrdl1 {
        &self.pbist_fsrdl1
    }
    #[doc = "0x1b4 - Margin Mode"]
    #[inline(always)]
    pub const fn pbist_margin(&self) -> &PbistMargin {
        &self.pbist_margin
    }
    #[doc = "0x1b8 - WRENZ"]
    #[inline(always)]
    pub const fn pbist_wrenz(&self) -> &PbistWrenz {
        &self.pbist_wrenz
    }
    #[doc = "0x1bc - PAGE/PGS"]
    #[inline(always)]
    pub const fn pbist_pgs(&self) -> &PbistPgs {
        &self.pbist_pgs
    }
    #[doc = "0x1c0 - Rom Mask"]
    #[inline(always)]
    pub const fn pbist_rom(&self) -> &PbistRom {
        &self.pbist_rom
    }
    #[doc = "0x1c4 - ROM Algorithm Mask 0"]
    #[inline(always)]
    pub const fn pbist_algo(&self) -> &PbistAlgo {
        &self.pbist_algo
    }
    #[doc = "0x1c8 - RAM Info Mask Lower 0"]
    #[inline(always)]
    pub const fn pbist_rinfol(&self) -> &PbistRinfol {
        &self.pbist_rinfol
    }
    #[doc = "0x1cc - RAM Info Mask Upper 0"]
    #[inline(always)]
    pub const fn pbist_rinfou(&self) -> &PbistRinfou {
        &self.pbist_rinfou
    }
}
#[doc = "PBIST_A0 (rw) register accessor: Variable Address Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_a0`]
module"]
#[doc(alias = "PBIST_A0")]
pub type PbistA0 = crate::Reg<pbist_a0::PbistA0Spec>;
#[doc = "Variable Address Register0"]
pub mod pbist_a0;
#[doc = "PBIST_A1 (rw) register accessor: Variable Address Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_a1`]
module"]
#[doc(alias = "PBIST_A1")]
pub type PbistA1 = crate::Reg<pbist_a1::PbistA1Spec>;
#[doc = "Variable Address Register1"]
pub mod pbist_a1;
#[doc = "PBIST_A2 (rw) register accessor: Variable Address Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_a2`]
module"]
#[doc(alias = "PBIST_A2")]
pub type PbistA2 = crate::Reg<pbist_a2::PbistA2Spec>;
#[doc = "Variable Address Register2"]
pub mod pbist_a2;
#[doc = "PBIST_A3 (rw) register accessor: Variable Address Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_a3`]
module"]
#[doc(alias = "PBIST_A3")]
pub type PbistA3 = crate::Reg<pbist_a3::PbistA3Spec>;
#[doc = "Variable Address Register3"]
pub mod pbist_a3;
#[doc = "PBIST_L0 (rw) register accessor: Variable Loop Count Register L0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_l0`]
module"]
#[doc(alias = "PBIST_L0")]
pub type PbistL0 = crate::Reg<pbist_l0::PbistL0Spec>;
#[doc = "Variable Loop Count Register L0"]
pub mod pbist_l0;
#[doc = "PBIST_L1 (rw) register accessor: Variable Loop Count Register L1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_l1`]
module"]
#[doc(alias = "PBIST_L1")]
pub type PbistL1 = crate::Reg<pbist_l1::PbistL1Spec>;
#[doc = "Variable Loop Count Register L1"]
pub mod pbist_l1;
#[doc = "PBIST_L2 (rw) register accessor: Variable Loop Count Register L2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_l2`]
module"]
#[doc(alias = "PBIST_L2")]
pub type PbistL2 = crate::Reg<pbist_l2::PbistL2Spec>;
#[doc = "Variable Loop Count Register L2"]
pub mod pbist_l2;
#[doc = "PBIST_L3 (rw) register accessor: Variable Loop Count Register L3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_l3`]
module"]
#[doc(alias = "PBIST_L3")]
pub type PbistL3 = crate::Reg<pbist_l3::PbistL3Spec>;
#[doc = "Variable Loop Count Register L3"]
pub mod pbist_l3;
#[doc = "PBIST_DD10 (rw) register accessor: DD0 Data Register 16 (D0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_dd10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_dd10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_dd10`]
module"]
#[doc(alias = "PBIST_DD10")]
pub type PbistDd10 = crate::Reg<pbist_dd10::PbistDd10Spec>;
#[doc = "DD0 Data Register 16 (D0)"]
pub mod pbist_dd10;
#[doc = "PBIST_DE10 (rw) register accessor: DE0 Data Register 16 (D0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_de10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_de10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_de10`]
module"]
#[doc(alias = "PBIST_DE10")]
pub type PbistDe10 = crate::Reg<pbist_de10::PbistDe10Spec>;
#[doc = "DE0 Data Register 16 (D0)"]
pub mod pbist_de10;
#[doc = "PBIST_CA0 (rw) register accessor: Constant Address Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ca0`]
module"]
#[doc(alias = "PBIST_CA0")]
pub type PbistCa0 = crate::Reg<pbist_ca0::PbistCa0Spec>;
#[doc = "Constant Address Register0"]
pub mod pbist_ca0;
#[doc = "PBIST_CA1 (rw) register accessor: Constant Address Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ca1`]
module"]
#[doc(alias = "PBIST_CA1")]
pub type PbistCa1 = crate::Reg<pbist_ca1::PbistCa1Spec>;
#[doc = "Constant Address Register1"]
pub mod pbist_ca1;
#[doc = "PBIST_CA2 (rw) register accessor: Constant Address Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ca2`]
module"]
#[doc(alias = "PBIST_CA2")]
pub type PbistCa2 = crate::Reg<pbist_ca2::PbistCa2Spec>;
#[doc = "Constant Address Register2"]
pub mod pbist_ca2;
#[doc = "PBIST_CA3 (rw) register accessor: Constant Address Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ca3`]
module"]
#[doc(alias = "PBIST_CA3")]
pub type PbistCa3 = crate::Reg<pbist_ca3::PbistCa3Spec>;
#[doc = "Constant Address Register3"]
pub mod pbist_ca3;
#[doc = "PBIST_CL0 (rw) register accessor: Constant Loop Count Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cl0`]
module"]
#[doc(alias = "PBIST_CL0")]
pub type PbistCl0 = crate::Reg<pbist_cl0::PbistCl0Spec>;
#[doc = "Constant Loop Count Register0"]
pub mod pbist_cl0;
#[doc = "PBIST_CL1 (rw) register accessor: Constant Loop Count Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cl1`]
module"]
#[doc(alias = "PBIST_CL1")]
pub type PbistCl1 = crate::Reg<pbist_cl1::PbistCl1Spec>;
#[doc = "Constant Loop Count Register1"]
pub mod pbist_cl1;
#[doc = "PBIST_CL2 (rw) register accessor: Constant Loop Count Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cl2`]
module"]
#[doc(alias = "PBIST_CL2")]
pub type PbistCl2 = crate::Reg<pbist_cl2::PbistCl2Spec>;
#[doc = "Constant Loop Count Register2"]
pub mod pbist_cl2;
#[doc = "PBIST_CL3 (rw) register accessor: Constant Loop Count Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cl3`]
module"]
#[doc(alias = "PBIST_CL3")]
pub type PbistCl3 = crate::Reg<pbist_cl3::PbistCl3Spec>;
#[doc = "Constant Loop Count Register3"]
pub mod pbist_cl3;
#[doc = "PBIST_CI0 (rw) register accessor: Constant Increment Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ci0`]
module"]
#[doc(alias = "PBIST_CI0")]
pub type PbistCi0 = crate::Reg<pbist_ci0::PbistCi0Spec>;
#[doc = "Constant Increment Register0"]
pub mod pbist_ci0;
#[doc = "PBIST_CI1 (rw) register accessor: Constant Increment Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ci1`]
module"]
#[doc(alias = "PBIST_CI1")]
pub type PbistCi1 = crate::Reg<pbist_ci1::PbistCi1Spec>;
#[doc = "Constant Increment Register1"]
pub mod pbist_ci1;
#[doc = "PBIST_CI2 (rw) register accessor: Constant Increment Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ci2`]
module"]
#[doc(alias = "PBIST_CI2")]
pub type PbistCi2 = crate::Reg<pbist_ci2::PbistCi2Spec>;
#[doc = "Constant Increment Register2"]
pub mod pbist_ci2;
#[doc = "PBIST_CI3 (rw) register accessor: Constant Increment Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ci3`]
module"]
#[doc(alias = "PBIST_CI3")]
pub type PbistCi3 = crate::Reg<pbist_ci3::PbistCi3Spec>;
#[doc = "Constant Increment Register3"]
pub mod pbist_ci3;
#[doc = "PBIST_RAMT (rw) register accessor: RAM Configuration (RAMT -RAM)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ramt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ramt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ramt`]
module"]
#[doc(alias = "PBIST_RAMT")]
pub type PbistRamt = crate::Reg<pbist_ramt::PbistRamtSpec>;
#[doc = "RAM Configuration (RAMT -RAM)"]
pub mod pbist_ramt;
#[doc = "PBIST_DLR (rw) register accessor: Datalogger 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_dlr`]
module"]
#[doc(alias = "PBIST_DLR")]
pub type PbistDlr = crate::Reg<pbist_dlr::PbistDlrSpec>;
#[doc = "Datalogger 0"]
pub mod pbist_dlr;
#[doc = "PBIST_CMS (rw) register accessor: Clock mux select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cms`]
module"]
#[doc(alias = "PBIST_CMS")]
pub type PbistCms = crate::Reg<pbist_cms::PbistCmsSpec>;
#[doc = "Clock mux select"]
pub mod pbist_cms;
#[doc = "PBIST_PC (rw) register accessor: Program Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_pc`]
module"]
#[doc(alias = "PBIST_PC")]
pub type PbistPc = crate::Reg<pbist_pc::PbistPcSpec>;
#[doc = "Program Control"]
pub mod pbist_pc;
#[doc = "PBIST_SCR1 (rw) register accessor: Address Scramble 0 -3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_scr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_scr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_scr1`]
module"]
#[doc(alias = "PBIST_SCR1")]
pub type PbistScr1 = crate::Reg<pbist_scr1::PbistScr1Spec>;
#[doc = "Address Scramble 0 -3"]
pub mod pbist_scr1;
#[doc = "PBIST_SCR4 (rw) register accessor: Address Scramble 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_scr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_scr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_scr4`]
module"]
#[doc(alias = "PBIST_SCR4")]
pub type PbistScr4 = crate::Reg<pbist_scr4::PbistScr4Spec>;
#[doc = "Address Scramble 4-7"]
pub mod pbist_scr4;
#[doc = "PBIST_CS (rw) register accessor: Chip Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_cs`]
module"]
#[doc(alias = "PBIST_CS")]
pub type PbistCs = crate::Reg<pbist_cs::PbistCsSpec>;
#[doc = "Chip Select 0"]
pub mod pbist_cs;
#[doc = "PBIST_FDLY (rw) register accessor: Fail Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fdly`]
module"]
#[doc(alias = "PBIST_FDLY")]
pub type PbistFdly = crate::Reg<pbist_fdly::PbistFdlySpec>;
#[doc = "Fail Delay"]
pub mod pbist_fdly;
#[doc = "PBIST_PACT (rw) register accessor: Pbist Active\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_pact`]
module"]
#[doc(alias = "PBIST_PACT")]
pub type PbistPact = crate::Reg<pbist_pact::PbistPactSpec>;
#[doc = "Pbist Active"]
pub mod pbist_pact;
#[doc = "PBIST_ID (rw) register accessor: PBIST ID\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_id`]
module"]
#[doc(alias = "PBIST_ID")]
pub type PbistId = crate::Reg<pbist_id::PbistIdSpec>;
#[doc = "PBIST ID"]
pub mod pbist_id;
#[doc = "PBIST_OVR (rw) register accessor: PBIST Overrides\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ovr`]
module"]
#[doc(alias = "PBIST_OVR")]
pub type PbistOvr = crate::Reg<pbist_ovr::PbistOvrSpec>;
#[doc = "PBIST Overrides"]
pub mod pbist_ovr;
#[doc = "PBIST_FSFR0 (rw) register accessor: Fail status fail - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsfr0`]
module"]
#[doc(alias = "PBIST_FSFR0")]
pub type PbistFsfr0 = crate::Reg<pbist_fsfr0::PbistFsfr0Spec>;
#[doc = "Fail status fail - port 0"]
pub mod pbist_fsfr0;
#[doc = "PBIST_FSFR1 (rw) register accessor: Fail status fail - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsfr1`]
module"]
#[doc(alias = "PBIST_FSFR1")]
pub type PbistFsfr1 = crate::Reg<pbist_fsfr1::PbistFsfr1Spec>;
#[doc = "Fail status fail - port 1"]
pub mod pbist_fsfr1;
#[doc = "PBIST_FSRCR0 (rw) register accessor: Fail Count fail - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsrcr0`]
module"]
#[doc(alias = "PBIST_FSRCR0")]
pub type PbistFsrcr0 = crate::Reg<pbist_fsrcr0::PbistFsrcr0Spec>;
#[doc = "Fail Count fail - port 0"]
pub mod pbist_fsrcr0;
#[doc = "PBIST_FSRCR1 (rw) register accessor: Fail Count fail - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsrcr1`]
module"]
#[doc(alias = "PBIST_FSRCR1")]
pub type PbistFsrcr1 = crate::Reg<pbist_fsrcr1::PbistFsrcr1Spec>;
#[doc = "Fail Count fail - port 1"]
pub mod pbist_fsrcr1;
#[doc = "PBIST_FSRA0 (rw) register accessor: Fail status address - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsra0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsra0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsra0`]
module"]
#[doc(alias = "PBIST_FSRA0")]
pub type PbistFsra0 = crate::Reg<pbist_fsra0::PbistFsra0Spec>;
#[doc = "Fail status address - port 0"]
pub mod pbist_fsra0;
#[doc = "PBIST_FSRA1 (rw) register accessor: Fail status address - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsra1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsra1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsra1`]
module"]
#[doc(alias = "PBIST_FSRA1")]
pub type PbistFsra1 = crate::Reg<pbist_fsra1::PbistFsra1Spec>;
#[doc = "Fail status address - port 1"]
pub mod pbist_fsra1;
#[doc = "PBIST_FSRDL0 (rw) register accessor: Fail status Data - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrdl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrdl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsrdl0`]
module"]
#[doc(alias = "PBIST_FSRDL0")]
pub type PbistFsrdl0 = crate::Reg<pbist_fsrdl0::PbistFsrdl0Spec>;
#[doc = "Fail status Data - port 0"]
pub mod pbist_fsrdl0;
#[doc = "PBIST_FSRDL1 (rw) register accessor: Fail status Data - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrdl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrdl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsrdl1`]
module"]
#[doc(alias = "PBIST_FSRDL1")]
pub type PbistFsrdl1 = crate::Reg<pbist_fsrdl1::PbistFsrdl1Spec>;
#[doc = "Fail status Data - port 1"]
pub mod pbist_fsrdl1;
#[doc = "PBIST_MARGIN (rw) register accessor: Margin Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_margin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_margin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_margin`]
module"]
#[doc(alias = "PBIST_MARGIN")]
pub type PbistMargin = crate::Reg<pbist_margin::PbistMarginSpec>;
#[doc = "Margin Mode"]
pub mod pbist_margin;
#[doc = "PBIST_WRENZ (rw) register accessor: WRENZ\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_wrenz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_wrenz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_wrenz`]
module"]
#[doc(alias = "PBIST_WRENZ")]
pub type PbistWrenz = crate::Reg<pbist_wrenz::PbistWrenzSpec>;
#[doc = "WRENZ"]
pub mod pbist_wrenz;
#[doc = "PBIST_PGS (rw) register accessor: PAGE/PGS\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pgs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pgs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_pgs`]
module"]
#[doc(alias = "PBIST_PGS")]
pub type PbistPgs = crate::Reg<pbist_pgs::PbistPgsSpec>;
#[doc = "PAGE/PGS"]
pub mod pbist_pgs;
#[doc = "PBIST_ROM (rw) register accessor: Rom Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rom`]
module"]
#[doc(alias = "PBIST_ROM")]
pub type PbistRom = crate::Reg<pbist_rom::PbistRomSpec>;
#[doc = "Rom Mask"]
pub mod pbist_rom;
#[doc = "PBIST_ALGO (rw) register accessor: ROM Algorithm Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_algo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_algo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_algo`]
module"]
#[doc(alias = "PBIST_ALGO")]
pub type PbistAlgo = crate::Reg<pbist_algo::PbistAlgoSpec>;
#[doc = "ROM Algorithm Mask 0"]
pub mod pbist_algo;
#[doc = "PBIST_RINFOL (rw) register accessor: RAM Info Mask Lower 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rinfol`]
module"]
#[doc(alias = "PBIST_RINFOL")]
pub type PbistRinfol = crate::Reg<pbist_rinfol::PbistRinfolSpec>;
#[doc = "RAM Info Mask Lower 0"]
pub mod pbist_rinfol;
#[doc = "PBIST_RINFOU (rw) register accessor: RAM Info Mask Upper 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfou::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfou::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rinfou`]
module"]
#[doc(alias = "PBIST_RINFOU")]
pub type PbistRinfou = crate::Reg<pbist_rinfou::PbistRinfouSpec>;
#[doc = "RAM Info Mask Upper 0"]
pub mod pbist_rinfou;
