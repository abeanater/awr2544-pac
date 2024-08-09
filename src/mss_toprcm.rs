#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    hsi_clk_src_sel: HsiClkSrcSel,
    csirx_clk_src_sel: CsirxClkSrcSel,
    mcuclkout_clk_src_sel: McuclkoutClkSrcSel,
    pmicclkout_clk_src_sel: PmicclkoutClkSrcSel,
    obsclkout_clk_src_sel: ObsclkoutClkSrcSel,
    trcclkout_clk_src_sel: TrcclkoutClkSrcSel,
    _reserved11: [u8; 0x18],
    csirx_div_val: CsirxDivVal,
    mcuclkout_div_val: McuclkoutDivVal,
    pmicclkout_div_val: PmicclkoutDivVal,
    obsclkout_div_val: ObsclkoutDivVal,
    trcclkout_div_val: TrcclkoutDivVal,
    _reserved16: [u8; 0x2c],
    csirx_clk_gate: CsirxClkGate,
    mcuclkout_clk_gate: McuclkoutClkGate,
    pmicclkout_clk_gate: PmicclkoutClkGate,
    obsclkout_clk_gate: ObsclkoutClkGate,
    trcclkout_clk_gate: TrcclkoutClkGate,
    dss_clk_gate: DssClkGate,
    _reserved22: [u8; 0x24],
    hsi_clk_status: HsiClkStatus,
    _reserved23: [u8; 0x04],
    mcuclkout_clk_status: McuclkoutClkStatus,
    pmicclkout_clk_status: PmicclkoutClkStatus,
    obsclkout_clk_status: ObsclkoutClkStatus,
    trcclkout_clk_status: TrcclkoutClkStatus,
    _reserved27: [u8; 0x28],
    warm_reset_config: WarmResetConfig,
    sys_rst_cause: SysRstCause,
    sys_rst_cause_clr: SysRstCauseClr,
    dss_rst_ctrl: DssRstCtrl,
    _reserved31: [u8; 0xf4],
    rs232_bitinterval: Rs232Bitinterval,
    lvds_pad_ctrl0: LvdsPadCtrl0,
    lvds_pad_ctrl1: LvdsPadCtrl1,
    dft_dmled_exec: DftDmledExec,
    dft_dmled_status: DftDmledStatus,
    limp_mode_en: LimpModeEn,
    pmicclkout_dcdc_ctrl: PmicclkoutDcdcCtrl,
    pmicclkout_dcdc_slope: PmicclkoutDcdcSlope,
    rcosc32k_ctrl: Rcosc32kCtrl,
    ana_hsi2digclk_gate: AnaHsi2digclkGate,
    dsi_phy_pwrctrl: DsiPhyPwrctrl,
    rs232_sop11_bitinterval: Rs232Sop11Bitinterval,
    _reserved43: [u8; 0x01cc],
    pll_core_pwrctrl: PllCorePwrctrl,
    pll_core_clkctrl: PllCoreClkctrl,
    pll_core_tenable: PllCoreTenable,
    pll_core_tenablediv: PllCoreTenablediv,
    pll_core_m2ndiv: PllCoreM2ndiv,
    pll_core_mn2div: PllCoreMn2div,
    pll_core_fracdiv: PllCoreFracdiv,
    pll_core_bwctrl: PllCoreBwctrl,
    pll_core_fracctrl: PllCoreFracctrl,
    pll_core_status: PllCoreStatus,
    pll_core_hsdivider: PllCoreHsdivider,
    pll_core_hsdivider_clkout0: PllCoreHsdividerClkout0,
    pll_core_hsdivider_clkout1: PllCoreHsdividerClkout1,
    pll_core_hsdivider_clkout2: PllCoreHsdividerClkout2,
    pll_core_hsdivider_clkout3: PllCoreHsdividerClkout3,
    mss_cr5_clk_src_sel: MssCr5ClkSrcSel,
    mss_cr5_div_val: MssCr5DivVal,
    sys_clk_div_val: SysClkDivVal,
    mss_cr5_clk_gate: MssCr5ClkGate,
    sys_clk_gate: SysClkGate,
    sys_clk_status: SysClkStatus,
    mss_cr5_clk_status: MssCr5ClkStatus,
    pll_core_rstctrl: PllCoreRstctrl,
    pll_core_hsdivider_rstctrl: PllCoreHsdividerRstctrl,
    rss_clk_src_sel: RssClkSrcSel,
    pllc_clk2_src_sel: PllcClk2SrcSel,
    plld_clk1_src_sel: PlldClk1SrcSel,
    plld_clk2_src_sel: PlldClk2SrcSel,
    pllp_clk1_src_sel: PllpClk1SrcSel,
    rss_div_val: RssDivVal,
    rss_clk_gate: RssClkGate,
    pllc_clk2_gate: PllcClk2Gate,
    plld_clk1_gate: PlldClk1Gate,
    plld_clk2_gate: PlldClk2Gate,
    pllp_clk1_gate: PllpClk1Gate,
    rss_clk_status: RssClkStatus,
    pllc_clk2_status: PllcClk2Status,
    plld_clk1_status: PlldClk1Status,
    _reserved81: [u8; 0x04],
    pllp_clk1_status: PllpClk1Status,
    pll_1p2_hsdivider: Pll1p2Hsdivider,
    pll_1p2_hsdivider_clkout0: Pll1p2HsdividerClkout0,
    pll_1p2_hsdivider_clkout1: Pll1p2HsdividerClkout1,
    pll_1p2_hsdivider_clkout2: Pll1p2HsdividerClkout2,
    pll_1p2_hsdivider_clkout3: Pll1p2HsdividerClkout3,
    pll_1p2_hsdivider_rstctrl: Pll1p2HsdividerRstctrl,
    pll_1p8_hsdivider: Pll1p8Hsdivider,
    pll_1p8_hsdivider_clkout0: Pll1p8HsdividerClkout0,
    pll_1p8_hsdivider_clkout1: Pll1p8HsdividerClkout1,
    pll_1p8_hsdivider_clkout2: Pll1p8HsdividerClkout2,
    pll_1p8_hsdivider_clkout3: Pll1p8HsdividerClkout3,
    pll_1p8_hsdivider_rstctrl: Pll1p8HsdividerRstctrl,
    _reserved94: [u8; 0x0330],
    pll_dsp_pwrctrl: PllDspPwrctrl,
    pll_dsp_clkctrl: PllDspClkctrl,
    pll_dsp_tenable: PllDspTenable,
    pll_dsp_tenablediv: PllDspTenablediv,
    pll_dsp_m2ndiv: PllDspM2ndiv,
    pll_dsp_mn2div: PllDspMn2div,
    pll_dsp_fracdiv: PllDspFracdiv,
    pll_dsp_bwctrl: PllDspBwctrl,
    pll_dsp_fracctrl: PllDspFracctrl,
    _reserved103: [u8; 0x04],
    pll_dsp_hsdivider: PllDspHsdivider,
    pll_dsp_hsdivider_clkout0: PllDspHsdividerClkout0,
    pll_dsp_hsdivider_clkout1: PllDspHsdividerClkout1,
    pll_dsp_hsdivider_clkout2: PllDspHsdividerClkout2,
    pll_dsp_hsdivider_clkout3: PllDspHsdividerClkout3,
    pll_per_pwrctrl: PllPerPwrctrl,
    pll_per_clkctrl: PllPerClkctrl,
    pll_per_tenable: PllPerTenable,
    pll_per_tenablediv: PllPerTenablediv,
    pll_per_m2ndiv: PllPerM2ndiv,
    pll_per_mn2div: PllPerMn2div,
    pll_per_fracdiv: PllPerFracdiv,
    pll_per_bwctrl: PllPerBwctrl,
    pll_per_fracctrl: PllPerFracctrl,
    pll_per_status: PllPerStatus,
    pll_per_hsdivider: PllPerHsdivider,
    pll_per_hsdivider_clkout0: PllPerHsdividerClkout0,
    pll_per_hsdivider_clkout1: PllPerHsdividerClkout1,
    pll_per_hsdivider_clkout2: PllPerHsdividerClkout2,
    pll_per_hsdivider_clkout3: PllPerHsdividerClkout3,
    pll_dsp_rstctrl: PllDspRstctrl,
    pll_dsp_hsdivider_rstctrl: PllDspHsdividerRstctrl,
    pll_per_rstctrl: PllPerRstctrl,
    pll_per_hsdivider_rstctrl: PllPerHsdividerRstctrl,
    _reserved127: [u8; 0x0378],
    ana_reg_clk_ctrl_reg1_xo_slicer: AnaRegClkCtrlReg1XoSlicer,
    ana_reg_clk_ctrl_reg1_clktop: AnaRegClkCtrlReg1Clktop,
    ana_reg_clk_ctrl_reg2_clktop: AnaRegClkCtrlReg2Clktop,
    ana_reg_clk_ctrl_reg1_ldo_clktop: AnaRegClkCtrlReg1LdoClktop,
    ana_reg_clk_ctrl_reg2_ldo_clktop: AnaRegClkCtrlReg2LdoClktop,
    _reserved132: [u8; 0x04],
    ana_reg_clk_status_reg: AnaRegClkStatusReg,
    ana_reg_refsys_ctrl_reg_lowv: AnaRegRefsysCtrlRegLowv,
    ana_reg_refsys_tmux_ctrl_lowv: AnaRegRefsysTmuxCtrlLowv,
    ana_reg_refsys_spare_reg_lowv: AnaRegRefsysSpareRegLowv,
    ana_reg_wu_ctrl_reg_lowv: AnaRegWuCtrlRegLowv,
    ana_reg_wu_tmux_ctrl_lowv: AnaRegWuTmuxCtrlLowv,
    ana_reg_tw_ctrl_reg_lowv: AnaRegTwCtrlRegLowv,
    ana_reg_tw_ana_tmux_ctrl_lowv: AnaRegTwAnaTmuxCtrlLowv,
    _reserved140: [u8; 0x04],
    ana_reg_wu_mode_reg_lowv: AnaRegWuModeRegLowv,
    ana_reg_wu_status_reg_lowv: AnaRegWuStatusRegLowv,
    ana_reg_wu_spare_out_lowv: AnaRegWuSpareOutLowv,
    _reserved143: [u8; 0x0388],
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
    _reserved153: [u8; 0x10],
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
    #[doc = "0x14 - HSI_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn hsi_clk_src_sel(&self) -> &HsiClkSrcSel {
        &self.hsi_clk_src_sel
    }
    #[doc = "0x18 - CSIRX_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn csirx_clk_src_sel(&self) -> &CsirxClkSrcSel {
        &self.csirx_clk_src_sel
    }
    #[doc = "0x1c - MCUCLKOUT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mcuclkout_clk_src_sel(&self) -> &McuclkoutClkSrcSel {
        &self.mcuclkout_clk_src_sel
    }
    #[doc = "0x20 - PMICCLKOUT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn pmicclkout_clk_src_sel(&self) -> &PmicclkoutClkSrcSel {
        &self.pmicclkout_clk_src_sel
    }
    #[doc = "0x24 - OBSCLKOUT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn obsclkout_clk_src_sel(&self) -> &ObsclkoutClkSrcSel {
        &self.obsclkout_clk_src_sel
    }
    #[doc = "0x28 - TRCCLKOUT_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn trcclkout_clk_src_sel(&self) -> &TrcclkoutClkSrcSel {
        &self.trcclkout_clk_src_sel
    }
    #[doc = "0x44 - CSIRX_DIV_VAL"]
    #[inline(always)]
    pub const fn csirx_div_val(&self) -> &CsirxDivVal {
        &self.csirx_div_val
    }
    #[doc = "0x48 - MCUCLKOUT_DIV_VAL"]
    #[inline(always)]
    pub const fn mcuclkout_div_val(&self) -> &McuclkoutDivVal {
        &self.mcuclkout_div_val
    }
    #[doc = "0x4c - PMICCLKOUT_DIV_VAL"]
    #[inline(always)]
    pub const fn pmicclkout_div_val(&self) -> &PmicclkoutDivVal {
        &self.pmicclkout_div_val
    }
    #[doc = "0x50 - OBSCLKOUT_DIV_VAL"]
    #[inline(always)]
    pub const fn obsclkout_div_val(&self) -> &ObsclkoutDivVal {
        &self.obsclkout_div_val
    }
    #[doc = "0x54 - TRCCLKOUT_DIV_VAL"]
    #[inline(always)]
    pub const fn trcclkout_div_val(&self) -> &TrcclkoutDivVal {
        &self.trcclkout_div_val
    }
    #[doc = "0x84 - CSIRX_CLK_GATE"]
    #[inline(always)]
    pub const fn csirx_clk_gate(&self) -> &CsirxClkGate {
        &self.csirx_clk_gate
    }
    #[doc = "0x88 - MCUCLKOUT_CLK_GATE"]
    #[inline(always)]
    pub const fn mcuclkout_clk_gate(&self) -> &McuclkoutClkGate {
        &self.mcuclkout_clk_gate
    }
    #[doc = "0x8c - PMICCLKOUT_CLK_GATE"]
    #[inline(always)]
    pub const fn pmicclkout_clk_gate(&self) -> &PmicclkoutClkGate {
        &self.pmicclkout_clk_gate
    }
    #[doc = "0x90 - OBSCLKOUT_CLK_GATE"]
    #[inline(always)]
    pub const fn obsclkout_clk_gate(&self) -> &ObsclkoutClkGate {
        &self.obsclkout_clk_gate
    }
    #[doc = "0x94 - TRCCLKOUT_CLK_GATE"]
    #[inline(always)]
    pub const fn trcclkout_clk_gate(&self) -> &TrcclkoutClkGate {
        &self.trcclkout_clk_gate
    }
    #[doc = "0x98 - DSS_CLK_GATE"]
    #[inline(always)]
    pub const fn dss_clk_gate(&self) -> &DssClkGate {
        &self.dss_clk_gate
    }
    #[doc = "0xc0 - HSI_CLK_STATUS"]
    #[inline(always)]
    pub const fn hsi_clk_status(&self) -> &HsiClkStatus {
        &self.hsi_clk_status
    }
    #[doc = "0xc8 - MCUCLKOUT_CLK_STATUS"]
    #[inline(always)]
    pub const fn mcuclkout_clk_status(&self) -> &McuclkoutClkStatus {
        &self.mcuclkout_clk_status
    }
    #[doc = "0xcc - PMICCLKOUT_CLK_STATUS"]
    #[inline(always)]
    pub const fn pmicclkout_clk_status(&self) -> &PmicclkoutClkStatus {
        &self.pmicclkout_clk_status
    }
    #[doc = "0xd0 - OBSCLKOUT_CLK_STATUS"]
    #[inline(always)]
    pub const fn obsclkout_clk_status(&self) -> &ObsclkoutClkStatus {
        &self.obsclkout_clk_status
    }
    #[doc = "0xd4 - TRCCLKOUT_CLK_STATUS"]
    #[inline(always)]
    pub const fn trcclkout_clk_status(&self) -> &TrcclkoutClkStatus {
        &self.trcclkout_clk_status
    }
    #[doc = "0x100 - WARM_RESET_CONFIG"]
    #[inline(always)]
    pub const fn warm_reset_config(&self) -> &WarmResetConfig {
        &self.warm_reset_config
    }
    #[doc = "0x104 - SYS_RST_CAUSE"]
    #[inline(always)]
    pub const fn sys_rst_cause(&self) -> &SysRstCause {
        &self.sys_rst_cause
    }
    #[doc = "0x108 - SYS_RST_CAUSE_CLR"]
    #[inline(always)]
    pub const fn sys_rst_cause_clr(&self) -> &SysRstCauseClr {
        &self.sys_rst_cause_clr
    }
    #[doc = "0x10c - DSS_RST_CTRL"]
    #[inline(always)]
    pub const fn dss_rst_ctrl(&self) -> &DssRstCtrl {
        &self.dss_rst_ctrl
    }
    #[doc = "0x204 - RS232_BITINTERVAL"]
    #[inline(always)]
    pub const fn rs232_bitinterval(&self) -> &Rs232Bitinterval {
        &self.rs232_bitinterval
    }
    #[doc = "0x208 - LVDS_PAD_CTRL0"]
    #[inline(always)]
    pub const fn lvds_pad_ctrl0(&self) -> &LvdsPadCtrl0 {
        &self.lvds_pad_ctrl0
    }
    #[doc = "0x20c - LVDS_PAD_CTRL1"]
    #[inline(always)]
    pub const fn lvds_pad_ctrl1(&self) -> &LvdsPadCtrl1 {
        &self.lvds_pad_ctrl1
    }
    #[doc = "0x210 - DFT_DMLED_EXEC"]
    #[inline(always)]
    pub const fn dft_dmled_exec(&self) -> &DftDmledExec {
        &self.dft_dmled_exec
    }
    #[doc = "0x214 - DFT_DMLED_STATUS"]
    #[inline(always)]
    pub const fn dft_dmled_status(&self) -> &DftDmledStatus {
        &self.dft_dmled_status
    }
    #[doc = "0x218 - LIMP_MODE_EN"]
    #[inline(always)]
    pub const fn limp_mode_en(&self) -> &LimpModeEn {
        &self.limp_mode_en
    }
    #[doc = "0x21c - PMICCLKOUT_DCDC_CTRL"]
    #[inline(always)]
    pub const fn pmicclkout_dcdc_ctrl(&self) -> &PmicclkoutDcdcCtrl {
        &self.pmicclkout_dcdc_ctrl
    }
    #[doc = "0x220 - PMICCLKOUT_DCDC_SLOPE"]
    #[inline(always)]
    pub const fn pmicclkout_dcdc_slope(&self) -> &PmicclkoutDcdcSlope {
        &self.pmicclkout_dcdc_slope
    }
    #[doc = "0x224 - RCOSC32K_CTRL"]
    #[inline(always)]
    pub const fn rcosc32k_ctrl(&self) -> &Rcosc32kCtrl {
        &self.rcosc32k_ctrl
    }
    #[doc = "0x228 - ANA_HSI2DIGCLK_GATE"]
    #[inline(always)]
    pub const fn ana_hsi2digclk_gate(&self) -> &AnaHsi2digclkGate {
        &self.ana_hsi2digclk_gate
    }
    #[doc = "0x22c - DSI_PHY_PWRCTRL"]
    #[inline(always)]
    pub const fn dsi_phy_pwrctrl(&self) -> &DsiPhyPwrctrl {
        &self.dsi_phy_pwrctrl
    }
    #[doc = "0x230 - RS232_SOP11_BITINTERVAL"]
    #[inline(always)]
    pub const fn rs232_sop11_bitinterval(&self) -> &Rs232Sop11Bitinterval {
        &self.rs232_sop11_bitinterval
    }
    #[doc = "0x400 - PLL_CORE_PWRCTRL"]
    #[inline(always)]
    pub const fn pll_core_pwrctrl(&self) -> &PllCorePwrctrl {
        &self.pll_core_pwrctrl
    }
    #[doc = "0x404 - PLL_CORE_CLKCTRL"]
    #[inline(always)]
    pub const fn pll_core_clkctrl(&self) -> &PllCoreClkctrl {
        &self.pll_core_clkctrl
    }
    #[doc = "0x408 - PLL_CORE_TENABLE"]
    #[inline(always)]
    pub const fn pll_core_tenable(&self) -> &PllCoreTenable {
        &self.pll_core_tenable
    }
    #[doc = "0x40c - PLL_CORE_TENABLEDIV"]
    #[inline(always)]
    pub const fn pll_core_tenablediv(&self) -> &PllCoreTenablediv {
        &self.pll_core_tenablediv
    }
    #[doc = "0x410 - PLL_CORE_M2NDIV"]
    #[inline(always)]
    pub const fn pll_core_m2ndiv(&self) -> &PllCoreM2ndiv {
        &self.pll_core_m2ndiv
    }
    #[doc = "0x414 - PLL_CORE_MN2DIV"]
    #[inline(always)]
    pub const fn pll_core_mn2div(&self) -> &PllCoreMn2div {
        &self.pll_core_mn2div
    }
    #[doc = "0x418 - PLL_CORE_FRACDIV"]
    #[inline(always)]
    pub const fn pll_core_fracdiv(&self) -> &PllCoreFracdiv {
        &self.pll_core_fracdiv
    }
    #[doc = "0x41c - PLL_CORE_BWCTRL"]
    #[inline(always)]
    pub const fn pll_core_bwctrl(&self) -> &PllCoreBwctrl {
        &self.pll_core_bwctrl
    }
    #[doc = "0x420 - PLL_CORE_FRACCTRL"]
    #[inline(always)]
    pub const fn pll_core_fracctrl(&self) -> &PllCoreFracctrl {
        &self.pll_core_fracctrl
    }
    #[doc = "0x424 - PLL_CORE_STATUS"]
    #[inline(always)]
    pub const fn pll_core_status(&self) -> &PllCoreStatus {
        &self.pll_core_status
    }
    #[doc = "0x428 - PLL_CORE_HSDIVIDER"]
    #[inline(always)]
    pub const fn pll_core_hsdivider(&self) -> &PllCoreHsdivider {
        &self.pll_core_hsdivider
    }
    #[doc = "0x42c - PLL_CORE_HSDIVIDER_CLKOUT0"]
    #[inline(always)]
    pub const fn pll_core_hsdivider_clkout0(&self) -> &PllCoreHsdividerClkout0 {
        &self.pll_core_hsdivider_clkout0
    }
    #[doc = "0x430 - PLL_CORE_HSDIVIDER_CLKOUT1"]
    #[inline(always)]
    pub const fn pll_core_hsdivider_clkout1(&self) -> &PllCoreHsdividerClkout1 {
        &self.pll_core_hsdivider_clkout1
    }
    #[doc = "0x434 - PLL_CORE_HSDIVIDER_CLKOUT2"]
    #[inline(always)]
    pub const fn pll_core_hsdivider_clkout2(&self) -> &PllCoreHsdividerClkout2 {
        &self.pll_core_hsdivider_clkout2
    }
    #[doc = "0x438 - PLL_CORE_HSDIVIDER_CLKOUT3"]
    #[inline(always)]
    pub const fn pll_core_hsdivider_clkout3(&self) -> &PllCoreHsdividerClkout3 {
        &self.pll_core_hsdivider_clkout3
    }
    #[doc = "0x43c - MSS_CR5_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn mss_cr5_clk_src_sel(&self) -> &MssCr5ClkSrcSel {
        &self.mss_cr5_clk_src_sel
    }
    #[doc = "0x440 - MSS_CR5_DIV_VAL"]
    #[inline(always)]
    pub const fn mss_cr5_div_val(&self) -> &MssCr5DivVal {
        &self.mss_cr5_div_val
    }
    #[doc = "0x444 - SYS_CLK_DIV_VAL"]
    #[inline(always)]
    pub const fn sys_clk_div_val(&self) -> &SysClkDivVal {
        &self.sys_clk_div_val
    }
    #[doc = "0x448 - MSS_CR5_CLK_GATE"]
    #[inline(always)]
    pub const fn mss_cr5_clk_gate(&self) -> &MssCr5ClkGate {
        &self.mss_cr5_clk_gate
    }
    #[doc = "0x44c - SYS_CLK_GATE"]
    #[inline(always)]
    pub const fn sys_clk_gate(&self) -> &SysClkGate {
        &self.sys_clk_gate
    }
    #[doc = "0x450 - SYS_CLK_STATUS"]
    #[inline(always)]
    pub const fn sys_clk_status(&self) -> &SysClkStatus {
        &self.sys_clk_status
    }
    #[doc = "0x454 - MSS_CR5_CLK_STATUS"]
    #[inline(always)]
    pub const fn mss_cr5_clk_status(&self) -> &MssCr5ClkStatus {
        &self.mss_cr5_clk_status
    }
    #[doc = "0x458 - PLL_CORE_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_core_rstctrl(&self) -> &PllCoreRstctrl {
        &self.pll_core_rstctrl
    }
    #[doc = "0x45c - PLL_CORE_HSDIVIDER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_core_hsdivider_rstctrl(&self) -> &PllCoreHsdividerRstctrl {
        &self.pll_core_hsdivider_rstctrl
    }
    #[doc = "0x460 - RSS_CLK_SRC_SEL"]
    #[inline(always)]
    pub const fn rss_clk_src_sel(&self) -> &RssClkSrcSel {
        &self.rss_clk_src_sel
    }
    #[doc = "0x464 - PLLC_CLK2_SRC_SEL"]
    #[inline(always)]
    pub const fn pllc_clk2_src_sel(&self) -> &PllcClk2SrcSel {
        &self.pllc_clk2_src_sel
    }
    #[doc = "0x468 - PLLD_CLK1_SRC_SEL"]
    #[inline(always)]
    pub const fn plld_clk1_src_sel(&self) -> &PlldClk1SrcSel {
        &self.plld_clk1_src_sel
    }
    #[doc = "0x46c - PLLD_CLK2_SRC_SEL"]
    #[inline(always)]
    pub const fn plld_clk2_src_sel(&self) -> &PlldClk2SrcSel {
        &self.plld_clk2_src_sel
    }
    #[doc = "0x470 - PLLP_CLK1_SRC_SEL"]
    #[inline(always)]
    pub const fn pllp_clk1_src_sel(&self) -> &PllpClk1SrcSel {
        &self.pllp_clk1_src_sel
    }
    #[doc = "0x474 - RSS_DIV_VAL"]
    #[inline(always)]
    pub const fn rss_div_val(&self) -> &RssDivVal {
        &self.rss_div_val
    }
    #[doc = "0x478 - RSS_CLK_GATE"]
    #[inline(always)]
    pub const fn rss_clk_gate(&self) -> &RssClkGate {
        &self.rss_clk_gate
    }
    #[doc = "0x47c - PLLC_CLK2_GATE"]
    #[inline(always)]
    pub const fn pllc_clk2_gate(&self) -> &PllcClk2Gate {
        &self.pllc_clk2_gate
    }
    #[doc = "0x480 - PLLD_CLK1_GATE"]
    #[inline(always)]
    pub const fn plld_clk1_gate(&self) -> &PlldClk1Gate {
        &self.plld_clk1_gate
    }
    #[doc = "0x484 - PLLD_CLK2_GATE"]
    #[inline(always)]
    pub const fn plld_clk2_gate(&self) -> &PlldClk2Gate {
        &self.plld_clk2_gate
    }
    #[doc = "0x488 - PLLP_CLK1_GATE"]
    #[inline(always)]
    pub const fn pllp_clk1_gate(&self) -> &PllpClk1Gate {
        &self.pllp_clk1_gate
    }
    #[doc = "0x48c - RSS_CLK_STATUS"]
    #[inline(always)]
    pub const fn rss_clk_status(&self) -> &RssClkStatus {
        &self.rss_clk_status
    }
    #[doc = "0x490 - PLLC_CLK2_STATUS"]
    #[inline(always)]
    pub const fn pllc_clk2_status(&self) -> &PllcClk2Status {
        &self.pllc_clk2_status
    }
    #[doc = "0x494 - PLLD_CLK1_STATUS"]
    #[inline(always)]
    pub const fn plld_clk1_status(&self) -> &PlldClk1Status {
        &self.plld_clk1_status
    }
    #[doc = "0x49c - PLLP_CLK1_STATUS"]
    #[inline(always)]
    pub const fn pllp_clk1_status(&self) -> &PllpClk1Status {
        &self.pllp_clk1_status
    }
    #[doc = "0x4a0 - PLL_1P2_HSDIVIDER"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider(&self) -> &Pll1p2Hsdivider {
        &self.pll_1p2_hsdivider
    }
    #[doc = "0x4a4 - PLL_1P2_HSDIVIDER_CLKOUT0"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider_clkout0(&self) -> &Pll1p2HsdividerClkout0 {
        &self.pll_1p2_hsdivider_clkout0
    }
    #[doc = "0x4a8 - PLL_1P2_HSDIVIDER_CLKOUT1"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider_clkout1(&self) -> &Pll1p2HsdividerClkout1 {
        &self.pll_1p2_hsdivider_clkout1
    }
    #[doc = "0x4ac - PLL_1P2_HSDIVIDER_CLKOUT2"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider_clkout2(&self) -> &Pll1p2HsdividerClkout2 {
        &self.pll_1p2_hsdivider_clkout2
    }
    #[doc = "0x4b0 - PLL_1P2_HSDIVIDER_CLKOUT3"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider_clkout3(&self) -> &Pll1p2HsdividerClkout3 {
        &self.pll_1p2_hsdivider_clkout3
    }
    #[doc = "0x4b4 - PLL_1P2_HSDIVIDER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_1p2_hsdivider_rstctrl(&self) -> &Pll1p2HsdividerRstctrl {
        &self.pll_1p2_hsdivider_rstctrl
    }
    #[doc = "0x4b8 - PLL_1P8_HSDIVIDER"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider(&self) -> &Pll1p8Hsdivider {
        &self.pll_1p8_hsdivider
    }
    #[doc = "0x4bc - PLL_1P8_HSDIVIDER_CLKOUT0"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider_clkout0(&self) -> &Pll1p8HsdividerClkout0 {
        &self.pll_1p8_hsdivider_clkout0
    }
    #[doc = "0x4c0 - PLL_1P8_HSDIVIDER_CLKOUT1"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider_clkout1(&self) -> &Pll1p8HsdividerClkout1 {
        &self.pll_1p8_hsdivider_clkout1
    }
    #[doc = "0x4c4 - PLL_1P8_HSDIVIDER_CLKOUT2"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider_clkout2(&self) -> &Pll1p8HsdividerClkout2 {
        &self.pll_1p8_hsdivider_clkout2
    }
    #[doc = "0x4c8 - PLL_1P8_HSDIVIDER_CLKOUT3"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider_clkout3(&self) -> &Pll1p8HsdividerClkout3 {
        &self.pll_1p8_hsdivider_clkout3
    }
    #[doc = "0x4cc - PLL_1P8_HSDIVIDER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_1p8_hsdivider_rstctrl(&self) -> &Pll1p8HsdividerRstctrl {
        &self.pll_1p8_hsdivider_rstctrl
    }
    #[doc = "0x800 - PLL_DSP_PWRCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_pwrctrl(&self) -> &PllDspPwrctrl {
        &self.pll_dsp_pwrctrl
    }
    #[doc = "0x804 - PLL_DSP_CLKCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_clkctrl(&self) -> &PllDspClkctrl {
        &self.pll_dsp_clkctrl
    }
    #[doc = "0x808 - PLL_DSP_TENABLE"]
    #[inline(always)]
    pub const fn pll_dsp_tenable(&self) -> &PllDspTenable {
        &self.pll_dsp_tenable
    }
    #[doc = "0x80c - PLL_DSP_TENABLEDIV"]
    #[inline(always)]
    pub const fn pll_dsp_tenablediv(&self) -> &PllDspTenablediv {
        &self.pll_dsp_tenablediv
    }
    #[doc = "0x810 - PLL_DSP_M2NDIV"]
    #[inline(always)]
    pub const fn pll_dsp_m2ndiv(&self) -> &PllDspM2ndiv {
        &self.pll_dsp_m2ndiv
    }
    #[doc = "0x814 - PLL_DSP_MN2DIV"]
    #[inline(always)]
    pub const fn pll_dsp_mn2div(&self) -> &PllDspMn2div {
        &self.pll_dsp_mn2div
    }
    #[doc = "0x818 - PLL_DSP_FRACDIV"]
    #[inline(always)]
    pub const fn pll_dsp_fracdiv(&self) -> &PllDspFracdiv {
        &self.pll_dsp_fracdiv
    }
    #[doc = "0x81c - PLL_DSP_BWCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_bwctrl(&self) -> &PllDspBwctrl {
        &self.pll_dsp_bwctrl
    }
    #[doc = "0x820 - PLL_DSP_FRACCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_fracctrl(&self) -> &PllDspFracctrl {
        &self.pll_dsp_fracctrl
    }
    #[doc = "0x828 - PLL_DSP_HSDIVIDER"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider(&self) -> &PllDspHsdivider {
        &self.pll_dsp_hsdivider
    }
    #[doc = "0x82c - PLL_DSP_HSDIVIDER_CLKOUT0"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider_clkout0(&self) -> &PllDspHsdividerClkout0 {
        &self.pll_dsp_hsdivider_clkout0
    }
    #[doc = "0x830 - PLL_DSP_HSDIVIDER_CLKOUT1"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider_clkout1(&self) -> &PllDspHsdividerClkout1 {
        &self.pll_dsp_hsdivider_clkout1
    }
    #[doc = "0x834 - PLL_DSP_HSDIVIDER_CLKOUT2"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider_clkout2(&self) -> &PllDspHsdividerClkout2 {
        &self.pll_dsp_hsdivider_clkout2
    }
    #[doc = "0x838 - PLL_DSP_HSDIVIDER_CLKOUT3"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider_clkout3(&self) -> &PllDspHsdividerClkout3 {
        &self.pll_dsp_hsdivider_clkout3
    }
    #[doc = "0x83c - PLL_PER_PWRCTRL"]
    #[inline(always)]
    pub const fn pll_per_pwrctrl(&self) -> &PllPerPwrctrl {
        &self.pll_per_pwrctrl
    }
    #[doc = "0x840 - PLL_PER_CLKCTRL"]
    #[inline(always)]
    pub const fn pll_per_clkctrl(&self) -> &PllPerClkctrl {
        &self.pll_per_clkctrl
    }
    #[doc = "0x844 - PLL_PER_TENABLE"]
    #[inline(always)]
    pub const fn pll_per_tenable(&self) -> &PllPerTenable {
        &self.pll_per_tenable
    }
    #[doc = "0x848 - PLL_PER_TENABLEDIV"]
    #[inline(always)]
    pub const fn pll_per_tenablediv(&self) -> &PllPerTenablediv {
        &self.pll_per_tenablediv
    }
    #[doc = "0x84c - PLL_PER_M2NDIV"]
    #[inline(always)]
    pub const fn pll_per_m2ndiv(&self) -> &PllPerM2ndiv {
        &self.pll_per_m2ndiv
    }
    #[doc = "0x850 - PLL_PER_MN2DIV"]
    #[inline(always)]
    pub const fn pll_per_mn2div(&self) -> &PllPerMn2div {
        &self.pll_per_mn2div
    }
    #[doc = "0x854 - PLL_PER_FRACDIV"]
    #[inline(always)]
    pub const fn pll_per_fracdiv(&self) -> &PllPerFracdiv {
        &self.pll_per_fracdiv
    }
    #[doc = "0x858 - PLL_PER_BWCTRL"]
    #[inline(always)]
    pub const fn pll_per_bwctrl(&self) -> &PllPerBwctrl {
        &self.pll_per_bwctrl
    }
    #[doc = "0x85c - PLL_PER_FRACCTRL"]
    #[inline(always)]
    pub const fn pll_per_fracctrl(&self) -> &PllPerFracctrl {
        &self.pll_per_fracctrl
    }
    #[doc = "0x860 - PLL_PER_STATUS"]
    #[inline(always)]
    pub const fn pll_per_status(&self) -> &PllPerStatus {
        &self.pll_per_status
    }
    #[doc = "0x864 - PLL_PER_HSDIVIDER"]
    #[inline(always)]
    pub const fn pll_per_hsdivider(&self) -> &PllPerHsdivider {
        &self.pll_per_hsdivider
    }
    #[doc = "0x868 - PLL_PER_HSDIVIDER_CLKOUT0"]
    #[inline(always)]
    pub const fn pll_per_hsdivider_clkout0(&self) -> &PllPerHsdividerClkout0 {
        &self.pll_per_hsdivider_clkout0
    }
    #[doc = "0x86c - PLL_PER_HSDIVIDER_CLKOUT1"]
    #[inline(always)]
    pub const fn pll_per_hsdivider_clkout1(&self) -> &PllPerHsdividerClkout1 {
        &self.pll_per_hsdivider_clkout1
    }
    #[doc = "0x870 - PLL_PER_HSDIVIDER_CLKOUT2"]
    #[inline(always)]
    pub const fn pll_per_hsdivider_clkout2(&self) -> &PllPerHsdividerClkout2 {
        &self.pll_per_hsdivider_clkout2
    }
    #[doc = "0x874 - PLL_PER_HSDIVIDER_CLKOUT3"]
    #[inline(always)]
    pub const fn pll_per_hsdivider_clkout3(&self) -> &PllPerHsdividerClkout3 {
        &self.pll_per_hsdivider_clkout3
    }
    #[doc = "0x878 - PLL_DSP_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_rstctrl(&self) -> &PllDspRstctrl {
        &self.pll_dsp_rstctrl
    }
    #[doc = "0x87c - PLL_DSP_HSDIVIDER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_dsp_hsdivider_rstctrl(&self) -> &PllDspHsdividerRstctrl {
        &self.pll_dsp_hsdivider_rstctrl
    }
    #[doc = "0x880 - PLL_PER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_per_rstctrl(&self) -> &PllPerRstctrl {
        &self.pll_per_rstctrl
    }
    #[doc = "0x884 - PLL_PER_HSDIVIDER_RSTCTRL"]
    #[inline(always)]
    pub const fn pll_per_hsdivider_rstctrl(&self) -> &PllPerHsdividerRstctrl {
        &self.pll_per_hsdivider_rstctrl
    }
    #[doc = "0xc00 - ANA_REG_CLK_CTRL_REG1_XO_SLICER"]
    #[inline(always)]
    pub const fn ana_reg_clk_ctrl_reg1_xo_slicer(&self) -> &AnaRegClkCtrlReg1XoSlicer {
        &self.ana_reg_clk_ctrl_reg1_xo_slicer
    }
    #[doc = "0xc04 - ANA_REG_CLK_CTRL_REG1_CLKTOP"]
    #[inline(always)]
    pub const fn ana_reg_clk_ctrl_reg1_clktop(&self) -> &AnaRegClkCtrlReg1Clktop {
        &self.ana_reg_clk_ctrl_reg1_clktop
    }
    #[doc = "0xc08 - ANA_REG_CLK_CTRL_REG2_CLKTOP"]
    #[inline(always)]
    pub const fn ana_reg_clk_ctrl_reg2_clktop(&self) -> &AnaRegClkCtrlReg2Clktop {
        &self.ana_reg_clk_ctrl_reg2_clktop
    }
    #[doc = "0xc0c - ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP"]
    #[inline(always)]
    pub const fn ana_reg_clk_ctrl_reg1_ldo_clktop(&self) -> &AnaRegClkCtrlReg1LdoClktop {
        &self.ana_reg_clk_ctrl_reg1_ldo_clktop
    }
    #[doc = "0xc10 - ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP"]
    #[inline(always)]
    pub const fn ana_reg_clk_ctrl_reg2_ldo_clktop(&self) -> &AnaRegClkCtrlReg2LdoClktop {
        &self.ana_reg_clk_ctrl_reg2_ldo_clktop
    }
    #[doc = "0xc18 - ANA_REG_CLK_STATUS_REG"]
    #[inline(always)]
    pub const fn ana_reg_clk_status_reg(&self) -> &AnaRegClkStatusReg {
        &self.ana_reg_clk_status_reg
    }
    #[doc = "0xc1c - ANA_REG_REFSYS_CTRL_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_refsys_ctrl_reg_lowv(&self) -> &AnaRegRefsysCtrlRegLowv {
        &self.ana_reg_refsys_ctrl_reg_lowv
    }
    #[doc = "0xc20 - ANA_REG_REFSYS_TMUX_CTRL_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_refsys_tmux_ctrl_lowv(&self) -> &AnaRegRefsysTmuxCtrlLowv {
        &self.ana_reg_refsys_tmux_ctrl_lowv
    }
    #[doc = "0xc24 - ANA_REG_REFSYS_SPARE_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_refsys_spare_reg_lowv(&self) -> &AnaRegRefsysSpareRegLowv {
        &self.ana_reg_refsys_spare_reg_lowv
    }
    #[doc = "0xc28 - ANA_REG_WU_CTRL_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_wu_ctrl_reg_lowv(&self) -> &AnaRegWuCtrlRegLowv {
        &self.ana_reg_wu_ctrl_reg_lowv
    }
    #[doc = "0xc2c - ANA_REG_WU_TMUX_CTRL_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_wu_tmux_ctrl_lowv(&self) -> &AnaRegWuTmuxCtrlLowv {
        &self.ana_reg_wu_tmux_ctrl_lowv
    }
    #[doc = "0xc30 - ANA_REG_TW_CTRL_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_tw_ctrl_reg_lowv(&self) -> &AnaRegTwCtrlRegLowv {
        &self.ana_reg_tw_ctrl_reg_lowv
    }
    #[doc = "0xc34 - ANA_REG_TW_ANA_TMUX_CTRL_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_tw_ana_tmux_ctrl_lowv(&self) -> &AnaRegTwAnaTmuxCtrlLowv {
        &self.ana_reg_tw_ana_tmux_ctrl_lowv
    }
    #[doc = "0xc3c - ANA_REG_WU_MODE_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_wu_mode_reg_lowv(&self) -> &AnaRegWuModeRegLowv {
        &self.ana_reg_wu_mode_reg_lowv
    }
    #[doc = "0xc40 - ANA_REG_WU_STATUS_REG_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_wu_status_reg_lowv(&self) -> &AnaRegWuStatusRegLowv {
        &self.ana_reg_wu_status_reg_lowv
    }
    #[doc = "0xc44 - ANA_REG_WU_SPARE_OUT_LOWV"]
    #[inline(always)]
    pub const fn ana_reg_wu_spare_out_lowv(&self) -> &AnaRegWuSpareOutLowv {
        &self.ana_reg_wu_spare_out_lowv
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
#[doc = "HSI_CLK_SRC_SEL (rw) register accessor: HSI_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsi_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsi_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsi_clk_src_sel`]
module"]
#[doc(alias = "HSI_CLK_SRC_SEL")]
pub type HsiClkSrcSel = crate::Reg<hsi_clk_src_sel::HsiClkSrcSelSpec>;
#[doc = "HSI_CLK_SRC_SEL"]
pub mod hsi_clk_src_sel;
#[doc = "CSIRX_CLK_SRC_SEL (rw) register accessor: CSIRX_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`csirx_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csirx_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csirx_clk_src_sel`]
module"]
#[doc(alias = "CSIRX_CLK_SRC_SEL")]
pub type CsirxClkSrcSel = crate::Reg<csirx_clk_src_sel::CsirxClkSrcSelSpec>;
#[doc = "CSIRX_CLK_SRC_SEL"]
pub mod csirx_clk_src_sel;
#[doc = "MCUCLKOUT_CLK_SRC_SEL (rw) register accessor: MCUCLKOUT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuclkout_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuclkout_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuclkout_clk_src_sel`]
module"]
#[doc(alias = "MCUCLKOUT_CLK_SRC_SEL")]
pub type McuclkoutClkSrcSel = crate::Reg<mcuclkout_clk_src_sel::McuclkoutClkSrcSelSpec>;
#[doc = "MCUCLKOUT_CLK_SRC_SEL"]
pub mod mcuclkout_clk_src_sel;
#[doc = "PMICCLKOUT_CLK_SRC_SEL (rw) register accessor: PMICCLKOUT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_clk_src_sel`]
module"]
#[doc(alias = "PMICCLKOUT_CLK_SRC_SEL")]
pub type PmicclkoutClkSrcSel = crate::Reg<pmicclkout_clk_src_sel::PmicclkoutClkSrcSelSpec>;
#[doc = "PMICCLKOUT_CLK_SRC_SEL"]
pub mod pmicclkout_clk_src_sel;
#[doc = "OBSCLKOUT_CLK_SRC_SEL (rw) register accessor: OBSCLKOUT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclkout_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclkout_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsclkout_clk_src_sel`]
module"]
#[doc(alias = "OBSCLKOUT_CLK_SRC_SEL")]
pub type ObsclkoutClkSrcSel = crate::Reg<obsclkout_clk_src_sel::ObsclkoutClkSrcSelSpec>;
#[doc = "OBSCLKOUT_CLK_SRC_SEL"]
pub mod obsclkout_clk_src_sel;
#[doc = "TRCCLKOUT_CLK_SRC_SEL (rw) register accessor: TRCCLKOUT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclkout_clk_src_sel`]
module"]
#[doc(alias = "TRCCLKOUT_CLK_SRC_SEL")]
pub type TrcclkoutClkSrcSel = crate::Reg<trcclkout_clk_src_sel::TrcclkoutClkSrcSelSpec>;
#[doc = "TRCCLKOUT_CLK_SRC_SEL"]
pub mod trcclkout_clk_src_sel;
#[doc = "CSIRX_DIV_VAL (rw) register accessor: CSIRX_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`csirx_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csirx_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csirx_div_val`]
module"]
#[doc(alias = "CSIRX_DIV_VAL")]
pub type CsirxDivVal = crate::Reg<csirx_div_val::CsirxDivValSpec>;
#[doc = "CSIRX_DIV_VAL"]
pub mod csirx_div_val;
#[doc = "MCUCLKOUT_DIV_VAL (rw) register accessor: MCUCLKOUT_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuclkout_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuclkout_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuclkout_div_val`]
module"]
#[doc(alias = "MCUCLKOUT_DIV_VAL")]
pub type McuclkoutDivVal = crate::Reg<mcuclkout_div_val::McuclkoutDivValSpec>;
#[doc = "MCUCLKOUT_DIV_VAL"]
pub mod mcuclkout_div_val;
#[doc = "PMICCLKOUT_DIV_VAL (rw) register accessor: PMICCLKOUT_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_div_val`]
module"]
#[doc(alias = "PMICCLKOUT_DIV_VAL")]
pub type PmicclkoutDivVal = crate::Reg<pmicclkout_div_val::PmicclkoutDivValSpec>;
#[doc = "PMICCLKOUT_DIV_VAL"]
pub mod pmicclkout_div_val;
#[doc = "OBSCLKOUT_DIV_VAL (rw) register accessor: OBSCLKOUT_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclkout_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclkout_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsclkout_div_val`]
module"]
#[doc(alias = "OBSCLKOUT_DIV_VAL")]
pub type ObsclkoutDivVal = crate::Reg<obsclkout_div_val::ObsclkoutDivValSpec>;
#[doc = "OBSCLKOUT_DIV_VAL"]
pub mod obsclkout_div_val;
#[doc = "TRCCLKOUT_DIV_VAL (rw) register accessor: TRCCLKOUT_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclkout_div_val`]
module"]
#[doc(alias = "TRCCLKOUT_DIV_VAL")]
pub type TrcclkoutDivVal = crate::Reg<trcclkout_div_val::TrcclkoutDivValSpec>;
#[doc = "TRCCLKOUT_DIV_VAL"]
pub mod trcclkout_div_val;
#[doc = "CSIRX_CLK_GATE (rw) register accessor: CSIRX_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`csirx_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csirx_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csirx_clk_gate`]
module"]
#[doc(alias = "CSIRX_CLK_GATE")]
pub type CsirxClkGate = crate::Reg<csirx_clk_gate::CsirxClkGateSpec>;
#[doc = "CSIRX_CLK_GATE"]
pub mod csirx_clk_gate;
#[doc = "MCUCLKOUT_CLK_GATE (rw) register accessor: MCUCLKOUT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuclkout_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuclkout_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuclkout_clk_gate`]
module"]
#[doc(alias = "MCUCLKOUT_CLK_GATE")]
pub type McuclkoutClkGate = crate::Reg<mcuclkout_clk_gate::McuclkoutClkGateSpec>;
#[doc = "MCUCLKOUT_CLK_GATE"]
pub mod mcuclkout_clk_gate;
#[doc = "PMICCLKOUT_CLK_GATE (rw) register accessor: PMICCLKOUT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_clk_gate`]
module"]
#[doc(alias = "PMICCLKOUT_CLK_GATE")]
pub type PmicclkoutClkGate = crate::Reg<pmicclkout_clk_gate::PmicclkoutClkGateSpec>;
#[doc = "PMICCLKOUT_CLK_GATE"]
pub mod pmicclkout_clk_gate;
#[doc = "OBSCLKOUT_CLK_GATE (rw) register accessor: OBSCLKOUT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclkout_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclkout_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsclkout_clk_gate`]
module"]
#[doc(alias = "OBSCLKOUT_CLK_GATE")]
pub type ObsclkoutClkGate = crate::Reg<obsclkout_clk_gate::ObsclkoutClkGateSpec>;
#[doc = "OBSCLKOUT_CLK_GATE"]
pub mod obsclkout_clk_gate;
#[doc = "TRCCLKOUT_CLK_GATE (rw) register accessor: TRCCLKOUT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclkout_clk_gate`]
module"]
#[doc(alias = "TRCCLKOUT_CLK_GATE")]
pub type TrcclkoutClkGate = crate::Reg<trcclkout_clk_gate::TrcclkoutClkGateSpec>;
#[doc = "TRCCLKOUT_CLK_GATE"]
pub mod trcclkout_clk_gate;
#[doc = "DSS_CLK_GATE (rw) register accessor: DSS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_clk_gate`]
module"]
#[doc(alias = "DSS_CLK_GATE")]
pub type DssClkGate = crate::Reg<dss_clk_gate::DssClkGateSpec>;
#[doc = "DSS_CLK_GATE"]
pub mod dss_clk_gate;
#[doc = "HSI_CLK_STATUS (rw) register accessor: HSI_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hsi_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsi_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsi_clk_status`]
module"]
#[doc(alias = "HSI_CLK_STATUS")]
pub type HsiClkStatus = crate::Reg<hsi_clk_status::HsiClkStatusSpec>;
#[doc = "HSI_CLK_STATUS"]
pub mod hsi_clk_status;
#[doc = "MCUCLKOUT_CLK_STATUS (rw) register accessor: MCUCLKOUT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuclkout_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuclkout_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuclkout_clk_status`]
module"]
#[doc(alias = "MCUCLKOUT_CLK_STATUS")]
pub type McuclkoutClkStatus = crate::Reg<mcuclkout_clk_status::McuclkoutClkStatusSpec>;
#[doc = "MCUCLKOUT_CLK_STATUS"]
pub mod mcuclkout_clk_status;
#[doc = "PMICCLKOUT_CLK_STATUS (rw) register accessor: PMICCLKOUT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_clk_status`]
module"]
#[doc(alias = "PMICCLKOUT_CLK_STATUS")]
pub type PmicclkoutClkStatus = crate::Reg<pmicclkout_clk_status::PmicclkoutClkStatusSpec>;
#[doc = "PMICCLKOUT_CLK_STATUS"]
pub mod pmicclkout_clk_status;
#[doc = "OBSCLKOUT_CLK_STATUS (rw) register accessor: OBSCLKOUT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclkout_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclkout_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsclkout_clk_status`]
module"]
#[doc(alias = "OBSCLKOUT_CLK_STATUS")]
pub type ObsclkoutClkStatus = crate::Reg<obsclkout_clk_status::ObsclkoutClkStatusSpec>;
#[doc = "OBSCLKOUT_CLK_STATUS"]
pub mod obsclkout_clk_status;
#[doc = "TRCCLKOUT_CLK_STATUS (rw) register accessor: TRCCLKOUT_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclkout_clk_status`]
module"]
#[doc(alias = "TRCCLKOUT_CLK_STATUS")]
pub type TrcclkoutClkStatus = crate::Reg<trcclkout_clk_status::TrcclkoutClkStatusSpec>;
#[doc = "TRCCLKOUT_CLK_STATUS"]
pub mod trcclkout_clk_status;
#[doc = "WARM_RESET_CONFIG (rw) register accessor: WARM_RESET_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`warm_reset_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warm_reset_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@warm_reset_config`]
module"]
#[doc(alias = "WARM_RESET_CONFIG")]
pub type WarmResetConfig = crate::Reg<warm_reset_config::WarmResetConfigSpec>;
#[doc = "WARM_RESET_CONFIG"]
pub mod warm_reset_config;
#[doc = "SYS_RST_CAUSE (rw) register accessor: SYS_RST_CAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_rst_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_rst_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_rst_cause`]
module"]
#[doc(alias = "SYS_RST_CAUSE")]
pub type SysRstCause = crate::Reg<sys_rst_cause::SysRstCauseSpec>;
#[doc = "SYS_RST_CAUSE"]
pub mod sys_rst_cause;
#[doc = "SYS_RST_CAUSE_CLR (rw) register accessor: SYS_RST_CAUSE_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_rst_cause_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_rst_cause_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_rst_cause_clr`]
module"]
#[doc(alias = "SYS_RST_CAUSE_CLR")]
pub type SysRstCauseClr = crate::Reg<sys_rst_cause_clr::SysRstCauseClrSpec>;
#[doc = "SYS_RST_CAUSE_CLR"]
pub mod sys_rst_cause_clr;
#[doc = "DSS_RST_CTRL (rw) register accessor: DSS_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss_rst_ctrl`]
module"]
#[doc(alias = "DSS_RST_CTRL")]
pub type DssRstCtrl = crate::Reg<dss_rst_ctrl::DssRstCtrlSpec>;
#[doc = "DSS_RST_CTRL"]
pub mod dss_rst_ctrl;
#[doc = "RS232_BITINTERVAL (rw) register accessor: RS232_BITINTERVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_bitinterval`]
module"]
#[doc(alias = "RS232_BITINTERVAL")]
pub type Rs232Bitinterval = crate::Reg<rs232_bitinterval::Rs232BitintervalSpec>;
#[doc = "RS232_BITINTERVAL"]
pub mod rs232_bitinterval;
#[doc = "LVDS_PAD_CTRL0 (rw) register accessor: LVDS_PAD_CTRL0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvds_pad_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvds_pad_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvds_pad_ctrl0`]
module"]
#[doc(alias = "LVDS_PAD_CTRL0")]
pub type LvdsPadCtrl0 = crate::Reg<lvds_pad_ctrl0::LvdsPadCtrl0Spec>;
#[doc = "LVDS_PAD_CTRL0"]
pub mod lvds_pad_ctrl0;
#[doc = "LVDS_PAD_CTRL1 (rw) register accessor: LVDS_PAD_CTRL1\n\nYou can [`read`](crate::Reg::read) this register and get [`lvds_pad_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvds_pad_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvds_pad_ctrl1`]
module"]
#[doc(alias = "LVDS_PAD_CTRL1")]
pub type LvdsPadCtrl1 = crate::Reg<lvds_pad_ctrl1::LvdsPadCtrl1Spec>;
#[doc = "LVDS_PAD_CTRL1"]
pub mod lvds_pad_ctrl1;
#[doc = "DFT_DMLED_EXEC (rw) register accessor: DFT_DMLED_EXEC\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_dmled_exec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_dmled_exec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_dmled_exec`]
module"]
#[doc(alias = "DFT_DMLED_EXEC")]
pub type DftDmledExec = crate::Reg<dft_dmled_exec::DftDmledExecSpec>;
#[doc = "DFT_DMLED_EXEC"]
pub mod dft_dmled_exec;
#[doc = "DFT_DMLED_STATUS (rw) register accessor: DFT_DMLED_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_dmled_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_dmled_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_dmled_status`]
module"]
#[doc(alias = "DFT_DMLED_STATUS")]
pub type DftDmledStatus = crate::Reg<dft_dmled_status::DftDmledStatusSpec>;
#[doc = "DFT_DMLED_STATUS"]
pub mod dft_dmled_status;
#[doc = "LIMP_MODE_EN (rw) register accessor: LIMP_MODE_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`limp_mode_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limp_mode_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limp_mode_en`]
module"]
#[doc(alias = "LIMP_MODE_EN")]
pub type LimpModeEn = crate::Reg<limp_mode_en::LimpModeEnSpec>;
#[doc = "LIMP_MODE_EN"]
pub mod limp_mode_en;
#[doc = "PMICCLKOUT_DCDC_CTRL (rw) register accessor: PMICCLKOUT_DCDC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_dcdc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_dcdc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_dcdc_ctrl`]
module"]
#[doc(alias = "PMICCLKOUT_DCDC_CTRL")]
pub type PmicclkoutDcdcCtrl = crate::Reg<pmicclkout_dcdc_ctrl::PmicclkoutDcdcCtrlSpec>;
#[doc = "PMICCLKOUT_DCDC_CTRL"]
pub mod pmicclkout_dcdc_ctrl;
#[doc = "PMICCLKOUT_DCDC_SLOPE (rw) register accessor: PMICCLKOUT_DCDC_SLOPE\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_dcdc_slope::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_dcdc_slope::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicclkout_dcdc_slope`]
module"]
#[doc(alias = "PMICCLKOUT_DCDC_SLOPE")]
pub type PmicclkoutDcdcSlope = crate::Reg<pmicclkout_dcdc_slope::PmicclkoutDcdcSlopeSpec>;
#[doc = "PMICCLKOUT_DCDC_SLOPE"]
pub mod pmicclkout_dcdc_slope;
#[doc = "RCOSC32K_CTRL (rw) register accessor: RCOSC32K_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rcosc32k_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc32k_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcosc32k_ctrl`]
module"]
#[doc(alias = "RCOSC32K_CTRL")]
pub type Rcosc32kCtrl = crate::Reg<rcosc32k_ctrl::Rcosc32kCtrlSpec>;
#[doc = "RCOSC32K_CTRL"]
pub mod rcosc32k_ctrl;
#[doc = "ANA_HSI2DIGCLK_GATE (rw) register accessor: ANA_HSI2DIGCLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_hsi2digclk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_hsi2digclk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_hsi2digclk_gate`]
module"]
#[doc(alias = "ANA_HSI2DIGCLK_GATE")]
pub type AnaHsi2digclkGate = crate::Reg<ana_hsi2digclk_gate::AnaHsi2digclkGateSpec>;
#[doc = "ANA_HSI2DIGCLK_GATE"]
pub mod ana_hsi2digclk_gate;
#[doc = "DSI_PHY_PWRCTRL (rw) register accessor: DSI_PHY_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_phy_pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_phy_pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_phy_pwrctrl`]
module"]
#[doc(alias = "DSI_PHY_PWRCTRL")]
pub type DsiPhyPwrctrl = crate::Reg<dsi_phy_pwrctrl::DsiPhyPwrctrlSpec>;
#[doc = "DSI_PHY_PWRCTRL"]
pub mod dsi_phy_pwrctrl;
#[doc = "RS232_SOP11_BITINTERVAL (rw) register accessor: RS232_SOP11_BITINTERVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_sop11_bitinterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_sop11_bitinterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_sop11_bitinterval`]
module"]
#[doc(alias = "RS232_SOP11_BITINTERVAL")]
pub type Rs232Sop11Bitinterval = crate::Reg<rs232_sop11_bitinterval::Rs232Sop11BitintervalSpec>;
#[doc = "RS232_SOP11_BITINTERVAL"]
pub mod rs232_sop11_bitinterval;
#[doc = "PLL_CORE_PWRCTRL (rw) register accessor: PLL_CORE_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_pwrctrl`]
module"]
#[doc(alias = "PLL_CORE_PWRCTRL")]
pub type PllCorePwrctrl = crate::Reg<pll_core_pwrctrl::PllCorePwrctrlSpec>;
#[doc = "PLL_CORE_PWRCTRL"]
pub mod pll_core_pwrctrl;
#[doc = "PLL_CORE_CLKCTRL (rw) register accessor: PLL_CORE_CLKCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_clkctrl`]
module"]
#[doc(alias = "PLL_CORE_CLKCTRL")]
pub type PllCoreClkctrl = crate::Reg<pll_core_clkctrl::PllCoreClkctrlSpec>;
#[doc = "PLL_CORE_CLKCTRL"]
pub mod pll_core_clkctrl;
#[doc = "PLL_CORE_TENABLE (rw) register accessor: PLL_CORE_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_tenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_tenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_tenable`]
module"]
#[doc(alias = "PLL_CORE_TENABLE")]
pub type PllCoreTenable = crate::Reg<pll_core_tenable::PllCoreTenableSpec>;
#[doc = "PLL_CORE_TENABLE"]
pub mod pll_core_tenable;
#[doc = "PLL_CORE_TENABLEDIV (rw) register accessor: PLL_CORE_TENABLEDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_tenablediv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_tenablediv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_tenablediv`]
module"]
#[doc(alias = "PLL_CORE_TENABLEDIV")]
pub type PllCoreTenablediv = crate::Reg<pll_core_tenablediv::PllCoreTenabledivSpec>;
#[doc = "PLL_CORE_TENABLEDIV"]
pub mod pll_core_tenablediv;
#[doc = "PLL_CORE_M2NDIV (rw) register accessor: PLL_CORE_M2NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_m2ndiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_m2ndiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_m2ndiv`]
module"]
#[doc(alias = "PLL_CORE_M2NDIV")]
pub type PllCoreM2ndiv = crate::Reg<pll_core_m2ndiv::PllCoreM2ndivSpec>;
#[doc = "PLL_CORE_M2NDIV"]
pub mod pll_core_m2ndiv;
#[doc = "PLL_CORE_MN2DIV (rw) register accessor: PLL_CORE_MN2DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_mn2div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_mn2div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_mn2div`]
module"]
#[doc(alias = "PLL_CORE_MN2DIV")]
pub type PllCoreMn2div = crate::Reg<pll_core_mn2div::PllCoreMn2divSpec>;
#[doc = "PLL_CORE_MN2DIV"]
pub mod pll_core_mn2div;
#[doc = "PLL_CORE_FRACDIV (rw) register accessor: PLL_CORE_FRACDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_fracdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_fracdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_fracdiv`]
module"]
#[doc(alias = "PLL_CORE_FRACDIV")]
pub type PllCoreFracdiv = crate::Reg<pll_core_fracdiv::PllCoreFracdivSpec>;
#[doc = "PLL_CORE_FRACDIV"]
pub mod pll_core_fracdiv;
#[doc = "PLL_CORE_BWCTRL (rw) register accessor: PLL_CORE_BWCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_bwctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_bwctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_bwctrl`]
module"]
#[doc(alias = "PLL_CORE_BWCTRL")]
pub type PllCoreBwctrl = crate::Reg<pll_core_bwctrl::PllCoreBwctrlSpec>;
#[doc = "PLL_CORE_BWCTRL"]
pub mod pll_core_bwctrl;
#[doc = "PLL_CORE_FRACCTRL (rw) register accessor: PLL_CORE_FRACCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_fracctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_fracctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_fracctrl`]
module"]
#[doc(alias = "PLL_CORE_FRACCTRL")]
pub type PllCoreFracctrl = crate::Reg<pll_core_fracctrl::PllCoreFracctrlSpec>;
#[doc = "PLL_CORE_FRACCTRL"]
pub mod pll_core_fracctrl;
#[doc = "PLL_CORE_STATUS (rw) register accessor: PLL_CORE_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_status`]
module"]
#[doc(alias = "PLL_CORE_STATUS")]
pub type PllCoreStatus = crate::Reg<pll_core_status::PllCoreStatusSpec>;
#[doc = "PLL_CORE_STATUS"]
pub mod pll_core_status;
#[doc = "PLL_CORE_HSDIVIDER (rw) register accessor: PLL_CORE_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER")]
pub type PllCoreHsdivider = crate::Reg<pll_core_hsdivider::PllCoreHsdividerSpec>;
#[doc = "PLL_CORE_HSDIVIDER"]
pub mod pll_core_hsdivider;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT0 (rw) register accessor: PLL_CORE_HSDIVIDER_CLKOUT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider_clkout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider_clkout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider_clkout0`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER_CLKOUT0")]
pub type PllCoreHsdividerClkout0 =
    crate::Reg<pll_core_hsdivider_clkout0::PllCoreHsdividerClkout0Spec>;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT0"]
pub mod pll_core_hsdivider_clkout0;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT1 (rw) register accessor: PLL_CORE_HSDIVIDER_CLKOUT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider_clkout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider_clkout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider_clkout1`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER_CLKOUT1")]
pub type PllCoreHsdividerClkout1 =
    crate::Reg<pll_core_hsdivider_clkout1::PllCoreHsdividerClkout1Spec>;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT1"]
pub mod pll_core_hsdivider_clkout1;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT2 (rw) register accessor: PLL_CORE_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider_clkout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider_clkout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider_clkout2`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER_CLKOUT2")]
pub type PllCoreHsdividerClkout2 =
    crate::Reg<pll_core_hsdivider_clkout2::PllCoreHsdividerClkout2Spec>;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT2"]
pub mod pll_core_hsdivider_clkout2;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT3 (rw) register accessor: PLL_CORE_HSDIVIDER_CLKOUT3\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider_clkout3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider_clkout3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider_clkout3`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER_CLKOUT3")]
pub type PllCoreHsdividerClkout3 =
    crate::Reg<pll_core_hsdivider_clkout3::PllCoreHsdividerClkout3Spec>;
#[doc = "PLL_CORE_HSDIVIDER_CLKOUT3"]
pub mod pll_core_hsdivider_clkout3;
#[doc = "MSS_CR5_CLK_SRC_SEL (rw) register accessor: MSS_CR5_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5_clk_src_sel`]
module"]
#[doc(alias = "MSS_CR5_CLK_SRC_SEL")]
pub type MssCr5ClkSrcSel = crate::Reg<mss_cr5_clk_src_sel::MssCr5ClkSrcSelSpec>;
#[doc = "MSS_CR5_CLK_SRC_SEL"]
pub mod mss_cr5_clk_src_sel;
#[doc = "MSS_CR5_DIV_VAL (rw) register accessor: MSS_CR5_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5_div_val`]
module"]
#[doc(alias = "MSS_CR5_DIV_VAL")]
pub type MssCr5DivVal = crate::Reg<mss_cr5_div_val::MssCr5DivValSpec>;
#[doc = "MSS_CR5_DIV_VAL"]
pub mod mss_cr5_div_val;
#[doc = "SYS_CLK_DIV_VAL (rw) register accessor: SYS_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_div_val`]
module"]
#[doc(alias = "SYS_CLK_DIV_VAL")]
pub type SysClkDivVal = crate::Reg<sys_clk_div_val::SysClkDivValSpec>;
#[doc = "SYS_CLK_DIV_VAL"]
pub mod sys_clk_div_val;
#[doc = "MSS_CR5_CLK_GATE (rw) register accessor: MSS_CR5_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5_clk_gate`]
module"]
#[doc(alias = "MSS_CR5_CLK_GATE")]
pub type MssCr5ClkGate = crate::Reg<mss_cr5_clk_gate::MssCr5ClkGateSpec>;
#[doc = "MSS_CR5_CLK_GATE"]
pub mod mss_cr5_clk_gate;
#[doc = "SYS_CLK_GATE (rw) register accessor: SYS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_gate`]
module"]
#[doc(alias = "SYS_CLK_GATE")]
pub type SysClkGate = crate::Reg<sys_clk_gate::SysClkGateSpec>;
#[doc = "SYS_CLK_GATE"]
pub mod sys_clk_gate;
#[doc = "SYS_CLK_STATUS (rw) register accessor: SYS_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_clk_status`]
module"]
#[doc(alias = "SYS_CLK_STATUS")]
pub type SysClkStatus = crate::Reg<sys_clk_status::SysClkStatusSpec>;
#[doc = "SYS_CLK_STATUS"]
pub mod sys_clk_status;
#[doc = "MSS_CR5_CLK_STATUS (rw) register accessor: MSS_CR5_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5_clk_status`]
module"]
#[doc(alias = "MSS_CR5_CLK_STATUS")]
pub type MssCr5ClkStatus = crate::Reg<mss_cr5_clk_status::MssCr5ClkStatusSpec>;
#[doc = "MSS_CR5_CLK_STATUS"]
pub mod mss_cr5_clk_status;
#[doc = "PLL_CORE_RSTCTRL (rw) register accessor: PLL_CORE_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_rstctrl`]
module"]
#[doc(alias = "PLL_CORE_RSTCTRL")]
pub type PllCoreRstctrl = crate::Reg<pll_core_rstctrl::PllCoreRstctrlSpec>;
#[doc = "PLL_CORE_RSTCTRL"]
pub mod pll_core_rstctrl;
#[doc = "PLL_CORE_HSDIVIDER_RSTCTRL (rw) register accessor: PLL_CORE_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_hsdivider_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_hsdivider_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_core_hsdivider_rstctrl`]
module"]
#[doc(alias = "PLL_CORE_HSDIVIDER_RSTCTRL")]
pub type PllCoreHsdividerRstctrl =
    crate::Reg<pll_core_hsdivider_rstctrl::PllCoreHsdividerRstctrlSpec>;
#[doc = "PLL_CORE_HSDIVIDER_RSTCTRL"]
pub mod pll_core_hsdivider_rstctrl;
#[doc = "RSS_CLK_SRC_SEL (rw) register accessor: RSS_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_clk_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_clk_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_clk_src_sel`]
module"]
#[doc(alias = "RSS_CLK_SRC_SEL")]
pub type RssClkSrcSel = crate::Reg<rss_clk_src_sel::RssClkSrcSelSpec>;
#[doc = "RSS_CLK_SRC_SEL"]
pub mod rss_clk_src_sel;
#[doc = "PLLC_CLK2_SRC_SEL (rw) register accessor: PLLC_CLK2_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pllc_clk2_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllc_clk2_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllc_clk2_src_sel`]
module"]
#[doc(alias = "PLLC_CLK2_SRC_SEL")]
pub type PllcClk2SrcSel = crate::Reg<pllc_clk2_src_sel::PllcClk2SrcSelSpec>;
#[doc = "PLLC_CLK2_SRC_SEL"]
pub mod pllc_clk2_src_sel;
#[doc = "PLLD_CLK1_SRC_SEL (rw) register accessor: PLLD_CLK1_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk1_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk1_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plld_clk1_src_sel`]
module"]
#[doc(alias = "PLLD_CLK1_SRC_SEL")]
pub type PlldClk1SrcSel = crate::Reg<plld_clk1_src_sel::PlldClk1SrcSelSpec>;
#[doc = "PLLD_CLK1_SRC_SEL"]
pub mod plld_clk1_src_sel;
#[doc = "PLLD_CLK2_SRC_SEL (rw) register accessor: PLLD_CLK2_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk2_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk2_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plld_clk2_src_sel`]
module"]
#[doc(alias = "PLLD_CLK2_SRC_SEL")]
pub type PlldClk2SrcSel = crate::Reg<plld_clk2_src_sel::PlldClk2SrcSelSpec>;
#[doc = "PLLD_CLK2_SRC_SEL"]
pub mod plld_clk2_src_sel;
#[doc = "PLLP_CLK1_SRC_SEL (rw) register accessor: PLLP_CLK1_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pllp_clk1_src_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllp_clk1_src_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllp_clk1_src_sel`]
module"]
#[doc(alias = "PLLP_CLK1_SRC_SEL")]
pub type PllpClk1SrcSel = crate::Reg<pllp_clk1_src_sel::PllpClk1SrcSelSpec>;
#[doc = "PLLP_CLK1_SRC_SEL"]
pub mod pllp_clk1_src_sel;
#[doc = "RSS_DIV_VAL (rw) register accessor: RSS_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_div_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_div_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_div_val`]
module"]
#[doc(alias = "RSS_DIV_VAL")]
pub type RssDivVal = crate::Reg<rss_div_val::RssDivValSpec>;
#[doc = "RSS_DIV_VAL"]
pub mod rss_div_val;
#[doc = "RSS_CLK_GATE (rw) register accessor: RSS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_clk_gate`]
module"]
#[doc(alias = "RSS_CLK_GATE")]
pub type RssClkGate = crate::Reg<rss_clk_gate::RssClkGateSpec>;
#[doc = "RSS_CLK_GATE"]
pub mod rss_clk_gate;
#[doc = "PLLC_CLK2_GATE (rw) register accessor: PLLC_CLK2_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`pllc_clk2_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllc_clk2_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllc_clk2_gate`]
module"]
#[doc(alias = "PLLC_CLK2_GATE")]
pub type PllcClk2Gate = crate::Reg<pllc_clk2_gate::PllcClk2GateSpec>;
#[doc = "PLLC_CLK2_GATE"]
pub mod pllc_clk2_gate;
#[doc = "PLLD_CLK1_GATE (rw) register accessor: PLLD_CLK1_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk1_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk1_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plld_clk1_gate`]
module"]
#[doc(alias = "PLLD_CLK1_GATE")]
pub type PlldClk1Gate = crate::Reg<plld_clk1_gate::PlldClk1GateSpec>;
#[doc = "PLLD_CLK1_GATE"]
pub mod plld_clk1_gate;
#[doc = "PLLD_CLK2_GATE (rw) register accessor: PLLD_CLK2_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk2_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk2_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plld_clk2_gate`]
module"]
#[doc(alias = "PLLD_CLK2_GATE")]
pub type PlldClk2Gate = crate::Reg<plld_clk2_gate::PlldClk2GateSpec>;
#[doc = "PLLD_CLK2_GATE"]
pub mod plld_clk2_gate;
#[doc = "PLLP_CLK1_GATE (rw) register accessor: PLLP_CLK1_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`pllp_clk1_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllp_clk1_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllp_clk1_gate`]
module"]
#[doc(alias = "PLLP_CLK1_GATE")]
pub type PllpClk1Gate = crate::Reg<pllp_clk1_gate::PllpClk1GateSpec>;
#[doc = "PLLP_CLK1_GATE"]
pub mod pllp_clk1_gate;
#[doc = "RSS_CLK_STATUS (rw) register accessor: RSS_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_clk_status`]
module"]
#[doc(alias = "RSS_CLK_STATUS")]
pub type RssClkStatus = crate::Reg<rss_clk_status::RssClkStatusSpec>;
#[doc = "RSS_CLK_STATUS"]
pub mod rss_clk_status;
#[doc = "PLLC_CLK2_STATUS (rw) register accessor: PLLC_CLK2_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pllc_clk2_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllc_clk2_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllc_clk2_status`]
module"]
#[doc(alias = "PLLC_CLK2_STATUS")]
pub type PllcClk2Status = crate::Reg<pllc_clk2_status::PllcClk2StatusSpec>;
#[doc = "PLLC_CLK2_STATUS"]
pub mod pllc_clk2_status;
#[doc = "PLLD_CLK1_STATUS (rw) register accessor: PLLD_CLK1_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk1_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk1_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plld_clk1_status`]
module"]
#[doc(alias = "PLLD_CLK1_STATUS")]
pub type PlldClk1Status = crate::Reg<plld_clk1_status::PlldClk1StatusSpec>;
#[doc = "PLLD_CLK1_STATUS"]
pub mod plld_clk1_status;
#[doc = "PLLP_CLK1_STATUS (rw) register accessor: PLLP_CLK1_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pllp_clk1_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllp_clk1_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllp_clk1_status`]
module"]
#[doc(alias = "PLLP_CLK1_STATUS")]
pub type PllpClk1Status = crate::Reg<pllp_clk1_status::PllpClk1StatusSpec>;
#[doc = "PLLP_CLK1_STATUS"]
pub mod pllp_clk1_status;
#[doc = "PLL_1P2_HSDIVIDER (rw) register accessor: PLL_1P2_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER")]
pub type Pll1p2Hsdivider = crate::Reg<pll_1p2_hsdivider::Pll1p2HsdividerSpec>;
#[doc = "PLL_1P2_HSDIVIDER"]
pub mod pll_1p2_hsdivider;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT0 (rw) register accessor: PLL_1P2_HSDIVIDER_CLKOUT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider_clkout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider_clkout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider_clkout0`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER_CLKOUT0")]
pub type Pll1p2HsdividerClkout0 = crate::Reg<pll_1p2_hsdivider_clkout0::Pll1p2HsdividerClkout0Spec>;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT0"]
pub mod pll_1p2_hsdivider_clkout0;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT1 (rw) register accessor: PLL_1P2_HSDIVIDER_CLKOUT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider_clkout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider_clkout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider_clkout1`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER_CLKOUT1")]
pub type Pll1p2HsdividerClkout1 = crate::Reg<pll_1p2_hsdivider_clkout1::Pll1p2HsdividerClkout1Spec>;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT1"]
pub mod pll_1p2_hsdivider_clkout1;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT2 (rw) register accessor: PLL_1P2_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider_clkout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider_clkout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider_clkout2`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER_CLKOUT2")]
pub type Pll1p2HsdividerClkout2 = crate::Reg<pll_1p2_hsdivider_clkout2::Pll1p2HsdividerClkout2Spec>;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT2"]
pub mod pll_1p2_hsdivider_clkout2;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT3 (rw) register accessor: PLL_1P2_HSDIVIDER_CLKOUT3\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider_clkout3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider_clkout3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider_clkout3`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER_CLKOUT3")]
pub type Pll1p2HsdividerClkout3 = crate::Reg<pll_1p2_hsdivider_clkout3::Pll1p2HsdividerClkout3Spec>;
#[doc = "PLL_1P2_HSDIVIDER_CLKOUT3"]
pub mod pll_1p2_hsdivider_clkout3;
#[doc = "PLL_1P2_HSDIVIDER_RSTCTRL (rw) register accessor: PLL_1P2_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p2_hsdivider_rstctrl`]
module"]
#[doc(alias = "PLL_1P2_HSDIVIDER_RSTCTRL")]
pub type Pll1p2HsdividerRstctrl = crate::Reg<pll_1p2_hsdivider_rstctrl::Pll1p2HsdividerRstctrlSpec>;
#[doc = "PLL_1P2_HSDIVIDER_RSTCTRL"]
pub mod pll_1p2_hsdivider_rstctrl;
#[doc = "PLL_1P8_HSDIVIDER (rw) register accessor: PLL_1P8_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER")]
pub type Pll1p8Hsdivider = crate::Reg<pll_1p8_hsdivider::Pll1p8HsdividerSpec>;
#[doc = "PLL_1P8_HSDIVIDER"]
pub mod pll_1p8_hsdivider;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT0 (rw) register accessor: PLL_1P8_HSDIVIDER_CLKOUT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider_clkout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider_clkout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider_clkout0`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER_CLKOUT0")]
pub type Pll1p8HsdividerClkout0 = crate::Reg<pll_1p8_hsdivider_clkout0::Pll1p8HsdividerClkout0Spec>;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT0"]
pub mod pll_1p8_hsdivider_clkout0;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT1 (rw) register accessor: PLL_1P8_HSDIVIDER_CLKOUT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider_clkout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider_clkout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider_clkout1`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER_CLKOUT1")]
pub type Pll1p8HsdividerClkout1 = crate::Reg<pll_1p8_hsdivider_clkout1::Pll1p8HsdividerClkout1Spec>;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT1"]
pub mod pll_1p8_hsdivider_clkout1;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT2 (rw) register accessor: PLL_1P8_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider_clkout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider_clkout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider_clkout2`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER_CLKOUT2")]
pub type Pll1p8HsdividerClkout2 = crate::Reg<pll_1p8_hsdivider_clkout2::Pll1p8HsdividerClkout2Spec>;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT2"]
pub mod pll_1p8_hsdivider_clkout2;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT3 (rw) register accessor: PLL_1P8_HSDIVIDER_CLKOUT3\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider_clkout3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider_clkout3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider_clkout3`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER_CLKOUT3")]
pub type Pll1p8HsdividerClkout3 = crate::Reg<pll_1p8_hsdivider_clkout3::Pll1p8HsdividerClkout3Spec>;
#[doc = "PLL_1P8_HSDIVIDER_CLKOUT3"]
pub mod pll_1p8_hsdivider_clkout3;
#[doc = "PLL_1P8_HSDIVIDER_RSTCTRL (rw) register accessor: PLL_1P8_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p8_hsdivider_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p8_hsdivider_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_1p8_hsdivider_rstctrl`]
module"]
#[doc(alias = "PLL_1P8_HSDIVIDER_RSTCTRL")]
pub type Pll1p8HsdividerRstctrl = crate::Reg<pll_1p8_hsdivider_rstctrl::Pll1p8HsdividerRstctrlSpec>;
#[doc = "PLL_1P8_HSDIVIDER_RSTCTRL"]
pub mod pll_1p8_hsdivider_rstctrl;
#[doc = "PLL_DSP_PWRCTRL (rw) register accessor: PLL_DSP_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_pwrctrl`]
module"]
#[doc(alias = "PLL_DSP_PWRCTRL")]
pub type PllDspPwrctrl = crate::Reg<pll_dsp_pwrctrl::PllDspPwrctrlSpec>;
#[doc = "PLL_DSP_PWRCTRL"]
pub mod pll_dsp_pwrctrl;
#[doc = "PLL_DSP_CLKCTRL (rw) register accessor: PLL_DSP_CLKCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_clkctrl`]
module"]
#[doc(alias = "PLL_DSP_CLKCTRL")]
pub type PllDspClkctrl = crate::Reg<pll_dsp_clkctrl::PllDspClkctrlSpec>;
#[doc = "PLL_DSP_CLKCTRL"]
pub mod pll_dsp_clkctrl;
#[doc = "PLL_DSP_TENABLE (rw) register accessor: PLL_DSP_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_tenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_tenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_tenable`]
module"]
#[doc(alias = "PLL_DSP_TENABLE")]
pub type PllDspTenable = crate::Reg<pll_dsp_tenable::PllDspTenableSpec>;
#[doc = "PLL_DSP_TENABLE"]
pub mod pll_dsp_tenable;
#[doc = "PLL_DSP_TENABLEDIV (rw) register accessor: PLL_DSP_TENABLEDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_tenablediv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_tenablediv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_tenablediv`]
module"]
#[doc(alias = "PLL_DSP_TENABLEDIV")]
pub type PllDspTenablediv = crate::Reg<pll_dsp_tenablediv::PllDspTenabledivSpec>;
#[doc = "PLL_DSP_TENABLEDIV"]
pub mod pll_dsp_tenablediv;
#[doc = "PLL_DSP_M2NDIV (rw) register accessor: PLL_DSP_M2NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_m2ndiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_m2ndiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_m2ndiv`]
module"]
#[doc(alias = "PLL_DSP_M2NDIV")]
pub type PllDspM2ndiv = crate::Reg<pll_dsp_m2ndiv::PllDspM2ndivSpec>;
#[doc = "PLL_DSP_M2NDIV"]
pub mod pll_dsp_m2ndiv;
#[doc = "PLL_DSP_MN2DIV (rw) register accessor: PLL_DSP_MN2DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_mn2div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_mn2div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_mn2div`]
module"]
#[doc(alias = "PLL_DSP_MN2DIV")]
pub type PllDspMn2div = crate::Reg<pll_dsp_mn2div::PllDspMn2divSpec>;
#[doc = "PLL_DSP_MN2DIV"]
pub mod pll_dsp_mn2div;
#[doc = "PLL_DSP_FRACDIV (rw) register accessor: PLL_DSP_FRACDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_fracdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_fracdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_fracdiv`]
module"]
#[doc(alias = "PLL_DSP_FRACDIV")]
pub type PllDspFracdiv = crate::Reg<pll_dsp_fracdiv::PllDspFracdivSpec>;
#[doc = "PLL_DSP_FRACDIV"]
pub mod pll_dsp_fracdiv;
#[doc = "PLL_DSP_BWCTRL (rw) register accessor: PLL_DSP_BWCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_bwctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_bwctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_bwctrl`]
module"]
#[doc(alias = "PLL_DSP_BWCTRL")]
pub type PllDspBwctrl = crate::Reg<pll_dsp_bwctrl::PllDspBwctrlSpec>;
#[doc = "PLL_DSP_BWCTRL"]
pub mod pll_dsp_bwctrl;
#[doc = "PLL_DSP_FRACCTRL (rw) register accessor: PLL_DSP_FRACCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_fracctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_fracctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_fracctrl`]
module"]
#[doc(alias = "PLL_DSP_FRACCTRL")]
pub type PllDspFracctrl = crate::Reg<pll_dsp_fracctrl::PllDspFracctrlSpec>;
#[doc = "PLL_DSP_FRACCTRL"]
pub mod pll_dsp_fracctrl;
#[doc = "PLL_DSP_HSDIVIDER (rw) register accessor: PLL_DSP_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER")]
pub type PllDspHsdivider = crate::Reg<pll_dsp_hsdivider::PllDspHsdividerSpec>;
#[doc = "PLL_DSP_HSDIVIDER"]
pub mod pll_dsp_hsdivider;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT0 (rw) register accessor: PLL_DSP_HSDIVIDER_CLKOUT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_clkout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_clkout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider_clkout0`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER_CLKOUT0")]
pub type PllDspHsdividerClkout0 = crate::Reg<pll_dsp_hsdivider_clkout0::PllDspHsdividerClkout0Spec>;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT0"]
pub mod pll_dsp_hsdivider_clkout0;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT1 (rw) register accessor: PLL_DSP_HSDIVIDER_CLKOUT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_clkout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_clkout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider_clkout1`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER_CLKOUT1")]
pub type PllDspHsdividerClkout1 = crate::Reg<pll_dsp_hsdivider_clkout1::PllDspHsdividerClkout1Spec>;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT1"]
pub mod pll_dsp_hsdivider_clkout1;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT2 (rw) register accessor: PLL_DSP_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_clkout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_clkout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider_clkout2`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER_CLKOUT2")]
pub type PllDspHsdividerClkout2 = crate::Reg<pll_dsp_hsdivider_clkout2::PllDspHsdividerClkout2Spec>;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT2"]
pub mod pll_dsp_hsdivider_clkout2;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT3 (rw) register accessor: PLL_DSP_HSDIVIDER_CLKOUT3\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_clkout3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_clkout3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider_clkout3`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER_CLKOUT3")]
pub type PllDspHsdividerClkout3 = crate::Reg<pll_dsp_hsdivider_clkout3::PllDspHsdividerClkout3Spec>;
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT3"]
pub mod pll_dsp_hsdivider_clkout3;
#[doc = "PLL_PER_PWRCTRL (rw) register accessor: PLL_PER_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_pwrctrl`]
module"]
#[doc(alias = "PLL_PER_PWRCTRL")]
pub type PllPerPwrctrl = crate::Reg<pll_per_pwrctrl::PllPerPwrctrlSpec>;
#[doc = "PLL_PER_PWRCTRL"]
pub mod pll_per_pwrctrl;
#[doc = "PLL_PER_CLKCTRL (rw) register accessor: PLL_PER_CLKCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_clkctrl`]
module"]
#[doc(alias = "PLL_PER_CLKCTRL")]
pub type PllPerClkctrl = crate::Reg<pll_per_clkctrl::PllPerClkctrlSpec>;
#[doc = "PLL_PER_CLKCTRL"]
pub mod pll_per_clkctrl;
#[doc = "PLL_PER_TENABLE (rw) register accessor: PLL_PER_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_tenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_tenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_tenable`]
module"]
#[doc(alias = "PLL_PER_TENABLE")]
pub type PllPerTenable = crate::Reg<pll_per_tenable::PllPerTenableSpec>;
#[doc = "PLL_PER_TENABLE"]
pub mod pll_per_tenable;
#[doc = "PLL_PER_TENABLEDIV (rw) register accessor: PLL_PER_TENABLEDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_tenablediv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_tenablediv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_tenablediv`]
module"]
#[doc(alias = "PLL_PER_TENABLEDIV")]
pub type PllPerTenablediv = crate::Reg<pll_per_tenablediv::PllPerTenabledivSpec>;
#[doc = "PLL_PER_TENABLEDIV"]
pub mod pll_per_tenablediv;
#[doc = "PLL_PER_M2NDIV (rw) register accessor: PLL_PER_M2NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_m2ndiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_m2ndiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_m2ndiv`]
module"]
#[doc(alias = "PLL_PER_M2NDIV")]
pub type PllPerM2ndiv = crate::Reg<pll_per_m2ndiv::PllPerM2ndivSpec>;
#[doc = "PLL_PER_M2NDIV"]
pub mod pll_per_m2ndiv;
#[doc = "PLL_PER_MN2DIV (rw) register accessor: PLL_PER_MN2DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_mn2div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_mn2div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_mn2div`]
module"]
#[doc(alias = "PLL_PER_MN2DIV")]
pub type PllPerMn2div = crate::Reg<pll_per_mn2div::PllPerMn2divSpec>;
#[doc = "PLL_PER_MN2DIV"]
pub mod pll_per_mn2div;
#[doc = "PLL_PER_FRACDIV (rw) register accessor: PLL_PER_FRACDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_fracdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_fracdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_fracdiv`]
module"]
#[doc(alias = "PLL_PER_FRACDIV")]
pub type PllPerFracdiv = crate::Reg<pll_per_fracdiv::PllPerFracdivSpec>;
#[doc = "PLL_PER_FRACDIV"]
pub mod pll_per_fracdiv;
#[doc = "PLL_PER_BWCTRL (rw) register accessor: PLL_PER_BWCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_bwctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_bwctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_bwctrl`]
module"]
#[doc(alias = "PLL_PER_BWCTRL")]
pub type PllPerBwctrl = crate::Reg<pll_per_bwctrl::PllPerBwctrlSpec>;
#[doc = "PLL_PER_BWCTRL"]
pub mod pll_per_bwctrl;
#[doc = "PLL_PER_FRACCTRL (rw) register accessor: PLL_PER_FRACCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_fracctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_fracctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_fracctrl`]
module"]
#[doc(alias = "PLL_PER_FRACCTRL")]
pub type PllPerFracctrl = crate::Reg<pll_per_fracctrl::PllPerFracctrlSpec>;
#[doc = "PLL_PER_FRACCTRL"]
pub mod pll_per_fracctrl;
#[doc = "PLL_PER_STATUS (rw) register accessor: PLL_PER_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_status`]
module"]
#[doc(alias = "PLL_PER_STATUS")]
pub type PllPerStatus = crate::Reg<pll_per_status::PllPerStatusSpec>;
#[doc = "PLL_PER_STATUS"]
pub mod pll_per_status;
#[doc = "PLL_PER_HSDIVIDER (rw) register accessor: PLL_PER_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER")]
pub type PllPerHsdivider = crate::Reg<pll_per_hsdivider::PllPerHsdividerSpec>;
#[doc = "PLL_PER_HSDIVIDER"]
pub mod pll_per_hsdivider;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT0 (rw) register accessor: PLL_PER_HSDIVIDER_CLKOUT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_clkout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_clkout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider_clkout0`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER_CLKOUT0")]
pub type PllPerHsdividerClkout0 = crate::Reg<pll_per_hsdivider_clkout0::PllPerHsdividerClkout0Spec>;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT0"]
pub mod pll_per_hsdivider_clkout0;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT1 (rw) register accessor: PLL_PER_HSDIVIDER_CLKOUT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_clkout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_clkout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider_clkout1`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER_CLKOUT1")]
pub type PllPerHsdividerClkout1 = crate::Reg<pll_per_hsdivider_clkout1::PllPerHsdividerClkout1Spec>;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT1"]
pub mod pll_per_hsdivider_clkout1;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT2 (rw) register accessor: PLL_PER_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_clkout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_clkout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider_clkout2`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER_CLKOUT2")]
pub type PllPerHsdividerClkout2 = crate::Reg<pll_per_hsdivider_clkout2::PllPerHsdividerClkout2Spec>;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT2"]
pub mod pll_per_hsdivider_clkout2;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT3 (rw) register accessor: PLL_PER_HSDIVIDER_CLKOUT3\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_clkout3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_clkout3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider_clkout3`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER_CLKOUT3")]
pub type PllPerHsdividerClkout3 = crate::Reg<pll_per_hsdivider_clkout3::PllPerHsdividerClkout3Spec>;
#[doc = "PLL_PER_HSDIVIDER_CLKOUT3"]
pub mod pll_per_hsdivider_clkout3;
#[doc = "PLL_DSP_RSTCTRL (rw) register accessor: PLL_DSP_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_rstctrl`]
module"]
#[doc(alias = "PLL_DSP_RSTCTRL")]
pub type PllDspRstctrl = crate::Reg<pll_dsp_rstctrl::PllDspRstctrlSpec>;
#[doc = "PLL_DSP_RSTCTRL"]
pub mod pll_dsp_rstctrl;
#[doc = "PLL_DSP_HSDIVIDER_RSTCTRL (rw) register accessor: PLL_DSP_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_dsp_hsdivider_rstctrl`]
module"]
#[doc(alias = "PLL_DSP_HSDIVIDER_RSTCTRL")]
pub type PllDspHsdividerRstctrl = crate::Reg<pll_dsp_hsdivider_rstctrl::PllDspHsdividerRstctrlSpec>;
#[doc = "PLL_DSP_HSDIVIDER_RSTCTRL"]
pub mod pll_dsp_hsdivider_rstctrl;
#[doc = "PLL_PER_RSTCTRL (rw) register accessor: PLL_PER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_rstctrl`]
module"]
#[doc(alias = "PLL_PER_RSTCTRL")]
pub type PllPerRstctrl = crate::Reg<pll_per_rstctrl::PllPerRstctrlSpec>;
#[doc = "PLL_PER_RSTCTRL"]
pub mod pll_per_rstctrl;
#[doc = "PLL_PER_HSDIVIDER_RSTCTRL (rw) register accessor: PLL_PER_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_per_hsdivider_rstctrl`]
module"]
#[doc(alias = "PLL_PER_HSDIVIDER_RSTCTRL")]
pub type PllPerHsdividerRstctrl = crate::Reg<pll_per_hsdivider_rstctrl::PllPerHsdividerRstctrlSpec>;
#[doc = "PLL_PER_HSDIVIDER_RSTCTRL"]
pub mod pll_per_hsdivider_rstctrl;
#[doc = "ANA_REG_CLK_CTRL_REG1_XO_SLICER (rw) register accessor: ANA_REG_CLK_CTRL_REG1_XO_SLICER\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_xo_slicer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_xo_slicer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_ctrl_reg1_xo_slicer`]
module"]
#[doc(alias = "ANA_REG_CLK_CTRL_REG1_XO_SLICER")]
pub type AnaRegClkCtrlReg1XoSlicer =
    crate::Reg<ana_reg_clk_ctrl_reg1_xo_slicer::AnaRegClkCtrlReg1XoSlicerSpec>;
#[doc = "ANA_REG_CLK_CTRL_REG1_XO_SLICER"]
pub mod ana_reg_clk_ctrl_reg1_xo_slicer;
#[doc = "ANA_REG_CLK_CTRL_REG1_CLKTOP (rw) register accessor: ANA_REG_CLK_CTRL_REG1_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_clktop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_clktop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_ctrl_reg1_clktop`]
module"]
#[doc(alias = "ANA_REG_CLK_CTRL_REG1_CLKTOP")]
pub type AnaRegClkCtrlReg1Clktop =
    crate::Reg<ana_reg_clk_ctrl_reg1_clktop::AnaRegClkCtrlReg1ClktopSpec>;
#[doc = "ANA_REG_CLK_CTRL_REG1_CLKTOP"]
pub mod ana_reg_clk_ctrl_reg1_clktop;
#[doc = "ANA_REG_CLK_CTRL_REG2_CLKTOP (rw) register accessor: ANA_REG_CLK_CTRL_REG2_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg2_clktop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg2_clktop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_ctrl_reg2_clktop`]
module"]
#[doc(alias = "ANA_REG_CLK_CTRL_REG2_CLKTOP")]
pub type AnaRegClkCtrlReg2Clktop =
    crate::Reg<ana_reg_clk_ctrl_reg2_clktop::AnaRegClkCtrlReg2ClktopSpec>;
#[doc = "ANA_REG_CLK_CTRL_REG2_CLKTOP"]
pub mod ana_reg_clk_ctrl_reg2_clktop;
#[doc = "ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP (rw) register accessor: ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_ldo_clktop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_ldo_clktop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_ctrl_reg1_ldo_clktop`]
module"]
#[doc(alias = "ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP")]
pub type AnaRegClkCtrlReg1LdoClktop =
    crate::Reg<ana_reg_clk_ctrl_reg1_ldo_clktop::AnaRegClkCtrlReg1LdoClktopSpec>;
#[doc = "ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP"]
pub mod ana_reg_clk_ctrl_reg1_ldo_clktop;
#[doc = "ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP (rw) register accessor: ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg2_ldo_clktop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg2_ldo_clktop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_ctrl_reg2_ldo_clktop`]
module"]
#[doc(alias = "ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP")]
pub type AnaRegClkCtrlReg2LdoClktop =
    crate::Reg<ana_reg_clk_ctrl_reg2_ldo_clktop::AnaRegClkCtrlReg2LdoClktopSpec>;
#[doc = "ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP"]
pub mod ana_reg_clk_ctrl_reg2_ldo_clktop;
#[doc = "ANA_REG_CLK_STATUS_REG (rw) register accessor: ANA_REG_CLK_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_clk_status_reg`]
module"]
#[doc(alias = "ANA_REG_CLK_STATUS_REG")]
pub type AnaRegClkStatusReg = crate::Reg<ana_reg_clk_status_reg::AnaRegClkStatusRegSpec>;
#[doc = "ANA_REG_CLK_STATUS_REG"]
pub mod ana_reg_clk_status_reg;
#[doc = "ANA_REG_REFSYS_CTRL_REG_LOWV (rw) register accessor: ANA_REG_REFSYS_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_ctrl_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_ctrl_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_refsys_ctrl_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_REFSYS_CTRL_REG_LOWV")]
pub type AnaRegRefsysCtrlRegLowv =
    crate::Reg<ana_reg_refsys_ctrl_reg_lowv::AnaRegRefsysCtrlRegLowvSpec>;
#[doc = "ANA_REG_REFSYS_CTRL_REG_LOWV"]
pub mod ana_reg_refsys_ctrl_reg_lowv;
#[doc = "ANA_REG_REFSYS_TMUX_CTRL_LOWV (rw) register accessor: ANA_REG_REFSYS_TMUX_CTRL_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_tmux_ctrl_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_tmux_ctrl_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_refsys_tmux_ctrl_lowv`]
module"]
#[doc(alias = "ANA_REG_REFSYS_TMUX_CTRL_LOWV")]
pub type AnaRegRefsysTmuxCtrlLowv =
    crate::Reg<ana_reg_refsys_tmux_ctrl_lowv::AnaRegRefsysTmuxCtrlLowvSpec>;
#[doc = "ANA_REG_REFSYS_TMUX_CTRL_LOWV"]
pub mod ana_reg_refsys_tmux_ctrl_lowv;
#[doc = "ANA_REG_REFSYS_SPARE_REG_LOWV (rw) register accessor: ANA_REG_REFSYS_SPARE_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_spare_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_spare_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_refsys_spare_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_REFSYS_SPARE_REG_LOWV")]
pub type AnaRegRefsysSpareRegLowv =
    crate::Reg<ana_reg_refsys_spare_reg_lowv::AnaRegRefsysSpareRegLowvSpec>;
#[doc = "ANA_REG_REFSYS_SPARE_REG_LOWV"]
pub mod ana_reg_refsys_spare_reg_lowv;
#[doc = "ANA_REG_WU_CTRL_REG_LOWV (rw) register accessor: ANA_REG_WU_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_ctrl_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_ctrl_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_wu_ctrl_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_WU_CTRL_REG_LOWV")]
pub type AnaRegWuCtrlRegLowv = crate::Reg<ana_reg_wu_ctrl_reg_lowv::AnaRegWuCtrlRegLowvSpec>;
#[doc = "ANA_REG_WU_CTRL_REG_LOWV"]
pub mod ana_reg_wu_ctrl_reg_lowv;
#[doc = "ANA_REG_WU_TMUX_CTRL_LOWV (rw) register accessor: ANA_REG_WU_TMUX_CTRL_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_tmux_ctrl_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_tmux_ctrl_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_wu_tmux_ctrl_lowv`]
module"]
#[doc(alias = "ANA_REG_WU_TMUX_CTRL_LOWV")]
pub type AnaRegWuTmuxCtrlLowv = crate::Reg<ana_reg_wu_tmux_ctrl_lowv::AnaRegWuTmuxCtrlLowvSpec>;
#[doc = "ANA_REG_WU_TMUX_CTRL_LOWV"]
pub mod ana_reg_wu_tmux_ctrl_lowv;
#[doc = "ANA_REG_TW_CTRL_REG_LOWV (rw) register accessor: ANA_REG_TW_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_tw_ctrl_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_tw_ctrl_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_tw_ctrl_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_TW_CTRL_REG_LOWV")]
pub type AnaRegTwCtrlRegLowv = crate::Reg<ana_reg_tw_ctrl_reg_lowv::AnaRegTwCtrlRegLowvSpec>;
#[doc = "ANA_REG_TW_CTRL_REG_LOWV"]
pub mod ana_reg_tw_ctrl_reg_lowv;
#[doc = "ANA_REG_TW_ANA_TMUX_CTRL_LOWV (rw) register accessor: ANA_REG_TW_ANA_TMUX_CTRL_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_tw_ana_tmux_ctrl_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_tw_ana_tmux_ctrl_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_tw_ana_tmux_ctrl_lowv`]
module"]
#[doc(alias = "ANA_REG_TW_ANA_TMUX_CTRL_LOWV")]
pub type AnaRegTwAnaTmuxCtrlLowv =
    crate::Reg<ana_reg_tw_ana_tmux_ctrl_lowv::AnaRegTwAnaTmuxCtrlLowvSpec>;
#[doc = "ANA_REG_TW_ANA_TMUX_CTRL_LOWV"]
pub mod ana_reg_tw_ana_tmux_ctrl_lowv;
#[doc = "ANA_REG_WU_MODE_REG_LOWV (rw) register accessor: ANA_REG_WU_MODE_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_mode_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_mode_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_wu_mode_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_WU_MODE_REG_LOWV")]
pub type AnaRegWuModeRegLowv = crate::Reg<ana_reg_wu_mode_reg_lowv::AnaRegWuModeRegLowvSpec>;
#[doc = "ANA_REG_WU_MODE_REG_LOWV"]
pub mod ana_reg_wu_mode_reg_lowv;
#[doc = "ANA_REG_WU_STATUS_REG_LOWV (rw) register accessor: ANA_REG_WU_STATUS_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_status_reg_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_status_reg_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_wu_status_reg_lowv`]
module"]
#[doc(alias = "ANA_REG_WU_STATUS_REG_LOWV")]
pub type AnaRegWuStatusRegLowv = crate::Reg<ana_reg_wu_status_reg_lowv::AnaRegWuStatusRegLowvSpec>;
#[doc = "ANA_REG_WU_STATUS_REG_LOWV"]
pub mod ana_reg_wu_status_reg_lowv;
#[doc = "ANA_REG_WU_SPARE_OUT_LOWV (rw) register accessor: ANA_REG_WU_SPARE_OUT_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_spare_out_lowv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_spare_out_lowv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_reg_wu_spare_out_lowv`]
module"]
#[doc(alias = "ANA_REG_WU_SPARE_OUT_LOWV")]
pub type AnaRegWuSpareOutLowv = crate::Reg<ana_reg_wu_spare_out_lowv::AnaRegWuSpareOutLowvSpec>;
#[doc = "ANA_REG_WU_SPARE_OUT_LOWV"]
pub mod ana_reg_wu_spare_out_lowv;
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
