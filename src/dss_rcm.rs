#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    _reserved5: [u8; 0x7c],
    dss_hwa_clk_src_sel: DssHwaClkSrcSel,
    _reserved6: [u8; 0x28],
    dss_hwa_clk_gate: DssHwaClkGate,
    _reserved7: [u8; 0x10],
    dss_cbuff_clk_gate: DssCbuffClkGate,
    _reserved8: [u8; 0x04],
    dss_hwa_clk_status: DssHwaClkStatus,
    _reserved9: [u8; 0x30],
    dss_mcrc_rst_ctrl: DssMcrcRstCtrl,
    _reserved10: [u8; 0x08],
    dss_l3_banka0_pd_ctrl: DssL3Banka0PdCtrl,
    dss_l3_banka1_pd_ctrl: DssL3Banka1PdCtrl,
    _reserved12: [u8; 0x08],
    dss_l3_bankb0_pd_ctrl: DssL3Bankb0PdCtrl,
    dss_l3_bankb1_pd_ctrl: DssL3Bankb1PdCtrl,
    _reserved14: [u8; 0x28],
    dss_hwa_pd_ctrl: DssHwaPdCtrl,
    _reserved15: [u8; 0x04],
    dss_l3_banka0_pd_status: DssL3Banka0PdStatus,
    dss_l3_banka1_pd_status: DssL3Banka1PdStatus,
    _reserved17: [u8; 0x08],
    dss_l3_bankb0_pd_status: DssL3Bankb0PdStatus,
    dss_l3_bankb1_pd_status: DssL3Bankb1PdStatus,
    _reserved19: [u8; 0x28],
    dss_hwa_pd_status: DssHwaPdStatus,
    _reserved20: [u8; 0x0c],
    dss_l3_pd_ctrl_stickybit: DssL3PdCtrlStickybit,
    _reserved21: [u8; 0x14],
    dss_hwa_rst_ctrl: DssHwaRstCtrl,
    dss_hwa_rst_ctrl_: DssHwaRstCtrl_,
    _reserved23: [u8; 0x08],
    dss_edma_rst_ctrl: DssEdmaRstCtrl,
    _reserved24: [u8; 0x08],
    dss_csi_clk_gate: DssCsiClkGate,
    dss_ip_clk_cfg: DssIpClkCfg,
    _reserved26: [u8; 0x0de4],
    hw_spare_rw0: HwSpareRw0,
    hw_spare_rw1: HwSpareRw1,
    hw_spare_rw2: HwSpareRw2,
    hw_spare_rw3: HwSpareRw3,
    hw_spare_ro0: HwSpareRo0,
    hw_spare_ro1: HwSpareRo1,
    hw_spare_ro2: HwSpareRo2,
    hw_spare_ro3: HwSpareRo3,
    hw_spare_wph: HwSpareWph,
    hw_spare_rec: HwSpareRec,
    _reserved36: [u8; 0x10],
    lock0_kick0: Lock0Kick0,
    lock0_kick1: Lock0Kick1,
    intr_raw_status: IntrRawStatus,
    intr_enabled_status_clear: IntrEnabledStatusClear,
    intr_enable: IntrEnable,
    intr_enable_clear: IntrEnableClear,
    eoi: Eoi,
    fault_address: FaultAddress,
    fault_type_status: FaultTypeStatus,
    fault_attr_status: FaultAttrStatus,
    fault_clear: FaultClear,
}
impl RegisterBlock {
    #[doc = "0x00 - PID register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - HW_REG0"]
    #[inline(always)]
    pub const fn hw_reg0(&self) -> &HwReg0 {
        &self.hw_reg0
    }
    #[doc = "0x08 - HW_REG1"]
    #[inline(always)]
    pub const fn hw_reg1(&self) -> &HwReg1 {
        &self.hw_reg1
    }
    #[doc = "0x0c - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x10 - HW_REG3"]
    #[inline(always)]
    pub const fn hw_reg3(&self) -> &HwReg3 {
        &self.hw_reg3
    }
    #[doc = "0x90 - DSS_HWA_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn dss_hwa_clk_src_sel(&self) -> &DssHwaClkSrcSel {
        &self.dss_hwa_clk_src_sel
    }
    #[doc = "0xbc - DSS_HWA_CLK_GATE"]
    #[inline(always)]
    pub const fn dss_hwa_clk_gate(&self) -> &DssHwaClkGate {
        &self.dss_hwa_clk_gate
    }
    #[doc = "0xd0 - DSS_CBUFF_CLK_GATE"]
    #[inline(always)]
    pub const fn dss_cbuff_clk_gate(&self) -> &DssCbuffClkGate {
        &self.dss_cbuff_clk_gate
    }
    #[doc = "0xd8 - DSS_HWA_CLK_STATUS"]
    #[inline(always)]
    pub const fn dss_hwa_clk_status(&self) -> &DssHwaClkStatus {
        &self.dss_hwa_clk_status
    }
    #[doc = "0x10c - DSS_MCRC_RST_CTRL"]
    #[inline(always)]
    pub const fn dss_mcrc_rst_ctrl(&self) -> &DssMcrcRstCtrl {
        &self.dss_mcrc_rst_ctrl
    }
    #[doc = "0x118 - DSS_L3_BANKA0_PD_CTRL"]
    #[inline(always)]
    pub const fn dss_l3_banka0_pd_ctrl(&self) -> &DssL3Banka0PdCtrl {
        &self.dss_l3_banka0_pd_ctrl
    }
    #[doc = "0x11c - DSS_L3_BANKA1_PD_CTRL"]
    #[inline(always)]
    pub const fn dss_l3_banka1_pd_ctrl(&self) -> &DssL3Banka1PdCtrl {
        &self.dss_l3_banka1_pd_ctrl
    }
    #[doc = "0x128 - DSS_L3_BANKB0_PD_CTRL"]
    #[inline(always)]
    pub const fn dss_l3_bankb0_pd_ctrl(&self) -> &DssL3Bankb0PdCtrl {
        &self.dss_l3_bankb0_pd_ctrl
    }
    #[doc = "0x12c - DSS_L3_BANKB1_PD_CTRL"]
    #[inline(always)]
    pub const fn dss_l3_bankb1_pd_ctrl(&self) -> &DssL3Bankb1PdCtrl {
        &self.dss_l3_bankb1_pd_ctrl
    }
    #[doc = "0x158 - DSS_HWA_PD_CTRL"]
    #[inline(always)]
    pub const fn dss_hwa_pd_ctrl(&self) -> &DssHwaPdCtrl {
        &self.dss_hwa_pd_ctrl
    }
    #[doc = "0x160 - DSS_L3_BANKA0_PD_STATUS"]
    #[inline(always)]
    pub const fn dss_l3_banka0_pd_status(&self) -> &DssL3Banka0PdStatus {
        &self.dss_l3_banka0_pd_status
    }
    #[doc = "0x164 - DSS_L3_BANKA1_PD_STATUS"]
    #[inline(always)]
    pub const fn dss_l3_banka1_pd_status(&self) -> &DssL3Banka1PdStatus {
        &self.dss_l3_banka1_pd_status
    }
    #[doc = "0x170 - DSS_L3_BANKB0_PD_STATUS"]
    #[inline(always)]
    pub const fn dss_l3_bankb0_pd_status(&self) -> &DssL3Bankb0PdStatus {
        &self.dss_l3_bankb0_pd_status
    }
    #[doc = "0x174 - DSS_L3_BANKB1_PD_STATUS"]
    #[inline(always)]
    pub const fn dss_l3_bankb1_pd_status(&self) -> &DssL3Bankb1PdStatus {
        &self.dss_l3_bankb1_pd_status
    }
    #[doc = "0x1a0 - DSS_HWA_PD_STATUS"]
    #[inline(always)]
    pub const fn dss_hwa_pd_status(&self) -> &DssHwaPdStatus {
        &self.dss_hwa_pd_status
    }
    #[doc = "0x1b0 - DSS_L3_PD_CTRL_STICKYBIT"]
    #[inline(always)]
    pub const fn dss_l3_pd_ctrl_stickybit(&self) -> &DssL3PdCtrlStickybit {
        &self.dss_l3_pd_ctrl_stickybit
    }
    #[doc = "0x1c8 - DSS_HWA_RST_CTRL"]
    #[inline(always)]
    pub const fn dss_hwa_rst_ctrl(&self) -> &DssHwaRstCtrl {
        &self.dss_hwa_rst_ctrl
    }
    #[doc = "0x1cc - DSS_HWA_RST_CTRL"]
    #[inline(always)]
    pub const fn dss_hwa_rst_ctrl_(&self) -> &DssHwaRstCtrl_ {
        &self.dss_hwa_rst_ctrl_
    }
    #[doc = "0x1d8 - DSS_EDMA_RST_CTRL"]
    #[inline(always)]
    pub const fn dss_edma_rst_ctrl(&self) -> &DssEdmaRstCtrl {
        &self.dss_edma_rst_ctrl
    }
    #[doc = "0x1e4 - DSS_CSI_CLK_GATE"]
    #[inline(always)]
    pub const fn dss_csi_clk_gate(&self) -> &DssCsiClkGate {
        &self.dss_csi_clk_gate
    }
    #[doc = "0x1e8 - DSS_IP_CLK_CFG"]
    #[inline(always)]
    pub const fn dss_ip_clk_cfg(&self) -> &DssIpClkCfg {
        &self.dss_ip_clk_cfg
    }
    #[doc = "0xfd0 - HW_SPARE_RW0"]
    #[inline(always)]
    pub const fn hw_spare_rw0(&self) -> &HwSpareRw0 {
        &self.hw_spare_rw0
    }
    #[doc = "0xfd4 - HW_SPARE_RW1"]
    #[inline(always)]
    pub const fn hw_spare_rw1(&self) -> &HwSpareRw1 {
        &self.hw_spare_rw1
    }
    #[doc = "0xfd8 - HW_SPARE_RW2"]
    #[inline(always)]
    pub const fn hw_spare_rw2(&self) -> &HwSpareRw2 {
        &self.hw_spare_rw2
    }
    #[doc = "0xfdc - HW_SPARE_RW3"]
    #[inline(always)]
    pub const fn hw_spare_rw3(&self) -> &HwSpareRw3 {
        &self.hw_spare_rw3
    }
    #[doc = "0xfe0 - HW_SPARE_RO0"]
    #[inline(always)]
    pub const fn hw_spare_ro0(&self) -> &HwSpareRo0 {
        &self.hw_spare_ro0
    }
    #[doc = "0xfe4 - HW_SPARE_RO1"]
    #[inline(always)]
    pub const fn hw_spare_ro1(&self) -> &HwSpareRo1 {
        &self.hw_spare_ro1
    }
    #[doc = "0xfe8 - HW_SPARE_RO2"]
    #[inline(always)]
    pub const fn hw_spare_ro2(&self) -> &HwSpareRo2 {
        &self.hw_spare_ro2
    }
    #[doc = "0xfec - HW_SPARE_RO3"]
    #[inline(always)]
    pub const fn hw_spare_ro3(&self) -> &HwSpareRo3 {
        &self.hw_spare_ro3
    }
    #[doc = "0xff0 - HW_SPARE_WPH"]
    #[inline(always)]
    pub const fn hw_spare_wph(&self) -> &HwSpareWph {
        &self.hw_spare_wph
    }
    #[doc = "0xff4 - HW_SPARE_REC"]
    #[inline(always)]
    pub const fn hw_spare_rec(&self) -> &HwSpareRec {
        &self.hw_spare_rec
    }
    #[doc = "0x1008 - - KICK0 component"]
    #[inline(always)]
    pub const fn lock0_kick0(&self) -> &Lock0Kick0 {
        &self.lock0_kick0
    }
    #[doc = "0x100c - - KICK1 component"]
    #[inline(always)]
    pub const fn lock0_kick1(&self) -> &Lock0Kick1 {
        &self.lock0_kick1
    }
    #[doc = "0x1010 - Interrupt Raw Status/Set Register"]
    #[inline(always)]
    pub const fn intr_raw_status(&self) -> &IntrRawStatus {
        &self.intr_raw_status
    }
    #[doc = "0x1014 - Interrupt Enabled Status/Clear register"]
    #[inline(always)]
    pub const fn intr_enabled_status_clear(&self) -> &IntrEnabledStatusClear {
        &self.intr_enabled_status_clear
    }
    #[doc = "0x1018 - Interrupt Enable register"]
    #[inline(always)]
    pub const fn intr_enable(&self) -> &IntrEnable {
        &self.intr_enable
    }
    #[doc = "0x101c - Interrupt Enable Clear register"]
    #[inline(always)]
    pub const fn intr_enable_clear(&self) -> &IntrEnableClear {
        &self.intr_enable_clear
    }
    #[doc = "0x1020 - EOI register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x1024 - Fault Address register"]
    #[inline(always)]
    pub const fn fault_address(&self) -> &FaultAddress {
        &self.fault_address
    }
    #[doc = "0x1028 - Fault Type Status register"]
    #[inline(always)]
    pub const fn fault_type_status(&self) -> &FaultTypeStatus {
        &self.fault_type_status
    }
    #[doc = "0x102c - Fault Attribute Status register"]
    #[inline(always)]
    pub const fn fault_attr_status(&self) -> &FaultAttrStatus {
        &self.fault_attr_status
    }
    #[doc = "0x1030 - Fault Clear register"]
    #[inline(always)]
    pub const fn fault_clear(&self) -> &FaultClear {
        &self.fault_clear
    }
}
#[doc = "PID (rw) register accessor: PID register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "PID register"]
pub mod pid;
#[doc = "HW_REG0 (rw) register accessor: HW_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg0`]
module"]
#[doc(alias = "HW_REG0")]
pub type HwReg0 = crate::Reg<hw_reg0::HwReg0Spec>;
#[doc = "HW_REG0"]
pub mod hw_reg0;
#[doc = "HW_REG1 (rw) register accessor: HW_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg1`]
module"]
#[doc(alias = "HW_REG1")]
pub type HwReg1 = crate::Reg<hw_reg1::HwReg1Spec>;
#[doc = "HW_REG1"]
pub mod hw_reg1;
#[doc = "PREVIOUS_NAME (rw) register accessor: PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@previous_name`]
module"]
#[doc(alias = "PREVIOUS_NAME")]
pub type PreviousName = crate::Reg<previous_name::PreviousNameSpec>;
#[doc = "PREVIOUS_NAME"]
pub mod previous_name;
#[doc = "HW_REG3 (rw) register accessor: HW_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg3`]
module"]
#[doc(alias = "HW_REG3")]
pub type HwReg3 = crate::Reg<hw_reg3::HwReg3Spec>;
#[doc = "HW_REG3"]
pub mod hw_reg3;
#[doc = "DSS_HWA_CLK_SRC_SEL (rw) register accessor: DSS_HWA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_clk_src_sel`]
module"]
#[doc(alias = "DSS_HWA_CLK_SRC_SEL")]
pub type DssHwaClkSrcSel = crate::Reg<dss_hwa_clk_src_sel::DssHwaClkSrcSelSpec>;
#[doc = "DSS_HWA_CLK_SRC_SEL"]
pub mod dss_hwa_clk_src_sel;
#[doc = "DSS_HWA_CLK_GATE (rw) register accessor: DSS_HWA_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_clk_gate`]
module"]
#[doc(alias = "DSS_HWA_CLK_GATE")]
pub type DssHwaClkGate = crate::Reg<dss_hwa_clk_gate::DssHwaClkGateSpec>;
#[doc = "DSS_HWA_CLK_GATE"]
pub mod dss_hwa_clk_gate;
#[doc = "DSS_CBUFF_CLK_GATE (rw) register accessor: DSS_CBUFF_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_cbuff_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_cbuff_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_cbuff_clk_gate`]
module"]
#[doc(alias = "DSS_CBUFF_CLK_GATE")]
pub type DssCbuffClkGate = crate::Reg<dss_cbuff_clk_gate::DssCbuffClkGateSpec>;
#[doc = "DSS_CBUFF_CLK_GATE"]
pub mod dss_cbuff_clk_gate;
#[doc = "DSS_HWA_CLK_STATUS (rw) register accessor: DSS_HWA_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_clk_status`]
module"]
#[doc(alias = "DSS_HWA_CLK_STATUS")]
pub type DssHwaClkStatus = crate::Reg<dss_hwa_clk_status::DssHwaClkStatusSpec>;
#[doc = "DSS_HWA_CLK_STATUS"]
pub mod dss_hwa_clk_status;
#[doc = "DSS_MCRC_RST_CTRL (rw) register accessor: DSS_MCRC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_mcrc_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_mcrc_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_mcrc_rst_ctrl`]
module"]
#[doc(alias = "DSS_MCRC_RST_CTRL")]
pub type DssMcrcRstCtrl = crate::Reg<dss_mcrc_rst_ctrl::DssMcrcRstCtrlSpec>;
#[doc = "DSS_MCRC_RST_CTRL"]
pub mod dss_mcrc_rst_ctrl;
#[doc = "DSS_L3_BANKA0_PD_CTRL (rw) register accessor: DSS_L3_BANKA0_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_banka0_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_banka0_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_banka0_pd_ctrl`]
module"]
#[doc(alias = "DSS_L3_BANKA0_PD_CTRL")]
pub type DssL3Banka0PdCtrl = crate::Reg<dss_l3_banka0_pd_ctrl::DssL3Banka0PdCtrlSpec>;
#[doc = "DSS_L3_BANKA0_PD_CTRL"]
pub mod dss_l3_banka0_pd_ctrl;
#[doc = "DSS_L3_BANKA1_PD_CTRL (rw) register accessor: DSS_L3_BANKA1_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_banka1_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_banka1_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_banka1_pd_ctrl`]
module"]
#[doc(alias = "DSS_L3_BANKA1_PD_CTRL")]
pub type DssL3Banka1PdCtrl = crate::Reg<dss_l3_banka1_pd_ctrl::DssL3Banka1PdCtrlSpec>;
#[doc = "DSS_L3_BANKA1_PD_CTRL"]
pub mod dss_l3_banka1_pd_ctrl;
#[doc = "DSS_L3_BANKB0_PD_CTRL (rw) register accessor: DSS_L3_BANKB0_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_bankb0_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_bankb0_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_bankb0_pd_ctrl`]
module"]
#[doc(alias = "DSS_L3_BANKB0_PD_CTRL")]
pub type DssL3Bankb0PdCtrl = crate::Reg<dss_l3_bankb0_pd_ctrl::DssL3Bankb0PdCtrlSpec>;
#[doc = "DSS_L3_BANKB0_PD_CTRL"]
pub mod dss_l3_bankb0_pd_ctrl;
#[doc = "DSS_L3_BANKB1_PD_CTRL (rw) register accessor: DSS_L3_BANKB1_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_bankb1_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_bankb1_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_bankb1_pd_ctrl`]
module"]
#[doc(alias = "DSS_L3_BANKB1_PD_CTRL")]
pub type DssL3Bankb1PdCtrl = crate::Reg<dss_l3_bankb1_pd_ctrl::DssL3Bankb1PdCtrlSpec>;
#[doc = "DSS_L3_BANKB1_PD_CTRL"]
pub mod dss_l3_bankb1_pd_ctrl;
#[doc = "DSS_HWA_PD_CTRL (rw) register accessor: DSS_HWA_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_pd_ctrl`]
module"]
#[doc(alias = "DSS_HWA_PD_CTRL")]
pub type DssHwaPdCtrl = crate::Reg<dss_hwa_pd_ctrl::DssHwaPdCtrlSpec>;
#[doc = "DSS_HWA_PD_CTRL"]
pub mod dss_hwa_pd_ctrl;
#[doc = "DSS_L3_BANKA0_PD_STATUS (rw) register accessor: DSS_L3_BANKA0_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_banka0_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_banka0_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_banka0_pd_status`]
module"]
#[doc(alias = "DSS_L3_BANKA0_PD_STATUS")]
pub type DssL3Banka0PdStatus = crate::Reg<dss_l3_banka0_pd_status::DssL3Banka0PdStatusSpec>;
#[doc = "DSS_L3_BANKA0_PD_STATUS"]
pub mod dss_l3_banka0_pd_status;
#[doc = "DSS_L3_BANKA1_PD_STATUS (rw) register accessor: DSS_L3_BANKA1_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_banka1_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_banka1_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_banka1_pd_status`]
module"]
#[doc(alias = "DSS_L3_BANKA1_PD_STATUS")]
pub type DssL3Banka1PdStatus = crate::Reg<dss_l3_banka1_pd_status::DssL3Banka1PdStatusSpec>;
#[doc = "DSS_L3_BANKA1_PD_STATUS"]
pub mod dss_l3_banka1_pd_status;
#[doc = "DSS_L3_BANKB0_PD_STATUS (rw) register accessor: DSS_L3_BANKB0_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_bankb0_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_bankb0_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_bankb0_pd_status`]
module"]
#[doc(alias = "DSS_L3_BANKB0_PD_STATUS")]
pub type DssL3Bankb0PdStatus = crate::Reg<dss_l3_bankb0_pd_status::DssL3Bankb0PdStatusSpec>;
#[doc = "DSS_L3_BANKB0_PD_STATUS"]
pub mod dss_l3_bankb0_pd_status;
#[doc = "DSS_L3_BANKB1_PD_STATUS (rw) register accessor: DSS_L3_BANKB1_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_bankb1_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_bankb1_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_bankb1_pd_status`]
module"]
#[doc(alias = "DSS_L3_BANKB1_PD_STATUS")]
pub type DssL3Bankb1PdStatus = crate::Reg<dss_l3_bankb1_pd_status::DssL3Bankb1PdStatusSpec>;
#[doc = "DSS_L3_BANKB1_PD_STATUS"]
pub mod dss_l3_bankb1_pd_status;
#[doc = "DSS_HWA_PD_STATUS (rw) register accessor: DSS_HWA_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_pd_status`]
module"]
#[doc(alias = "DSS_HWA_PD_STATUS")]
pub type DssHwaPdStatus = crate::Reg<dss_hwa_pd_status::DssHwaPdStatusSpec>;
#[doc = "DSS_HWA_PD_STATUS"]
pub mod dss_hwa_pd_status;
#[doc = "DSS_L3_PD_CTRL_STICKYBIT (rw) register accessor: DSS_L3_PD_CTRL_STICKYBIT\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_pd_ctrl_stickybit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_pd_ctrl_stickybit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3_pd_ctrl_stickybit`]
module"]
#[doc(alias = "DSS_L3_PD_CTRL_STICKYBIT")]
pub type DssL3PdCtrlStickybit = crate::Reg<dss_l3_pd_ctrl_stickybit::DssL3PdCtrlStickybitSpec>;
#[doc = "DSS_L3_PD_CTRL_STICKYBIT"]
pub mod dss_l3_pd_ctrl_stickybit;
#[doc = "DSS_HWA_RST_CTRL (rw) register accessor: DSS_HWA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_rst_ctrl`]
module"]
#[doc(alias = "DSS_HWA_RST_CTRL")]
pub type DssHwaRstCtrl = crate::Reg<dss_hwa_rst_ctrl::DssHwaRstCtrlSpec>;
#[doc = "DSS_HWA_RST_CTRL"]
pub mod dss_hwa_rst_ctrl;
#[doc = "DSS_HWA_RST_CTRL_ (rw) register accessor: DSS_HWA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_rst_ctrl_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_rst_ctrl_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_hwa_rst_ctrl_`]
module"]
#[doc(alias = "DSS_HWA_RST_CTRL_")]
pub type DssHwaRstCtrl_ = crate::Reg<dss_hwa_rst_ctrl_::DssHwaRstCtrl_Spec>;
#[doc = "DSS_HWA_RST_CTRL"]
pub mod dss_hwa_rst_ctrl_;
#[doc = "DSS_EDMA_RST_CTRL (rw) register accessor: DSS_EDMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_edma_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_edma_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_edma_rst_ctrl`]
module"]
#[doc(alias = "DSS_EDMA_RST_CTRL")]
pub type DssEdmaRstCtrl = crate::Reg<dss_edma_rst_ctrl::DssEdmaRstCtrlSpec>;
#[doc = "DSS_EDMA_RST_CTRL"]
pub mod dss_edma_rst_ctrl;
#[doc = "DSS_CSI_CLK_GATE (rw) register accessor: DSS_CSI_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_csi_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_csi_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_csi_clk_gate`]
module"]
#[doc(alias = "DSS_CSI_CLK_GATE")]
pub type DssCsiClkGate = crate::Reg<dss_csi_clk_gate::DssCsiClkGateSpec>;
#[doc = "DSS_CSI_CLK_GATE"]
pub mod dss_csi_clk_gate;
#[doc = "DSS_IP_CLK_CFG (rw) register accessor: DSS_IP_CLK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_ip_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_ip_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_ip_clk_cfg`]
module"]
#[doc(alias = "DSS_IP_CLK_CFG")]
pub type DssIpClkCfg = crate::Reg<dss_ip_clk_cfg::DssIpClkCfgSpec>;
#[doc = "DSS_IP_CLK_CFG"]
pub mod dss_ip_clk_cfg;
#[doc = "HW_SPARE_RW0 (rw) register accessor: HW_SPARE_RW0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw0`]
module"]
#[doc(alias = "HW_SPARE_RW0")]
pub type HwSpareRw0 = crate::Reg<hw_spare_rw0::HwSpareRw0Spec>;
#[doc = "HW_SPARE_RW0"]
pub mod hw_spare_rw0;
#[doc = "HW_SPARE_RW1 (rw) register accessor: HW_SPARE_RW1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw1`]
module"]
#[doc(alias = "HW_SPARE_RW1")]
pub type HwSpareRw1 = crate::Reg<hw_spare_rw1::HwSpareRw1Spec>;
#[doc = "HW_SPARE_RW1"]
pub mod hw_spare_rw1;
#[doc = "HW_SPARE_RW2 (rw) register accessor: HW_SPARE_RW2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw2`]
module"]
#[doc(alias = "HW_SPARE_RW2")]
pub type HwSpareRw2 = crate::Reg<hw_spare_rw2::HwSpareRw2Spec>;
#[doc = "HW_SPARE_RW2"]
pub mod hw_spare_rw2;
#[doc = "HW_SPARE_RW3 (rw) register accessor: HW_SPARE_RW3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw3`]
module"]
#[doc(alias = "HW_SPARE_RW3")]
pub type HwSpareRw3 = crate::Reg<hw_spare_rw3::HwSpareRw3Spec>;
#[doc = "HW_SPARE_RW3"]
pub mod hw_spare_rw3;
#[doc = "HW_SPARE_RO0 (rw) register accessor: HW_SPARE_RO0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro0`]
module"]
#[doc(alias = "HW_SPARE_RO0")]
pub type HwSpareRo0 = crate::Reg<hw_spare_ro0::HwSpareRo0Spec>;
#[doc = "HW_SPARE_RO0"]
pub mod hw_spare_ro0;
#[doc = "HW_SPARE_RO1 (rw) register accessor: HW_SPARE_RO1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro1`]
module"]
#[doc(alias = "HW_SPARE_RO1")]
pub type HwSpareRo1 = crate::Reg<hw_spare_ro1::HwSpareRo1Spec>;
#[doc = "HW_SPARE_RO1"]
pub mod hw_spare_ro1;
#[doc = "HW_SPARE_RO2 (rw) register accessor: HW_SPARE_RO2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro2`]
module"]
#[doc(alias = "HW_SPARE_RO2")]
pub type HwSpareRo2 = crate::Reg<hw_spare_ro2::HwSpareRo2Spec>;
#[doc = "HW_SPARE_RO2"]
pub mod hw_spare_ro2;
#[doc = "HW_SPARE_RO3 (rw) register accessor: HW_SPARE_RO3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro3`]
module"]
#[doc(alias = "HW_SPARE_RO3")]
pub type HwSpareRo3 = crate::Reg<hw_spare_ro3::HwSpareRo3Spec>;
#[doc = "HW_SPARE_RO3"]
pub mod hw_spare_ro3;
#[doc = "HW_SPARE_WPH (rw) register accessor: HW_SPARE_WPH\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_wph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_wph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_wph`]
module"]
#[doc(alias = "HW_SPARE_WPH")]
pub type HwSpareWph = crate::Reg<hw_spare_wph::HwSpareWphSpec>;
#[doc = "HW_SPARE_WPH"]
pub mod hw_spare_wph;
#[doc = "HW_SPARE_REC (rw) register accessor: HW_SPARE_REC\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rec`]
module"]
#[doc(alias = "HW_SPARE_REC")]
pub type HwSpareRec = crate::Reg<hw_spare_rec::HwSpareRecSpec>;
#[doc = "HW_SPARE_REC"]
pub mod hw_spare_rec;
#[doc = "LOCK0_KICK0 (rw) register accessor: - KICK0 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock0_kick0`]
module"]
#[doc(alias = "LOCK0_KICK0")]
pub type Lock0Kick0 = crate::Reg<lock0_kick0::Lock0Kick0Spec>;
#[doc = "- KICK0 component"]
pub mod lock0_kick0;
#[doc = "LOCK0_KICK1 (rw) register accessor: - KICK1 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock0_kick1`]
module"]
#[doc(alias = "LOCK0_KICK1")]
pub type Lock0Kick1 = crate::Reg<lock0_kick1::Lock0Kick1Spec>;
#[doc = "- KICK1 component"]
pub mod lock0_kick1;
#[doc = "intr_raw_status (rw) register accessor: Interrupt Raw Status/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_raw_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw_status`]
module"]
#[doc(alias = "intr_raw_status")]
pub type IntrRawStatus = crate::Reg<intr_raw_status::IntrRawStatusSpec>;
#[doc = "Interrupt Raw Status/Set Register"]
pub mod intr_raw_status;
#[doc = "intr_enabled_status_clear (rw) register accessor: Interrupt Enabled Status/Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enabled_status_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enabled_status_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enabled_status_clear`]
module"]
#[doc(alias = "intr_enabled_status_clear")]
pub type IntrEnabledStatusClear = crate::Reg<intr_enabled_status_clear::IntrEnabledStatusClearSpec>;
#[doc = "Interrupt Enabled Status/Clear register"]
pub mod intr_enabled_status_clear;
#[doc = "intr_enable (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable`]
module"]
#[doc(alias = "intr_enable")]
pub type IntrEnable = crate::Reg<intr_enable::IntrEnableSpec>;
#[doc = "Interrupt Enable register"]
pub mod intr_enable;
#[doc = "intr_enable_clear (rw) register accessor: Interrupt Enable Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable_clear`]
module"]
#[doc(alias = "intr_enable_clear")]
pub type IntrEnableClear = crate::Reg<intr_enable_clear::IntrEnableClearSpec>;
#[doc = "Interrupt Enable Clear register"]
pub mod intr_enable_clear;
#[doc = "eoi (rw) register accessor: EOI register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "eoi")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "EOI register"]
pub mod eoi;
#[doc = "fault_address (rw) register accessor: Fault Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_address`]
module"]
#[doc(alias = "fault_address")]
pub type FaultAddress = crate::Reg<fault_address::FaultAddressSpec>;
#[doc = "Fault Address register"]
pub mod fault_address;
#[doc = "fault_type_status (rw) register accessor: Fault Type Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_type_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_type_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_type_status`]
module"]
#[doc(alias = "fault_type_status")]
pub type FaultTypeStatus = crate::Reg<fault_type_status::FaultTypeStatusSpec>;
#[doc = "Fault Type Status register"]
pub mod fault_type_status;
#[doc = "fault_attr_status (rw) register accessor: Fault Attribute Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_attr_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_attr_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_attr_status`]
module"]
#[doc(alias = "fault_attr_status")]
pub type FaultAttrStatus = crate::Reg<fault_attr_status::FaultAttrStatusSpec>;
#[doc = "Fault Attribute Status register"]
pub mod fault_attr_status;
#[doc = "fault_clear (rw) register accessor: Fault Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_clear`]
module"]
#[doc(alias = "fault_clear")]
pub type FaultClear = crate::Reg<fault_clear::FaultClearSpec>;
#[doc = "Fault Clear register"]
pub mod fault_clear;
