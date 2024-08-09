#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    dss_sw_int: DssSwInt,
    dss_tpcc_a_erragg_mask: DssTpccAErraggMask,
    dss_tpcc_a_erragg_status: DssTpccAErraggStatus,
    dss_tpcc_a_erragg_status_raw: DssTpccAErraggStatusRaw,
    dss_tpcc_a_intagg_mask: DssTpccAIntaggMask,
    dss_tpcc_a_intagg_status: DssTpccAIntaggStatus,
    dss_tpcc_a_intagg_status_raw: DssTpccAIntaggStatusRaw,
    _reserved12: [u8; 0x30],
    dss_tpcc_meminit_start: DssTpccMeminitStart,
    dss_tpcc_meminit_status: DssTpccMeminitStatus,
    dss_tpcc_meminit_done: DssTpccMeminitDone,
    _reserved15: [u8; 0x2c],
    dss_l3ram_meminit_start: DssL3ramMeminitStart,
    dss_l3ram_meminit_status: DssL3ramMeminitStatus,
    dss_l3ram_meminit_done: DssL3ramMeminitDone,
    _reserved18: [u8; 0x18],
    dss_tpcc_a_parity_ctrl: DssTpccAParityCtrl,
    _reserved19: [u8; 0x08],
    dss_tpcc_a_parity_status: DssTpccAParityStatus,
    _reserved20: [u8; 0x08],
    tptc_dbs_config: TptcDbsConfig,
    dss_dsp_bootcfg: DssDspBootcfg,
    dss_dsp_nmi_gate: DssDspNmiGate,
    dss_pbist_key_reset: DssPbistKeyReset,
    dss_pbist_reg0: DssPbistReg0,
    dss_pbist_reg1: DssPbistReg1,
    dss_tptc_boundary_cfg0: DssTptcBoundaryCfg0,
    dss_tptc_boundary_cfg1: DssTptcBoundaryCfg1,
    dss_tptc_boundary_cfg2: DssTptcBoundaryCfg2,
    dss_tptc_xid_reorder_cfg0: DssTptcXidReorderCfg0,
    dss_tptc_xid_reorder_cfg1: DssTptcXidReorderCfg1,
    dss_tptc_xid_reorder_cfg2: DssTptcXidReorderCfg2,
    _reserved32: [u8; 0x04],
    esm_gating0: EsmGating0,
    esm_gating1: EsmGating1,
    esm_gating2: EsmGating2,
    esm_gating3: EsmGating3,
    _reserved36: [u8; 0x0448],
    dss_periph_erragg_mask0: DssPeriphErraggMask0,
    dss_periph_erragg_status0: DssPeriphErraggStatus0,
    dss_periph_erragg_status_raw0: DssPeriphErraggStatusRaw0,
    _reserved39: [u8; 0x18],
    dbg_ack_cpu_ctrl: DbgAckCpuCtrl,
    dbg_ack_ctl0: DbgAckCtl0,
    dbg_ack_ctl1: DbgAckCtl1,
    dss_dsp_int_sel: DssDspIntSel,
    dss_cbuff_trigger_sel: DssCbuffTriggerSel,
    _reserved44: [u8; 0x0750],
    csi2_tx_parstatcfg: Csi2TxParstatcfg,
    csi2_tx_cfg1: Csi2TxCfg1,
    csi2_tx_cfg2: Csi2TxCfg2,
    _reserved47: [u8; 0x02dc],
    hw_spare_rw0: HwSpareRw0,
    hw_spare_rw1: HwSpareRw1,
    hw_spare_rw2: HwSpareRw2,
    hw_spare_rw3: HwSpareRw3,
    hw_spare_ro0: HwSpareRo0,
    hw_spare_ro1: HwSpareRo1,
    hw_spare_ro2: HwSpareRo2,
    hw_spare_ro3: HwSpareRo3,
    _reserved55: [u8; 0x04],
    hw_spare_rec: HwSpareRec,
    _reserved56: [u8; 0x10],
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
    #[doc = "0x14 - DSS_SW_INT"]
    #[inline(always)]
    pub const fn dss_sw_int(&self) -> &DssSwInt {
        &self.dss_sw_int
    }
    #[doc = "0x18 - DSS_TPCC_A_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn dss_tpcc_a_erragg_mask(&self) -> &DssTpccAErraggMask {
        &self.dss_tpcc_a_erragg_mask
    }
    #[doc = "0x1c - DSS_TPCC_A_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn dss_tpcc_a_erragg_status(&self) -> &DssTpccAErraggStatus {
        &self.dss_tpcc_a_erragg_status
    }
    #[doc = "0x20 - DSS_TPCC_A_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn dss_tpcc_a_erragg_status_raw(&self) -> &DssTpccAErraggStatusRaw {
        &self.dss_tpcc_a_erragg_status_raw
    }
    #[doc = "0x24 - DSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    pub const fn dss_tpcc_a_intagg_mask(&self) -> &DssTpccAIntaggMask {
        &self.dss_tpcc_a_intagg_mask
    }
    #[doc = "0x28 - DSS_TPCC_A_INTAGG_STATUS"]
    #[inline(always)]
    pub const fn dss_tpcc_a_intagg_status(&self) -> &DssTpccAIntaggStatus {
        &self.dss_tpcc_a_intagg_status
    }
    #[doc = "0x2c - DSS_TPCC_A_INTAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn dss_tpcc_a_intagg_status_raw(&self) -> &DssTpccAIntaggStatusRaw {
        &self.dss_tpcc_a_intagg_status_raw
    }
    #[doc = "0x60 - DSS_TPCC_MEMINIT_START"]
    #[inline(always)]
    pub const fn dss_tpcc_meminit_start(&self) -> &DssTpccMeminitStart {
        &self.dss_tpcc_meminit_start
    }
    #[doc = "0x64 - DSS_TPCC_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn dss_tpcc_meminit_status(&self) -> &DssTpccMeminitStatus {
        &self.dss_tpcc_meminit_status
    }
    #[doc = "0x68 - DSS_TPCC_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn dss_tpcc_meminit_done(&self) -> &DssTpccMeminitDone {
        &self.dss_tpcc_meminit_done
    }
    #[doc = "0x98 - DSS_L3RAM_MEMINIT_START"]
    #[inline(always)]
    pub const fn dss_l3ram_meminit_start(&self) -> &DssL3ramMeminitStart {
        &self.dss_l3ram_meminit_start
    }
    #[doc = "0x9c - DSS_L3RAM_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn dss_l3ram_meminit_status(&self) -> &DssL3ramMeminitStatus {
        &self.dss_l3ram_meminit_status
    }
    #[doc = "0xa0 - DSS_L3RAM_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn dss_l3ram_meminit_done(&self) -> &DssL3ramMeminitDone {
        &self.dss_l3ram_meminit_done
    }
    #[doc = "0xbc - DSS_TPCC_A_PARITY_CTRL"]
    #[inline(always)]
    pub const fn dss_tpcc_a_parity_ctrl(&self) -> &DssTpccAParityCtrl {
        &self.dss_tpcc_a_parity_ctrl
    }
    #[doc = "0xc8 - DSS_TPCC_A_PARITY_STATUS"]
    #[inline(always)]
    pub const fn dss_tpcc_a_parity_status(&self) -> &DssTpccAParityStatus {
        &self.dss_tpcc_a_parity_status
    }
    #[doc = "0xd4 - TPTC_DBS_CONFIG"]
    #[inline(always)]
    pub const fn tptc_dbs_config(&self) -> &TptcDbsConfig {
        &self.tptc_dbs_config
    }
    #[doc = "0xd8 - DSS_DSP_BOOTCFG"]
    #[inline(always)]
    pub const fn dss_dsp_bootcfg(&self) -> &DssDspBootcfg {
        &self.dss_dsp_bootcfg
    }
    #[doc = "0xdc - DSS_DSP_NMI_GATE"]
    #[inline(always)]
    pub const fn dss_dsp_nmi_gate(&self) -> &DssDspNmiGate {
        &self.dss_dsp_nmi_gate
    }
    #[doc = "0xe0 - DSS_PBIST_KEY_RESET"]
    #[inline(always)]
    pub const fn dss_pbist_key_reset(&self) -> &DssPbistKeyReset {
        &self.dss_pbist_key_reset
    }
    #[doc = "0xe4 - DSS_PBIST_REG0"]
    #[inline(always)]
    pub const fn dss_pbist_reg0(&self) -> &DssPbistReg0 {
        &self.dss_pbist_reg0
    }
    #[doc = "0xe8 - DSS_PBIST_REG1"]
    #[inline(always)]
    pub const fn dss_pbist_reg1(&self) -> &DssPbistReg1 {
        &self.dss_pbist_reg1
    }
    #[doc = "0xec - DSS_TPTC_BOUNDARY_CFG0"]
    #[inline(always)]
    pub const fn dss_tptc_boundary_cfg0(&self) -> &DssTptcBoundaryCfg0 {
        &self.dss_tptc_boundary_cfg0
    }
    #[doc = "0xf0 - DSS_TPTC_BOUNDARY_CFG1"]
    #[inline(always)]
    pub const fn dss_tptc_boundary_cfg1(&self) -> &DssTptcBoundaryCfg1 {
        &self.dss_tptc_boundary_cfg1
    }
    #[doc = "0xf4 - DSS_TPTC_BOUNDARY_CFG2"]
    #[inline(always)]
    pub const fn dss_tptc_boundary_cfg2(&self) -> &DssTptcBoundaryCfg2 {
        &self.dss_tptc_boundary_cfg2
    }
    #[doc = "0xf8 - DSS_TPTC_XID_REORDER_CFG0"]
    #[inline(always)]
    pub const fn dss_tptc_xid_reorder_cfg0(&self) -> &DssTptcXidReorderCfg0 {
        &self.dss_tptc_xid_reorder_cfg0
    }
    #[doc = "0xfc - DSS_TPTC_XID_REORDER_CFG1"]
    #[inline(always)]
    pub const fn dss_tptc_xid_reorder_cfg1(&self) -> &DssTptcXidReorderCfg1 {
        &self.dss_tptc_xid_reorder_cfg1
    }
    #[doc = "0x100 - DSS_TPTC_XID_REORDER_CFG2"]
    #[inline(always)]
    pub const fn dss_tptc_xid_reorder_cfg2(&self) -> &DssTptcXidReorderCfg2 {
        &self.dss_tptc_xid_reorder_cfg2
    }
    #[doc = "0x108 - ESM_GATING0"]
    #[inline(always)]
    pub const fn esm_gating0(&self) -> &EsmGating0 {
        &self.esm_gating0
    }
    #[doc = "0x10c - ESM_GATING1"]
    #[inline(always)]
    pub const fn esm_gating1(&self) -> &EsmGating1 {
        &self.esm_gating1
    }
    #[doc = "0x110 - ESM_GATING2"]
    #[inline(always)]
    pub const fn esm_gating2(&self) -> &EsmGating2 {
        &self.esm_gating2
    }
    #[doc = "0x114 - ESM_GATING3"]
    #[inline(always)]
    pub const fn esm_gating3(&self) -> &EsmGating3 {
        &self.esm_gating3
    }
    #[doc = "0x560 - DSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub const fn dss_periph_erragg_mask0(&self) -> &DssPeriphErraggMask0 {
        &self.dss_periph_erragg_mask0
    }
    #[doc = "0x564 - DSS_PERIPH_ERRAGG_STATUS0"]
    #[inline(always)]
    pub const fn dss_periph_erragg_status0(&self) -> &DssPeriphErraggStatus0 {
        &self.dss_periph_erragg_status0
    }
    #[doc = "0x568 - DSS_PERIPH_ERRAGG_STATUS_RAW0"]
    #[inline(always)]
    pub const fn dss_periph_erragg_status_raw0(&self) -> &DssPeriphErraggStatusRaw0 {
        &self.dss_periph_erragg_status_raw0
    }
    #[doc = "0x584 - DBG_ACK_CPU_CTRL"]
    #[inline(always)]
    pub const fn dbg_ack_cpu_ctrl(&self) -> &DbgAckCpuCtrl {
        &self.dbg_ack_cpu_ctrl
    }
    #[doc = "0x588 - DBG_ACK_CTL0"]
    #[inline(always)]
    pub const fn dbg_ack_ctl0(&self) -> &DbgAckCtl0 {
        &self.dbg_ack_ctl0
    }
    #[doc = "0x58c - DBG_ACK_CTL1"]
    #[inline(always)]
    pub const fn dbg_ack_ctl1(&self) -> &DbgAckCtl1 {
        &self.dbg_ack_ctl1
    }
    #[doc = "0x590 - DSS_DSP_INT_SEL"]
    #[inline(always)]
    pub const fn dss_dsp_int_sel(&self) -> &DssDspIntSel {
        &self.dss_dsp_int_sel
    }
    #[doc = "0x594 - DSS_CBUFF_TRIGGER_SEL"]
    #[inline(always)]
    pub const fn dss_cbuff_trigger_sel(&self) -> &DssCbuffTriggerSel {
        &self.dss_cbuff_trigger_sel
    }
    #[doc = "0xce8 - CSI2_TX_PARSTATCFG"]
    #[inline(always)]
    pub const fn csi2_tx_parstatcfg(&self) -> &Csi2TxParstatcfg {
        &self.csi2_tx_parstatcfg
    }
    #[doc = "0xcec - CSI2_TX_CFG1"]
    #[inline(always)]
    pub const fn csi2_tx_cfg1(&self) -> &Csi2TxCfg1 {
        &self.csi2_tx_cfg1
    }
    #[doc = "0xcf0 - CSI2_TX_CFG2"]
    #[inline(always)]
    pub const fn csi2_tx_cfg2(&self) -> &Csi2TxCfg2 {
        &self.csi2_tx_cfg2
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
#[doc = "DSS_SW_INT (rw) register accessor: DSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_sw_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_sw_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_sw_int`]
module"]
#[doc(alias = "DSS_SW_INT")]
pub type DssSwInt = crate::Reg<dss_sw_int::DssSwIntSpec>;
#[doc = "DSS_SW_INT"]
pub mod dss_sw_int;
#[doc = "DSS_TPCC_A_ERRAGG_MASK (rw) register accessor: DSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_erragg_mask`]
module"]
#[doc(alias = "DSS_TPCC_A_ERRAGG_MASK")]
pub type DssTpccAErraggMask = crate::Reg<dss_tpcc_a_erragg_mask::DssTpccAErraggMaskSpec>;
#[doc = "DSS_TPCC_A_ERRAGG_MASK"]
pub mod dss_tpcc_a_erragg_mask;
#[doc = "DSS_TPCC_A_ERRAGG_STATUS (rw) register accessor: DSS_TPCC_A_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_erragg_status`]
module"]
#[doc(alias = "DSS_TPCC_A_ERRAGG_STATUS")]
pub type DssTpccAErraggStatus = crate::Reg<dss_tpcc_a_erragg_status::DssTpccAErraggStatusSpec>;
#[doc = "DSS_TPCC_A_ERRAGG_STATUS"]
pub mod dss_tpcc_a_erragg_status;
#[doc = "DSS_TPCC_A_ERRAGG_STATUS_RAW (rw) register accessor: DSS_TPCC_A_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_erragg_status_raw`]
module"]
#[doc(alias = "DSS_TPCC_A_ERRAGG_STATUS_RAW")]
pub type DssTpccAErraggStatusRaw =
    crate::Reg<dss_tpcc_a_erragg_status_raw::DssTpccAErraggStatusRawSpec>;
#[doc = "DSS_TPCC_A_ERRAGG_STATUS_RAW"]
pub mod dss_tpcc_a_erragg_status_raw;
#[doc = "DSS_TPCC_A_INTAGG_MASK (rw) register accessor: DSS_TPCC_A_INTAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_intagg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_intagg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_intagg_mask`]
module"]
#[doc(alias = "DSS_TPCC_A_INTAGG_MASK")]
pub type DssTpccAIntaggMask = crate::Reg<dss_tpcc_a_intagg_mask::DssTpccAIntaggMaskSpec>;
#[doc = "DSS_TPCC_A_INTAGG_MASK"]
pub mod dss_tpcc_a_intagg_mask;
#[doc = "DSS_TPCC_A_INTAGG_STATUS (rw) register accessor: DSS_TPCC_A_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_intagg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_intagg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_intagg_status`]
module"]
#[doc(alias = "DSS_TPCC_A_INTAGG_STATUS")]
pub type DssTpccAIntaggStatus = crate::Reg<dss_tpcc_a_intagg_status::DssTpccAIntaggStatusSpec>;
#[doc = "DSS_TPCC_A_INTAGG_STATUS"]
pub mod dss_tpcc_a_intagg_status;
#[doc = "DSS_TPCC_A_INTAGG_STATUS_RAW (rw) register accessor: DSS_TPCC_A_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_intagg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_intagg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_intagg_status_raw`]
module"]
#[doc(alias = "DSS_TPCC_A_INTAGG_STATUS_RAW")]
pub type DssTpccAIntaggStatusRaw =
    crate::Reg<dss_tpcc_a_intagg_status_raw::DssTpccAIntaggStatusRawSpec>;
#[doc = "DSS_TPCC_A_INTAGG_STATUS_RAW"]
pub mod dss_tpcc_a_intagg_status_raw;
#[doc = "DSS_TPCC_MEMINIT_START (rw) register accessor: DSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_meminit_start`]
module"]
#[doc(alias = "DSS_TPCC_MEMINIT_START")]
pub type DssTpccMeminitStart = crate::Reg<dss_tpcc_meminit_start::DssTpccMeminitStartSpec>;
#[doc = "DSS_TPCC_MEMINIT_START"]
pub mod dss_tpcc_meminit_start;
#[doc = "DSS_TPCC_MEMINIT_STATUS (rw) register accessor: DSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_meminit_status`]
module"]
#[doc(alias = "DSS_TPCC_MEMINIT_STATUS")]
pub type DssTpccMeminitStatus = crate::Reg<dss_tpcc_meminit_status::DssTpccMeminitStatusSpec>;
#[doc = "DSS_TPCC_MEMINIT_STATUS"]
pub mod dss_tpcc_meminit_status;
#[doc = "DSS_TPCC_MEMINIT_DONE (rw) register accessor: DSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_meminit_done`]
module"]
#[doc(alias = "DSS_TPCC_MEMINIT_DONE")]
pub type DssTpccMeminitDone = crate::Reg<dss_tpcc_meminit_done::DssTpccMeminitDoneSpec>;
#[doc = "DSS_TPCC_MEMINIT_DONE"]
pub mod dss_tpcc_meminit_done;
#[doc = "DSS_L3RAM_MEMINIT_START (rw) register accessor: DSS_L3RAM_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3ram_meminit_start`]
module"]
#[doc(alias = "DSS_L3RAM_MEMINIT_START")]
pub type DssL3ramMeminitStart = crate::Reg<dss_l3ram_meminit_start::DssL3ramMeminitStartSpec>;
#[doc = "DSS_L3RAM_MEMINIT_START"]
pub mod dss_l3ram_meminit_start;
#[doc = "DSS_L3RAM_MEMINIT_STATUS (rw) register accessor: DSS_L3RAM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3ram_meminit_status`]
module"]
#[doc(alias = "DSS_L3RAM_MEMINIT_STATUS")]
pub type DssL3ramMeminitStatus = crate::Reg<dss_l3ram_meminit_status::DssL3ramMeminitStatusSpec>;
#[doc = "DSS_L3RAM_MEMINIT_STATUS"]
pub mod dss_l3ram_meminit_status;
#[doc = "DSS_L3RAM_MEMINIT_DONE (rw) register accessor: DSS_L3RAM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_l3ram_meminit_done`]
module"]
#[doc(alias = "DSS_L3RAM_MEMINIT_DONE")]
pub type DssL3ramMeminitDone = crate::Reg<dss_l3ram_meminit_done::DssL3ramMeminitDoneSpec>;
#[doc = "DSS_L3RAM_MEMINIT_DONE"]
pub mod dss_l3ram_meminit_done;
#[doc = "DSS_TPCC_A_PARITY_CTRL (rw) register accessor: DSS_TPCC_A_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_parity_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_parity_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_parity_ctrl`]
module"]
#[doc(alias = "DSS_TPCC_A_PARITY_CTRL")]
pub type DssTpccAParityCtrl = crate::Reg<dss_tpcc_a_parity_ctrl::DssTpccAParityCtrlSpec>;
#[doc = "DSS_TPCC_A_PARITY_CTRL"]
pub mod dss_tpcc_a_parity_ctrl;
#[doc = "DSS_TPCC_A_PARITY_STATUS (rw) register accessor: DSS_TPCC_A_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_parity_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_parity_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tpcc_a_parity_status`]
module"]
#[doc(alias = "DSS_TPCC_A_PARITY_STATUS")]
pub type DssTpccAParityStatus = crate::Reg<dss_tpcc_a_parity_status::DssTpccAParityStatusSpec>;
#[doc = "DSS_TPCC_A_PARITY_STATUS"]
pub mod dss_tpcc_a_parity_status;
#[doc = "TPTC_DBS_CONFIG (rw) register accessor: TPTC_DBS_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tptc_dbs_config`]
module"]
#[doc(alias = "TPTC_DBS_CONFIG")]
pub type TptcDbsConfig = crate::Reg<tptc_dbs_config::TptcDbsConfigSpec>;
#[doc = "TPTC_DBS_CONFIG"]
pub mod tptc_dbs_config;
#[doc = "DSS_DSP_BOOTCFG (rw) register accessor: DSS_DSP_BOOTCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_bootcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_bootcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_dsp_bootcfg`]
module"]
#[doc(alias = "DSS_DSP_BOOTCFG")]
pub type DssDspBootcfg = crate::Reg<dss_dsp_bootcfg::DssDspBootcfgSpec>;
#[doc = "DSS_DSP_BOOTCFG"]
pub mod dss_dsp_bootcfg;
#[doc = "DSS_DSP_NMI_GATE (rw) register accessor: DSS_DSP_NMI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_nmi_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_nmi_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_dsp_nmi_gate`]
module"]
#[doc(alias = "DSS_DSP_NMI_GATE")]
pub type DssDspNmiGate = crate::Reg<dss_dsp_nmi_gate::DssDspNmiGateSpec>;
#[doc = "DSS_DSP_NMI_GATE"]
pub mod dss_dsp_nmi_gate;
#[doc = "DSS_PBIST_KEY_RESET (rw) register accessor: DSS_PBIST_KEY_RESET\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_key_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_key_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_pbist_key_reset`]
module"]
#[doc(alias = "DSS_PBIST_KEY_RESET")]
pub type DssPbistKeyReset = crate::Reg<dss_pbist_key_reset::DssPbistKeyResetSpec>;
#[doc = "DSS_PBIST_KEY_RESET"]
pub mod dss_pbist_key_reset;
#[doc = "DSS_PBIST_REG0 (rw) register accessor: DSS_PBIST_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_pbist_reg0`]
module"]
#[doc(alias = "DSS_PBIST_REG0")]
pub type DssPbistReg0 = crate::Reg<dss_pbist_reg0::DssPbistReg0Spec>;
#[doc = "DSS_PBIST_REG0"]
pub mod dss_pbist_reg0;
#[doc = "DSS_PBIST_REG1 (rw) register accessor: DSS_PBIST_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_pbist_reg1`]
module"]
#[doc(alias = "DSS_PBIST_REG1")]
pub type DssPbistReg1 = crate::Reg<dss_pbist_reg1::DssPbistReg1Spec>;
#[doc = "DSS_PBIST_REG1"]
pub mod dss_pbist_reg1;
#[doc = "DSS_TPTC_BOUNDARY_CFG0 (rw) register accessor: DSS_TPTC_BOUNDARY_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_boundary_cfg0`]
module"]
#[doc(alias = "DSS_TPTC_BOUNDARY_CFG0")]
pub type DssTptcBoundaryCfg0 = crate::Reg<dss_tptc_boundary_cfg0::DssTptcBoundaryCfg0Spec>;
#[doc = "DSS_TPTC_BOUNDARY_CFG0"]
pub mod dss_tptc_boundary_cfg0;
#[doc = "DSS_TPTC_BOUNDARY_CFG1 (rw) register accessor: DSS_TPTC_BOUNDARY_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_boundary_cfg1`]
module"]
#[doc(alias = "DSS_TPTC_BOUNDARY_CFG1")]
pub type DssTptcBoundaryCfg1 = crate::Reg<dss_tptc_boundary_cfg1::DssTptcBoundaryCfg1Spec>;
#[doc = "DSS_TPTC_BOUNDARY_CFG1"]
pub mod dss_tptc_boundary_cfg1;
#[doc = "DSS_TPTC_BOUNDARY_CFG2 (rw) register accessor: DSS_TPTC_BOUNDARY_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_boundary_cfg2`]
module"]
#[doc(alias = "DSS_TPTC_BOUNDARY_CFG2")]
pub type DssTptcBoundaryCfg2 = crate::Reg<dss_tptc_boundary_cfg2::DssTptcBoundaryCfg2Spec>;
#[doc = "DSS_TPTC_BOUNDARY_CFG2"]
pub mod dss_tptc_boundary_cfg2;
#[doc = "DSS_TPTC_XID_REORDER_CFG0 (rw) register accessor: DSS_TPTC_XID_REORDER_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_xid_reorder_cfg0`]
module"]
#[doc(alias = "DSS_TPTC_XID_REORDER_CFG0")]
pub type DssTptcXidReorderCfg0 = crate::Reg<dss_tptc_xid_reorder_cfg0::DssTptcXidReorderCfg0Spec>;
#[doc = "DSS_TPTC_XID_REORDER_CFG0"]
pub mod dss_tptc_xid_reorder_cfg0;
#[doc = "DSS_TPTC_XID_REORDER_CFG1 (rw) register accessor: DSS_TPTC_XID_REORDER_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_xid_reorder_cfg1`]
module"]
#[doc(alias = "DSS_TPTC_XID_REORDER_CFG1")]
pub type DssTptcXidReorderCfg1 = crate::Reg<dss_tptc_xid_reorder_cfg1::DssTptcXidReorderCfg1Spec>;
#[doc = "DSS_TPTC_XID_REORDER_CFG1"]
pub mod dss_tptc_xid_reorder_cfg1;
#[doc = "DSS_TPTC_XID_REORDER_CFG2 (rw) register accessor: DSS_TPTC_XID_REORDER_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_tptc_xid_reorder_cfg2`]
module"]
#[doc(alias = "DSS_TPTC_XID_REORDER_CFG2")]
pub type DssTptcXidReorderCfg2 = crate::Reg<dss_tptc_xid_reorder_cfg2::DssTptcXidReorderCfg2Spec>;
#[doc = "DSS_TPTC_XID_REORDER_CFG2"]
pub mod dss_tptc_xid_reorder_cfg2;
#[doc = "ESM_GATING0 (rw) register accessor: ESM_GATING0\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating0`]
module"]
#[doc(alias = "ESM_GATING0")]
pub type EsmGating0 = crate::Reg<esm_gating0::EsmGating0Spec>;
#[doc = "ESM_GATING0"]
pub mod esm_gating0;
#[doc = "ESM_GATING1 (rw) register accessor: ESM_GATING1\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating1`]
module"]
#[doc(alias = "ESM_GATING1")]
pub type EsmGating1 = crate::Reg<esm_gating1::EsmGating1Spec>;
#[doc = "ESM_GATING1"]
pub mod esm_gating1;
#[doc = "ESM_GATING2 (rw) register accessor: ESM_GATING2\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating2`]
module"]
#[doc(alias = "ESM_GATING2")]
pub type EsmGating2 = crate::Reg<esm_gating2::EsmGating2Spec>;
#[doc = "ESM_GATING2"]
pub mod esm_gating2;
#[doc = "ESM_GATING3 (rw) register accessor: ESM_GATING3\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating3`]
module"]
#[doc(alias = "ESM_GATING3")]
pub type EsmGating3 = crate::Reg<esm_gating3::EsmGating3Spec>;
#[doc = "ESM_GATING3"]
pub mod esm_gating3;
#[doc = "DSS_PERIPH_ERRAGG_MASK0 (rw) register accessor: DSS_PERIPH_ERRAGG_MASK0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_periph_erragg_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_periph_erragg_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_periph_erragg_mask0`]
module"]
#[doc(alias = "DSS_PERIPH_ERRAGG_MASK0")]
pub type DssPeriphErraggMask0 = crate::Reg<dss_periph_erragg_mask0::DssPeriphErraggMask0Spec>;
#[doc = "DSS_PERIPH_ERRAGG_MASK0"]
pub mod dss_periph_erragg_mask0;
#[doc = "DSS_PERIPH_ERRAGG_STATUS0 (rw) register accessor: DSS_PERIPH_ERRAGG_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_periph_erragg_status0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_periph_erragg_status0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_periph_erragg_status0`]
module"]
#[doc(alias = "DSS_PERIPH_ERRAGG_STATUS0")]
pub type DssPeriphErraggStatus0 = crate::Reg<dss_periph_erragg_status0::DssPeriphErraggStatus0Spec>;
#[doc = "DSS_PERIPH_ERRAGG_STATUS0"]
pub mod dss_periph_erragg_status0;
#[doc = "DSS_PERIPH_ERRAGG_STATUS_RAW0 (rw) register accessor: DSS_PERIPH_ERRAGG_STATUS_RAW0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_periph_erragg_status_raw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_periph_erragg_status_raw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_periph_erragg_status_raw0`]
module"]
#[doc(alias = "DSS_PERIPH_ERRAGG_STATUS_RAW0")]
pub type DssPeriphErraggStatusRaw0 =
    crate::Reg<dss_periph_erragg_status_raw0::DssPeriphErraggStatusRaw0Spec>;
#[doc = "DSS_PERIPH_ERRAGG_STATUS_RAW0"]
pub mod dss_periph_erragg_status_raw0;
#[doc = "DBG_ACK_CPU_CTRL (rw) register accessor: DBG_ACK_CPU_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_cpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_cpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_ack_cpu_ctrl`]
module"]
#[doc(alias = "DBG_ACK_CPU_CTRL")]
pub type DbgAckCpuCtrl = crate::Reg<dbg_ack_cpu_ctrl::DbgAckCpuCtrlSpec>;
#[doc = "DBG_ACK_CPU_CTRL"]
pub mod dbg_ack_cpu_ctrl;
#[doc = "DBG_ACK_CTL0 (rw) register accessor: DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_ack_ctl0`]
module"]
#[doc(alias = "DBG_ACK_CTL0")]
pub type DbgAckCtl0 = crate::Reg<dbg_ack_ctl0::DbgAckCtl0Spec>;
#[doc = "DBG_ACK_CTL0"]
pub mod dbg_ack_ctl0;
#[doc = "DBG_ACK_CTL1 (rw) register accessor: DBG_ACK_CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_ack_ctl1`]
module"]
#[doc(alias = "DBG_ACK_CTL1")]
pub type DbgAckCtl1 = crate::Reg<dbg_ack_ctl1::DbgAckCtl1Spec>;
#[doc = "DBG_ACK_CTL1"]
pub mod dbg_ack_ctl1;
#[doc = "DSS_DSP_INT_SEL (rw) register accessor: DSS_DSP_INT_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_int_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_int_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_dsp_int_sel`]
module"]
#[doc(alias = "DSS_DSP_INT_SEL")]
pub type DssDspIntSel = crate::Reg<dss_dsp_int_sel::DssDspIntSelSpec>;
#[doc = "DSS_DSP_INT_SEL"]
pub mod dss_dsp_int_sel;
#[doc = "DSS_CBUFF_TRIGGER_SEL (rw) register accessor: DSS_CBUFF_TRIGGER_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_cbuff_trigger_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_cbuff_trigger_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_cbuff_trigger_sel`]
module"]
#[doc(alias = "DSS_CBUFF_TRIGGER_SEL")]
pub type DssCbuffTriggerSel = crate::Reg<dss_cbuff_trigger_sel::DssCbuffTriggerSelSpec>;
#[doc = "DSS_CBUFF_TRIGGER_SEL"]
pub mod dss_cbuff_trigger_sel;
#[doc = "CSI2_TX_PARSTATCFG (rw) register accessor: CSI2_TX_PARSTATCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_parstatcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_parstatcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_tx_parstatcfg`]
module"]
#[doc(alias = "CSI2_TX_PARSTATCFG")]
pub type Csi2TxParstatcfg = crate::Reg<csi2_tx_parstatcfg::Csi2TxParstatcfgSpec>;
#[doc = "CSI2_TX_PARSTATCFG"]
pub mod csi2_tx_parstatcfg;
#[doc = "CSI2_TX_CFG1 (rw) register accessor: CSI2_TX_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_tx_cfg1`]
module"]
#[doc(alias = "CSI2_TX_CFG1")]
pub type Csi2TxCfg1 = crate::Reg<csi2_tx_cfg1::Csi2TxCfg1Spec>;
#[doc = "CSI2_TX_CFG1"]
pub mod csi2_tx_cfg1;
#[doc = "CSI2_TX_CFG2 (rw) register accessor: CSI2_TX_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_tx_cfg2`]
module"]
#[doc(alias = "CSI2_TX_CFG2")]
pub type Csi2TxCfg2 = crate::Reg<csi2_tx_cfg2::Csi2TxCfg2Spec>;
#[doc = "CSI2_TX_CFG2"]
pub mod csi2_tx_cfg2;
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
