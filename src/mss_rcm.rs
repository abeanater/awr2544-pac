#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    mss_rst_cause_clr: MssRstCauseClr,
    mss_rst_status: MssRstStatus,
    sysrst_by_dbg_rst: SysrstByDbgRst,
    rst_asserdly: RstAsserdly,
    rst2assertdly: Rst2assertdly,
    rst_wficheck: RstWficheck,
    _reserved7: [u8; 0x08],
    mss_qspi_clk_src_sel: MssQspiClkSrcSel,
    mss_rtia_clk_src_sel: MssRtiaClkSrcSel,
    mss_rtib_clk_src_sel: MssRtibClkSrcSel,
    mss_rtic_clk_src_sel: MssRticClkSrcSel,
    mss_wdt_clk_src_sel: MssWdtClkSrcSel,
    _reserved12: [u8; 0x04],
    mss_spib_clk_src_sel: MssSpibClkSrcSel,
    mss_i2c_clk_src_sel: MssI2cClkSrcSel,
    mss_scia_clk_src_sel: MssSciaClkSrcSel,
    mss_scib_clk_src_sel: MssScibClkSrcSel,
    mss_cpts_clk_src_sel: MssCptsClkSrcSel,
    mss_cpsw_clk_src_sel: MssCpswClkSrcSel,
    _reserved18: [u8; 0x08],
    mss_qspi_clk_div_val: MssQspiClkDivVal,
    mss_rtia_clk_div_val: MssRtiaClkDivVal,
    mss_rtib_clk_div_val: MssRtibClkDivVal,
    mss_rtic_clk_div_val: MssRticClkDivVal,
    mss_wdt_clk_div_val: MssWdtClkDivVal,
    _reserved23: [u8; 0x04],
    mss_spib_clk_div_val: MssSpibClkDivVal,
    mss_i2c_clk_div_val: MssI2cClkDivVal,
    mss_scia_clk_div_val: MssSciaClkDivVal,
    mss_scib_clk_div_val: MssScibClkDivVal,
    mss_cpts_clk_div_val: MssCptsClkDivVal,
    mss_cpsw_clk_div_val: MssCpswClkDivVal,
    mss_rgmii_clk_div_val: MssRgmiiClkDivVal,
    mss_mii100_clk_div_val: MssMii100ClkDivVal,
    mss_mii10_clk_div_val: MssMii10ClkDivVal,
    mss_gpadc_clk_div_val: MssGpadcClkDivVal,
    _reserved33: [u8; 0x08],
    mss_qspi_clk_gate: MssQspiClkGate,
    mss_rtia_clk_gate: MssRtiaClkGate,
    mss_rtib_clk_gate: MssRtibClkGate,
    mss_rtic_clk_gate: MssRticClkGate,
    mss_wdt_clk_gate: MssWdtClkGate,
    _reserved38: [u8; 0x04],
    mss_spib_clk_gate: MssSpibClkGate,
    mss_i2c_clk_gate: MssI2cClkGate,
    mss_scia_clk_gate: MssSciaClkGate,
    mss_scib_clk_gate: MssScibClkGate,
    mss_cpts_clk_gate: MssCptsClkGate,
    mss_cpsw_clk_gate: MssCpswClkGate,
    mss_rgmii_clk_gate: MssRgmiiClkGate,
    mss_mii100_clk_gate: MssMii100ClkGate,
    mss_mii10_clk_gate: MssMii10ClkGate,
    mss_gpadc_clk_gate: MssGpadcClkGate,
    _reserved48: [u8; 0x08],
    mss_qspi_clk_status: MssQspiClkStatus,
    mss_rtia_clk_status: MssRtiaClkStatus,
    mss_rtib_clk_status: MssRtibClkStatus,
    mss_rtic_clk_status: MssRticClkStatus,
    mss_wdt_clk_status: MssWdtClkStatus,
    _reserved53: [u8; 0x04],
    mss_spib_clk_status: MssSpibClkStatus,
    mss_i2c_clk_status: MssI2cClkStatus,
    mss_scia_clk_status: MssSciaClkStatus,
    mss_scib_clk_status: MssScibClkStatus,
    mss_cpts_clk_status: MssCptsClkStatus,
    mss_cpsw_clk_status: MssCpswClkStatus,
    mss_rgmii_clk_status: MssRgmiiClkStatus,
    mss_mii100_clk_status: MssMii100ClkStatus,
    mss_mii10_clk_status: MssMii10ClkStatus,
    mss_gpadc_clk_status: MssGpadcClkStatus,
    mss_cr5ss_por_rst_ctrl: MssCr5ssPorRstCtrl,
    mss_cr5ssa_rst_ctrl: MssCr5ssaRstCtrl,
    mss_cr5ssb_rst_ctrl: MssCr5ssbRstCtrl,
    mss_cr5a_rst_ctrl: MssCr5aRstCtrl,
    mss_cr5b_rst_ctrl: MssCr5bRstCtrl,
    mss_vima_rst_ctrl: MssVimaRstCtrl,
    mss_vimb_rst_ctrl: MssVimbRstCtrl,
    mss_crc_rst_ctrl: MssCrcRstCtrl,
    mss_rtia_rst_ctrl: MssRtiaRstCtrl,
    mss_rtib_rst_ctrl: MssRtibRstCtrl,
    mss_rtic_rst_ctrl: MssRticRstCtrl,
    mss_wdt_rst_ctrl: MssWdtRstCtrl,
    mss_esm_rst_ctrl: MssEsmRstCtrl,
    mss_dcca_rst_ctrl: MssDccaRstCtrl,
    mss_dccb_rst_ctrl: MssDccbRstCtrl,
    mss_dccc_rst_ctrl: MssDcccRstCtrl,
    mss_dccd_rst_ctrl: MssDccdRstCtrl,
    mss_gio_rst_ctrl: MssGioRstCtrl,
    _reserved81: [u8; 0x04],
    mss_spib_rst_ctrl: MssSpibRstCtrl,
    mss_qspi_rst_ctrl: MssQspiRstCtrl,
    mss_pwm1_rst_ctrl: MssPwm1RstCtrl,
    mss_pwm2_rst_ctrl: MssPwm2RstCtrl,
    mss_pwm3_rst_ctrl: MssPwm3RstCtrl,
    _reserved86: [u8; 0x08],
    mss_i2c_rst_ctrl: MssI2cRstCtrl,
    mss_scia_rst_ctrl: MssSciaRstCtrl,
    mss_scib_rst_ctrl: MssScibRstCtrl,
    mss_edma_rst_ctrl: MssEdmaRstCtrl,
    mss_infra_rst_ctrl: MssInfraRstCtrl,
    mss_cpsw_rst_ctrl: MssCpswRstCtrl,
    mss_gpadc_rst_ctrl: MssGpadcRstCtrl,
    mss_dmm_rst_ctrl: MssDmmRstCtrl,
    r5_corea_gate: R5CoreaGate,
    r5_coreb_gate: R5CorebGate,
    mss_l2_banka_pd_ctrl: MssL2BankaPdCtrl,
    mss_l2_bankb_pd_ctrl: MssL2BankbPdCtrl,
    mss_l2_banka_pd_status: MssL2BankaPdStatus,
    mss_l2_bankb_pd_status: MssL2BankbPdStatus,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    mss_cr5f_clk_src_sel_ctrl: MssCr5fClkSrcSelCtrl,
    mss_cpsw_mii_clk_src_sel: MssCpswMiiClkSrcSel,
    mss_cpsw_mii_clk_status: MssCpswMiiClkStatus,
    mss_l2_bankc_pd_ctrl: MssL2BankcPdCtrl,
    mss_l2_bankc_pd_status: MssL2BankcPdStatus,
    mss_ip_clk_cfg: MssIpClkCfg,
    _reserved110: [u8; 0x020c],
    hsm_rtia_clk_src_sel: HsmRtiaClkSrcSel,
    hsm_wdt_clk_src_sel: HsmWdtClkSrcSel,
    hsm_rtc_clk_src_sel: HsmRtcClkSrcSel,
    hsm_dmta_clk_src_sel: HsmDmtaClkSrcSel,
    hsm_dmtb_clk_src_sel: HsmDmtbClkSrcSel,
    hsm_rti_clk_div_val: HsmRtiClkDivVal,
    hsm_wdt_clk_div_val: HsmWdtClkDivVal,
    hsm_rtc_clk_div_val: HsmRtcClkDivVal,
    hsm_dmta_clk_div_val: HsmDmtaClkDivVal,
    hsm_dmtb_clk_div_val: HsmDmtbClkDivVal,
    hsm_rti_clk_gate: HsmRtiClkGate,
    hsm_wdt_clk_gate: HsmWdtClkGate,
    hsm_rtc_clk_gate: HsmRtcClkGate,
    hsm_dmta_clk_gate: HsmDmtaClkGate,
    hsm_dmtb_clk_gate: HsmDmtbClkGate,
    hsm_rti_clk_status: HsmRtiClkStatus,
    hsm_wdt_clk_status: HsmWdtClkStatus,
    hsm_rtc_clk_status: HsmRtcClkStatus,
    hsm_dmta_clk_status: HsmDmtaClkStatus,
    hsm_dmtb_clk_status: HsmDmtbClkStatus,
    _reserved130: [u8; 0x0b80],
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
    _reserved140: [u8; 0x10],
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
    #[doc = "0x04 - MSS_RST_CAUSE_CLR"]
    #[inline(always)]
    pub const fn mss_rst_cause_clr(&self) -> &MssRstCauseClr {
        &self.mss_rst_cause_clr
    }
    #[doc = "0x08 - MSS_RST_STATUS"]
    #[inline(always)]
    pub const fn mss_rst_status(&self) -> &MssRstStatus {
        &self.mss_rst_status
    }
    #[doc = "0x0c - SYSRST_BY_DBG_RST"]
    #[inline(always)]
    pub const fn sysrst_by_dbg_rst(&self) -> &SysrstByDbgRst {
        &self.sysrst_by_dbg_rst
    }
    #[doc = "0x10 - RST_ASSERDLY"]
    #[inline(always)]
    pub const fn rst_asserdly(&self) -> &RstAsserdly {
        &self.rst_asserdly
    }
    #[doc = "0x14 - RST2ASSERTDLY"]
    #[inline(always)]
    pub const fn rst2assertdly(&self) -> &Rst2assertdly {
        &self.rst2assertdly
    }
    #[doc = "0x18 - RST_WFICHECK"]
    #[inline(always)]
    pub const fn rst_wficheck(&self) -> &RstWficheck {
        &self.rst_wficheck
    }
    #[doc = "0x24 - MSS_QSPI_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_qspi_clk_src_sel(&self) -> &MssQspiClkSrcSel {
        &self.mss_qspi_clk_src_sel
    }
    #[doc = "0x28 - MSS_RTIA_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_rtia_clk_src_sel(&self) -> &MssRtiaClkSrcSel {
        &self.mss_rtia_clk_src_sel
    }
    #[doc = "0x2c - MSS_RTIB_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_rtib_clk_src_sel(&self) -> &MssRtibClkSrcSel {
        &self.mss_rtib_clk_src_sel
    }
    #[doc = "0x30 - MSS_RTIC_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_rtic_clk_src_sel(&self) -> &MssRticClkSrcSel {
        &self.mss_rtic_clk_src_sel
    }
    #[doc = "0x34 - MSS_WDT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_wdt_clk_src_sel(&self) -> &MssWdtClkSrcSel {
        &self.mss_wdt_clk_src_sel
    }
    #[doc = "0x3c - MSS_SPIB_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_spib_clk_src_sel(&self) -> &MssSpibClkSrcSel {
        &self.mss_spib_clk_src_sel
    }
    #[doc = "0x40 - MSS_I2C_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_i2c_clk_src_sel(&self) -> &MssI2cClkSrcSel {
        &self.mss_i2c_clk_src_sel
    }
    #[doc = "0x44 - MSS_SCIA_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_scia_clk_src_sel(&self) -> &MssSciaClkSrcSel {
        &self.mss_scia_clk_src_sel
    }
    #[doc = "0x48 - MSS_SCIB_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_scib_clk_src_sel(&self) -> &MssScibClkSrcSel {
        &self.mss_scib_clk_src_sel
    }
    #[doc = "0x4c - MSS_CPTS_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_cpts_clk_src_sel(&self) -> &MssCptsClkSrcSel {
        &self.mss_cpts_clk_src_sel
    }
    #[doc = "0x50 - MSS_CPSW_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_cpsw_clk_src_sel(&self) -> &MssCpswClkSrcSel {
        &self.mss_cpsw_clk_src_sel
    }
    #[doc = "0x5c - MSS_QSPI_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_qspi_clk_div_val(&self) -> &MssQspiClkDivVal {
        &self.mss_qspi_clk_div_val
    }
    #[doc = "0x60 - MSS_RTIA_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_rtia_clk_div_val(&self) -> &MssRtiaClkDivVal {
        &self.mss_rtia_clk_div_val
    }
    #[doc = "0x64 - MSS_RTIB_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_rtib_clk_div_val(&self) -> &MssRtibClkDivVal {
        &self.mss_rtib_clk_div_val
    }
    #[doc = "0x68 - MSS_RTIC_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_rtic_clk_div_val(&self) -> &MssRticClkDivVal {
        &self.mss_rtic_clk_div_val
    }
    #[doc = "0x6c - MSS_WDT_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_wdt_clk_div_val(&self) -> &MssWdtClkDivVal {
        &self.mss_wdt_clk_div_val
    }
    #[doc = "0x74 - MSS_SPIB_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_spib_clk_div_val(&self) -> &MssSpibClkDivVal {
        &self.mss_spib_clk_div_val
    }
    #[doc = "0x78 - MSS_I2C_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_i2c_clk_div_val(&self) -> &MssI2cClkDivVal {
        &self.mss_i2c_clk_div_val
    }
    #[doc = "0x7c - MSS_SCIA_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_scia_clk_div_val(&self) -> &MssSciaClkDivVal {
        &self.mss_scia_clk_div_val
    }
    #[doc = "0x80 - MSS_SCIB_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_scib_clk_div_val(&self) -> &MssScibClkDivVal {
        &self.mss_scib_clk_div_val
    }
    #[doc = "0x84 - MSS_CPTS_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_cpts_clk_div_val(&self) -> &MssCptsClkDivVal {
        &self.mss_cpts_clk_div_val
    }
    #[doc = "0x88 - MSS_CPSW_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_cpsw_clk_div_val(&self) -> &MssCpswClkDivVal {
        &self.mss_cpsw_clk_div_val
    }
    #[doc = "0x8c - MSS_RGMII_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_rgmii_clk_div_val(&self) -> &MssRgmiiClkDivVal {
        &self.mss_rgmii_clk_div_val
    }
    #[doc = "0x90 - MSS_MII100_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_mii100_clk_div_val(&self) -> &MssMii100ClkDivVal {
        &self.mss_mii100_clk_div_val
    }
    #[doc = "0x94 - MSS_MII10_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_mii10_clk_div_val(&self) -> &MssMii10ClkDivVal {
        &self.mss_mii10_clk_div_val
    }
    #[doc = "0x98 - MSS_GPADC_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_gpadc_clk_div_val(&self) -> &MssGpadcClkDivVal {
        &self.mss_gpadc_clk_div_val
    }
    #[doc = "0xa4 - MSS_QSPI_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_qspi_clk_gate(&self) -> &MssQspiClkGate {
        &self.mss_qspi_clk_gate
    }
    #[doc = "0xa8 - MSS_RTIA_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_rtia_clk_gate(&self) -> &MssRtiaClkGate {
        &self.mss_rtia_clk_gate
    }
    #[doc = "0xac - MSS_RTIB_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_rtib_clk_gate(&self) -> &MssRtibClkGate {
        &self.mss_rtib_clk_gate
    }
    #[doc = "0xb0 - MSS_RTIC_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_rtic_clk_gate(&self) -> &MssRticClkGate {
        &self.mss_rtic_clk_gate
    }
    #[doc = "0xb4 - MSS_WDT_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_wdt_clk_gate(&self) -> &MssWdtClkGate {
        &self.mss_wdt_clk_gate
    }
    #[doc = "0xbc - MSS_SPIB_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_spib_clk_gate(&self) -> &MssSpibClkGate {
        &self.mss_spib_clk_gate
    }
    #[doc = "0xc0 - MSS_I2C_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_i2c_clk_gate(&self) -> &MssI2cClkGate {
        &self.mss_i2c_clk_gate
    }
    #[doc = "0xc4 - MSS_SCIA_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_scia_clk_gate(&self) -> &MssSciaClkGate {
        &self.mss_scia_clk_gate
    }
    #[doc = "0xc8 - MSS_SCIB_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_scib_clk_gate(&self) -> &MssScibClkGate {
        &self.mss_scib_clk_gate
    }
    #[doc = "0xcc - MSS_CPTS_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_cpts_clk_gate(&self) -> &MssCptsClkGate {
        &self.mss_cpts_clk_gate
    }
    #[doc = "0xd0 - MSS_CPSW_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_cpsw_clk_gate(&self) -> &MssCpswClkGate {
        &self.mss_cpsw_clk_gate
    }
    #[doc = "0xd4 - MSS_RGMII_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_rgmii_clk_gate(&self) -> &MssRgmiiClkGate {
        &self.mss_rgmii_clk_gate
    }
    #[doc = "0xd8 - MSS_MII100_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_mii100_clk_gate(&self) -> &MssMii100ClkGate {
        &self.mss_mii100_clk_gate
    }
    #[doc = "0xdc - MSS_MII10_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_mii10_clk_gate(&self) -> &MssMii10ClkGate {
        &self.mss_mii10_clk_gate
    }
    #[doc = "0xe0 - MSS_GPADC_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_gpadc_clk_gate(&self) -> &MssGpadcClkGate {
        &self.mss_gpadc_clk_gate
    }
    #[doc = "0xec - MSS_QSPI_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_qspi_clk_status(&self) -> &MssQspiClkStatus {
        &self.mss_qspi_clk_status
    }
    #[doc = "0xf0 - MSS_RTIA_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_rtia_clk_status(&self) -> &MssRtiaClkStatus {
        &self.mss_rtia_clk_status
    }
    #[doc = "0xf4 - MSS_RTIB_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_rtib_clk_status(&self) -> &MssRtibClkStatus {
        &self.mss_rtib_clk_status
    }
    #[doc = "0xf8 - MSS_RTIC_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_rtic_clk_status(&self) -> &MssRticClkStatus {
        &self.mss_rtic_clk_status
    }
    #[doc = "0xfc - MSS_WDT_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_wdt_clk_status(&self) -> &MssWdtClkStatus {
        &self.mss_wdt_clk_status
    }
    #[doc = "0x104 - MSS_SPIB_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_spib_clk_status(&self) -> &MssSpibClkStatus {
        &self.mss_spib_clk_status
    }
    #[doc = "0x108 - MSS_I2C_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_i2c_clk_status(&self) -> &MssI2cClkStatus {
        &self.mss_i2c_clk_status
    }
    #[doc = "0x10c - MSS_SCIA_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_scia_clk_status(&self) -> &MssSciaClkStatus {
        &self.mss_scia_clk_status
    }
    #[doc = "0x110 - MSS_SCIB_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_scib_clk_status(&self) -> &MssScibClkStatus {
        &self.mss_scib_clk_status
    }
    #[doc = "0x114 - MSS_CPTS_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_cpts_clk_status(&self) -> &MssCptsClkStatus {
        &self.mss_cpts_clk_status
    }
    #[doc = "0x118 - MSS_CPSW_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_cpsw_clk_status(&self) -> &MssCpswClkStatus {
        &self.mss_cpsw_clk_status
    }
    #[doc = "0x11c - MSS_RGMII_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_rgmii_clk_status(&self) -> &MssRgmiiClkStatus {
        &self.mss_rgmii_clk_status
    }
    #[doc = "0x120 - MSS_MII100_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_mii100_clk_status(&self) -> &MssMii100ClkStatus {
        &self.mss_mii100_clk_status
    }
    #[doc = "0x124 - MSS_MII10_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_mii10_clk_status(&self) -> &MssMii10ClkStatus {
        &self.mss_mii10_clk_status
    }
    #[doc = "0x128 - MSS_GPADC_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_gpadc_clk_status(&self) -> &MssGpadcClkStatus {
        &self.mss_gpadc_clk_status
    }
    #[doc = "0x12c - MSS_CR5SS_POR_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5ss_por_rst_ctrl(&self) -> &MssCr5ssPorRstCtrl {
        &self.mss_cr5ss_por_rst_ctrl
    }
    #[doc = "0x130 - MSS_CR5SSA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5ssa_rst_ctrl(&self) -> &MssCr5ssaRstCtrl {
        &self.mss_cr5ssa_rst_ctrl
    }
    #[doc = "0x134 - MSS_CR5SSB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5ssb_rst_ctrl(&self) -> &MssCr5ssbRstCtrl {
        &self.mss_cr5ssb_rst_ctrl
    }
    #[doc = "0x138 - MSS_CR5A_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5a_rst_ctrl(&self) -> &MssCr5aRstCtrl {
        &self.mss_cr5a_rst_ctrl
    }
    #[doc = "0x13c - MSS_CR5B_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5b_rst_ctrl(&self) -> &MssCr5bRstCtrl {
        &self.mss_cr5b_rst_ctrl
    }
    #[doc = "0x140 - MSS_VIMA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_vima_rst_ctrl(&self) -> &MssVimaRstCtrl {
        &self.mss_vima_rst_ctrl
    }
    #[doc = "0x144 - MSS_VIMB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_vimb_rst_ctrl(&self) -> &MssVimbRstCtrl {
        &self.mss_vimb_rst_ctrl
    }
    #[doc = "0x148 - MSS_CRC_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_crc_rst_ctrl(&self) -> &MssCrcRstCtrl {
        &self.mss_crc_rst_ctrl
    }
    #[doc = "0x14c - MSS_RTIA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_rtia_rst_ctrl(&self) -> &MssRtiaRstCtrl {
        &self.mss_rtia_rst_ctrl
    }
    #[doc = "0x150 - MSS_RTIB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_rtib_rst_ctrl(&self) -> &MssRtibRstCtrl {
        &self.mss_rtib_rst_ctrl
    }
    #[doc = "0x154 - MSS_RTIC_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_rtic_rst_ctrl(&self) -> &MssRticRstCtrl {
        &self.mss_rtic_rst_ctrl
    }
    #[doc = "0x158 - MSS_WDT_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_wdt_rst_ctrl(&self) -> &MssWdtRstCtrl {
        &self.mss_wdt_rst_ctrl
    }
    #[doc = "0x15c - MSS_ESM_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_esm_rst_ctrl(&self) -> &MssEsmRstCtrl {
        &self.mss_esm_rst_ctrl
    }
    #[doc = "0x160 - MSS_DCCA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_dcca_rst_ctrl(&self) -> &MssDccaRstCtrl {
        &self.mss_dcca_rst_ctrl
    }
    #[doc = "0x164 - MSS_DCCB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_dccb_rst_ctrl(&self) -> &MssDccbRstCtrl {
        &self.mss_dccb_rst_ctrl
    }
    #[doc = "0x168 - MSS_DCCC_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_dccc_rst_ctrl(&self) -> &MssDcccRstCtrl {
        &self.mss_dccc_rst_ctrl
    }
    #[doc = "0x16c - MSS_DCCD_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_dccd_rst_ctrl(&self) -> &MssDccdRstCtrl {
        &self.mss_dccd_rst_ctrl
    }
    #[doc = "0x170 - MSS_GIO_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_gio_rst_ctrl(&self) -> &MssGioRstCtrl {
        &self.mss_gio_rst_ctrl
    }
    #[doc = "0x178 - MSS_SPIB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_spib_rst_ctrl(&self) -> &MssSpibRstCtrl {
        &self.mss_spib_rst_ctrl
    }
    #[doc = "0x17c - MSS_QSPI_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_qspi_rst_ctrl(&self) -> &MssQspiRstCtrl {
        &self.mss_qspi_rst_ctrl
    }
    #[doc = "0x180 - MSS_PWM1_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_pwm1_rst_ctrl(&self) -> &MssPwm1RstCtrl {
        &self.mss_pwm1_rst_ctrl
    }
    #[doc = "0x184 - MSS_PWM2_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_pwm2_rst_ctrl(&self) -> &MssPwm2RstCtrl {
        &self.mss_pwm2_rst_ctrl
    }
    #[doc = "0x188 - MSS_PWM3_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_pwm3_rst_ctrl(&self) -> &MssPwm3RstCtrl {
        &self.mss_pwm3_rst_ctrl
    }
    #[doc = "0x194 - MSS_I2C_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_i2c_rst_ctrl(&self) -> &MssI2cRstCtrl {
        &self.mss_i2c_rst_ctrl
    }
    #[doc = "0x198 - MSS_SCIA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_scia_rst_ctrl(&self) -> &MssSciaRstCtrl {
        &self.mss_scia_rst_ctrl
    }
    #[doc = "0x19c - MSS_SCIB_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_scib_rst_ctrl(&self) -> &MssScibRstCtrl {
        &self.mss_scib_rst_ctrl
    }
    #[doc = "0x1a0 - MSS_EDMA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_edma_rst_ctrl(&self) -> &MssEdmaRstCtrl {
        &self.mss_edma_rst_ctrl
    }
    #[doc = "0x1a4 - MSS_INFRA_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_infra_rst_ctrl(&self) -> &MssInfraRstCtrl {
        &self.mss_infra_rst_ctrl
    }
    #[doc = "0x1a8 - MSS_CPSW_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_cpsw_rst_ctrl(&self) -> &MssCpswRstCtrl {
        &self.mss_cpsw_rst_ctrl
    }
    #[doc = "0x1ac - MSS_GPADC_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_gpadc_rst_ctrl(&self) -> &MssGpadcRstCtrl {
        &self.mss_gpadc_rst_ctrl
    }
    #[doc = "0x1b0 - MSS_DMM_RST_CTRL"]
    #[inline(always)]
    pub const fn mss_dmm_rst_ctrl(&self) -> &MssDmmRstCtrl {
        &self.mss_dmm_rst_ctrl
    }
    #[doc = "0x1b4 - R5_COREA_GATE"]
    #[inline(always)]
    pub const fn r5_corea_gate(&self) -> &R5CoreaGate {
        &self.r5_corea_gate
    }
    #[doc = "0x1b8 - R5_COREB_GATE"]
    #[inline(always)]
    pub const fn r5_coreb_gate(&self) -> &R5CorebGate {
        &self.r5_coreb_gate
    }
    #[doc = "0x1bc - MSS_L2_BANKA_PD_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_banka_pd_ctrl(&self) -> &MssL2BankaPdCtrl {
        &self.mss_l2_banka_pd_ctrl
    }
    #[doc = "0x1c0 - MSS_L2_BANKB_PD_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_bankb_pd_ctrl(&self) -> &MssL2BankbPdCtrl {
        &self.mss_l2_bankb_pd_ctrl
    }
    #[doc = "0x1c4 - MSS_L2_BANKA_PD_STATUS"]
    #[inline(always)]
    pub const fn mss_l2_banka_pd_status(&self) -> &MssL2BankaPdStatus {
        &self.mss_l2_banka_pd_status
    }
    #[doc = "0x1c8 - MSS_L2_BANKB_PD_STATUS"]
    #[inline(always)]
    pub const fn mss_l2_bankb_pd_status(&self) -> &MssL2BankbPdStatus {
        &self.mss_l2_bankb_pd_status
    }
    #[doc = "0x1cc - HW_REG0"]
    #[inline(always)]
    pub const fn hw_reg0(&self) -> &HwReg0 {
        &self.hw_reg0
    }
    #[doc = "0x1d0 - HW_REG1"]
    #[inline(always)]
    pub const fn hw_reg1(&self) -> &HwReg1 {
        &self.hw_reg1
    }
    #[doc = "0x1d4 - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x1d8 - HW_REG3"]
    #[inline(always)]
    pub const fn hw_reg3(&self) -> &HwReg3 {
        &self.hw_reg3
    }
    #[doc = "0x1dc - MSS_CR5F_CLK_SRC_SEL_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5f_clk_src_sel_ctrl(&self) -> &MssCr5fClkSrcSelCtrl {
        &self.mss_cr5f_clk_src_sel_ctrl
    }
    #[doc = "0x1e0 - MSS_CPSW_MII_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_cpsw_mii_clk_src_sel(&self) -> &MssCpswMiiClkSrcSel {
        &self.mss_cpsw_mii_clk_src_sel
    }
    #[doc = "0x1e4 - MSS_CPSW_MII_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_cpsw_mii_clk_status(&self) -> &MssCpswMiiClkStatus {
        &self.mss_cpsw_mii_clk_status
    }
    #[doc = "0x1e8 - MSS_L2_BANKC_PD_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_bankc_pd_ctrl(&self) -> &MssL2BankcPdCtrl {
        &self.mss_l2_bankc_pd_ctrl
    }
    #[doc = "0x1ec - MSS_L2_BANKC_PD_STATUS"]
    #[inline(always)]
    pub const fn mss_l2_bankc_pd_status(&self) -> &MssL2BankcPdStatus {
        &self.mss_l2_bankc_pd_status
    }
    #[doc = "0x1f0 - MSS_IP_CLK_CFG"]
    #[inline(always)]
    pub const fn mss_ip_clk_cfg(&self) -> &MssIpClkCfg {
        &self.mss_ip_clk_cfg
    }
    #[doc = "0x400 - HSM_RTIA_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsm_rtia_clk_src_sel(&self) -> &HsmRtiaClkSrcSel {
        &self.hsm_rtia_clk_src_sel
    }
    #[doc = "0x404 - HSM_WDT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsm_wdt_clk_src_sel(&self) -> &HsmWdtClkSrcSel {
        &self.hsm_wdt_clk_src_sel
    }
    #[doc = "0x408 - HSM_RTC_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsm_rtc_clk_src_sel(&self) -> &HsmRtcClkSrcSel {
        &self.hsm_rtc_clk_src_sel
    }
    #[doc = "0x40c - HSM_DMTA_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsm_dmta_clk_src_sel(&self) -> &HsmDmtaClkSrcSel {
        &self.hsm_dmta_clk_src_sel
    }
    #[doc = "0x410 - HSM_DMTB_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsm_dmtb_clk_src_sel(&self) -> &HsmDmtbClkSrcSel {
        &self.hsm_dmtb_clk_src_sel
    }
    #[doc = "0x414 - HSM_RTI_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn hsm_rti_clk_div_val(&self) -> &HsmRtiClkDivVal {
        &self.hsm_rti_clk_div_val
    }
    #[doc = "0x418 - HSM_WDT_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn hsm_wdt_clk_div_val(&self) -> &HsmWdtClkDivVal {
        &self.hsm_wdt_clk_div_val
    }
    #[doc = "0x41c - HSM_RTC_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn hsm_rtc_clk_div_val(&self) -> &HsmRtcClkDivVal {
        &self.hsm_rtc_clk_div_val
    }
    #[doc = "0x420 - HSM_DMTA_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn hsm_dmta_clk_div_val(&self) -> &HsmDmtaClkDivVal {
        &self.hsm_dmta_clk_div_val
    }
    #[doc = "0x424 - HSM_DMTB_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn hsm_dmtb_clk_div_val(&self) -> &HsmDmtbClkDivVal {
        &self.hsm_dmtb_clk_div_val
    }
    #[doc = "0x428 - HSM_RTI_CLK_GATE"]
    #[inline(always)]
    pub const fn hsm_rti_clk_gate(&self) -> &HsmRtiClkGate {
        &self.hsm_rti_clk_gate
    }
    #[doc = "0x42c - HSM_WDT_CLK_GATE"]
    #[inline(always)]
    pub const fn hsm_wdt_clk_gate(&self) -> &HsmWdtClkGate {
        &self.hsm_wdt_clk_gate
    }
    #[doc = "0x430 - HSM_RTC_CLK_GATE"]
    #[inline(always)]
    pub const fn hsm_rtc_clk_gate(&self) -> &HsmRtcClkGate {
        &self.hsm_rtc_clk_gate
    }
    #[doc = "0x434 - HSM_DMTA_CLK_GATE"]
    #[inline(always)]
    pub const fn hsm_dmta_clk_gate(&self) -> &HsmDmtaClkGate {
        &self.hsm_dmta_clk_gate
    }
    #[doc = "0x438 - HSM_DMTB_CLK_GATE"]
    #[inline(always)]
    pub const fn hsm_dmtb_clk_gate(&self) -> &HsmDmtbClkGate {
        &self.hsm_dmtb_clk_gate
    }
    #[doc = "0x43c - HSM_RTI_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsm_rti_clk_status(&self) -> &HsmRtiClkStatus {
        &self.hsm_rti_clk_status
    }
    #[doc = "0x440 - HSM_WDT_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsm_wdt_clk_status(&self) -> &HsmWdtClkStatus {
        &self.hsm_wdt_clk_status
    }
    #[doc = "0x444 - HSM_RTC_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsm_rtc_clk_status(&self) -> &HsmRtcClkStatus {
        &self.hsm_rtc_clk_status
    }
    #[doc = "0x448 - HSM_DMTA_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsm_dmta_clk_status(&self) -> &HsmDmtaClkStatus {
        &self.hsm_dmta_clk_status
    }
    #[doc = "0x44c - HSM_DMTB_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsm_dmtb_clk_status(&self) -> &HsmDmtbClkStatus {
        &self.hsm_dmtb_clk_status
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
#[doc = "MSS_RST_CAUSE_CLR (rw) register accessor: MSS_RST_CAUSE_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rst_cause_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rst_cause_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rst_cause_clr`]
module"]
#[doc(alias = "MSS_RST_CAUSE_CLR")]
pub type MssRstCauseClr = crate::Reg<mss_rst_cause_clr::MssRstCauseClrSpec>;
#[doc = "MSS_RST_CAUSE_CLR"]
pub mod mss_rst_cause_clr;
#[doc = "MSS_RST_STATUS (rw) register accessor: MSS_RST_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rst_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rst_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rst_status`]
module"]
#[doc(alias = "MSS_RST_STATUS")]
pub type MssRstStatus = crate::Reg<mss_rst_status::MssRstStatusSpec>;
#[doc = "MSS_RST_STATUS"]
pub mod mss_rst_status;
#[doc = "SYSRST_BY_DBG_RST (rw) register accessor: SYSRST_BY_DBG_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrst_by_dbg_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrst_by_dbg_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrst_by_dbg_rst`]
module"]
#[doc(alias = "SYSRST_BY_DBG_RST")]
pub type SysrstByDbgRst = crate::Reg<sysrst_by_dbg_rst::SysrstByDbgRstSpec>;
#[doc = "SYSRST_BY_DBG_RST"]
pub mod sysrst_by_dbg_rst;
#[doc = "RST_ASSERDLY (rw) register accessor: RST_ASSERDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_asserdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_asserdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_asserdly`]
module"]
#[doc(alias = "RST_ASSERDLY")]
pub type RstAsserdly = crate::Reg<rst_asserdly::RstAsserdlySpec>;
#[doc = "RST_ASSERDLY"]
pub mod rst_asserdly;
#[doc = "RST2ASSERTDLY (rw) register accessor: RST2ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst2assertdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst2assertdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst2assertdly`]
module"]
#[doc(alias = "RST2ASSERTDLY")]
pub type Rst2assertdly = crate::Reg<rst2assertdly::Rst2assertdlySpec>;
#[doc = "RST2ASSERTDLY"]
pub mod rst2assertdly;
#[doc = "RST_WFICHECK (rw) register accessor: RST_WFICHECK\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_wficheck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_wficheck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_wficheck`]
module"]
#[doc(alias = "RST_WFICHECK")]
pub type RstWficheck = crate::Reg<rst_wficheck::RstWficheckSpec>;
#[doc = "RST_WFICHECK"]
pub mod rst_wficheck;
#[doc = "MSS_QSPI_CLK_SRC_SEL (rw) register accessor: MSS_QSPI_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_clk_src_sel`]
module"]
#[doc(alias = "MSS_QSPI_CLK_SRC_SEL")]
pub type MssQspiClkSrcSel = crate::Reg<mss_qspi_clk_src_sel::MssQspiClkSrcSelSpec>;
#[doc = "MSS_QSPI_CLK_SRC_SEL"]
pub mod mss_qspi_clk_src_sel;
#[doc = "MSS_RTIA_CLK_SRC_SEL (rw) register accessor: MSS_RTIA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtia_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtia_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtia_clk_src_sel`]
module"]
#[doc(alias = "MSS_RTIA_CLK_SRC_SEL")]
pub type MssRtiaClkSrcSel = crate::Reg<mss_rtia_clk_src_sel::MssRtiaClkSrcSelSpec>;
#[doc = "MSS_RTIA_CLK_SRC_SEL"]
pub mod mss_rtia_clk_src_sel;
#[doc = "MSS_RTIB_CLK_SRC_SEL (rw) register accessor: MSS_RTIB_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtib_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtib_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtib_clk_src_sel`]
module"]
#[doc(alias = "MSS_RTIB_CLK_SRC_SEL")]
pub type MssRtibClkSrcSel = crate::Reg<mss_rtib_clk_src_sel::MssRtibClkSrcSelSpec>;
#[doc = "MSS_RTIB_CLK_SRC_SEL"]
pub mod mss_rtib_clk_src_sel;
#[doc = "MSS_RTIC_CLK_SRC_SEL (rw) register accessor: MSS_RTIC_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtic_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtic_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtic_clk_src_sel`]
module"]
#[doc(alias = "MSS_RTIC_CLK_SRC_SEL")]
pub type MssRticClkSrcSel = crate::Reg<mss_rtic_clk_src_sel::MssRticClkSrcSelSpec>;
#[doc = "MSS_RTIC_CLK_SRC_SEL"]
pub mod mss_rtic_clk_src_sel;
#[doc = "MSS_WDT_CLK_SRC_SEL (rw) register accessor: MSS_WDT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_wdt_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_wdt_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_wdt_clk_src_sel`]
module"]
#[doc(alias = "MSS_WDT_CLK_SRC_SEL")]
pub type MssWdtClkSrcSel = crate::Reg<mss_wdt_clk_src_sel::MssWdtClkSrcSelSpec>;
#[doc = "MSS_WDT_CLK_SRC_SEL"]
pub mod mss_wdt_clk_src_sel;
#[doc = "MSS_SPIB_CLK_SRC_SEL (rw) register accessor: MSS_SPIB_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_clk_src_sel`]
module"]
#[doc(alias = "MSS_SPIB_CLK_SRC_SEL")]
pub type MssSpibClkSrcSel = crate::Reg<mss_spib_clk_src_sel::MssSpibClkSrcSelSpec>;
#[doc = "MSS_SPIB_CLK_SRC_SEL"]
pub mod mss_spib_clk_src_sel;
#[doc = "MSS_I2C_CLK_SRC_SEL (rw) register accessor: MSS_I2C_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_i2c_clk_src_sel`]
module"]
#[doc(alias = "MSS_I2C_CLK_SRC_SEL")]
pub type MssI2cClkSrcSel = crate::Reg<mss_i2c_clk_src_sel::MssI2cClkSrcSelSpec>;
#[doc = "MSS_I2C_CLK_SRC_SEL"]
pub mod mss_i2c_clk_src_sel;
#[doc = "MSS_SCIA_CLK_SRC_SEL (rw) register accessor: MSS_SCIA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scia_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scia_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scia_clk_src_sel`]
module"]
#[doc(alias = "MSS_SCIA_CLK_SRC_SEL")]
pub type MssSciaClkSrcSel = crate::Reg<mss_scia_clk_src_sel::MssSciaClkSrcSelSpec>;
#[doc = "MSS_SCIA_CLK_SRC_SEL"]
pub mod mss_scia_clk_src_sel;
#[doc = "MSS_SCIB_CLK_SRC_SEL (rw) register accessor: MSS_SCIB_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scib_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scib_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scib_clk_src_sel`]
module"]
#[doc(alias = "MSS_SCIB_CLK_SRC_SEL")]
pub type MssScibClkSrcSel = crate::Reg<mss_scib_clk_src_sel::MssScibClkSrcSelSpec>;
#[doc = "MSS_SCIB_CLK_SRC_SEL"]
pub mod mss_scib_clk_src_sel;
#[doc = "MSS_CPTS_CLK_SRC_SEL (rw) register accessor: MSS_CPTS_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpts_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpts_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpts_clk_src_sel`]
module"]
#[doc(alias = "MSS_CPTS_CLK_SRC_SEL")]
pub type MssCptsClkSrcSel = crate::Reg<mss_cpts_clk_src_sel::MssCptsClkSrcSelSpec>;
#[doc = "MSS_CPTS_CLK_SRC_SEL"]
pub mod mss_cpts_clk_src_sel;
#[doc = "MSS_CPSW_CLK_SRC_SEL (rw) register accessor: MSS_CPSW_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_clk_src_sel`]
module"]
#[doc(alias = "MSS_CPSW_CLK_SRC_SEL")]
pub type MssCpswClkSrcSel = crate::Reg<mss_cpsw_clk_src_sel::MssCpswClkSrcSelSpec>;
#[doc = "MSS_CPSW_CLK_SRC_SEL"]
pub mod mss_cpsw_clk_src_sel;
#[doc = "MSS_QSPI_CLK_DIV_VAL (rw) register accessor: MSS_QSPI_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_clk_div_val`]
module"]
#[doc(alias = "MSS_QSPI_CLK_DIV_VAL")]
pub type MssQspiClkDivVal = crate::Reg<mss_qspi_clk_div_val::MssQspiClkDivValSpec>;
#[doc = "MSS_QSPI_CLK_DIV_VAL"]
pub mod mss_qspi_clk_div_val;
#[doc = "MSS_RTIA_CLK_DIV_VAL (rw) register accessor: MSS_RTIA_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtia_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtia_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtia_clk_div_val`]
module"]
#[doc(alias = "MSS_RTIA_CLK_DIV_VAL")]
pub type MssRtiaClkDivVal = crate::Reg<mss_rtia_clk_div_val::MssRtiaClkDivValSpec>;
#[doc = "MSS_RTIA_CLK_DIV_VAL"]
pub mod mss_rtia_clk_div_val;
#[doc = "MSS_RTIB_CLK_DIV_VAL (rw) register accessor: MSS_RTIB_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtib_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtib_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtib_clk_div_val`]
module"]
#[doc(alias = "MSS_RTIB_CLK_DIV_VAL")]
pub type MssRtibClkDivVal = crate::Reg<mss_rtib_clk_div_val::MssRtibClkDivValSpec>;
#[doc = "MSS_RTIB_CLK_DIV_VAL"]
pub mod mss_rtib_clk_div_val;
#[doc = "MSS_RTIC_CLK_DIV_VAL (rw) register accessor: MSS_RTIC_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtic_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtic_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtic_clk_div_val`]
module"]
#[doc(alias = "MSS_RTIC_CLK_DIV_VAL")]
pub type MssRticClkDivVal = crate::Reg<mss_rtic_clk_div_val::MssRticClkDivValSpec>;
#[doc = "MSS_RTIC_CLK_DIV_VAL"]
pub mod mss_rtic_clk_div_val;
#[doc = "MSS_WDT_CLK_DIV_VAL (rw) register accessor: MSS_WDT_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_wdt_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_wdt_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_wdt_clk_div_val`]
module"]
#[doc(alias = "MSS_WDT_CLK_DIV_VAL")]
pub type MssWdtClkDivVal = crate::Reg<mss_wdt_clk_div_val::MssWdtClkDivValSpec>;
#[doc = "MSS_WDT_CLK_DIV_VAL"]
pub mod mss_wdt_clk_div_val;
#[doc = "MSS_SPIB_CLK_DIV_VAL (rw) register accessor: MSS_SPIB_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_clk_div_val`]
module"]
#[doc(alias = "MSS_SPIB_CLK_DIV_VAL")]
pub type MssSpibClkDivVal = crate::Reg<mss_spib_clk_div_val::MssSpibClkDivValSpec>;
#[doc = "MSS_SPIB_CLK_DIV_VAL"]
pub mod mss_spib_clk_div_val;
#[doc = "MSS_I2C_CLK_DIV_VAL (rw) register accessor: MSS_I2C_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_i2c_clk_div_val`]
module"]
#[doc(alias = "MSS_I2C_CLK_DIV_VAL")]
pub type MssI2cClkDivVal = crate::Reg<mss_i2c_clk_div_val::MssI2cClkDivValSpec>;
#[doc = "MSS_I2C_CLK_DIV_VAL"]
pub mod mss_i2c_clk_div_val;
#[doc = "MSS_SCIA_CLK_DIV_VAL (rw) register accessor: MSS_SCIA_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scia_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scia_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scia_clk_div_val`]
module"]
#[doc(alias = "MSS_SCIA_CLK_DIV_VAL")]
pub type MssSciaClkDivVal = crate::Reg<mss_scia_clk_div_val::MssSciaClkDivValSpec>;
#[doc = "MSS_SCIA_CLK_DIV_VAL"]
pub mod mss_scia_clk_div_val;
#[doc = "MSS_SCIB_CLK_DIV_VAL (rw) register accessor: MSS_SCIB_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scib_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scib_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scib_clk_div_val`]
module"]
#[doc(alias = "MSS_SCIB_CLK_DIV_VAL")]
pub type MssScibClkDivVal = crate::Reg<mss_scib_clk_div_val::MssScibClkDivValSpec>;
#[doc = "MSS_SCIB_CLK_DIV_VAL"]
pub mod mss_scib_clk_div_val;
#[doc = "MSS_CPTS_CLK_DIV_VAL (rw) register accessor: MSS_CPTS_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpts_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpts_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpts_clk_div_val`]
module"]
#[doc(alias = "MSS_CPTS_CLK_DIV_VAL")]
pub type MssCptsClkDivVal = crate::Reg<mss_cpts_clk_div_val::MssCptsClkDivValSpec>;
#[doc = "MSS_CPTS_CLK_DIV_VAL"]
pub mod mss_cpts_clk_div_val;
#[doc = "MSS_CPSW_CLK_DIV_VAL (rw) register accessor: MSS_CPSW_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_clk_div_val`]
module"]
#[doc(alias = "MSS_CPSW_CLK_DIV_VAL")]
pub type MssCpswClkDivVal = crate::Reg<mss_cpsw_clk_div_val::MssCpswClkDivValSpec>;
#[doc = "MSS_CPSW_CLK_DIV_VAL"]
pub mod mss_cpsw_clk_div_val;
#[doc = "MSS_RGMII_CLK_DIV_VAL (rw) register accessor: MSS_RGMII_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rgmii_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rgmii_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rgmii_clk_div_val`]
module"]
#[doc(alias = "MSS_RGMII_CLK_DIV_VAL")]
pub type MssRgmiiClkDivVal = crate::Reg<mss_rgmii_clk_div_val::MssRgmiiClkDivValSpec>;
#[doc = "MSS_RGMII_CLK_DIV_VAL"]
pub mod mss_rgmii_clk_div_val;
#[doc = "MSS_MII100_CLK_DIV_VAL (rw) register accessor: MSS_MII100_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii100_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii100_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii100_clk_div_val`]
module"]
#[doc(alias = "MSS_MII100_CLK_DIV_VAL")]
pub type MssMii100ClkDivVal = crate::Reg<mss_mii100_clk_div_val::MssMii100ClkDivValSpec>;
#[doc = "MSS_MII100_CLK_DIV_VAL"]
pub mod mss_mii100_clk_div_val;
#[doc = "MSS_MII10_CLK_DIV_VAL (rw) register accessor: MSS_MII10_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii10_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii10_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii10_clk_div_val`]
module"]
#[doc(alias = "MSS_MII10_CLK_DIV_VAL")]
pub type MssMii10ClkDivVal = crate::Reg<mss_mii10_clk_div_val::MssMii10ClkDivValSpec>;
#[doc = "MSS_MII10_CLK_DIV_VAL"]
pub mod mss_mii10_clk_div_val;
#[doc = "MSS_GPADC_CLK_DIV_VAL (rw) register accessor: MSS_GPADC_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_clk_div_val`]
module"]
#[doc(alias = "MSS_GPADC_CLK_DIV_VAL")]
pub type MssGpadcClkDivVal = crate::Reg<mss_gpadc_clk_div_val::MssGpadcClkDivValSpec>;
#[doc = "MSS_GPADC_CLK_DIV_VAL"]
pub mod mss_gpadc_clk_div_val;
#[doc = "MSS_QSPI_CLK_GATE (rw) register accessor: MSS_QSPI_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_clk_gate`]
module"]
#[doc(alias = "MSS_QSPI_CLK_GATE")]
pub type MssQspiClkGate = crate::Reg<mss_qspi_clk_gate::MssQspiClkGateSpec>;
#[doc = "MSS_QSPI_CLK_GATE"]
pub mod mss_qspi_clk_gate;
#[doc = "MSS_RTIA_CLK_GATE (rw) register accessor: MSS_RTIA_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtia_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtia_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtia_clk_gate`]
module"]
#[doc(alias = "MSS_RTIA_CLK_GATE")]
pub type MssRtiaClkGate = crate::Reg<mss_rtia_clk_gate::MssRtiaClkGateSpec>;
#[doc = "MSS_RTIA_CLK_GATE"]
pub mod mss_rtia_clk_gate;
#[doc = "MSS_RTIB_CLK_GATE (rw) register accessor: MSS_RTIB_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtib_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtib_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtib_clk_gate`]
module"]
#[doc(alias = "MSS_RTIB_CLK_GATE")]
pub type MssRtibClkGate = crate::Reg<mss_rtib_clk_gate::MssRtibClkGateSpec>;
#[doc = "MSS_RTIB_CLK_GATE"]
pub mod mss_rtib_clk_gate;
#[doc = "MSS_RTIC_CLK_GATE (rw) register accessor: MSS_RTIC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtic_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtic_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtic_clk_gate`]
module"]
#[doc(alias = "MSS_RTIC_CLK_GATE")]
pub type MssRticClkGate = crate::Reg<mss_rtic_clk_gate::MssRticClkGateSpec>;
#[doc = "MSS_RTIC_CLK_GATE"]
pub mod mss_rtic_clk_gate;
#[doc = "MSS_WDT_CLK_GATE (rw) register accessor: MSS_WDT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_wdt_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_wdt_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_wdt_clk_gate`]
module"]
#[doc(alias = "MSS_WDT_CLK_GATE")]
pub type MssWdtClkGate = crate::Reg<mss_wdt_clk_gate::MssWdtClkGateSpec>;
#[doc = "MSS_WDT_CLK_GATE"]
pub mod mss_wdt_clk_gate;
#[doc = "MSS_SPIB_CLK_GATE (rw) register accessor: MSS_SPIB_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_clk_gate`]
module"]
#[doc(alias = "MSS_SPIB_CLK_GATE")]
pub type MssSpibClkGate = crate::Reg<mss_spib_clk_gate::MssSpibClkGateSpec>;
#[doc = "MSS_SPIB_CLK_GATE"]
pub mod mss_spib_clk_gate;
#[doc = "MSS_I2C_CLK_GATE (rw) register accessor: MSS_I2C_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_i2c_clk_gate`]
module"]
#[doc(alias = "MSS_I2C_CLK_GATE")]
pub type MssI2cClkGate = crate::Reg<mss_i2c_clk_gate::MssI2cClkGateSpec>;
#[doc = "MSS_I2C_CLK_GATE"]
pub mod mss_i2c_clk_gate;
#[doc = "MSS_SCIA_CLK_GATE (rw) register accessor: MSS_SCIA_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scia_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scia_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scia_clk_gate`]
module"]
#[doc(alias = "MSS_SCIA_CLK_GATE")]
pub type MssSciaClkGate = crate::Reg<mss_scia_clk_gate::MssSciaClkGateSpec>;
#[doc = "MSS_SCIA_CLK_GATE"]
pub mod mss_scia_clk_gate;
#[doc = "MSS_SCIB_CLK_GATE (rw) register accessor: MSS_SCIB_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scib_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scib_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scib_clk_gate`]
module"]
#[doc(alias = "MSS_SCIB_CLK_GATE")]
pub type MssScibClkGate = crate::Reg<mss_scib_clk_gate::MssScibClkGateSpec>;
#[doc = "MSS_SCIB_CLK_GATE"]
pub mod mss_scib_clk_gate;
#[doc = "MSS_CPTS_CLK_GATE (rw) register accessor: MSS_CPTS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpts_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpts_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpts_clk_gate`]
module"]
#[doc(alias = "MSS_CPTS_CLK_GATE")]
pub type MssCptsClkGate = crate::Reg<mss_cpts_clk_gate::MssCptsClkGateSpec>;
#[doc = "MSS_CPTS_CLK_GATE"]
pub mod mss_cpts_clk_gate;
#[doc = "MSS_CPSW_CLK_GATE (rw) register accessor: MSS_CPSW_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_clk_gate`]
module"]
#[doc(alias = "MSS_CPSW_CLK_GATE")]
pub type MssCpswClkGate = crate::Reg<mss_cpsw_clk_gate::MssCpswClkGateSpec>;
#[doc = "MSS_CPSW_CLK_GATE"]
pub mod mss_cpsw_clk_gate;
#[doc = "MSS_RGMII_CLK_GATE (rw) register accessor: MSS_RGMII_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rgmii_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rgmii_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rgmii_clk_gate`]
module"]
#[doc(alias = "MSS_RGMII_CLK_GATE")]
pub type MssRgmiiClkGate = crate::Reg<mss_rgmii_clk_gate::MssRgmiiClkGateSpec>;
#[doc = "MSS_RGMII_CLK_GATE"]
pub mod mss_rgmii_clk_gate;
#[doc = "MSS_MII100_CLK_GATE (rw) register accessor: MSS_MII100_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii100_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii100_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii100_clk_gate`]
module"]
#[doc(alias = "MSS_MII100_CLK_GATE")]
pub type MssMii100ClkGate = crate::Reg<mss_mii100_clk_gate::MssMii100ClkGateSpec>;
#[doc = "MSS_MII100_CLK_GATE"]
pub mod mss_mii100_clk_gate;
#[doc = "MSS_MII10_CLK_GATE (rw) register accessor: MSS_MII10_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii10_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii10_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii10_clk_gate`]
module"]
#[doc(alias = "MSS_MII10_CLK_GATE")]
pub type MssMii10ClkGate = crate::Reg<mss_mii10_clk_gate::MssMii10ClkGateSpec>;
#[doc = "MSS_MII10_CLK_GATE"]
pub mod mss_mii10_clk_gate;
#[doc = "MSS_GPADC_CLK_GATE (rw) register accessor: MSS_GPADC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_clk_gate`]
module"]
#[doc(alias = "MSS_GPADC_CLK_GATE")]
pub type MssGpadcClkGate = crate::Reg<mss_gpadc_clk_gate::MssGpadcClkGateSpec>;
#[doc = "MSS_GPADC_CLK_GATE"]
pub mod mss_gpadc_clk_gate;
#[doc = "MSS_QSPI_CLK_STATUS (rw) register accessor: MSS_QSPI_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_clk_status`]
module"]
#[doc(alias = "MSS_QSPI_CLK_STATUS")]
pub type MssQspiClkStatus = crate::Reg<mss_qspi_clk_status::MssQspiClkStatusSpec>;
#[doc = "MSS_QSPI_CLK_STATUS"]
pub mod mss_qspi_clk_status;
#[doc = "MSS_RTIA_CLK_STATUS (rw) register accessor: MSS_RTIA_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtia_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtia_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtia_clk_status`]
module"]
#[doc(alias = "MSS_RTIA_CLK_STATUS")]
pub type MssRtiaClkStatus = crate::Reg<mss_rtia_clk_status::MssRtiaClkStatusSpec>;
#[doc = "MSS_RTIA_CLK_STATUS"]
pub mod mss_rtia_clk_status;
#[doc = "MSS_RTIB_CLK_STATUS (rw) register accessor: MSS_RTIB_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtib_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtib_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtib_clk_status`]
module"]
#[doc(alias = "MSS_RTIB_CLK_STATUS")]
pub type MssRtibClkStatus = crate::Reg<mss_rtib_clk_status::MssRtibClkStatusSpec>;
#[doc = "MSS_RTIB_CLK_STATUS"]
pub mod mss_rtib_clk_status;
#[doc = "MSS_RTIC_CLK_STATUS (rw) register accessor: MSS_RTIC_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtic_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtic_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtic_clk_status`]
module"]
#[doc(alias = "MSS_RTIC_CLK_STATUS")]
pub type MssRticClkStatus = crate::Reg<mss_rtic_clk_status::MssRticClkStatusSpec>;
#[doc = "MSS_RTIC_CLK_STATUS"]
pub mod mss_rtic_clk_status;
#[doc = "MSS_WDT_CLK_STATUS (rw) register accessor: MSS_WDT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_wdt_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_wdt_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_wdt_clk_status`]
module"]
#[doc(alias = "MSS_WDT_CLK_STATUS")]
pub type MssWdtClkStatus = crate::Reg<mss_wdt_clk_status::MssWdtClkStatusSpec>;
#[doc = "MSS_WDT_CLK_STATUS"]
pub mod mss_wdt_clk_status;
#[doc = "MSS_SPIB_CLK_STATUS (rw) register accessor: MSS_SPIB_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_clk_status`]
module"]
#[doc(alias = "MSS_SPIB_CLK_STATUS")]
pub type MssSpibClkStatus = crate::Reg<mss_spib_clk_status::MssSpibClkStatusSpec>;
#[doc = "MSS_SPIB_CLK_STATUS"]
pub mod mss_spib_clk_status;
#[doc = "MSS_I2C_CLK_STATUS (rw) register accessor: MSS_I2C_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_i2c_clk_status`]
module"]
#[doc(alias = "MSS_I2C_CLK_STATUS")]
pub type MssI2cClkStatus = crate::Reg<mss_i2c_clk_status::MssI2cClkStatusSpec>;
#[doc = "MSS_I2C_CLK_STATUS"]
pub mod mss_i2c_clk_status;
#[doc = "MSS_SCIA_CLK_STATUS (rw) register accessor: MSS_SCIA_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scia_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scia_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scia_clk_status`]
module"]
#[doc(alias = "MSS_SCIA_CLK_STATUS")]
pub type MssSciaClkStatus = crate::Reg<mss_scia_clk_status::MssSciaClkStatusSpec>;
#[doc = "MSS_SCIA_CLK_STATUS"]
pub mod mss_scia_clk_status;
#[doc = "MSS_SCIB_CLK_STATUS (rw) register accessor: MSS_SCIB_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scib_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scib_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scib_clk_status`]
module"]
#[doc(alias = "MSS_SCIB_CLK_STATUS")]
pub type MssScibClkStatus = crate::Reg<mss_scib_clk_status::MssScibClkStatusSpec>;
#[doc = "MSS_SCIB_CLK_STATUS"]
pub mod mss_scib_clk_status;
#[doc = "MSS_CPTS_CLK_STATUS (rw) register accessor: MSS_CPTS_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpts_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpts_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpts_clk_status`]
module"]
#[doc(alias = "MSS_CPTS_CLK_STATUS")]
pub type MssCptsClkStatus = crate::Reg<mss_cpts_clk_status::MssCptsClkStatusSpec>;
#[doc = "MSS_CPTS_CLK_STATUS"]
pub mod mss_cpts_clk_status;
#[doc = "MSS_CPSW_CLK_STATUS (rw) register accessor: MSS_CPSW_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_clk_status`]
module"]
#[doc(alias = "MSS_CPSW_CLK_STATUS")]
pub type MssCpswClkStatus = crate::Reg<mss_cpsw_clk_status::MssCpswClkStatusSpec>;
#[doc = "MSS_CPSW_CLK_STATUS"]
pub mod mss_cpsw_clk_status;
#[doc = "MSS_RGMII_CLK_STATUS (rw) register accessor: MSS_RGMII_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rgmii_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rgmii_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rgmii_clk_status`]
module"]
#[doc(alias = "MSS_RGMII_CLK_STATUS")]
pub type MssRgmiiClkStatus = crate::Reg<mss_rgmii_clk_status::MssRgmiiClkStatusSpec>;
#[doc = "MSS_RGMII_CLK_STATUS"]
pub mod mss_rgmii_clk_status;
#[doc = "MSS_MII100_CLK_STATUS (rw) register accessor: MSS_MII100_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii100_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii100_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii100_clk_status`]
module"]
#[doc(alias = "MSS_MII100_CLK_STATUS")]
pub type MssMii100ClkStatus = crate::Reg<mss_mii100_clk_status::MssMii100ClkStatusSpec>;
#[doc = "MSS_MII100_CLK_STATUS"]
pub mod mss_mii100_clk_status;
#[doc = "MSS_MII10_CLK_STATUS (rw) register accessor: MSS_MII10_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mii10_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mii10_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mii10_clk_status`]
module"]
#[doc(alias = "MSS_MII10_CLK_STATUS")]
pub type MssMii10ClkStatus = crate::Reg<mss_mii10_clk_status::MssMii10ClkStatusSpec>;
#[doc = "MSS_MII10_CLK_STATUS"]
pub mod mss_mii10_clk_status;
#[doc = "MSS_GPADC_CLK_STATUS (rw) register accessor: MSS_GPADC_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_clk_status`]
module"]
#[doc(alias = "MSS_GPADC_CLK_STATUS")]
pub type MssGpadcClkStatus = crate::Reg<mss_gpadc_clk_status::MssGpadcClkStatusSpec>;
#[doc = "MSS_GPADC_CLK_STATUS"]
pub mod mss_gpadc_clk_status;
#[doc = "MSS_CR5SS_POR_RST_CTRL (rw) register accessor: MSS_CR5SS_POR_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5ss_por_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5ss_por_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5ss_por_rst_ctrl`]
module"]
#[doc(alias = "MSS_CR5SS_POR_RST_CTRL")]
pub type MssCr5ssPorRstCtrl = crate::Reg<mss_cr5ss_por_rst_ctrl::MssCr5ssPorRstCtrlSpec>;
#[doc = "MSS_CR5SS_POR_RST_CTRL"]
pub mod mss_cr5ss_por_rst_ctrl;
#[doc = "MSS_CR5SSA_RST_CTRL (rw) register accessor: MSS_CR5SSA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5ssa_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5ssa_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5ssa_rst_ctrl`]
module"]
#[doc(alias = "MSS_CR5SSA_RST_CTRL")]
pub type MssCr5ssaRstCtrl = crate::Reg<mss_cr5ssa_rst_ctrl::MssCr5ssaRstCtrlSpec>;
#[doc = "MSS_CR5SSA_RST_CTRL"]
pub mod mss_cr5ssa_rst_ctrl;
#[doc = "MSS_CR5SSB_RST_CTRL (rw) register accessor: MSS_CR5SSB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5ssb_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5ssb_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5ssb_rst_ctrl`]
module"]
#[doc(alias = "MSS_CR5SSB_RST_CTRL")]
pub type MssCr5ssbRstCtrl = crate::Reg<mss_cr5ssb_rst_ctrl::MssCr5ssbRstCtrlSpec>;
#[doc = "MSS_CR5SSB_RST_CTRL"]
pub mod mss_cr5ssb_rst_ctrl;
#[doc = "MSS_CR5A_RST_CTRL (rw) register accessor: MSS_CR5A_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_rst_ctrl`]
module"]
#[doc(alias = "MSS_CR5A_RST_CTRL")]
pub type MssCr5aRstCtrl = crate::Reg<mss_cr5a_rst_ctrl::MssCr5aRstCtrlSpec>;
#[doc = "MSS_CR5A_RST_CTRL"]
pub mod mss_cr5a_rst_ctrl;
#[doc = "MSS_CR5B_RST_CTRL (rw) register accessor: MSS_CR5B_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5b_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5b_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5b_rst_ctrl`]
module"]
#[doc(alias = "MSS_CR5B_RST_CTRL")]
pub type MssCr5bRstCtrl = crate::Reg<mss_cr5b_rst_ctrl::MssCr5bRstCtrlSpec>;
#[doc = "MSS_CR5B_RST_CTRL"]
pub mod mss_cr5b_rst_ctrl;
#[doc = "MSS_VIMA_RST_CTRL (rw) register accessor: MSS_VIMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_vima_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_vima_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_vima_rst_ctrl`]
module"]
#[doc(alias = "MSS_VIMA_RST_CTRL")]
pub type MssVimaRstCtrl = crate::Reg<mss_vima_rst_ctrl::MssVimaRstCtrlSpec>;
#[doc = "MSS_VIMA_RST_CTRL"]
pub mod mss_vima_rst_ctrl;
#[doc = "MSS_VIMB_RST_CTRL (rw) register accessor: MSS_VIMB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_vimb_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_vimb_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_vimb_rst_ctrl`]
module"]
#[doc(alias = "MSS_VIMB_RST_CTRL")]
pub type MssVimbRstCtrl = crate::Reg<mss_vimb_rst_ctrl::MssVimbRstCtrlSpec>;
#[doc = "MSS_VIMB_RST_CTRL"]
pub mod mss_vimb_rst_ctrl;
#[doc = "MSS_CRC_RST_CTRL (rw) register accessor: MSS_CRC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_crc_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_crc_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_crc_rst_ctrl`]
module"]
#[doc(alias = "MSS_CRC_RST_CTRL")]
pub type MssCrcRstCtrl = crate::Reg<mss_crc_rst_ctrl::MssCrcRstCtrlSpec>;
#[doc = "MSS_CRC_RST_CTRL"]
pub mod mss_crc_rst_ctrl;
#[doc = "MSS_RTIA_RST_CTRL (rw) register accessor: MSS_RTIA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtia_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtia_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtia_rst_ctrl`]
module"]
#[doc(alias = "MSS_RTIA_RST_CTRL")]
pub type MssRtiaRstCtrl = crate::Reg<mss_rtia_rst_ctrl::MssRtiaRstCtrlSpec>;
#[doc = "MSS_RTIA_RST_CTRL"]
pub mod mss_rtia_rst_ctrl;
#[doc = "MSS_RTIB_RST_CTRL (rw) register accessor: MSS_RTIB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtib_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtib_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtib_rst_ctrl`]
module"]
#[doc(alias = "MSS_RTIB_RST_CTRL")]
pub type MssRtibRstCtrl = crate::Reg<mss_rtib_rst_ctrl::MssRtibRstCtrlSpec>;
#[doc = "MSS_RTIB_RST_CTRL"]
pub mod mss_rtib_rst_ctrl;
#[doc = "MSS_RTIC_RST_CTRL (rw) register accessor: MSS_RTIC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rtic_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rtic_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_rtic_rst_ctrl`]
module"]
#[doc(alias = "MSS_RTIC_RST_CTRL")]
pub type MssRticRstCtrl = crate::Reg<mss_rtic_rst_ctrl::MssRticRstCtrlSpec>;
#[doc = "MSS_RTIC_RST_CTRL"]
pub mod mss_rtic_rst_ctrl;
#[doc = "MSS_WDT_RST_CTRL (rw) register accessor: MSS_WDT_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_wdt_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_wdt_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_wdt_rst_ctrl`]
module"]
#[doc(alias = "MSS_WDT_RST_CTRL")]
pub type MssWdtRstCtrl = crate::Reg<mss_wdt_rst_ctrl::MssWdtRstCtrlSpec>;
#[doc = "MSS_WDT_RST_CTRL"]
pub mod mss_wdt_rst_ctrl;
#[doc = "MSS_ESM_RST_CTRL (rw) register accessor: MSS_ESM_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_esm_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_esm_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_esm_rst_ctrl`]
module"]
#[doc(alias = "MSS_ESM_RST_CTRL")]
pub type MssEsmRstCtrl = crate::Reg<mss_esm_rst_ctrl::MssEsmRstCtrlSpec>;
#[doc = "MSS_ESM_RST_CTRL"]
pub mod mss_esm_rst_ctrl;
#[doc = "MSS_DCCA_RST_CTRL (rw) register accessor: MSS_DCCA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dcca_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dcca_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dcca_rst_ctrl`]
module"]
#[doc(alias = "MSS_DCCA_RST_CTRL")]
pub type MssDccaRstCtrl = crate::Reg<mss_dcca_rst_ctrl::MssDccaRstCtrlSpec>;
#[doc = "MSS_DCCA_RST_CTRL"]
pub mod mss_dcca_rst_ctrl;
#[doc = "MSS_DCCB_RST_CTRL (rw) register accessor: MSS_DCCB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dccb_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dccb_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dccb_rst_ctrl`]
module"]
#[doc(alias = "MSS_DCCB_RST_CTRL")]
pub type MssDccbRstCtrl = crate::Reg<mss_dccb_rst_ctrl::MssDccbRstCtrlSpec>;
#[doc = "MSS_DCCB_RST_CTRL"]
pub mod mss_dccb_rst_ctrl;
#[doc = "MSS_DCCC_RST_CTRL (rw) register accessor: MSS_DCCC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dccc_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dccc_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dccc_rst_ctrl`]
module"]
#[doc(alias = "MSS_DCCC_RST_CTRL")]
pub type MssDcccRstCtrl = crate::Reg<mss_dccc_rst_ctrl::MssDcccRstCtrlSpec>;
#[doc = "MSS_DCCC_RST_CTRL"]
pub mod mss_dccc_rst_ctrl;
#[doc = "MSS_DCCD_RST_CTRL (rw) register accessor: MSS_DCCD_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dccd_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dccd_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dccd_rst_ctrl`]
module"]
#[doc(alias = "MSS_DCCD_RST_CTRL")]
pub type MssDccdRstCtrl = crate::Reg<mss_dccd_rst_ctrl::MssDccdRstCtrlSpec>;
#[doc = "MSS_DCCD_RST_CTRL"]
pub mod mss_dccd_rst_ctrl;
#[doc = "MSS_GIO_RST_CTRL (rw) register accessor: MSS_GIO_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gio_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gio_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gio_rst_ctrl`]
module"]
#[doc(alias = "MSS_GIO_RST_CTRL")]
pub type MssGioRstCtrl = crate::Reg<mss_gio_rst_ctrl::MssGioRstCtrlSpec>;
#[doc = "MSS_GIO_RST_CTRL"]
pub mod mss_gio_rst_ctrl;
#[doc = "MSS_SPIB_RST_CTRL (rw) register accessor: MSS_SPIB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_rst_ctrl`]
module"]
#[doc(alias = "MSS_SPIB_RST_CTRL")]
pub type MssSpibRstCtrl = crate::Reg<mss_spib_rst_ctrl::MssSpibRstCtrlSpec>;
#[doc = "MSS_SPIB_RST_CTRL"]
pub mod mss_spib_rst_ctrl;
#[doc = "MSS_QSPI_RST_CTRL (rw) register accessor: MSS_QSPI_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_rst_ctrl`]
module"]
#[doc(alias = "MSS_QSPI_RST_CTRL")]
pub type MssQspiRstCtrl = crate::Reg<mss_qspi_rst_ctrl::MssQspiRstCtrlSpec>;
#[doc = "MSS_QSPI_RST_CTRL"]
pub mod mss_qspi_rst_ctrl;
#[doc = "MSS_PWM1_RST_CTRL (rw) register accessor: MSS_PWM1_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pwm1_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pwm1_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pwm1_rst_ctrl`]
module"]
#[doc(alias = "MSS_PWM1_RST_CTRL")]
pub type MssPwm1RstCtrl = crate::Reg<mss_pwm1_rst_ctrl::MssPwm1RstCtrlSpec>;
#[doc = "MSS_PWM1_RST_CTRL"]
pub mod mss_pwm1_rst_ctrl;
#[doc = "MSS_PWM2_RST_CTRL (rw) register accessor: MSS_PWM2_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pwm2_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pwm2_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pwm2_rst_ctrl`]
module"]
#[doc(alias = "MSS_PWM2_RST_CTRL")]
pub type MssPwm2RstCtrl = crate::Reg<mss_pwm2_rst_ctrl::MssPwm2RstCtrlSpec>;
#[doc = "MSS_PWM2_RST_CTRL"]
pub mod mss_pwm2_rst_ctrl;
#[doc = "MSS_PWM3_RST_CTRL (rw) register accessor: MSS_PWM3_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pwm3_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pwm3_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pwm3_rst_ctrl`]
module"]
#[doc(alias = "MSS_PWM3_RST_CTRL")]
pub type MssPwm3RstCtrl = crate::Reg<mss_pwm3_rst_ctrl::MssPwm3RstCtrlSpec>;
#[doc = "MSS_PWM3_RST_CTRL"]
pub mod mss_pwm3_rst_ctrl;
#[doc = "MSS_I2C_RST_CTRL (rw) register accessor: MSS_I2C_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_i2c_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_i2c_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_i2c_rst_ctrl`]
module"]
#[doc(alias = "MSS_I2C_RST_CTRL")]
pub type MssI2cRstCtrl = crate::Reg<mss_i2c_rst_ctrl::MssI2cRstCtrlSpec>;
#[doc = "MSS_I2C_RST_CTRL"]
pub mod mss_i2c_rst_ctrl;
#[doc = "MSS_SCIA_RST_CTRL (rw) register accessor: MSS_SCIA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scia_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scia_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scia_rst_ctrl`]
module"]
#[doc(alias = "MSS_SCIA_RST_CTRL")]
pub type MssSciaRstCtrl = crate::Reg<mss_scia_rst_ctrl::MssSciaRstCtrlSpec>;
#[doc = "MSS_SCIA_RST_CTRL"]
pub mod mss_scia_rst_ctrl;
#[doc = "MSS_SCIB_RST_CTRL (rw) register accessor: MSS_SCIB_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_scib_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_scib_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_scib_rst_ctrl`]
module"]
#[doc(alias = "MSS_SCIB_RST_CTRL")]
pub type MssScibRstCtrl = crate::Reg<mss_scib_rst_ctrl::MssScibRstCtrlSpec>;
#[doc = "MSS_SCIB_RST_CTRL"]
pub mod mss_scib_rst_ctrl;
#[doc = "MSS_EDMA_RST_CTRL (rw) register accessor: MSS_EDMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_edma_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_edma_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_edma_rst_ctrl`]
module"]
#[doc(alias = "MSS_EDMA_RST_CTRL")]
pub type MssEdmaRstCtrl = crate::Reg<mss_edma_rst_ctrl::MssEdmaRstCtrlSpec>;
#[doc = "MSS_EDMA_RST_CTRL"]
pub mod mss_edma_rst_ctrl;
#[doc = "MSS_INFRA_RST_CTRL (rw) register accessor: MSS_INFRA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_infra_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_infra_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_infra_rst_ctrl`]
module"]
#[doc(alias = "MSS_INFRA_RST_CTRL")]
pub type MssInfraRstCtrl = crate::Reg<mss_infra_rst_ctrl::MssInfraRstCtrlSpec>;
#[doc = "MSS_INFRA_RST_CTRL"]
pub mod mss_infra_rst_ctrl;
#[doc = "MSS_CPSW_RST_CTRL (rw) register accessor: MSS_CPSW_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_rst_ctrl`]
module"]
#[doc(alias = "MSS_CPSW_RST_CTRL")]
pub type MssCpswRstCtrl = crate::Reg<mss_cpsw_rst_ctrl::MssCpswRstCtrlSpec>;
#[doc = "MSS_CPSW_RST_CTRL"]
pub mod mss_cpsw_rst_ctrl;
#[doc = "MSS_GPADC_RST_CTRL (rw) register accessor: MSS_GPADC_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_rst_ctrl`]
module"]
#[doc(alias = "MSS_GPADC_RST_CTRL")]
pub type MssGpadcRstCtrl = crate::Reg<mss_gpadc_rst_ctrl::MssGpadcRstCtrlSpec>;
#[doc = "MSS_GPADC_RST_CTRL"]
pub mod mss_gpadc_rst_ctrl;
#[doc = "MSS_DMM_RST_CTRL (rw) register accessor: MSS_DMM_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_rst_ctrl`]
module"]
#[doc(alias = "MSS_DMM_RST_CTRL")]
pub type MssDmmRstCtrl = crate::Reg<mss_dmm_rst_ctrl::MssDmmRstCtrlSpec>;
#[doc = "MSS_DMM_RST_CTRL"]
pub mod mss_dmm_rst_ctrl;
#[doc = "R5_COREA_GATE (rw) register accessor: R5_COREA_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_corea_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_corea_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_corea_gate`]
module"]
#[doc(alias = "R5_COREA_GATE")]
pub type R5CoreaGate = crate::Reg<r5_corea_gate::R5CoreaGateSpec>;
#[doc = "R5_COREA_GATE"]
pub mod r5_corea_gate;
#[doc = "R5_COREB_GATE (rw) register accessor: R5_COREB_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_coreb_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_coreb_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_coreb_gate`]
module"]
#[doc(alias = "R5_COREB_GATE")]
pub type R5CorebGate = crate::Reg<r5_coreb_gate::R5CorebGateSpec>;
#[doc = "R5_COREB_GATE"]
pub mod r5_coreb_gate;
#[doc = "MSS_L2_BANKA_PD_CTRL (rw) register accessor: MSS_L2_BANKA_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_banka_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_banka_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_banka_pd_ctrl`]
module"]
#[doc(alias = "MSS_L2_BANKA_PD_CTRL")]
pub type MssL2BankaPdCtrl = crate::Reg<mss_l2_banka_pd_ctrl::MssL2BankaPdCtrlSpec>;
#[doc = "MSS_L2_BANKA_PD_CTRL"]
pub mod mss_l2_banka_pd_ctrl;
#[doc = "MSS_L2_BANKB_PD_CTRL (rw) register accessor: MSS_L2_BANKB_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankb_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankb_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_bankb_pd_ctrl`]
module"]
#[doc(alias = "MSS_L2_BANKB_PD_CTRL")]
pub type MssL2BankbPdCtrl = crate::Reg<mss_l2_bankb_pd_ctrl::MssL2BankbPdCtrlSpec>;
#[doc = "MSS_L2_BANKB_PD_CTRL"]
pub mod mss_l2_bankb_pd_ctrl;
#[doc = "MSS_L2_BANKA_PD_STATUS (rw) register accessor: MSS_L2_BANKA_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_banka_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_banka_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_banka_pd_status`]
module"]
#[doc(alias = "MSS_L2_BANKA_PD_STATUS")]
pub type MssL2BankaPdStatus = crate::Reg<mss_l2_banka_pd_status::MssL2BankaPdStatusSpec>;
#[doc = "MSS_L2_BANKA_PD_STATUS"]
pub mod mss_l2_banka_pd_status;
#[doc = "MSS_L2_BANKB_PD_STATUS (rw) register accessor: MSS_L2_BANKB_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankb_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankb_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_bankb_pd_status`]
module"]
#[doc(alias = "MSS_L2_BANKB_PD_STATUS")]
pub type MssL2BankbPdStatus = crate::Reg<mss_l2_bankb_pd_status::MssL2BankbPdStatusSpec>;
#[doc = "MSS_L2_BANKB_PD_STATUS"]
pub mod mss_l2_bankb_pd_status;
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
#[doc = "MSS_CR5F_CLK_SRC_SEL_CTRL (rw) register accessor: MSS_CR5F_CLK_SRC_SEL_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5f_clk_src_sel_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5f_clk_src_sel_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5f_clk_src_sel_ctrl`]
module"]
#[doc(alias = "MSS_CR5F_CLK_SRC_SEL_CTRL")]
pub type MssCr5fClkSrcSelCtrl = crate::Reg<mss_cr5f_clk_src_sel_ctrl::MssCr5fClkSrcSelCtrlSpec>;
#[doc = "MSS_CR5F_CLK_SRC_SEL_CTRL"]
pub mod mss_cr5f_clk_src_sel_ctrl;
#[doc = "MSS_CPSW_MII_CLK_SRC_SEL (rw) register accessor: MSS_CPSW_MII_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_mii_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_mii_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_mii_clk_src_sel`]
module"]
#[doc(alias = "MSS_CPSW_MII_CLK_SRC_SEL")]
pub type MssCpswMiiClkSrcSel = crate::Reg<mss_cpsw_mii_clk_src_sel::MssCpswMiiClkSrcSelSpec>;
#[doc = "MSS_CPSW_MII_CLK_SRC_SEL"]
pub mod mss_cpsw_mii_clk_src_sel;
#[doc = "MSS_CPSW_MII_CLK_STATUS (rw) register accessor: MSS_CPSW_MII_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_mii_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_mii_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cpsw_mii_clk_status`]
module"]
#[doc(alias = "MSS_CPSW_MII_CLK_STATUS")]
pub type MssCpswMiiClkStatus = crate::Reg<mss_cpsw_mii_clk_status::MssCpswMiiClkStatusSpec>;
#[doc = "MSS_CPSW_MII_CLK_STATUS"]
pub mod mss_cpsw_mii_clk_status;
#[doc = "MSS_L2_BANKC_PD_CTRL (rw) register accessor: MSS_L2_BANKC_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankc_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankc_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_bankc_pd_ctrl`]
module"]
#[doc(alias = "MSS_L2_BANKC_PD_CTRL")]
pub type MssL2BankcPdCtrl = crate::Reg<mss_l2_bankc_pd_ctrl::MssL2BankcPdCtrlSpec>;
#[doc = "MSS_L2_BANKC_PD_CTRL"]
pub mod mss_l2_bankc_pd_ctrl;
#[doc = "MSS_L2_BANKC_PD_STATUS (rw) register accessor: MSS_L2_BANKC_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankc_pd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankc_pd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_bankc_pd_status`]
module"]
#[doc(alias = "MSS_L2_BANKC_PD_STATUS")]
pub type MssL2BankcPdStatus = crate::Reg<mss_l2_bankc_pd_status::MssL2BankcPdStatusSpec>;
#[doc = "MSS_L2_BANKC_PD_STATUS"]
pub mod mss_l2_bankc_pd_status;
#[doc = "MSS_IP_CLK_CFG (rw) register accessor: MSS_IP_CLK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_ip_clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_ip_clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_ip_clk_cfg`]
module"]
#[doc(alias = "MSS_IP_CLK_CFG")]
pub type MssIpClkCfg = crate::Reg<mss_ip_clk_cfg::MssIpClkCfgSpec>;
#[doc = "MSS_IP_CLK_CFG"]
pub mod mss_ip_clk_cfg;
#[doc = "HSM_RTIA_CLK_SRC_SEL (rw) register accessor: HSM_RTIA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rtia_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rtia_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rtia_clk_src_sel`]
module"]
#[doc(alias = "HSM_RTIA_CLK_SRC_SEL")]
pub type HsmRtiaClkSrcSel = crate::Reg<hsm_rtia_clk_src_sel::HsmRtiaClkSrcSelSpec>;
#[doc = "HSM_RTIA_CLK_SRC_SEL"]
pub mod hsm_rtia_clk_src_sel;
#[doc = "HSM_WDT_CLK_SRC_SEL (rw) register accessor: HSM_WDT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_wdt_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_wdt_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_wdt_clk_src_sel`]
module"]
#[doc(alias = "HSM_WDT_CLK_SRC_SEL")]
pub type HsmWdtClkSrcSel = crate::Reg<hsm_wdt_clk_src_sel::HsmWdtClkSrcSelSpec>;
#[doc = "HSM_WDT_CLK_SRC_SEL"]
pub mod hsm_wdt_clk_src_sel;
#[doc = "HSM_RTC_CLK_SRC_SEL (rw) register accessor: HSM_RTC_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rtc_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rtc_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rtc_clk_src_sel`]
module"]
#[doc(alias = "HSM_RTC_CLK_SRC_SEL")]
pub type HsmRtcClkSrcSel = crate::Reg<hsm_rtc_clk_src_sel::HsmRtcClkSrcSelSpec>;
#[doc = "HSM_RTC_CLK_SRC_SEL"]
pub mod hsm_rtc_clk_src_sel;
#[doc = "HSM_DMTA_CLK_SRC_SEL (rw) register accessor: HSM_DMTA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmta_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmta_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmta_clk_src_sel`]
module"]
#[doc(alias = "HSM_DMTA_CLK_SRC_SEL")]
pub type HsmDmtaClkSrcSel = crate::Reg<hsm_dmta_clk_src_sel::HsmDmtaClkSrcSelSpec>;
#[doc = "HSM_DMTA_CLK_SRC_SEL"]
pub mod hsm_dmta_clk_src_sel;
#[doc = "HSM_DMTB_CLK_SRC_SEL (rw) register accessor: HSM_DMTB_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmtb_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmtb_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmtb_clk_src_sel`]
module"]
#[doc(alias = "HSM_DMTB_CLK_SRC_SEL")]
pub type HsmDmtbClkSrcSel = crate::Reg<hsm_dmtb_clk_src_sel::HsmDmtbClkSrcSelSpec>;
#[doc = "HSM_DMTB_CLK_SRC_SEL"]
pub mod hsm_dmtb_clk_src_sel;
#[doc = "HSM_RTI_CLK_DIV_VAL (rw) register accessor: HSM_RTI_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rti_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rti_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rti_clk_div_val`]
module"]
#[doc(alias = "HSM_RTI_CLK_DIV_VAL")]
pub type HsmRtiClkDivVal = crate::Reg<hsm_rti_clk_div_val::HsmRtiClkDivValSpec>;
#[doc = "HSM_RTI_CLK_DIV_VAL"]
pub mod hsm_rti_clk_div_val;
#[doc = "HSM_WDT_CLK_DIV_VAL (rw) register accessor: HSM_WDT_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_wdt_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_wdt_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_wdt_clk_div_val`]
module"]
#[doc(alias = "HSM_WDT_CLK_DIV_VAL")]
pub type HsmWdtClkDivVal = crate::Reg<hsm_wdt_clk_div_val::HsmWdtClkDivValSpec>;
#[doc = "HSM_WDT_CLK_DIV_VAL"]
pub mod hsm_wdt_clk_div_val;
#[doc = "HSM_RTC_CLK_DIV_VAL (rw) register accessor: HSM_RTC_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rtc_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rtc_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rtc_clk_div_val`]
module"]
#[doc(alias = "HSM_RTC_CLK_DIV_VAL")]
pub type HsmRtcClkDivVal = crate::Reg<hsm_rtc_clk_div_val::HsmRtcClkDivValSpec>;
#[doc = "HSM_RTC_CLK_DIV_VAL"]
pub mod hsm_rtc_clk_div_val;
#[doc = "HSM_DMTA_CLK_DIV_VAL (rw) register accessor: HSM_DMTA_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmta_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmta_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmta_clk_div_val`]
module"]
#[doc(alias = "HSM_DMTA_CLK_DIV_VAL")]
pub type HsmDmtaClkDivVal = crate::Reg<hsm_dmta_clk_div_val::HsmDmtaClkDivValSpec>;
#[doc = "HSM_DMTA_CLK_DIV_VAL"]
pub mod hsm_dmta_clk_div_val;
#[doc = "HSM_DMTB_CLK_DIV_VAL (rw) register accessor: HSM_DMTB_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmtb_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmtb_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmtb_clk_div_val`]
module"]
#[doc(alias = "HSM_DMTB_CLK_DIV_VAL")]
pub type HsmDmtbClkDivVal = crate::Reg<hsm_dmtb_clk_div_val::HsmDmtbClkDivValSpec>;
#[doc = "HSM_DMTB_CLK_DIV_VAL"]
pub mod hsm_dmtb_clk_div_val;
#[doc = "HSM_RTI_CLK_GATE (rw) register accessor: HSM_RTI_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rti_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rti_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rti_clk_gate`]
module"]
#[doc(alias = "HSM_RTI_CLK_GATE")]
pub type HsmRtiClkGate = crate::Reg<hsm_rti_clk_gate::HsmRtiClkGateSpec>;
#[doc = "HSM_RTI_CLK_GATE"]
pub mod hsm_rti_clk_gate;
#[doc = "HSM_WDT_CLK_GATE (rw) register accessor: HSM_WDT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_wdt_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_wdt_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_wdt_clk_gate`]
module"]
#[doc(alias = "HSM_WDT_CLK_GATE")]
pub type HsmWdtClkGate = crate::Reg<hsm_wdt_clk_gate::HsmWdtClkGateSpec>;
#[doc = "HSM_WDT_CLK_GATE"]
pub mod hsm_wdt_clk_gate;
#[doc = "HSM_RTC_CLK_GATE (rw) register accessor: HSM_RTC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rtc_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rtc_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rtc_clk_gate`]
module"]
#[doc(alias = "HSM_RTC_CLK_GATE")]
pub type HsmRtcClkGate = crate::Reg<hsm_rtc_clk_gate::HsmRtcClkGateSpec>;
#[doc = "HSM_RTC_CLK_GATE"]
pub mod hsm_rtc_clk_gate;
#[doc = "HSM_DMTA_CLK_GATE (rw) register accessor: HSM_DMTA_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmta_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmta_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmta_clk_gate`]
module"]
#[doc(alias = "HSM_DMTA_CLK_GATE")]
pub type HsmDmtaClkGate = crate::Reg<hsm_dmta_clk_gate::HsmDmtaClkGateSpec>;
#[doc = "HSM_DMTA_CLK_GATE"]
pub mod hsm_dmta_clk_gate;
#[doc = "HSM_DMTB_CLK_GATE (rw) register accessor: HSM_DMTB_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmtb_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmtb_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmtb_clk_gate`]
module"]
#[doc(alias = "HSM_DMTB_CLK_GATE")]
pub type HsmDmtbClkGate = crate::Reg<hsm_dmtb_clk_gate::HsmDmtbClkGateSpec>;
#[doc = "HSM_DMTB_CLK_GATE"]
pub mod hsm_dmtb_clk_gate;
#[doc = "HSM_RTI_CLK_STATUS (rw) register accessor: HSM_RTI_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rti_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rti_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rti_clk_status`]
module"]
#[doc(alias = "HSM_RTI_CLK_STATUS")]
pub type HsmRtiClkStatus = crate::Reg<hsm_rti_clk_status::HsmRtiClkStatusSpec>;
#[doc = "HSM_RTI_CLK_STATUS"]
pub mod hsm_rti_clk_status;
#[doc = "HSM_WDT_CLK_STATUS (rw) register accessor: HSM_WDT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_wdt_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_wdt_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_wdt_clk_status`]
module"]
#[doc(alias = "HSM_WDT_CLK_STATUS")]
pub type HsmWdtClkStatus = crate::Reg<hsm_wdt_clk_status::HsmWdtClkStatusSpec>;
#[doc = "HSM_WDT_CLK_STATUS"]
pub mod hsm_wdt_clk_status;
#[doc = "HSM_RTC_CLK_STATUS (rw) register accessor: HSM_RTC_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rtc_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rtc_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_rtc_clk_status`]
module"]
#[doc(alias = "HSM_RTC_CLK_STATUS")]
pub type HsmRtcClkStatus = crate::Reg<hsm_rtc_clk_status::HsmRtcClkStatusSpec>;
#[doc = "HSM_RTC_CLK_STATUS"]
pub mod hsm_rtc_clk_status;
#[doc = "HSM_DMTA_CLK_STATUS (rw) register accessor: HSM_DMTA_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmta_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmta_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmta_clk_status`]
module"]
#[doc(alias = "HSM_DMTA_CLK_STATUS")]
pub type HsmDmtaClkStatus = crate::Reg<hsm_dmta_clk_status::HsmDmtaClkStatusSpec>;
#[doc = "HSM_DMTA_CLK_STATUS"]
pub mod hsm_dmta_clk_status;
#[doc = "HSM_DMTB_CLK_STATUS (rw) register accessor: HSM_DMTB_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_dmtb_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_dmtb_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsm_dmtb_clk_status`]
module"]
#[doc(alias = "HSM_DMTB_CLK_STATUS")]
pub type HsmDmtbClkStatus = crate::Reg<hsm_dmtb_clk_status::HsmDmtbClkStatusSpec>;
#[doc = "HSM_DMTB_CLK_STATUS"]
pub mod hsm_dmtb_clk_status;
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
