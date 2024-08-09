#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    mss_sw_int: MssSwInt,
    mss_capevnt_sel: MssCapevntSel,
    mss_dma_req_sel: MssDmaReqSel,
    mss_dma1_req_sel: MssDma1ReqSel,
    mss_irq_req_sel: MssIrqReqSel,
    mss_spi_trig_src: MssSpiTrigSrc,
    mss_atcm_mem_init: MssAtcmMemInit,
    mss_atcm_mem_init_done: MssAtcmMemInitDone,
    mss_atcm_mem_init_status: MssAtcmMemInitStatus,
    mss_btcm_mem_init: MssBtcmMemInit,
    mss_btcm_mem_init_done: MssBtcmMemInitDone,
    mss_btcm_mem_init_status: MssBtcmMemInitStatus,
    mss_l2_mem_init: MssL2MemInit,
    mss_l2_mem_init_done: MssL2MemInitDone,
    mss_l2_mem_init_status: MssL2MemInitStatus,
    mss_mailbox_mem_init: MssMailboxMemInit,
    mss_mail_box_mem_init_done: MssMailBoxMemInitDone,
    mss_mailbox_mem_init_status: MssMailboxMemInitStatus,
    mss_retram_mem_init: MssRetramMemInit,
    mss_retram_mem_init_done: MssRetramMemInitDone,
    mss_retram_mem_init_status: MssRetramMemInitStatus,
    mss_spia_mem_init: MssSpiaMemInit,
    mss_spia_mem_init_done: MssSpiaMemInitDone,
    mss_spia_mem_init_status: MssSpiaMemInitStatus,
    mss_spib_mem_init: MssSpibMemInit,
    mss_spib_mem_init_done: MssSpibMemInitDone,
    mss_spib_mem_init_status: MssSpibMemInitStatus,
    mss_tpcc_meminit_start: MssTpccMeminitStart,
    mss_tpcc_meminit_done: MssTpccMeminitDone,
    mss_tpcc_meminit_status: MssTpccMeminitStatus,
    mss_gpadc_mem_init: MssGpadcMemInit,
    mss_gpadc_mem_init_done: MssGpadcMemInitDone,
    mss_gpadc_mem_init_status: MssGpadcMemInitStatus,
    mss_spia_cfg: MssSpiaCfg,
    mss_spib_cfg: MssSpibCfg,
    mss_epwm_cfg: MssEpwmCfg,
    mss_gio_cfg: MssGioCfg,
    _reserved38: [u8; 0x04],
    hw_spare_reg1: HwSpareReg1,
    _reserved39: [u8; 0x0c],
    hw_spare_reg2: HwSpareReg2,
    ccc_err_status: CccErrStatus,
    ccca_cfg0: CccaCfg0,
    ccca_cfg1: CccaCfg1,
    ccca_cfg2: CccaCfg2,
    ccca_cfg3: CccaCfg3,
    ccca_cntval: CccaCntval,
    cccb_cfg0: CccbCfg0,
    cccb_cfg1: CccbCfg1,
    cccb_cfg2: CccbCfg2,
    cccb_cfg3: CccbCfg3,
    cccb_cntval: CccbCntval,
    ccc_dcc_common: CccDccCommon,
    r5_global_config: R5GlobalConfig,
    r5_ahb_en: R5AhbEn,
    r5a_ahb_base: R5aAhbBase,
    r5a_ahb_size: R5aAhbSize,
    r5b_ahb_base: R5bAhbBase,
    r5b_ahb_size: R5bAhbSize,
    r5_tcm_ext_err_en: R5TcmExtErrEn,
    r5_tcm_err_en: R5TcmErrEn,
    r5_init_tcm: R5InitTcm,
    r5_tcm_ecc_wrenz_en: R5TcmEccWrenzEn,
    esm_gating0: EsmGating0,
    esm_gating1: EsmGating1,
    esm_gating2: EsmGating2,
    esm_gating3: EsmGating3,
    esm_gating4: EsmGating4,
    esm_gating5: EsmGating5,
    esm_gating6: EsmGating6,
    esm_gating7: EsmGating7,
    err_parity_atcm0: ErrParityAtcm0,
    err_parity_atcm1: ErrParityAtcm1,
    err_parity_b0tcm0: ErrParityB0tcm0,
    err_parity_b0tcm1: ErrParityB0tcm1,
    err_parity_b1tcm0: ErrParityB1tcm0,
    err_parity_b1tcm1: ErrParityB1tcm1,
    tcm_parity_ctrl: TcmParityCtrl,
    tcm_parity_errfrc: TcmParityErrfrc,
    hw_spare_reg3: HwSpareReg3,
    spia_io_cfg: SpiaIoCfg,
    spib_io_cfg: SpibIoCfg,
    spi_host_irq: SpiHostIrq,
    tptc_dbs_config: TptcDbsConfig,
    tpcc_parity_ctrl: TpccParityCtrl,
    tpcc_parity_status: TpccParityStatus,
    mss_dbg_ack_ctl0: MssDbgAckCtl0,
    mss_dbg_ack_ctl1: MssDbgAckCtl1,
    cpsw_control: CpswControl,
    mss_tpcc_a_erragg_mask: MssTpccAErraggMask,
    mss_tpcc_a_erragg_status: MssTpccAErraggStatus,
    mss_tpcc_a_erragg_status_raw: MssTpccAErraggStatusRaw,
    mss_tpcc_a_intagg_mask: MssTpccAIntaggMask,
    mss_tpcc_a_intagg_status: MssTpccAIntaggStatus,
    mss_tpcc_a_intagg_status_raw: MssTpccAIntaggStatusRaw,
    _reserved94: [u8; 0x18],
    mss_bus_safety_ctrl: MssBusSafetyCtrl,
    mss_cr5a_axi_rd_bus_safety_ctrl: MssCr5aAxiRdBusSafetyCtrl,
    mss_cr5a_axi_rd_bus_safety_fi: MssCr5aAxiRdBusSafetyFi,
    mss_cr5a_axi_rd_bus_safety_err: MssCr5aAxiRdBusSafetyErr,
    mss_cr5a_axi_rd_bus_safety_err_stat_data0: MssCr5aAxiRdBusSafetyErrStatData0,
    mss_cr5a_axi_rd_bus_safety_err_stat_cmd: MssCr5aAxiRdBusSafetyErrStatCmd,
    mss_cr5a_axi_rd_bus_safety_err_stat_read: MssCr5aAxiRdBusSafetyErrStatRead,
    _reserved101: [u8; 0x18],
    mss_cr5a_axi_wr_bus_safety_ctrl: MssCr5aAxiWrBusSafetyCtrl,
    mss_cr5a_axi_wr_bus_safety_fi: MssCr5aAxiWrBusSafetyFi,
    mss_cr5a_axi_wr_bus_safety_err: MssCr5aAxiWrBusSafetyErr,
    mss_cr5a_axi_wr_bus_safety_err_stat_data0: MssCr5aAxiWrBusSafetyErrStatData0,
    mss_cr5a_axi_wr_bus_safety_err_stat_cmd: MssCr5aAxiWrBusSafetyErrStatCmd,
    mss_cr5a_axi_wr_bus_safety_err_stat_write: MssCr5aAxiWrBusSafetyErrStatWrite,
    mss_cr5a_axi_wr_bus_safety_err_stat_writeresp: MssCr5aAxiWrBusSafetyErrStatWriteresp,
    _reserved108: [u8; 0x1c],
    mss_cr5a_axi_s_bus_safety_ctrl: MssCr5aAxiSBusSafetyCtrl,
    mss_cr5a_axi_s_bus_safety_fi: MssCr5aAxiSBusSafetyFi,
    mss_cr5a_axi_s_bus_safety_err: MssCr5aAxiSBusSafetyErr,
    mss_cr5a_axi_s_bus_safety_err_stat_data0: MssCr5aAxiSBusSafetyErrStatData0,
    mss_cr5a_axi_s_bus_safety_err_stat_cmd: MssCr5aAxiSBusSafetyErrStatCmd,
    mss_cr5a_axi_s_bus_safety_err_stat_write: MssCr5aAxiSBusSafetyErrStatWrite,
    mss_cr5a_axi_s_bus_safety_err_stat_read: MssCr5aAxiSBusSafetyErrStatRead,
    mss_cr5a_axi_s_bus_safety_err_stat_writeresp: MssCr5aAxiSBusSafetyErrStatWriteresp,
    _reserved116: [u8; 0x0244],
    mss_l2_a_bus_safety_ctrl: MssL2ABusSafetyCtrl,
    mss_l2_a_bus_safety_fi: MssL2ABusSafetyFi,
    mss_l2_a_bus_safety_err: MssL2ABusSafetyErr,
    mss_l2_a_bus_safety_err_stat_data0: MssL2ABusSafetyErrStatData0,
    mss_l2_a_bus_safety_err_stat_cmd: MssL2ABusSafetyErrStatCmd,
    mss_l2_a_bus_safety_err_stat_write: MssL2ABusSafetyErrStatWrite,
    mss_l2_a_bus_safety_err_stat_read: MssL2ABusSafetyErrStatRead,
    mss_l2_a_bus_safety_err_stat_writeresp: MssL2ABusSafetyErrStatWriteresp,
    mss_l2_b_bus_safety_ctrl: MssL2BBusSafetyCtrl,
    mss_l2_b_bus_safety_fi: MssL2BBusSafetyFi,
    mss_l2_b_bus_safety_err: MssL2BBusSafetyErr,
    mss_l2_b_bus_safety_err_stat_data0: MssL2BBusSafetyErrStatData0,
    mss_l2_b_bus_safety_err_stat_cmd: MssL2BBusSafetyErrStatCmd,
    mss_l2_b_bus_safety_err_stat_write: MssL2BBusSafetyErrStatWrite,
    mss_l2_b_bus_safety_err_stat_read: MssL2BBusSafetyErrStatRead,
    mss_l2_b_bus_safety_err_stat_writeresp: MssL2BBusSafetyErrStatWriteresp,
    _reserved132: [u8; 0x60],
    mss_bus_safety_sec_err_stat0: MssBusSafetySecErrStat0,
    mss_bus_safety_sec_err_stat1: MssBusSafetySecErrStat1,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    hw_reg4: HwReg4,
    hw_reg5: HwReg5,
    hw_reg6: HwReg6,
    hw_reg7: HwReg7,
    _reserved142: [u8; 0x80],
    mss_cr5a_ahb_bus_safety_ctrl: MssCr5aAhbBusSafetyCtrl,
    mss_cr5a_ahb_bus_safety_fi: MssCr5aAhbBusSafetyFi,
    mss_cr5a_ahb_bus_safety_err: MssCr5aAhbBusSafetyErr,
    mss_cr5a_ahb_bus_safety_err_stat_data0: MssCr5aAhbBusSafetyErrStatData0,
    mss_cr5a_ahb_bus_safety_err_stat_cmd: MssCr5aAhbBusSafetyErrStatCmd,
    mss_cr5a_ahb_bus_safety_err_stat_write: MssCr5aAhbBusSafetyErrStatWrite,
    mss_cr5a_ahb_bus_safety_err_stat_read: MssCr5aAhbBusSafetyErrStatRead,
    mss_cr5a_ahb_bus_safety_err_stat_writeresp: MssCr5aAhbBusSafetyErrStatWriteresp,
    _reserved150: [u8; 0x20],
    dmm_ctrl_reg: DmmCtrlReg,
    mss_cr5a_mbox_write_done: MssCr5aMboxWriteDone,
    mss_cr5a_mbox_read_req: MssCr5aMboxReadReq,
    mss_cr5a_mbox_read_done: MssCr5aMboxReadDone,
    _reserved154: [u8; 0x0c],
    mss_pbist_key_rst: MssPbistKeyRst,
    mss_pbist_reg0: MssPbistReg0,
    mss_pbist_reg1: MssPbistReg1,
    mss_pbist_reg2: MssPbistReg2,
    mss_qspi_config: MssQspiConfig,
    mss_stc_control: MssStcControl,
    mss_cti_trig_sel: MssCtiTrigSel,
    mss_dbgss_cti_trig_sel: MssDbgssCtiTrigSel,
    mss_boot_info_reg0: MssBootInfoReg0,
    mss_boot_info_reg1: MssBootInfoReg1,
    mss_boot_info_reg2: MssBootInfoReg2,
    mss_boot_info_reg3: MssBootInfoReg3,
    mss_boot_info_reg4: MssBootInfoReg4,
    mss_boot_info_reg5: MssBootInfoReg5,
    mss_boot_info_reg6: MssBootInfoReg6,
    mss_boot_info_reg7: MssBootInfoReg7,
    mss_tptc_eccaggr_clk_cntrl: MssTptcEccaggrClkCntrl,
    mss_periph_erragg_mask0: MssPeriphErraggMask0,
    mss_periph_erragg_status0: MssPeriphErraggStatus0,
    mss_periph_erragg_status_raw0: MssPeriphErraggStatusRaw0,
    mss_periph_erragg_mask1: MssPeriphErraggMask1,
    mss_periph_erragg_status1: MssPeriphErraggStatus1,
    mss_periph_erragg_status_raw1: MssPeriphErraggStatusRaw1,
    mss_dmm_event0_reg: MssDmmEvent0Reg,
    mss_dmm_event1_reg: MssDmmEvent1Reg,
    mss_dmm_event2_reg: MssDmmEvent2Reg,
    mss_dmm_event3_reg: MssDmmEvent3Reg,
    mss_dmm_event4_reg: MssDmmEvent4Reg,
    mss_dmm_event5_reg: MssDmmEvent5Reg,
    mss_dmm_event6_reg: MssDmmEvent6Reg,
    mss_dmm_event7_reg: MssDmmEvent7Reg,
    mss_dmm_event8_reg: MssDmmEvent8Reg,
    mss_dmm_event9_reg: MssDmmEvent9Reg,
    mss_dmm_event10_reg: MssDmmEvent10Reg,
    mss_dmm_event11_reg: MssDmmEvent11Reg,
    mss_dmm_event12_reg: MssDmmEvent12Reg,
    mss_dmm_event13_reg: MssDmmEvent13Reg,
    mss_dmm_event14_reg: MssDmmEvent14Reg,
    mss_dmm_event15_reg: MssDmmEvent15Reg,
    mss_tptc_boundary_cfg: MssTptcBoundaryCfg,
    mss_tptc_xid_reorder_cfg: MssTptcXidReorderCfg,
    gpadc_ctrl: GpadcCtrl,
    hw_sync_fe_ctrl: HwSyncFeCtrl,
    debugss_csetb_flush: DebugssCsetbFlush,
    analog_wu_status_reg_polarity_inv: AnalogWuStatusRegPolarityInv,
    analog_clk_status_reg_polarity_inv: AnalogClkStatusRegPolarityInv,
    analog_wu_status_reg_grp1_mask: AnalogWuStatusRegGrp1Mask,
    analog_clk_status_reg_grp1_mask: AnalogClkStatusRegGrp1Mask,
    analog_wu_status_reg_grp2_mask: AnalogWuStatusRegGrp2Mask,
    analog_clk_status_reg_grp2_mask: AnalogClkStatusRegGrp2Mask,
    nerror_mask: NerrorMask,
    mss2r5ss_bus_safety_ctrl: Mss2r5ssBusSafetyCtrl,
    mss2r5ss_bus_safety_fi: Mss2r5ssBusSafetyFi,
    mss2r5ss_bus_safety_err: Mss2r5ssBusSafetyErr,
    mss2r5ss_bus_safety_err_stat_data0: Mss2r5ssBusSafetyErrStatData0,
    mss2r5ss_bus_safety_err_stat_cmd: Mss2r5ssBusSafetyErrStatCmd,
    mss2r5ss_bus_safety_err_stat_write: Mss2r5ssBusSafetyErrStatWrite,
    mss2r5ss_bus_safety_err_stat_read: Mss2r5ssBusSafetyErrStatRead,
    mss2r5ss_bus_safety_err_stat_writeresp: Mss2r5ssBusSafetyErrStatWriteresp,
    dss2r5ss_bus_safety_ctrl: Dss2r5ssBusSafetyCtrl,
    dss2r5ss_bus_safety_fi: Dss2r5ssBusSafetyFi,
    dss2r5ss_bus_safety_err: Dss2r5ssBusSafetyErr,
    dss2r5ss_bus_safety_err_stat_data0: Dss2r5ssBusSafetyErrStatData0,
    dss2r5ss_bus_safety_err_stat_cmd: Dss2r5ssBusSafetyErrStatCmd,
    dss2r5ss_bus_safety_err_stat_write: Dss2r5ssBusSafetyErrStatWrite,
    dss2r5ss_bus_safety_err_stat_read: Dss2r5ssBusSafetyErrStatRead,
    dss2r5ss_bus_safety_err_stat_writeresp: Dss2r5ssBusSafetyErrStatWriteresp,
    mss_dmm_access_mode: MssDmmAccessMode,
    cpsw_hw_trig_ctrl: CpswHwTrigCtrl,
    cpsw_hw_trig_val: CpswHwTrigVal,
    cpsw_trig_capture_count: CpswTrigCaptureCount,
    mss_l2_c_bus_safety_ctrl: MssL2CBusSafetyCtrl,
    mss_l2_c_bus_safety_fi: MssL2CBusSafetyFi,
    mss_l2_c_bus_safety_err: MssL2CBusSafetyErr,
    mss_l2_c_bus_safety_err_stat_data0: MssL2CBusSafetyErrStatData0,
    mss_l2_c_bus_safety_err_stat_cmd: MssL2CBusSafetyErrStatCmd,
    mss_l2_c_bus_safety_err_stat_write: MssL2CBusSafetyErrStatWrite,
    mss_l2_c_bus_safety_err_stat_read: MssL2CBusSafetyErrStatRead,
    mss_l2_c_bus_safety_err_stat_writeresp: MssL2CBusSafetyErrStatWriteresp,
    r5ss2dss_bus_safety_ctrl: R5ss2dssBusSafetyCtrl,
    r5ss2dss_bus_safety_fi: R5ss2dssBusSafetyFi,
    r5ss2dss_bus_safety_err: R5ss2dssBusSafetyErr,
    r5ss2dss_bus_safety_err_stat_data0: R5ss2dssBusSafetyErrStatData0,
    r5ss2dss_bus_safety_err_stat_cmd: R5ss2dssBusSafetyErrStatCmd,
    r5ss2dss_bus_safety_err_stat_write: R5ss2dssBusSafetyErrStatWrite,
    r5ss2dss_bus_safety_err_stat_read: R5ss2dssBusSafetyErrStatRead,
    r5ss2dss_bus_safety_err_stat_writeresp: R5ss2dssBusSafetyErrStatWriteresp,
    r5ss2mss_bus_safety_ctrl: R5ss2mssBusSafetyCtrl,
    r5ss2mss_bus_safety_fi: R5ss2mssBusSafetyFi,
    r5ss2mss_bus_safety_err: R5ss2mssBusSafetyErr,
    r5ss2mss_bus_safety_err_stat_data0: R5ss2mssBusSafetyErrStatData0,
    r5ss2mss_bus_safety_err_stat_cmd: R5ss2mssBusSafetyErrStatCmd,
    r5ss2mss_bus_safety_err_stat_write: R5ss2mssBusSafetyErrStatWrite,
    r5ss2mss_bus_safety_err_stat_read: R5ss2mssBusSafetyErrStatRead,
    r5ss2mss_bus_safety_err_stat_writeresp: R5ss2mssBusSafetyErrStatWriteresp,
    nw_packet_count: NwPacketCount,
    nw_packet_count_reset: NwPacketCountReset,
    cpsw_crc_ping_addr: CpswCrcPingAddr,
    cpsw_crc_pong_addr: CpswCrcPongAddr,
    _reserved253: [u8; 0x60],
    r5_control: R5Control,
    r5_rom_eclipse: R5RomEclipse,
    r5_corea_halt: R5CoreaHalt,
    r5_coreb_halt: R5CorebHalt,
    r5_status_reg: R5StatusReg,
    _reserved258: [u8; 0x07bc],
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
    _reserved268: [u8; 0x10],
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
    #[doc = "0x04 - MSS_SW_INT"]
    #[inline(always)]
    pub const fn mss_sw_int(&self) -> &MssSwInt {
        &self.mss_sw_int
    }
    #[doc = "0x08 - MSS_CAPEVNT_SEL"]
    #[inline(always)]
    pub const fn mss_capevnt_sel(&self) -> &MssCapevntSel {
        &self.mss_capevnt_sel
    }
    #[doc = "0x0c - MSS_DMA_REQ_SEL"]
    #[inline(always)]
    pub const fn mss_dma_req_sel(&self) -> &MssDmaReqSel {
        &self.mss_dma_req_sel
    }
    #[doc = "0x10 - MSS_DMA1_REQ_SEL"]
    #[inline(always)]
    pub const fn mss_dma1_req_sel(&self) -> &MssDma1ReqSel {
        &self.mss_dma1_req_sel
    }
    #[doc = "0x14 - MSS_IRQ_REQ_SEL"]
    #[inline(always)]
    pub const fn mss_irq_req_sel(&self) -> &MssIrqReqSel {
        &self.mss_irq_req_sel
    }
    #[doc = "0x18 - MSS_SPI_TRIG_SRC"]
    #[inline(always)]
    pub const fn mss_spi_trig_src(&self) -> &MssSpiTrigSrc {
        &self.mss_spi_trig_src
    }
    #[doc = "0x1c - MSS_ATCM_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_atcm_mem_init(&self) -> &MssAtcmMemInit {
        &self.mss_atcm_mem_init
    }
    #[doc = "0x20 - MSS_ATCM_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_atcm_mem_init_done(&self) -> &MssAtcmMemInitDone {
        &self.mss_atcm_mem_init_done
    }
    #[doc = "0x24 - MSS_ATCM_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_atcm_mem_init_status(&self) -> &MssAtcmMemInitStatus {
        &self.mss_atcm_mem_init_status
    }
    #[doc = "0x28 - MSS_BTCM_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_btcm_mem_init(&self) -> &MssBtcmMemInit {
        &self.mss_btcm_mem_init
    }
    #[doc = "0x2c - MSS_BTCM_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_btcm_mem_init_done(&self) -> &MssBtcmMemInitDone {
        &self.mss_btcm_mem_init_done
    }
    #[doc = "0x30 - MSS_BTCM_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_btcm_mem_init_status(&self) -> &MssBtcmMemInitStatus {
        &self.mss_btcm_mem_init_status
    }
    #[doc = "0x34 - MSS_L2_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_l2_mem_init(&self) -> &MssL2MemInit {
        &self.mss_l2_mem_init
    }
    #[doc = "0x38 - MSS_L2_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_l2_mem_init_done(&self) -> &MssL2MemInitDone {
        &self.mss_l2_mem_init_done
    }
    #[doc = "0x3c - MSS_L2_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_l2_mem_init_status(&self) -> &MssL2MemInitStatus {
        &self.mss_l2_mem_init_status
    }
    #[doc = "0x40 - MSS_MAILBOX_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_mailbox_mem_init(&self) -> &MssMailboxMemInit {
        &self.mss_mailbox_mem_init
    }
    #[doc = "0x44 - MSS_MAIlBOX_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_mail_box_mem_init_done(&self) -> &MssMailBoxMemInitDone {
        &self.mss_mail_box_mem_init_done
    }
    #[doc = "0x48 - MSS_MAILBOX_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_mailbox_mem_init_status(&self) -> &MssMailboxMemInitStatus {
        &self.mss_mailbox_mem_init_status
    }
    #[doc = "0x4c - MSS_RETRAM_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_retram_mem_init(&self) -> &MssRetramMemInit {
        &self.mss_retram_mem_init
    }
    #[doc = "0x50 - MSS_RETRAM_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_retram_mem_init_done(&self) -> &MssRetramMemInitDone {
        &self.mss_retram_mem_init_done
    }
    #[doc = "0x54 - MSS_RETRAM_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_retram_mem_init_status(&self) -> &MssRetramMemInitStatus {
        &self.mss_retram_mem_init_status
    }
    #[doc = "0x58 - MSS_SPIA_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_spia_mem_init(&self) -> &MssSpiaMemInit {
        &self.mss_spia_mem_init
    }
    #[doc = "0x5c - MSS_SPIA_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_spia_mem_init_done(&self) -> &MssSpiaMemInitDone {
        &self.mss_spia_mem_init_done
    }
    #[doc = "0x60 - MSS_SPIA_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_spia_mem_init_status(&self) -> &MssSpiaMemInitStatus {
        &self.mss_spia_mem_init_status
    }
    #[doc = "0x64 - MSS_SPIB_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_spib_mem_init(&self) -> &MssSpibMemInit {
        &self.mss_spib_mem_init
    }
    #[doc = "0x68 - MSS_SPIB_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_spib_mem_init_done(&self) -> &MssSpibMemInitDone {
        &self.mss_spib_mem_init_done
    }
    #[doc = "0x6c - MSS_SPIB_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_spib_mem_init_status(&self) -> &MssSpibMemInitStatus {
        &self.mss_spib_mem_init_status
    }
    #[doc = "0x70 - MSS_TPCC_MEMINIT_START"]
    #[inline(always)]
    pub const fn mss_tpcc_meminit_start(&self) -> &MssTpccMeminitStart {
        &self.mss_tpcc_meminit_start
    }
    #[doc = "0x74 - MSS_TPCC_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn mss_tpcc_meminit_done(&self) -> &MssTpccMeminitDone {
        &self.mss_tpcc_meminit_done
    }
    #[doc = "0x78 - MSS_TPCC_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn mss_tpcc_meminit_status(&self) -> &MssTpccMeminitStatus {
        &self.mss_tpcc_meminit_status
    }
    #[doc = "0x7c - MSS_GPADC_MEM_INIT"]
    #[inline(always)]
    pub const fn mss_gpadc_mem_init(&self) -> &MssGpadcMemInit {
        &self.mss_gpadc_mem_init
    }
    #[doc = "0x80 - MSS_GPADC_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mss_gpadc_mem_init_done(&self) -> &MssGpadcMemInitDone {
        &self.mss_gpadc_mem_init_done
    }
    #[doc = "0x84 - MSS_GPADC_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mss_gpadc_mem_init_status(&self) -> &MssGpadcMemInitStatus {
        &self.mss_gpadc_mem_init_status
    }
    #[doc = "0x88 - RESERVED: Dont Use"]
    #[inline(always)]
    pub const fn mss_spia_cfg(&self) -> &MssSpiaCfg {
        &self.mss_spia_cfg
    }
    #[doc = "0x8c - MSS_SPIB_CFG"]
    #[inline(always)]
    pub const fn mss_spib_cfg(&self) -> &MssSpibCfg {
        &self.mss_spib_cfg
    }
    #[doc = "0x90 - MSS_EPWM_CFG"]
    #[inline(always)]
    pub const fn mss_epwm_cfg(&self) -> &MssEpwmCfg {
        &self.mss_epwm_cfg
    }
    #[doc = "0x94 - MSS_GIO_CFG"]
    #[inline(always)]
    pub const fn mss_gio_cfg(&self) -> &MssGioCfg {
        &self.mss_gio_cfg
    }
    #[doc = "0x9c - HW_SPARE_REG1"]
    #[inline(always)]
    pub const fn hw_spare_reg1(&self) -> &HwSpareReg1 {
        &self.hw_spare_reg1
    }
    #[doc = "0xac - HW_SPARE_REG2"]
    #[inline(always)]
    pub const fn hw_spare_reg2(&self) -> &HwSpareReg2 {
        &self.hw_spare_reg2
    }
    #[doc = "0xb0 - CCC_ERR_STATUS"]
    #[inline(always)]
    pub const fn ccc_err_status(&self) -> &CccErrStatus {
        &self.ccc_err_status
    }
    #[doc = "0xb4 - CCCA_CFG0"]
    #[inline(always)]
    pub const fn ccca_cfg0(&self) -> &CccaCfg0 {
        &self.ccca_cfg0
    }
    #[doc = "0xb8 - CCCA_CFG1"]
    #[inline(always)]
    pub const fn ccca_cfg1(&self) -> &CccaCfg1 {
        &self.ccca_cfg1
    }
    #[doc = "0xbc - CCCA_CFG2"]
    #[inline(always)]
    pub const fn ccca_cfg2(&self) -> &CccaCfg2 {
        &self.ccca_cfg2
    }
    #[doc = "0xc0 - CCCA_CFG3"]
    #[inline(always)]
    pub const fn ccca_cfg3(&self) -> &CccaCfg3 {
        &self.ccca_cfg3
    }
    #[doc = "0xc4 - CCCA_CNTVAL"]
    #[inline(always)]
    pub const fn ccca_cntval(&self) -> &CccaCntval {
        &self.ccca_cntval
    }
    #[doc = "0xc8 - CCCB_CFG0"]
    #[inline(always)]
    pub const fn cccb_cfg0(&self) -> &CccbCfg0 {
        &self.cccb_cfg0
    }
    #[doc = "0xcc - CCCB_CFG1"]
    #[inline(always)]
    pub const fn cccb_cfg1(&self) -> &CccbCfg1 {
        &self.cccb_cfg1
    }
    #[doc = "0xd0 - CCCB_CFG2"]
    #[inline(always)]
    pub const fn cccb_cfg2(&self) -> &CccbCfg2 {
        &self.cccb_cfg2
    }
    #[doc = "0xd4 - CCCB_CFG3"]
    #[inline(always)]
    pub const fn cccb_cfg3(&self) -> &CccbCfg3 {
        &self.cccb_cfg3
    }
    #[doc = "0xd8 - CCCB_CNTVAL"]
    #[inline(always)]
    pub const fn cccb_cntval(&self) -> &CccbCntval {
        &self.cccb_cntval
    }
    #[doc = "0xdc - CCC_DCC_COMMON"]
    #[inline(always)]
    pub const fn ccc_dcc_common(&self) -> &CccDccCommon {
        &self.ccc_dcc_common
    }
    #[doc = "0xe0 - R5_GLOBAL_CONFIG"]
    #[inline(always)]
    pub const fn r5_global_config(&self) -> &R5GlobalConfig {
        &self.r5_global_config
    }
    #[doc = "0xe4 - R5_AHB_EN"]
    #[inline(always)]
    pub const fn r5_ahb_en(&self) -> &R5AhbEn {
        &self.r5_ahb_en
    }
    #[doc = "0xe8 - R5A_AHB_BASE"]
    #[inline(always)]
    pub const fn r5a_ahb_base(&self) -> &R5aAhbBase {
        &self.r5a_ahb_base
    }
    #[doc = "0xec - R5A_AHB_SIZE"]
    #[inline(always)]
    pub const fn r5a_ahb_size(&self) -> &R5aAhbSize {
        &self.r5a_ahb_size
    }
    #[doc = "0xf0 - R5B_AHB_BASE"]
    #[inline(always)]
    pub const fn r5b_ahb_base(&self) -> &R5bAhbBase {
        &self.r5b_ahb_base
    }
    #[doc = "0xf4 - R5B_AHB_SIZE"]
    #[inline(always)]
    pub const fn r5b_ahb_size(&self) -> &R5bAhbSize {
        &self.r5b_ahb_size
    }
    #[doc = "0xf8 - R5_TCM_EXT_ERR_EN"]
    #[inline(always)]
    pub const fn r5_tcm_ext_err_en(&self) -> &R5TcmExtErrEn {
        &self.r5_tcm_ext_err_en
    }
    #[doc = "0xfc - R5_TCM_ERR_EN"]
    #[inline(always)]
    pub const fn r5_tcm_err_en(&self) -> &R5TcmErrEn {
        &self.r5_tcm_err_en
    }
    #[doc = "0x100 - R5_INIT_TCM"]
    #[inline(always)]
    pub const fn r5_init_tcm(&self) -> &R5InitTcm {
        &self.r5_init_tcm
    }
    #[doc = "0x104 - R5_TCM_ECC_WRENZ_EN"]
    #[inline(always)]
    pub const fn r5_tcm_ecc_wrenz_en(&self) -> &R5TcmEccWrenzEn {
        &self.r5_tcm_ecc_wrenz_en
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
    #[doc = "0x118 - ESM_GATING4"]
    #[inline(always)]
    pub const fn esm_gating4(&self) -> &EsmGating4 {
        &self.esm_gating4
    }
    #[doc = "0x11c - ESM_GATING5"]
    #[inline(always)]
    pub const fn esm_gating5(&self) -> &EsmGating5 {
        &self.esm_gating5
    }
    #[doc = "0x120 - ESM_GATING6"]
    #[inline(always)]
    pub const fn esm_gating6(&self) -> &EsmGating6 {
        &self.esm_gating6
    }
    #[doc = "0x124 - ESM_GATING7"]
    #[inline(always)]
    pub const fn esm_gating7(&self) -> &EsmGating7 {
        &self.esm_gating7
    }
    #[doc = "0x128 - ERR_PARITY_ATCM0"]
    #[inline(always)]
    pub const fn err_parity_atcm0(&self) -> &ErrParityAtcm0 {
        &self.err_parity_atcm0
    }
    #[doc = "0x12c - ERR_PARITY_ATCM1"]
    #[inline(always)]
    pub const fn err_parity_atcm1(&self) -> &ErrParityAtcm1 {
        &self.err_parity_atcm1
    }
    #[doc = "0x130 - ERR_PARITY_B0TCM0"]
    #[inline(always)]
    pub const fn err_parity_b0tcm0(&self) -> &ErrParityB0tcm0 {
        &self.err_parity_b0tcm0
    }
    #[doc = "0x134 - ERR_PARITY_B0TCM1"]
    #[inline(always)]
    pub const fn err_parity_b0tcm1(&self) -> &ErrParityB0tcm1 {
        &self.err_parity_b0tcm1
    }
    #[doc = "0x138 - ERR_PARITY_B1TCM0"]
    #[inline(always)]
    pub const fn err_parity_b1tcm0(&self) -> &ErrParityB1tcm0 {
        &self.err_parity_b1tcm0
    }
    #[doc = "0x13c - ERR_PARITY_B1TCM1"]
    #[inline(always)]
    pub const fn err_parity_b1tcm1(&self) -> &ErrParityB1tcm1 {
        &self.err_parity_b1tcm1
    }
    #[doc = "0x140 - TCM_PARITY_CTRL"]
    #[inline(always)]
    pub const fn tcm_parity_ctrl(&self) -> &TcmParityCtrl {
        &self.tcm_parity_ctrl
    }
    #[doc = "0x144 - TCM_PARITY_ERRFRC"]
    #[inline(always)]
    pub const fn tcm_parity_errfrc(&self) -> &TcmParityErrfrc {
        &self.tcm_parity_errfrc
    }
    #[doc = "0x148 - HW_SPARE_REG3"]
    #[inline(always)]
    pub const fn hw_spare_reg3(&self) -> &HwSpareReg3 {
        &self.hw_spare_reg3
    }
    #[doc = "0x14c - SPIA_IO_CFG"]
    #[inline(always)]
    pub const fn spia_io_cfg(&self) -> &SpiaIoCfg {
        &self.spia_io_cfg
    }
    #[doc = "0x150 - SPIB_IO_CFG"]
    #[inline(always)]
    pub const fn spib_io_cfg(&self) -> &SpibIoCfg {
        &self.spib_io_cfg
    }
    #[doc = "0x154 - SPI_HOST_IRQ"]
    #[inline(always)]
    pub const fn spi_host_irq(&self) -> &SpiHostIrq {
        &self.spi_host_irq
    }
    #[doc = "0x158 - TPTC_DBS_CONFIG"]
    #[inline(always)]
    pub const fn tptc_dbs_config(&self) -> &TptcDbsConfig {
        &self.tptc_dbs_config
    }
    #[doc = "0x15c - TPCC_PARITY_CTRL"]
    #[inline(always)]
    pub const fn tpcc_parity_ctrl(&self) -> &TpccParityCtrl {
        &self.tpcc_parity_ctrl
    }
    #[doc = "0x160 - TPCC_PARITY_STATUS"]
    #[inline(always)]
    pub const fn tpcc_parity_status(&self) -> &TpccParityStatus {
        &self.tpcc_parity_status
    }
    #[doc = "0x164 - MSS_DBG_ACK_CTL0"]
    #[inline(always)]
    pub const fn mss_dbg_ack_ctl0(&self) -> &MssDbgAckCtl0 {
        &self.mss_dbg_ack_ctl0
    }
    #[doc = "0x168 - MSS_DBG_ACK_CTL1"]
    #[inline(always)]
    pub const fn mss_dbg_ack_ctl1(&self) -> &MssDbgAckCtl1 {
        &self.mss_dbg_ack_ctl1
    }
    #[doc = "0x16c - CPSW_CONTROL"]
    #[inline(always)]
    pub const fn cpsw_control(&self) -> &CpswControl {
        &self.cpsw_control
    }
    #[doc = "0x170 - MSS_TPCC_A_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn mss_tpcc_a_erragg_mask(&self) -> &MssTpccAErraggMask {
        &self.mss_tpcc_a_erragg_mask
    }
    #[doc = "0x174 - MSS_TPCC_A_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn mss_tpcc_a_erragg_status(&self) -> &MssTpccAErraggStatus {
        &self.mss_tpcc_a_erragg_status
    }
    #[doc = "0x178 - MSS_TPCC_A_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn mss_tpcc_a_erragg_status_raw(&self) -> &MssTpccAErraggStatusRaw {
        &self.mss_tpcc_a_erragg_status_raw
    }
    #[doc = "0x17c - MSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    pub const fn mss_tpcc_a_intagg_mask(&self) -> &MssTpccAIntaggMask {
        &self.mss_tpcc_a_intagg_mask
    }
    #[doc = "0x180 - MSS_TPCC_A_INTAGG_STATUS"]
    #[inline(always)]
    pub const fn mss_tpcc_a_intagg_status(&self) -> &MssTpccAIntaggStatus {
        &self.mss_tpcc_a_intagg_status
    }
    #[doc = "0x184 - MSS_TPCC_A_INTAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn mss_tpcc_a_intagg_status_raw(&self) -> &MssTpccAIntaggStatusRaw {
        &self.mss_tpcc_a_intagg_status_raw
    }
    #[doc = "0x1a0 - MSS_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_bus_safety_ctrl(&self) -> &MssBusSafetyCtrl {
        &self.mss_bus_safety_ctrl
    }
    #[doc = "0x1a4 - MSS_CR5A_AXI_RD_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_ctrl(&self) -> &MssCr5aAxiRdBusSafetyCtrl {
        &self.mss_cr5a_axi_rd_bus_safety_ctrl
    }
    #[doc = "0x1a8 - MSS_CR5A_AXI_RD_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_fi(&self) -> &MssCr5aAxiRdBusSafetyFi {
        &self.mss_cr5a_axi_rd_bus_safety_fi
    }
    #[doc = "0x1ac - MSS_CR5A_AXI_RD_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_err(&self) -> &MssCr5aAxiRdBusSafetyErr {
        &self.mss_cr5a_axi_rd_bus_safety_err
    }
    #[doc = "0x1b0 - MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_err_stat_data0(
        &self,
    ) -> &MssCr5aAxiRdBusSafetyErrStatData0 {
        &self.mss_cr5a_axi_rd_bus_safety_err_stat_data0
    }
    #[doc = "0x1b4 - MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_err_stat_cmd(
        &self,
    ) -> &MssCr5aAxiRdBusSafetyErrStatCmd {
        &self.mss_cr5a_axi_rd_bus_safety_err_stat_cmd
    }
    #[doc = "0x1b8 - MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_rd_bus_safety_err_stat_read(
        &self,
    ) -> &MssCr5aAxiRdBusSafetyErrStatRead {
        &self.mss_cr5a_axi_rd_bus_safety_err_stat_read
    }
    #[doc = "0x1d4 - MSS_CR5A_AXI_WR_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_ctrl(&self) -> &MssCr5aAxiWrBusSafetyCtrl {
        &self.mss_cr5a_axi_wr_bus_safety_ctrl
    }
    #[doc = "0x1d8 - MSS_CR5A_AXI_WR_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_fi(&self) -> &MssCr5aAxiWrBusSafetyFi {
        &self.mss_cr5a_axi_wr_bus_safety_fi
    }
    #[doc = "0x1dc - MSS_CR5A_AXI_WR_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_err(&self) -> &MssCr5aAxiWrBusSafetyErr {
        &self.mss_cr5a_axi_wr_bus_safety_err
    }
    #[doc = "0x1e0 - MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_err_stat_data0(
        &self,
    ) -> &MssCr5aAxiWrBusSafetyErrStatData0 {
        &self.mss_cr5a_axi_wr_bus_safety_err_stat_data0
    }
    #[doc = "0x1e4 - MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_err_stat_cmd(
        &self,
    ) -> &MssCr5aAxiWrBusSafetyErrStatCmd {
        &self.mss_cr5a_axi_wr_bus_safety_err_stat_cmd
    }
    #[doc = "0x1e8 - MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_err_stat_write(
        &self,
    ) -> &MssCr5aAxiWrBusSafetyErrStatWrite {
        &self.mss_cr5a_axi_wr_bus_safety_err_stat_write
    }
    #[doc = "0x1ec - MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_wr_bus_safety_err_stat_writeresp(
        &self,
    ) -> &MssCr5aAxiWrBusSafetyErrStatWriteresp {
        &self.mss_cr5a_axi_wr_bus_safety_err_stat_writeresp
    }
    #[doc = "0x20c - MSS_CR5A_AXI_S_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_ctrl(&self) -> &MssCr5aAxiSBusSafetyCtrl {
        &self.mss_cr5a_axi_s_bus_safety_ctrl
    }
    #[doc = "0x210 - MSS_CR5A_AXI_S_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_fi(&self) -> &MssCr5aAxiSBusSafetyFi {
        &self.mss_cr5a_axi_s_bus_safety_fi
    }
    #[doc = "0x214 - MSS_CR5A_AXI_S_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err(&self) -> &MssCr5aAxiSBusSafetyErr {
        &self.mss_cr5a_axi_s_bus_safety_err
    }
    #[doc = "0x218 - MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err_stat_data0(
        &self,
    ) -> &MssCr5aAxiSBusSafetyErrStatData0 {
        &self.mss_cr5a_axi_s_bus_safety_err_stat_data0
    }
    #[doc = "0x21c - MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err_stat_cmd(&self) -> &MssCr5aAxiSBusSafetyErrStatCmd {
        &self.mss_cr5a_axi_s_bus_safety_err_stat_cmd
    }
    #[doc = "0x220 - MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err_stat_write(
        &self,
    ) -> &MssCr5aAxiSBusSafetyErrStatWrite {
        &self.mss_cr5a_axi_s_bus_safety_err_stat_write
    }
    #[doc = "0x224 - MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err_stat_read(
        &self,
    ) -> &MssCr5aAxiSBusSafetyErrStatRead {
        &self.mss_cr5a_axi_s_bus_safety_err_stat_read
    }
    #[doc = "0x228 - MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_cr5a_axi_s_bus_safety_err_stat_writeresp(
        &self,
    ) -> &MssCr5aAxiSBusSafetyErrStatWriteresp {
        &self.mss_cr5a_axi_s_bus_safety_err_stat_writeresp
    }
    #[doc = "0x470 - MSS_L2_A_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_ctrl(&self) -> &MssL2ABusSafetyCtrl {
        &self.mss_l2_a_bus_safety_ctrl
    }
    #[doc = "0x474 - MSS_L2_A_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_fi(&self) -> &MssL2ABusSafetyFi {
        &self.mss_l2_a_bus_safety_fi
    }
    #[doc = "0x478 - MSS_L2_A_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err(&self) -> &MssL2ABusSafetyErr {
        &self.mss_l2_a_bus_safety_err
    }
    #[doc = "0x47c - MSS_L2_A_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err_stat_data0(&self) -> &MssL2ABusSafetyErrStatData0 {
        &self.mss_l2_a_bus_safety_err_stat_data0
    }
    #[doc = "0x480 - MSS_L2_A_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err_stat_cmd(&self) -> &MssL2ABusSafetyErrStatCmd {
        &self.mss_l2_a_bus_safety_err_stat_cmd
    }
    #[doc = "0x484 - MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err_stat_write(&self) -> &MssL2ABusSafetyErrStatWrite {
        &self.mss_l2_a_bus_safety_err_stat_write
    }
    #[doc = "0x488 - MSS_L2_A_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err_stat_read(&self) -> &MssL2ABusSafetyErrStatRead {
        &self.mss_l2_a_bus_safety_err_stat_read
    }
    #[doc = "0x48c - MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_l2_a_bus_safety_err_stat_writeresp(&self) -> &MssL2ABusSafetyErrStatWriteresp {
        &self.mss_l2_a_bus_safety_err_stat_writeresp
    }
    #[doc = "0x490 - MSS_L2_B_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_ctrl(&self) -> &MssL2BBusSafetyCtrl {
        &self.mss_l2_b_bus_safety_ctrl
    }
    #[doc = "0x494 - MSS_L2_B_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_fi(&self) -> &MssL2BBusSafetyFi {
        &self.mss_l2_b_bus_safety_fi
    }
    #[doc = "0x498 - MSS_L2_B_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err(&self) -> &MssL2BBusSafetyErr {
        &self.mss_l2_b_bus_safety_err
    }
    #[doc = "0x49c - MSS_L2_B_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err_stat_data0(&self) -> &MssL2BBusSafetyErrStatData0 {
        &self.mss_l2_b_bus_safety_err_stat_data0
    }
    #[doc = "0x4a0 - MSS_L2_B_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err_stat_cmd(&self) -> &MssL2BBusSafetyErrStatCmd {
        &self.mss_l2_b_bus_safety_err_stat_cmd
    }
    #[doc = "0x4a4 - MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err_stat_write(&self) -> &MssL2BBusSafetyErrStatWrite {
        &self.mss_l2_b_bus_safety_err_stat_write
    }
    #[doc = "0x4a8 - MSS_L2_B_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err_stat_read(&self) -> &MssL2BBusSafetyErrStatRead {
        &self.mss_l2_b_bus_safety_err_stat_read
    }
    #[doc = "0x4ac - MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_l2_b_bus_safety_err_stat_writeresp(&self) -> &MssL2BBusSafetyErrStatWriteresp {
        &self.mss_l2_b_bus_safety_err_stat_writeresp
    }
    #[doc = "0x510 - MSS_BUS_SAFETY_SEC_ERR_STAT0"]
    #[inline(always)]
    pub const fn mss_bus_safety_sec_err_stat0(&self) -> &MssBusSafetySecErrStat0 {
        &self.mss_bus_safety_sec_err_stat0
    }
    #[doc = "0x514 - MSS_BUS_SAFETY_SEC_ERR_STAT1"]
    #[inline(always)]
    pub const fn mss_bus_safety_sec_err_stat1(&self) -> &MssBusSafetySecErrStat1 {
        &self.mss_bus_safety_sec_err_stat1
    }
    #[doc = "0x518 - HW_REG0"]
    #[inline(always)]
    pub const fn hw_reg0(&self) -> &HwReg0 {
        &self.hw_reg0
    }
    #[doc = "0x51c - HW_REG1"]
    #[inline(always)]
    pub const fn hw_reg1(&self) -> &HwReg1 {
        &self.hw_reg1
    }
    #[doc = "0x520 - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x524 - HW_REG3"]
    #[inline(always)]
    pub const fn hw_reg3(&self) -> &HwReg3 {
        &self.hw_reg3
    }
    #[doc = "0x528 - HW_REG4"]
    #[inline(always)]
    pub const fn hw_reg4(&self) -> &HwReg4 {
        &self.hw_reg4
    }
    #[doc = "0x52c - HW_REG5"]
    #[inline(always)]
    pub const fn hw_reg5(&self) -> &HwReg5 {
        &self.hw_reg5
    }
    #[doc = "0x530 - HW_REG6"]
    #[inline(always)]
    pub const fn hw_reg6(&self) -> &HwReg6 {
        &self.hw_reg6
    }
    #[doc = "0x534 - HW_REG7"]
    #[inline(always)]
    pub const fn hw_reg7(&self) -> &HwReg7 {
        &self.hw_reg7
    }
    #[doc = "0x5b8 - MSS_CR5A_AHB_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_ctrl(&self) -> &MssCr5aAhbBusSafetyCtrl {
        &self.mss_cr5a_ahb_bus_safety_ctrl
    }
    #[doc = "0x5bc - MSS_CR5A_AHB_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_fi(&self) -> &MssCr5aAhbBusSafetyFi {
        &self.mss_cr5a_ahb_bus_safety_fi
    }
    #[doc = "0x5c0 - MSS_CR5A_AHB_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err(&self) -> &MssCr5aAhbBusSafetyErr {
        &self.mss_cr5a_ahb_bus_safety_err
    }
    #[doc = "0x5c4 - MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err_stat_data0(&self) -> &MssCr5aAhbBusSafetyErrStatData0 {
        &self.mss_cr5a_ahb_bus_safety_err_stat_data0
    }
    #[doc = "0x5c8 - MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err_stat_cmd(&self) -> &MssCr5aAhbBusSafetyErrStatCmd {
        &self.mss_cr5a_ahb_bus_safety_err_stat_cmd
    }
    #[doc = "0x5cc - MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err_stat_write(&self) -> &MssCr5aAhbBusSafetyErrStatWrite {
        &self.mss_cr5a_ahb_bus_safety_err_stat_write
    }
    #[doc = "0x5d0 - MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err_stat_read(&self) -> &MssCr5aAhbBusSafetyErrStatRead {
        &self.mss_cr5a_ahb_bus_safety_err_stat_read
    }
    #[doc = "0x5d4 - MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_cr5a_ahb_bus_safety_err_stat_writeresp(
        &self,
    ) -> &MssCr5aAhbBusSafetyErrStatWriteresp {
        &self.mss_cr5a_ahb_bus_safety_err_stat_writeresp
    }
    #[doc = "0x5f8 - DMM_CTRL_REG"]
    #[inline(always)]
    pub const fn dmm_ctrl_reg(&self) -> &DmmCtrlReg {
        &self.dmm_ctrl_reg
    }
    #[doc = "0x5fc - MSS_CR5A_MBOX_WRITE_DONE"]
    #[inline(always)]
    pub const fn mss_cr5a_mbox_write_done(&self) -> &MssCr5aMboxWriteDone {
        &self.mss_cr5a_mbox_write_done
    }
    #[doc = "0x600 - MSS_CR5A_MBOX_READ_REQ"]
    #[inline(always)]
    pub const fn mss_cr5a_mbox_read_req(&self) -> &MssCr5aMboxReadReq {
        &self.mss_cr5a_mbox_read_req
    }
    #[doc = "0x604 - MSS_CR5A_MBOX_READ_DONE"]
    #[inline(always)]
    pub const fn mss_cr5a_mbox_read_done(&self) -> &MssCr5aMboxReadDone {
        &self.mss_cr5a_mbox_read_done
    }
    #[doc = "0x614 - MSS_PBIST_KEY_RST"]
    #[inline(always)]
    pub const fn mss_pbist_key_rst(&self) -> &MssPbistKeyRst {
        &self.mss_pbist_key_rst
    }
    #[doc = "0x618 - MSS_PBIST_REG0"]
    #[inline(always)]
    pub const fn mss_pbist_reg0(&self) -> &MssPbistReg0 {
        &self.mss_pbist_reg0
    }
    #[doc = "0x61c - MSS_PBIST_REG1"]
    #[inline(always)]
    pub const fn mss_pbist_reg1(&self) -> &MssPbistReg1 {
        &self.mss_pbist_reg1
    }
    #[doc = "0x620 - MSS_PBIST_REG2"]
    #[inline(always)]
    pub const fn mss_pbist_reg2(&self) -> &MssPbistReg2 {
        &self.mss_pbist_reg2
    }
    #[doc = "0x624 - MSS_QSPI_CONFIG"]
    #[inline(always)]
    pub const fn mss_qspi_config(&self) -> &MssQspiConfig {
        &self.mss_qspi_config
    }
    #[doc = "0x628 - MSS_STC_CONTROL"]
    #[inline(always)]
    pub const fn mss_stc_control(&self) -> &MssStcControl {
        &self.mss_stc_control
    }
    #[doc = "0x62c - MSS_CTI_TRIG_SEL"]
    #[inline(always)]
    pub const fn mss_cti_trig_sel(&self) -> &MssCtiTrigSel {
        &self.mss_cti_trig_sel
    }
    #[doc = "0x630 - MSS_DBGSS_CTI_TRIG_SEL"]
    #[inline(always)]
    pub const fn mss_dbgss_cti_trig_sel(&self) -> &MssDbgssCtiTrigSel {
        &self.mss_dbgss_cti_trig_sel
    }
    #[doc = "0x634 - MSS_BOOT_INFO_REG0"]
    #[inline(always)]
    pub const fn mss_boot_info_reg0(&self) -> &MssBootInfoReg0 {
        &self.mss_boot_info_reg0
    }
    #[doc = "0x638 - MSS_BOOT_INFO_REG1"]
    #[inline(always)]
    pub const fn mss_boot_info_reg1(&self) -> &MssBootInfoReg1 {
        &self.mss_boot_info_reg1
    }
    #[doc = "0x63c - MSS_BOOT_INFO_REG2"]
    #[inline(always)]
    pub const fn mss_boot_info_reg2(&self) -> &MssBootInfoReg2 {
        &self.mss_boot_info_reg2
    }
    #[doc = "0x640 - MSS_BOOT_INFO_REG3"]
    #[inline(always)]
    pub const fn mss_boot_info_reg3(&self) -> &MssBootInfoReg3 {
        &self.mss_boot_info_reg3
    }
    #[doc = "0x644 - MSS_BOOT_INFO_REG4"]
    #[inline(always)]
    pub const fn mss_boot_info_reg4(&self) -> &MssBootInfoReg4 {
        &self.mss_boot_info_reg4
    }
    #[doc = "0x648 - MSS_BOOT_INFO_REG5"]
    #[inline(always)]
    pub const fn mss_boot_info_reg5(&self) -> &MssBootInfoReg5 {
        &self.mss_boot_info_reg5
    }
    #[doc = "0x64c - MSS_BOOT_INFO_REG6"]
    #[inline(always)]
    pub const fn mss_boot_info_reg6(&self) -> &MssBootInfoReg6 {
        &self.mss_boot_info_reg6
    }
    #[doc = "0x650 - MSS_BOOT_INFO_REG7"]
    #[inline(always)]
    pub const fn mss_boot_info_reg7(&self) -> &MssBootInfoReg7 {
        &self.mss_boot_info_reg7
    }
    #[doc = "0x654 - MSS_TPTC_ECCAGGR_CLK_CNTRL"]
    #[inline(always)]
    pub const fn mss_tptc_eccaggr_clk_cntrl(&self) -> &MssTptcEccaggrClkCntrl {
        &self.mss_tptc_eccaggr_clk_cntrl
    }
    #[doc = "0x658 - MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub const fn mss_periph_erragg_mask0(&self) -> &MssPeriphErraggMask0 {
        &self.mss_periph_erragg_mask0
    }
    #[doc = "0x65c - MSS_PERIPH_ERRAGG_STATUS0"]
    #[inline(always)]
    pub const fn mss_periph_erragg_status0(&self) -> &MssPeriphErraggStatus0 {
        &self.mss_periph_erragg_status0
    }
    #[doc = "0x660 - MSS_PERIPH_ERRAGG_STATUS_RAW0"]
    #[inline(always)]
    pub const fn mss_periph_erragg_status_raw0(&self) -> &MssPeriphErraggStatusRaw0 {
        &self.mss_periph_erragg_status_raw0
    }
    #[doc = "0x664 - MSS_PERIPH_ERRAGG_MASK1"]
    #[inline(always)]
    pub const fn mss_periph_erragg_mask1(&self) -> &MssPeriphErraggMask1 {
        &self.mss_periph_erragg_mask1
    }
    #[doc = "0x668 - MSS_PERIPH_ERRAGG_STATUS1"]
    #[inline(always)]
    pub const fn mss_periph_erragg_status1(&self) -> &MssPeriphErraggStatus1 {
        &self.mss_periph_erragg_status1
    }
    #[doc = "0x66c - MSS_PERIPH_ERRAGG_STATUS_RAW1"]
    #[inline(always)]
    pub const fn mss_periph_erragg_status_raw1(&self) -> &MssPeriphErraggStatusRaw1 {
        &self.mss_periph_erragg_status_raw1
    }
    #[doc = "0x670 - MSS_DMM_EVENT0_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event0_reg(&self) -> &MssDmmEvent0Reg {
        &self.mss_dmm_event0_reg
    }
    #[doc = "0x674 - MSS_DMM_EVENT1_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event1_reg(&self) -> &MssDmmEvent1Reg {
        &self.mss_dmm_event1_reg
    }
    #[doc = "0x678 - MSS_DMM_EVENT2_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event2_reg(&self) -> &MssDmmEvent2Reg {
        &self.mss_dmm_event2_reg
    }
    #[doc = "0x67c - MSS_DMM_EVENT3_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event3_reg(&self) -> &MssDmmEvent3Reg {
        &self.mss_dmm_event3_reg
    }
    #[doc = "0x680 - MSS_DMM_EVENT4_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event4_reg(&self) -> &MssDmmEvent4Reg {
        &self.mss_dmm_event4_reg
    }
    #[doc = "0x684 - MSS_DMM_EVENT5_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event5_reg(&self) -> &MssDmmEvent5Reg {
        &self.mss_dmm_event5_reg
    }
    #[doc = "0x688 - MSS_DMM_EVENT6_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event6_reg(&self) -> &MssDmmEvent6Reg {
        &self.mss_dmm_event6_reg
    }
    #[doc = "0x68c - MSS_DMM_EVENT7_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event7_reg(&self) -> &MssDmmEvent7Reg {
        &self.mss_dmm_event7_reg
    }
    #[doc = "0x690 - MSS_DMM_EVENT8_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event8_reg(&self) -> &MssDmmEvent8Reg {
        &self.mss_dmm_event8_reg
    }
    #[doc = "0x694 - MSS_DMM_EVENT9_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event9_reg(&self) -> &MssDmmEvent9Reg {
        &self.mss_dmm_event9_reg
    }
    #[doc = "0x698 - MSS_DMM_EVENT10_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event10_reg(&self) -> &MssDmmEvent10Reg {
        &self.mss_dmm_event10_reg
    }
    #[doc = "0x69c - MSS_DMM_EVENT11_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event11_reg(&self) -> &MssDmmEvent11Reg {
        &self.mss_dmm_event11_reg
    }
    #[doc = "0x6a0 - MSS_DMM_EVENT12_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event12_reg(&self) -> &MssDmmEvent12Reg {
        &self.mss_dmm_event12_reg
    }
    #[doc = "0x6a4 - MSS_DMM_EVENT13_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event13_reg(&self) -> &MssDmmEvent13Reg {
        &self.mss_dmm_event13_reg
    }
    #[doc = "0x6a8 - MSS_DMM_EVENT14_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event14_reg(&self) -> &MssDmmEvent14Reg {
        &self.mss_dmm_event14_reg
    }
    #[doc = "0x6ac - MSS_DMM_EVENT15_REG"]
    #[inline(always)]
    pub const fn mss_dmm_event15_reg(&self) -> &MssDmmEvent15Reg {
        &self.mss_dmm_event15_reg
    }
    #[doc = "0x6b0 - MSS_TPTC_BOUNDARY_CFG"]
    #[inline(always)]
    pub const fn mss_tptc_boundary_cfg(&self) -> &MssTptcBoundaryCfg {
        &self.mss_tptc_boundary_cfg
    }
    #[doc = "0x6b4 - MSS_TPTC_XID_REORDER_CFG"]
    #[inline(always)]
    pub const fn mss_tptc_xid_reorder_cfg(&self) -> &MssTptcXidReorderCfg {
        &self.mss_tptc_xid_reorder_cfg
    }
    #[doc = "0x6b8 - GPADC_CTRL"]
    #[inline(always)]
    pub const fn gpadc_ctrl(&self) -> &GpadcCtrl {
        &self.gpadc_ctrl
    }
    #[doc = "0x6bc - HW_Sync_FE_CTRL"]
    #[inline(always)]
    pub const fn hw_sync_fe_ctrl(&self) -> &HwSyncFeCtrl {
        &self.hw_sync_fe_ctrl
    }
    #[doc = "0x6c0 - DEBUGSS_CSETB_FLUSH"]
    #[inline(always)]
    pub const fn debugss_csetb_flush(&self) -> &DebugssCsetbFlush {
        &self.debugss_csetb_flush
    }
    #[doc = "0x6c4 - ANALOG_WU_STATUS_REG_POLARITY_INV"]
    #[inline(always)]
    pub const fn analog_wu_status_reg_polarity_inv(&self) -> &AnalogWuStatusRegPolarityInv {
        &self.analog_wu_status_reg_polarity_inv
    }
    #[doc = "0x6c8 - ANALOG_CLK_STATUS_REG_POLARITY_INV"]
    #[inline(always)]
    pub const fn analog_clk_status_reg_polarity_inv(&self) -> &AnalogClkStatusRegPolarityInv {
        &self.analog_clk_status_reg_polarity_inv
    }
    #[doc = "0x6cc - ANALOG_WU_STATUS_REG_GRP1_MASK"]
    #[inline(always)]
    pub const fn analog_wu_status_reg_grp1_mask(&self) -> &AnalogWuStatusRegGrp1Mask {
        &self.analog_wu_status_reg_grp1_mask
    }
    #[doc = "0x6d0 - ANALOG_CLK_STATUS_REG_GRP1_MASK"]
    #[inline(always)]
    pub const fn analog_clk_status_reg_grp1_mask(&self) -> &AnalogClkStatusRegGrp1Mask {
        &self.analog_clk_status_reg_grp1_mask
    }
    #[doc = "0x6d4 - ANALOG_WU_STATUS_REG_GRP2_MASK"]
    #[inline(always)]
    pub const fn analog_wu_status_reg_grp2_mask(&self) -> &AnalogWuStatusRegGrp2Mask {
        &self.analog_wu_status_reg_grp2_mask
    }
    #[doc = "0x6d8 - ANALOG_CLK_STATUS_REG_GRP2_MASK"]
    #[inline(always)]
    pub const fn analog_clk_status_reg_grp2_mask(&self) -> &AnalogClkStatusRegGrp2Mask {
        &self.analog_clk_status_reg_grp2_mask
    }
    #[doc = "0x6dc - NERROR_MASK"]
    #[inline(always)]
    pub const fn nerror_mask(&self) -> &NerrorMask {
        &self.nerror_mask
    }
    #[doc = "0x6e0 - MSS2R5SS_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_ctrl(&self) -> &Mss2r5ssBusSafetyCtrl {
        &self.mss2r5ss_bus_safety_ctrl
    }
    #[doc = "0x6e4 - MSS2R5SS_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_fi(&self) -> &Mss2r5ssBusSafetyFi {
        &self.mss2r5ss_bus_safety_fi
    }
    #[doc = "0x6e8 - MSS2R5SS_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err(&self) -> &Mss2r5ssBusSafetyErr {
        &self.mss2r5ss_bus_safety_err
    }
    #[doc = "0x6ec - MSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err_stat_data0(&self) -> &Mss2r5ssBusSafetyErrStatData0 {
        &self.mss2r5ss_bus_safety_err_stat_data0
    }
    #[doc = "0x6f0 - MSS2R5SS_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err_stat_cmd(&self) -> &Mss2r5ssBusSafetyErrStatCmd {
        &self.mss2r5ss_bus_safety_err_stat_cmd
    }
    #[doc = "0x6f4 - MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err_stat_write(&self) -> &Mss2r5ssBusSafetyErrStatWrite {
        &self.mss2r5ss_bus_safety_err_stat_write
    }
    #[doc = "0x6f8 - MSS2R5SS_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err_stat_read(&self) -> &Mss2r5ssBusSafetyErrStatRead {
        &self.mss2r5ss_bus_safety_err_stat_read
    }
    #[doc = "0x6fc - MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss2r5ss_bus_safety_err_stat_writeresp(
        &self,
    ) -> &Mss2r5ssBusSafetyErrStatWriteresp {
        &self.mss2r5ss_bus_safety_err_stat_writeresp
    }
    #[doc = "0x700 - DSS2R5SS_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_ctrl(&self) -> &Dss2r5ssBusSafetyCtrl {
        &self.dss2r5ss_bus_safety_ctrl
    }
    #[doc = "0x704 - DSS2R5SS_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_fi(&self) -> &Dss2r5ssBusSafetyFi {
        &self.dss2r5ss_bus_safety_fi
    }
    #[doc = "0x708 - DSS2R5SS_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err(&self) -> &Dss2r5ssBusSafetyErr {
        &self.dss2r5ss_bus_safety_err
    }
    #[doc = "0x70c - DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err_stat_data0(&self) -> &Dss2r5ssBusSafetyErrStatData0 {
        &self.dss2r5ss_bus_safety_err_stat_data0
    }
    #[doc = "0x710 - DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err_stat_cmd(&self) -> &Dss2r5ssBusSafetyErrStatCmd {
        &self.dss2r5ss_bus_safety_err_stat_cmd
    }
    #[doc = "0x714 - DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err_stat_write(&self) -> &Dss2r5ssBusSafetyErrStatWrite {
        &self.dss2r5ss_bus_safety_err_stat_write
    }
    #[doc = "0x718 - DSS2R5SS_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err_stat_read(&self) -> &Dss2r5ssBusSafetyErrStatRead {
        &self.dss2r5ss_bus_safety_err_stat_read
    }
    #[doc = "0x71c - DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn dss2r5ss_bus_safety_err_stat_writeresp(
        &self,
    ) -> &Dss2r5ssBusSafetyErrStatWriteresp {
        &self.dss2r5ss_bus_safety_err_stat_writeresp
    }
    #[doc = "0x720 - MSS_DMM_ACCESS_MODE"]
    #[inline(always)]
    pub const fn mss_dmm_access_mode(&self) -> &MssDmmAccessMode {
        &self.mss_dmm_access_mode
    }
    #[doc = "0x724 - CPSW_HW_TRIG_CTRL"]
    #[inline(always)]
    pub const fn cpsw_hw_trig_ctrl(&self) -> &CpswHwTrigCtrl {
        &self.cpsw_hw_trig_ctrl
    }
    #[doc = "0x728 - CPSW_HW_TRIG_VAL"]
    #[inline(always)]
    pub const fn cpsw_hw_trig_val(&self) -> &CpswHwTrigVal {
        &self.cpsw_hw_trig_val
    }
    #[doc = "0x72c - CPSW_TRIG_CAPTURE_COUNT"]
    #[inline(always)]
    pub const fn cpsw_trig_capture_count(&self) -> &CpswTrigCaptureCount {
        &self.cpsw_trig_capture_count
    }
    #[doc = "0x730 - MSS_L2_C_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_ctrl(&self) -> &MssL2CBusSafetyCtrl {
        &self.mss_l2_c_bus_safety_ctrl
    }
    #[doc = "0x734 - MSS_L2_C_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_fi(&self) -> &MssL2CBusSafetyFi {
        &self.mss_l2_c_bus_safety_fi
    }
    #[doc = "0x738 - MSS_L2_C_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err(&self) -> &MssL2CBusSafetyErr {
        &self.mss_l2_c_bus_safety_err
    }
    #[doc = "0x73c - MSS_L2_C_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err_stat_data0(&self) -> &MssL2CBusSafetyErrStatData0 {
        &self.mss_l2_c_bus_safety_err_stat_data0
    }
    #[doc = "0x740 - MSS_L2_C_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err_stat_cmd(&self) -> &MssL2CBusSafetyErrStatCmd {
        &self.mss_l2_c_bus_safety_err_stat_cmd
    }
    #[doc = "0x744 - MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err_stat_write(&self) -> &MssL2CBusSafetyErrStatWrite {
        &self.mss_l2_c_bus_safety_err_stat_write
    }
    #[doc = "0x748 - MSS_L2_C_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err_stat_read(&self) -> &MssL2CBusSafetyErrStatRead {
        &self.mss_l2_c_bus_safety_err_stat_read
    }
    #[doc = "0x74c - MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn mss_l2_c_bus_safety_err_stat_writeresp(&self) -> &MssL2CBusSafetyErrStatWriteresp {
        &self.mss_l2_c_bus_safety_err_stat_writeresp
    }
    #[doc = "0x750 - R5SS2DSS_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_ctrl(&self) -> &R5ss2dssBusSafetyCtrl {
        &self.r5ss2dss_bus_safety_ctrl
    }
    #[doc = "0x754 - R5SS2DSS_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_fi(&self) -> &R5ss2dssBusSafetyFi {
        &self.r5ss2dss_bus_safety_fi
    }
    #[doc = "0x758 - R5SS2DSS_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err(&self) -> &R5ss2dssBusSafetyErr {
        &self.r5ss2dss_bus_safety_err
    }
    #[doc = "0x75c - R5SS2DSS_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err_stat_data0(&self) -> &R5ss2dssBusSafetyErrStatData0 {
        &self.r5ss2dss_bus_safety_err_stat_data0
    }
    #[doc = "0x760 - R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err_stat_cmd(&self) -> &R5ss2dssBusSafetyErrStatCmd {
        &self.r5ss2dss_bus_safety_err_stat_cmd
    }
    #[doc = "0x764 - R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err_stat_write(&self) -> &R5ss2dssBusSafetyErrStatWrite {
        &self.r5ss2dss_bus_safety_err_stat_write
    }
    #[doc = "0x768 - R5SS2DSS_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err_stat_read(&self) -> &R5ss2dssBusSafetyErrStatRead {
        &self.r5ss2dss_bus_safety_err_stat_read
    }
    #[doc = "0x76c - R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn r5ss2dss_bus_safety_err_stat_writeresp(
        &self,
    ) -> &R5ss2dssBusSafetyErrStatWriteresp {
        &self.r5ss2dss_bus_safety_err_stat_writeresp
    }
    #[doc = "0x770 - R5SS2MSS_BUS_SAFETY_CTRL"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_ctrl(&self) -> &R5ss2mssBusSafetyCtrl {
        &self.r5ss2mss_bus_safety_ctrl
    }
    #[doc = "0x774 - R5SS2MSS_BUS_SAFETY_FI"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_fi(&self) -> &R5ss2mssBusSafetyFi {
        &self.r5ss2mss_bus_safety_fi
    }
    #[doc = "0x778 - R5SS2MSS_BUS_SAFETY_ERR"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err(&self) -> &R5ss2mssBusSafetyErr {
        &self.r5ss2mss_bus_safety_err
    }
    #[doc = "0x77c - R5SS2MSS_BUS_SAFETY_ERR_STAT_DATA0"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err_stat_data0(&self) -> &R5ss2mssBusSafetyErrStatData0 {
        &self.r5ss2mss_bus_safety_err_stat_data0
    }
    #[doc = "0x780 - R5SS2MSS_BUS_SAFETY_ERR_STAT_CMD"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err_stat_cmd(&self) -> &R5ss2mssBusSafetyErrStatCmd {
        &self.r5ss2mss_bus_safety_err_stat_cmd
    }
    #[doc = "0x784 - R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITE"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err_stat_write(&self) -> &R5ss2mssBusSafetyErrStatWrite {
        &self.r5ss2mss_bus_safety_err_stat_write
    }
    #[doc = "0x788 - R5SS2MSS_BUS_SAFETY_ERR_STAT_READ"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err_stat_read(&self) -> &R5ss2mssBusSafetyErrStatRead {
        &self.r5ss2mss_bus_safety_err_stat_read
    }
    #[doc = "0x78c - R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITERESP"]
    #[inline(always)]
    pub const fn r5ss2mss_bus_safety_err_stat_writeresp(
        &self,
    ) -> &R5ss2mssBusSafetyErrStatWriteresp {
        &self.r5ss2mss_bus_safety_err_stat_writeresp
    }
    #[doc = "0x790 - NW_PACKET_COUNT"]
    #[inline(always)]
    pub const fn nw_packet_count(&self) -> &NwPacketCount {
        &self.nw_packet_count
    }
    #[doc = "0x794 - NW_PACKET_COUNT_RESET"]
    #[inline(always)]
    pub const fn nw_packet_count_reset(&self) -> &NwPacketCountReset {
        &self.nw_packet_count_reset
    }
    #[doc = "0x798 - CPSW_CRC_PING_ADDR"]
    #[inline(always)]
    pub const fn cpsw_crc_ping_addr(&self) -> &CpswCrcPingAddr {
        &self.cpsw_crc_ping_addr
    }
    #[doc = "0x79c - CPSW_CRC_PONG_ADDR"]
    #[inline(always)]
    pub const fn cpsw_crc_pong_addr(&self) -> &CpswCrcPongAddr {
        &self.cpsw_crc_pong_addr
    }
    #[doc = "0x800 - R5_CONTROL"]
    #[inline(always)]
    pub const fn r5_control(&self) -> &R5Control {
        &self.r5_control
    }
    #[doc = "0x804 - R5_ROM_ECLIPSE"]
    #[inline(always)]
    pub const fn r5_rom_eclipse(&self) -> &R5RomEclipse {
        &self.r5_rom_eclipse
    }
    #[doc = "0x808 - R5_COREA_HALT"]
    #[inline(always)]
    pub const fn r5_corea_halt(&self) -> &R5CoreaHalt {
        &self.r5_corea_halt
    }
    #[doc = "0x80c - R5_COREB_HALT"]
    #[inline(always)]
    pub const fn r5_coreb_halt(&self) -> &R5CorebHalt {
        &self.r5_coreb_halt
    }
    #[doc = "0x810 - R5_STATUS_REG"]
    #[inline(always)]
    pub const fn r5_status_reg(&self) -> &R5StatusReg {
        &self.r5_status_reg
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
#[doc = "MSS_SW_INT (rw) register accessor: MSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_sw_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_sw_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_sw_int`]
module"]
#[doc(alias = "MSS_SW_INT")]
pub type MssSwInt = crate::Reg<mss_sw_int::MssSwIntSpec>;
#[doc = "MSS_SW_INT"]
pub mod mss_sw_int;
#[doc = "MSS_CAPEVNT_SEL (rw) register accessor: MSS_CAPEVNT_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_capevnt_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_capevnt_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_capevnt_sel`]
module"]
#[doc(alias = "MSS_CAPEVNT_SEL")]
pub type MssCapevntSel = crate::Reg<mss_capevnt_sel::MssCapevntSelSpec>;
#[doc = "MSS_CAPEVNT_SEL"]
pub mod mss_capevnt_sel;
#[doc = "MSS_DMA_REQ_SEL (rw) register accessor: MSS_DMA_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dma_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dma_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dma_req_sel`]
module"]
#[doc(alias = "MSS_DMA_REQ_SEL")]
pub type MssDmaReqSel = crate::Reg<mss_dma_req_sel::MssDmaReqSelSpec>;
#[doc = "MSS_DMA_REQ_SEL"]
pub mod mss_dma_req_sel;
#[doc = "MSS_DMA1_REQ_SEL (rw) register accessor: MSS_DMA1_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dma1_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dma1_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dma1_req_sel`]
module"]
#[doc(alias = "MSS_DMA1_REQ_SEL")]
pub type MssDma1ReqSel = crate::Reg<mss_dma1_req_sel::MssDma1ReqSelSpec>;
#[doc = "MSS_DMA1_REQ_SEL"]
pub mod mss_dma1_req_sel;
#[doc = "MSS_IRQ_REQ_SEL (rw) register accessor: MSS_IRQ_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_irq_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_irq_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_irq_req_sel`]
module"]
#[doc(alias = "MSS_IRQ_REQ_SEL")]
pub type MssIrqReqSel = crate::Reg<mss_irq_req_sel::MssIrqReqSelSpec>;
#[doc = "MSS_IRQ_REQ_SEL"]
pub mod mss_irq_req_sel;
#[doc = "MSS_SPI_TRIG_SRC (rw) register accessor: MSS_SPI_TRIG_SRC\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spi_trig_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spi_trig_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spi_trig_src`]
module"]
#[doc(alias = "MSS_SPI_TRIG_SRC")]
pub type MssSpiTrigSrc = crate::Reg<mss_spi_trig_src::MssSpiTrigSrcSpec>;
#[doc = "MSS_SPI_TRIG_SRC"]
pub mod mss_spi_trig_src;
#[doc = "MSS_ATCM_MEM_INIT (rw) register accessor: MSS_ATCM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_atcm_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_atcm_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_atcm_mem_init`]
module"]
#[doc(alias = "MSS_ATCM_MEM_INIT")]
pub type MssAtcmMemInit = crate::Reg<mss_atcm_mem_init::MssAtcmMemInitSpec>;
#[doc = "MSS_ATCM_MEM_INIT"]
pub mod mss_atcm_mem_init;
#[doc = "MSS_ATCM_MEM_INIT_DONE (rw) register accessor: MSS_ATCM_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_atcm_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_atcm_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_atcm_mem_init_done`]
module"]
#[doc(alias = "MSS_ATCM_MEM_INIT_DONE")]
pub type MssAtcmMemInitDone = crate::Reg<mss_atcm_mem_init_done::MssAtcmMemInitDoneSpec>;
#[doc = "MSS_ATCM_MEM_INIT_DONE"]
pub mod mss_atcm_mem_init_done;
#[doc = "MSS_ATCM_MEM_INIT_STATUS (rw) register accessor: MSS_ATCM_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_atcm_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_atcm_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_atcm_mem_init_status`]
module"]
#[doc(alias = "MSS_ATCM_MEM_INIT_STATUS")]
pub type MssAtcmMemInitStatus = crate::Reg<mss_atcm_mem_init_status::MssAtcmMemInitStatusSpec>;
#[doc = "MSS_ATCM_MEM_INIT_STATUS"]
pub mod mss_atcm_mem_init_status;
#[doc = "MSS_BTCM_MEM_INIT (rw) register accessor: MSS_BTCM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_btcm_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_btcm_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_btcm_mem_init`]
module"]
#[doc(alias = "MSS_BTCM_MEM_INIT")]
pub type MssBtcmMemInit = crate::Reg<mss_btcm_mem_init::MssBtcmMemInitSpec>;
#[doc = "MSS_BTCM_MEM_INIT"]
pub mod mss_btcm_mem_init;
#[doc = "MSS_BTCM_MEM_INIT_DONE (rw) register accessor: MSS_BTCM_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_btcm_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_btcm_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_btcm_mem_init_done`]
module"]
#[doc(alias = "MSS_BTCM_MEM_INIT_DONE")]
pub type MssBtcmMemInitDone = crate::Reg<mss_btcm_mem_init_done::MssBtcmMemInitDoneSpec>;
#[doc = "MSS_BTCM_MEM_INIT_DONE"]
pub mod mss_btcm_mem_init_done;
#[doc = "MSS_BTCM_MEM_INIT_STATUS (rw) register accessor: MSS_BTCM_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_btcm_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_btcm_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_btcm_mem_init_status`]
module"]
#[doc(alias = "MSS_BTCM_MEM_INIT_STATUS")]
pub type MssBtcmMemInitStatus = crate::Reg<mss_btcm_mem_init_status::MssBtcmMemInitStatusSpec>;
#[doc = "MSS_BTCM_MEM_INIT_STATUS"]
pub mod mss_btcm_mem_init_status;
#[doc = "MSS_L2_MEM_INIT (rw) register accessor: MSS_L2_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_mem_init`]
module"]
#[doc(alias = "MSS_L2_MEM_INIT")]
pub type MssL2MemInit = crate::Reg<mss_l2_mem_init::MssL2MemInitSpec>;
#[doc = "MSS_L2_MEM_INIT"]
pub mod mss_l2_mem_init;
#[doc = "MSS_L2_MEM_INIT_DONE (rw) register accessor: MSS_L2_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_mem_init_done`]
module"]
#[doc(alias = "MSS_L2_MEM_INIT_DONE")]
pub type MssL2MemInitDone = crate::Reg<mss_l2_mem_init_done::MssL2MemInitDoneSpec>;
#[doc = "MSS_L2_MEM_INIT_DONE"]
pub mod mss_l2_mem_init_done;
#[doc = "MSS_L2_MEM_INIT_STATUS (rw) register accessor: MSS_L2_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_mem_init_status`]
module"]
#[doc(alias = "MSS_L2_MEM_INIT_STATUS")]
pub type MssL2MemInitStatus = crate::Reg<mss_l2_mem_init_status::MssL2MemInitStatusSpec>;
#[doc = "MSS_L2_MEM_INIT_STATUS"]
pub mod mss_l2_mem_init_status;
#[doc = "MSS_MAILBOX_MEM_INIT (rw) register accessor: MSS_MAILBOX_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mailbox_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mailbox_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mailbox_mem_init`]
module"]
#[doc(alias = "MSS_MAILBOX_MEM_INIT")]
pub type MssMailboxMemInit = crate::Reg<mss_mailbox_mem_init::MssMailboxMemInitSpec>;
#[doc = "MSS_MAILBOX_MEM_INIT"]
pub mod mss_mailbox_mem_init;
#[doc = "MSS_MAIlBOX_MEM_INIT_DONE (rw) register accessor: MSS_MAIlBOX_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mail_box_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mail_box_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mail_box_mem_init_done`]
module"]
#[doc(alias = "MSS_MAIlBOX_MEM_INIT_DONE")]
pub type MssMailBoxMemInitDone = crate::Reg<mss_mail_box_mem_init_done::MssMailBoxMemInitDoneSpec>;
#[doc = "MSS_MAIlBOX_MEM_INIT_DONE"]
pub mod mss_mail_box_mem_init_done;
#[doc = "MSS_MAILBOX_MEM_INIT_STATUS (rw) register accessor: MSS_MAILBOX_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mailbox_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mailbox_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_mailbox_mem_init_status`]
module"]
#[doc(alias = "MSS_MAILBOX_MEM_INIT_STATUS")]
pub type MssMailboxMemInitStatus =
    crate::Reg<mss_mailbox_mem_init_status::MssMailboxMemInitStatusSpec>;
#[doc = "MSS_MAILBOX_MEM_INIT_STATUS"]
pub mod mss_mailbox_mem_init_status;
#[doc = "MSS_RETRAM_MEM_INIT (rw) register accessor: MSS_RETRAM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_retram_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_retram_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_retram_mem_init`]
module"]
#[doc(alias = "MSS_RETRAM_MEM_INIT")]
pub type MssRetramMemInit = crate::Reg<mss_retram_mem_init::MssRetramMemInitSpec>;
#[doc = "MSS_RETRAM_MEM_INIT"]
pub mod mss_retram_mem_init;
#[doc = "MSS_RETRAM_MEM_INIT_DONE (rw) register accessor: MSS_RETRAM_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_retram_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_retram_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_retram_mem_init_done`]
module"]
#[doc(alias = "MSS_RETRAM_MEM_INIT_DONE")]
pub type MssRetramMemInitDone = crate::Reg<mss_retram_mem_init_done::MssRetramMemInitDoneSpec>;
#[doc = "MSS_RETRAM_MEM_INIT_DONE"]
pub mod mss_retram_mem_init_done;
#[doc = "MSS_RETRAM_MEM_INIT_STATUS (rw) register accessor: MSS_RETRAM_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_retram_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_retram_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_retram_mem_init_status`]
module"]
#[doc(alias = "MSS_RETRAM_MEM_INIT_STATUS")]
pub type MssRetramMemInitStatus =
    crate::Reg<mss_retram_mem_init_status::MssRetramMemInitStatusSpec>;
#[doc = "MSS_RETRAM_MEM_INIT_STATUS"]
pub mod mss_retram_mem_init_status;
#[doc = "MSS_SPIA_MEM_INIT (rw) register accessor: MSS_SPIA_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spia_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spia_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spia_mem_init`]
module"]
#[doc(alias = "MSS_SPIA_MEM_INIT")]
pub type MssSpiaMemInit = crate::Reg<mss_spia_mem_init::MssSpiaMemInitSpec>;
#[doc = "MSS_SPIA_MEM_INIT"]
pub mod mss_spia_mem_init;
#[doc = "MSS_SPIA_MEM_INIT_DONE (rw) register accessor: MSS_SPIA_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spia_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spia_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spia_mem_init_done`]
module"]
#[doc(alias = "MSS_SPIA_MEM_INIT_DONE")]
pub type MssSpiaMemInitDone = crate::Reg<mss_spia_mem_init_done::MssSpiaMemInitDoneSpec>;
#[doc = "MSS_SPIA_MEM_INIT_DONE"]
pub mod mss_spia_mem_init_done;
#[doc = "MSS_SPIA_MEM_INIT_STATUS (rw) register accessor: MSS_SPIA_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spia_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spia_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spia_mem_init_status`]
module"]
#[doc(alias = "MSS_SPIA_MEM_INIT_STATUS")]
pub type MssSpiaMemInitStatus = crate::Reg<mss_spia_mem_init_status::MssSpiaMemInitStatusSpec>;
#[doc = "MSS_SPIA_MEM_INIT_STATUS"]
pub mod mss_spia_mem_init_status;
#[doc = "MSS_SPIB_MEM_INIT (rw) register accessor: MSS_SPIB_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_mem_init`]
module"]
#[doc(alias = "MSS_SPIB_MEM_INIT")]
pub type MssSpibMemInit = crate::Reg<mss_spib_mem_init::MssSpibMemInitSpec>;
#[doc = "MSS_SPIB_MEM_INIT"]
pub mod mss_spib_mem_init;
#[doc = "MSS_SPIB_MEM_INIT_DONE (rw) register accessor: MSS_SPIB_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_mem_init_done`]
module"]
#[doc(alias = "MSS_SPIB_MEM_INIT_DONE")]
pub type MssSpibMemInitDone = crate::Reg<mss_spib_mem_init_done::MssSpibMemInitDoneSpec>;
#[doc = "MSS_SPIB_MEM_INIT_DONE"]
pub mod mss_spib_mem_init_done;
#[doc = "MSS_SPIB_MEM_INIT_STATUS (rw) register accessor: MSS_SPIB_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_mem_init_status`]
module"]
#[doc(alias = "MSS_SPIB_MEM_INIT_STATUS")]
pub type MssSpibMemInitStatus = crate::Reg<mss_spib_mem_init_status::MssSpibMemInitStatusSpec>;
#[doc = "MSS_SPIB_MEM_INIT_STATUS"]
pub mod mss_spib_mem_init_status;
#[doc = "MSS_TPCC_MEMINIT_START (rw) register accessor: MSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_meminit_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_meminit_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_meminit_start`]
module"]
#[doc(alias = "MSS_TPCC_MEMINIT_START")]
pub type MssTpccMeminitStart = crate::Reg<mss_tpcc_meminit_start::MssTpccMeminitStartSpec>;
#[doc = "MSS_TPCC_MEMINIT_START"]
pub mod mss_tpcc_meminit_start;
#[doc = "MSS_TPCC_MEMINIT_DONE (rw) register accessor: MSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_meminit_done`]
module"]
#[doc(alias = "MSS_TPCC_MEMINIT_DONE")]
pub type MssTpccMeminitDone = crate::Reg<mss_tpcc_meminit_done::MssTpccMeminitDoneSpec>;
#[doc = "MSS_TPCC_MEMINIT_DONE"]
pub mod mss_tpcc_meminit_done;
#[doc = "MSS_TPCC_MEMINIT_STATUS (rw) register accessor: MSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_meminit_status`]
module"]
#[doc(alias = "MSS_TPCC_MEMINIT_STATUS")]
pub type MssTpccMeminitStatus = crate::Reg<mss_tpcc_meminit_status::MssTpccMeminitStatusSpec>;
#[doc = "MSS_TPCC_MEMINIT_STATUS"]
pub mod mss_tpcc_meminit_status;
#[doc = "MSS_GPADC_MEM_INIT (rw) register accessor: MSS_GPADC_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_mem_init`]
module"]
#[doc(alias = "MSS_GPADC_MEM_INIT")]
pub type MssGpadcMemInit = crate::Reg<mss_gpadc_mem_init::MssGpadcMemInitSpec>;
#[doc = "MSS_GPADC_MEM_INIT"]
pub mod mss_gpadc_mem_init;
#[doc = "MSS_GPADC_MEM_INIT_DONE (rw) register accessor: MSS_GPADC_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_mem_init_done`]
module"]
#[doc(alias = "MSS_GPADC_MEM_INIT_DONE")]
pub type MssGpadcMemInitDone = crate::Reg<mss_gpadc_mem_init_done::MssGpadcMemInitDoneSpec>;
#[doc = "MSS_GPADC_MEM_INIT_DONE"]
pub mod mss_gpadc_mem_init_done;
#[doc = "MSS_GPADC_MEM_INIT_STATUS (rw) register accessor: MSS_GPADC_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gpadc_mem_init_status`]
module"]
#[doc(alias = "MSS_GPADC_MEM_INIT_STATUS")]
pub type MssGpadcMemInitStatus = crate::Reg<mss_gpadc_mem_init_status::MssGpadcMemInitStatusSpec>;
#[doc = "MSS_GPADC_MEM_INIT_STATUS"]
pub mod mss_gpadc_mem_init_status;
#[doc = "MSS_SPIA_CFG (rw) register accessor: RESERVED: Dont Use\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spia_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spia_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spia_cfg`]
module"]
#[doc(alias = "MSS_SPIA_CFG")]
pub type MssSpiaCfg = crate::Reg<mss_spia_cfg::MssSpiaCfgSpec>;
#[doc = "RESERVED: Dont Use"]
pub mod mss_spia_cfg;
#[doc = "MSS_SPIB_CFG (rw) register accessor: MSS_SPIB_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_spib_cfg`]
module"]
#[doc(alias = "MSS_SPIB_CFG")]
pub type MssSpibCfg = crate::Reg<mss_spib_cfg::MssSpibCfgSpec>;
#[doc = "MSS_SPIB_CFG"]
pub mod mss_spib_cfg;
#[doc = "MSS_EPWM_CFG (rw) register accessor: MSS_EPWM_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_epwm_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_epwm_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_epwm_cfg`]
module"]
#[doc(alias = "MSS_EPWM_CFG")]
pub type MssEpwmCfg = crate::Reg<mss_epwm_cfg::MssEpwmCfgSpec>;
#[doc = "MSS_EPWM_CFG"]
pub mod mss_epwm_cfg;
#[doc = "MSS_GIO_CFG (rw) register accessor: MSS_GIO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gio_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gio_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_gio_cfg`]
module"]
#[doc(alias = "MSS_GIO_CFG")]
pub type MssGioCfg = crate::Reg<mss_gio_cfg::MssGioCfgSpec>;
#[doc = "MSS_GIO_CFG"]
pub mod mss_gio_cfg;
#[doc = "HW_SPARE_REG1 (rw) register accessor: HW_SPARE_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg1`]
module"]
#[doc(alias = "HW_SPARE_REG1")]
pub type HwSpareReg1 = crate::Reg<hw_spare_reg1::HwSpareReg1Spec>;
#[doc = "HW_SPARE_REG1"]
pub mod hw_spare_reg1;
#[doc = "HW_SPARE_REG2 (rw) register accessor: HW_SPARE_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg2`]
module"]
#[doc(alias = "HW_SPARE_REG2")]
pub type HwSpareReg2 = crate::Reg<hw_spare_reg2::HwSpareReg2Spec>;
#[doc = "HW_SPARE_REG2"]
pub mod hw_spare_reg2;
#[doc = "CCC_ERR_STATUS (rw) register accessor: CCC_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ccc_err_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccc_err_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccc_err_status`]
module"]
#[doc(alias = "CCC_ERR_STATUS")]
pub type CccErrStatus = crate::Reg<ccc_err_status::CccErrStatusSpec>;
#[doc = "CCC_ERR_STATUS"]
pub mod ccc_err_status;
#[doc = "CCCA_CFG0 (rw) register accessor: CCCA_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccca_cfg0`]
module"]
#[doc(alias = "CCCA_CFG0")]
pub type CccaCfg0 = crate::Reg<ccca_cfg0::CccaCfg0Spec>;
#[doc = "CCCA_CFG0"]
pub mod ccca_cfg0;
#[doc = "CCCA_CFG1 (rw) register accessor: CCCA_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccca_cfg1`]
module"]
#[doc(alias = "CCCA_CFG1")]
pub type CccaCfg1 = crate::Reg<ccca_cfg1::CccaCfg1Spec>;
#[doc = "CCCA_CFG1"]
pub mod ccca_cfg1;
#[doc = "CCCA_CFG2 (rw) register accessor: CCCA_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccca_cfg2`]
module"]
#[doc(alias = "CCCA_CFG2")]
pub type CccaCfg2 = crate::Reg<ccca_cfg2::CccaCfg2Spec>;
#[doc = "CCCA_CFG2"]
pub mod ccca_cfg2;
#[doc = "CCCA_CFG3 (rw) register accessor: CCCA_CFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccca_cfg3`]
module"]
#[doc(alias = "CCCA_CFG3")]
pub type CccaCfg3 = crate::Reg<ccca_cfg3::CccaCfg3Spec>;
#[doc = "CCCA_CFG3"]
pub mod ccca_cfg3;
#[doc = "CCCA_CNTVAL (rw) register accessor: CCCA_CNTVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cntval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cntval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccca_cntval`]
module"]
#[doc(alias = "CCCA_CNTVAL")]
pub type CccaCntval = crate::Reg<ccca_cntval::CccaCntvalSpec>;
#[doc = "CCCA_CNTVAL"]
pub mod ccca_cntval;
#[doc = "CCCB_CFG0 (rw) register accessor: CCCB_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccb_cfg0`]
module"]
#[doc(alias = "CCCB_CFG0")]
pub type CccbCfg0 = crate::Reg<cccb_cfg0::CccbCfg0Spec>;
#[doc = "CCCB_CFG0"]
pub mod cccb_cfg0;
#[doc = "CCCB_CFG1 (rw) register accessor: CCCB_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccb_cfg1`]
module"]
#[doc(alias = "CCCB_CFG1")]
pub type CccbCfg1 = crate::Reg<cccb_cfg1::CccbCfg1Spec>;
#[doc = "CCCB_CFG1"]
pub mod cccb_cfg1;
#[doc = "CCCB_CFG2 (rw) register accessor: CCCB_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccb_cfg2`]
module"]
#[doc(alias = "CCCB_CFG2")]
pub type CccbCfg2 = crate::Reg<cccb_cfg2::CccbCfg2Spec>;
#[doc = "CCCB_CFG2"]
pub mod cccb_cfg2;
#[doc = "CCCB_CFG3 (rw) register accessor: CCCB_CFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccb_cfg3`]
module"]
#[doc(alias = "CCCB_CFG3")]
pub type CccbCfg3 = crate::Reg<cccb_cfg3::CccbCfg3Spec>;
#[doc = "CCCB_CFG3"]
pub mod cccb_cfg3;
#[doc = "CCCB_CNTVAL (rw) register accessor: CCCB_CNTVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cntval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cntval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccb_cntval`]
module"]
#[doc(alias = "CCCB_CNTVAL")]
pub type CccbCntval = crate::Reg<cccb_cntval::CccbCntvalSpec>;
#[doc = "CCCB_CNTVAL"]
pub mod cccb_cntval;
#[doc = "CCC_DCC_COMMON (rw) register accessor: CCC_DCC_COMMON\n\nYou can [`read`](crate::Reg::read) this register and get [`ccc_dcc_common::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccc_dcc_common::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccc_dcc_common`]
module"]
#[doc(alias = "CCC_DCC_COMMON")]
pub type CccDccCommon = crate::Reg<ccc_dcc_common::CccDccCommonSpec>;
#[doc = "CCC_DCC_COMMON"]
pub mod ccc_dcc_common;
#[doc = "R5_GLOBAL_CONFIG (rw) register accessor: R5_GLOBAL_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_global_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_global_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_global_config`]
module"]
#[doc(alias = "R5_GLOBAL_CONFIG")]
pub type R5GlobalConfig = crate::Reg<r5_global_config::R5GlobalConfigSpec>;
#[doc = "R5_GLOBAL_CONFIG"]
pub mod r5_global_config;
#[doc = "R5_AHB_EN (rw) register accessor: R5_AHB_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_ahb_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_ahb_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_ahb_en`]
module"]
#[doc(alias = "R5_AHB_EN")]
pub type R5AhbEn = crate::Reg<r5_ahb_en::R5AhbEnSpec>;
#[doc = "R5_AHB_EN"]
pub mod r5_ahb_en;
#[doc = "R5A_AHB_BASE (rw) register accessor: R5A_AHB_BASE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5a_ahb_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5a_ahb_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5a_ahb_base`]
module"]
#[doc(alias = "R5A_AHB_BASE")]
pub type R5aAhbBase = crate::Reg<r5a_ahb_base::R5aAhbBaseSpec>;
#[doc = "R5A_AHB_BASE"]
pub mod r5a_ahb_base;
#[doc = "R5A_AHB_SIZE (rw) register accessor: R5A_AHB_SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5a_ahb_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5a_ahb_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5a_ahb_size`]
module"]
#[doc(alias = "R5A_AHB_SIZE")]
pub type R5aAhbSize = crate::Reg<r5a_ahb_size::R5aAhbSizeSpec>;
#[doc = "R5A_AHB_SIZE"]
pub mod r5a_ahb_size;
#[doc = "R5B_AHB_BASE (rw) register accessor: R5B_AHB_BASE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5b_ahb_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5b_ahb_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5b_ahb_base`]
module"]
#[doc(alias = "R5B_AHB_BASE")]
pub type R5bAhbBase = crate::Reg<r5b_ahb_base::R5bAhbBaseSpec>;
#[doc = "R5B_AHB_BASE"]
pub mod r5b_ahb_base;
#[doc = "R5B_AHB_SIZE (rw) register accessor: R5B_AHB_SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5b_ahb_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5b_ahb_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5b_ahb_size`]
module"]
#[doc(alias = "R5B_AHB_SIZE")]
pub type R5bAhbSize = crate::Reg<r5b_ahb_size::R5bAhbSizeSpec>;
#[doc = "R5B_AHB_SIZE"]
pub mod r5b_ahb_size;
#[doc = "R5_TCM_EXT_ERR_EN (rw) register accessor: R5_TCM_EXT_ERR_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_tcm_ext_err_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_tcm_ext_err_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_tcm_ext_err_en`]
module"]
#[doc(alias = "R5_TCM_EXT_ERR_EN")]
pub type R5TcmExtErrEn = crate::Reg<r5_tcm_ext_err_en::R5TcmExtErrEnSpec>;
#[doc = "R5_TCM_EXT_ERR_EN"]
pub mod r5_tcm_ext_err_en;
#[doc = "R5_TCM_ERR_EN (rw) register accessor: R5_TCM_ERR_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_tcm_err_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_tcm_err_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_tcm_err_en`]
module"]
#[doc(alias = "R5_TCM_ERR_EN")]
pub type R5TcmErrEn = crate::Reg<r5_tcm_err_en::R5TcmErrEnSpec>;
#[doc = "R5_TCM_ERR_EN"]
pub mod r5_tcm_err_en;
#[doc = "R5_INIT_TCM (rw) register accessor: R5_INIT_TCM\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_init_tcm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_init_tcm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_init_tcm`]
module"]
#[doc(alias = "R5_INIT_TCM")]
pub type R5InitTcm = crate::Reg<r5_init_tcm::R5InitTcmSpec>;
#[doc = "R5_INIT_TCM"]
pub mod r5_init_tcm;
#[doc = "R5_TCM_ECC_WRENZ_EN (rw) register accessor: R5_TCM_ECC_WRENZ_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_tcm_ecc_wrenz_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_tcm_ecc_wrenz_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_tcm_ecc_wrenz_en`]
module"]
#[doc(alias = "R5_TCM_ECC_WRENZ_EN")]
pub type R5TcmEccWrenzEn = crate::Reg<r5_tcm_ecc_wrenz_en::R5TcmEccWrenzEnSpec>;
#[doc = "R5_TCM_ECC_WRENZ_EN"]
pub mod r5_tcm_ecc_wrenz_en;
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
#[doc = "ESM_GATING4 (rw) register accessor: ESM_GATING4\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating4`]
module"]
#[doc(alias = "ESM_GATING4")]
pub type EsmGating4 = crate::Reg<esm_gating4::EsmGating4Spec>;
#[doc = "ESM_GATING4"]
pub mod esm_gating4;
#[doc = "ESM_GATING5 (rw) register accessor: ESM_GATING5\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating5`]
module"]
#[doc(alias = "ESM_GATING5")]
pub type EsmGating5 = crate::Reg<esm_gating5::EsmGating5Spec>;
#[doc = "ESM_GATING5"]
pub mod esm_gating5;
#[doc = "ESM_GATING6 (rw) register accessor: ESM_GATING6\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating6`]
module"]
#[doc(alias = "ESM_GATING6")]
pub type EsmGating6 = crate::Reg<esm_gating6::EsmGating6Spec>;
#[doc = "ESM_GATING6"]
pub mod esm_gating6;
#[doc = "ESM_GATING7 (rw) register accessor: ESM_GATING7\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating7`]
module"]
#[doc(alias = "ESM_GATING7")]
pub type EsmGating7 = crate::Reg<esm_gating7::EsmGating7Spec>;
#[doc = "ESM_GATING7"]
pub mod esm_gating7;
#[doc = "ERR_PARITY_ATCM0 (rw) register accessor: ERR_PARITY_ATCM0\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_atcm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_atcm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_atcm0`]
module"]
#[doc(alias = "ERR_PARITY_ATCM0")]
pub type ErrParityAtcm0 = crate::Reg<err_parity_atcm0::ErrParityAtcm0Spec>;
#[doc = "ERR_PARITY_ATCM0"]
pub mod err_parity_atcm0;
#[doc = "ERR_PARITY_ATCM1 (rw) register accessor: ERR_PARITY_ATCM1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_atcm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_atcm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_atcm1`]
module"]
#[doc(alias = "ERR_PARITY_ATCM1")]
pub type ErrParityAtcm1 = crate::Reg<err_parity_atcm1::ErrParityAtcm1Spec>;
#[doc = "ERR_PARITY_ATCM1"]
pub mod err_parity_atcm1;
#[doc = "ERR_PARITY_B0TCM0 (rw) register accessor: ERR_PARITY_B0TCM0\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_b0tcm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_b0tcm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_b0tcm0`]
module"]
#[doc(alias = "ERR_PARITY_B0TCM0")]
pub type ErrParityB0tcm0 = crate::Reg<err_parity_b0tcm0::ErrParityB0tcm0Spec>;
#[doc = "ERR_PARITY_B0TCM0"]
pub mod err_parity_b0tcm0;
#[doc = "ERR_PARITY_B0TCM1 (rw) register accessor: ERR_PARITY_B0TCM1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_b0tcm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_b0tcm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_b0tcm1`]
module"]
#[doc(alias = "ERR_PARITY_B0TCM1")]
pub type ErrParityB0tcm1 = crate::Reg<err_parity_b0tcm1::ErrParityB0tcm1Spec>;
#[doc = "ERR_PARITY_B0TCM1"]
pub mod err_parity_b0tcm1;
#[doc = "ERR_PARITY_B1TCM0 (rw) register accessor: ERR_PARITY_B1TCM0\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_b1tcm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_b1tcm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_b1tcm0`]
module"]
#[doc(alias = "ERR_PARITY_B1TCM0")]
pub type ErrParityB1tcm0 = crate::Reg<err_parity_b1tcm0::ErrParityB1tcm0Spec>;
#[doc = "ERR_PARITY_B1TCM0"]
pub mod err_parity_b1tcm0;
#[doc = "ERR_PARITY_B1TCM1 (rw) register accessor: ERR_PARITY_B1TCM1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_b1tcm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_b1tcm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_parity_b1tcm1`]
module"]
#[doc(alias = "ERR_PARITY_B1TCM1")]
pub type ErrParityB1tcm1 = crate::Reg<err_parity_b1tcm1::ErrParityB1tcm1Spec>;
#[doc = "ERR_PARITY_B1TCM1"]
pub mod err_parity_b1tcm1;
#[doc = "TCM_PARITY_CTRL (rw) register accessor: TCM_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_parity_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_ctrl`]
module"]
#[doc(alias = "TCM_PARITY_CTRL")]
pub type TcmParityCtrl = crate::Reg<tcm_parity_ctrl::TcmParityCtrlSpec>;
#[doc = "TCM_PARITY_CTRL"]
pub mod tcm_parity_ctrl;
#[doc = "TCM_PARITY_ERRFRC (rw) register accessor: TCM_PARITY_ERRFRC\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_errfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_parity_errfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_errfrc`]
module"]
#[doc(alias = "TCM_PARITY_ERRFRC")]
pub type TcmParityErrfrc = crate::Reg<tcm_parity_errfrc::TcmParityErrfrcSpec>;
#[doc = "TCM_PARITY_ERRFRC"]
pub mod tcm_parity_errfrc;
#[doc = "HW_SPARE_REG3 (rw) register accessor: HW_SPARE_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg3`]
module"]
#[doc(alias = "HW_SPARE_REG3")]
pub type HwSpareReg3 = crate::Reg<hw_spare_reg3::HwSpareReg3Spec>;
#[doc = "HW_SPARE_REG3"]
pub mod hw_spare_reg3;
#[doc = "SPIA_IO_CFG (rw) register accessor: SPIA_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spia_io_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spia_io_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spia_io_cfg`]
module"]
#[doc(alias = "SPIA_IO_CFG")]
pub type SpiaIoCfg = crate::Reg<spia_io_cfg::SpiaIoCfgSpec>;
#[doc = "SPIA_IO_CFG"]
pub mod spia_io_cfg;
#[doc = "SPIB_IO_CFG (rw) register accessor: SPIB_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spib_io_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spib_io_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spib_io_cfg`]
module"]
#[doc(alias = "SPIB_IO_CFG")]
pub type SpibIoCfg = crate::Reg<spib_io_cfg::SpibIoCfgSpec>;
#[doc = "SPIB_IO_CFG"]
pub mod spib_io_cfg;
#[doc = "SPI_HOST_IRQ (rw) register accessor: SPI_HOST_IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_host_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_host_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_host_irq`]
module"]
#[doc(alias = "SPI_HOST_IRQ")]
pub type SpiHostIrq = crate::Reg<spi_host_irq::SpiHostIrqSpec>;
#[doc = "SPI_HOST_IRQ"]
pub mod spi_host_irq;
#[doc = "TPTC_DBS_CONFIG (rw) register accessor: TPTC_DBS_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tptc_dbs_config`]
module"]
#[doc(alias = "TPTC_DBS_CONFIG")]
pub type TptcDbsConfig = crate::Reg<tptc_dbs_config::TptcDbsConfigSpec>;
#[doc = "TPTC_DBS_CONFIG"]
pub mod tptc_dbs_config;
#[doc = "TPCC_PARITY_CTRL (rw) register accessor: TPCC_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpcc_parity_ctrl`]
module"]
#[doc(alias = "TPCC_PARITY_CTRL")]
pub type TpccParityCtrl = crate::Reg<tpcc_parity_ctrl::TpccParityCtrlSpec>;
#[doc = "TPCC_PARITY_CTRL"]
pub mod tpcc_parity_ctrl;
#[doc = "TPCC_PARITY_STATUS (rw) register accessor: TPCC_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpcc_parity_status`]
module"]
#[doc(alias = "TPCC_PARITY_STATUS")]
pub type TpccParityStatus = crate::Reg<tpcc_parity_status::TpccParityStatusSpec>;
#[doc = "TPCC_PARITY_STATUS"]
pub mod tpcc_parity_status;
#[doc = "MSS_DBG_ACK_CTL0 (rw) register accessor: MSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbg_ack_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbg_ack_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dbg_ack_ctl0`]
module"]
#[doc(alias = "MSS_DBG_ACK_CTL0")]
pub type MssDbgAckCtl0 = crate::Reg<mss_dbg_ack_ctl0::MssDbgAckCtl0Spec>;
#[doc = "MSS_DBG_ACK_CTL0"]
pub mod mss_dbg_ack_ctl0;
#[doc = "MSS_DBG_ACK_CTL1 (rw) register accessor: MSS_DBG_ACK_CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbg_ack_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbg_ack_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dbg_ack_ctl1`]
module"]
#[doc(alias = "MSS_DBG_ACK_CTL1")]
pub type MssDbgAckCtl1 = crate::Reg<mss_dbg_ack_ctl1::MssDbgAckCtl1Spec>;
#[doc = "MSS_DBG_ACK_CTL1"]
pub mod mss_dbg_ack_ctl1;
#[doc = "CPSW_CONTROL (rw) register accessor: CPSW_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_control`]
module"]
#[doc(alias = "CPSW_CONTROL")]
pub type CpswControl = crate::Reg<cpsw_control::CpswControlSpec>;
#[doc = "CPSW_CONTROL"]
pub mod cpsw_control;
#[doc = "MSS_TPCC_A_ERRAGG_MASK (rw) register accessor: MSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_erragg_mask`]
module"]
#[doc(alias = "MSS_TPCC_A_ERRAGG_MASK")]
pub type MssTpccAErraggMask = crate::Reg<mss_tpcc_a_erragg_mask::MssTpccAErraggMaskSpec>;
#[doc = "MSS_TPCC_A_ERRAGG_MASK"]
pub mod mss_tpcc_a_erragg_mask;
#[doc = "MSS_TPCC_A_ERRAGG_STATUS (rw) register accessor: MSS_TPCC_A_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_erragg_status`]
module"]
#[doc(alias = "MSS_TPCC_A_ERRAGG_STATUS")]
pub type MssTpccAErraggStatus = crate::Reg<mss_tpcc_a_erragg_status::MssTpccAErraggStatusSpec>;
#[doc = "MSS_TPCC_A_ERRAGG_STATUS"]
pub mod mss_tpcc_a_erragg_status;
#[doc = "MSS_TPCC_A_ERRAGG_STATUS_RAW (rw) register accessor: MSS_TPCC_A_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_erragg_status_raw`]
module"]
#[doc(alias = "MSS_TPCC_A_ERRAGG_STATUS_RAW")]
pub type MssTpccAErraggStatusRaw =
    crate::Reg<mss_tpcc_a_erragg_status_raw::MssTpccAErraggStatusRawSpec>;
#[doc = "MSS_TPCC_A_ERRAGG_STATUS_RAW"]
pub mod mss_tpcc_a_erragg_status_raw;
#[doc = "MSS_TPCC_A_INTAGG_MASK (rw) register accessor: MSS_TPCC_A_INTAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_intagg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_intagg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_intagg_mask`]
module"]
#[doc(alias = "MSS_TPCC_A_INTAGG_MASK")]
pub type MssTpccAIntaggMask = crate::Reg<mss_tpcc_a_intagg_mask::MssTpccAIntaggMaskSpec>;
#[doc = "MSS_TPCC_A_INTAGG_MASK"]
pub mod mss_tpcc_a_intagg_mask;
#[doc = "MSS_TPCC_A_INTAGG_STATUS (rw) register accessor: MSS_TPCC_A_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_intagg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_intagg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_intagg_status`]
module"]
#[doc(alias = "MSS_TPCC_A_INTAGG_STATUS")]
pub type MssTpccAIntaggStatus = crate::Reg<mss_tpcc_a_intagg_status::MssTpccAIntaggStatusSpec>;
#[doc = "MSS_TPCC_A_INTAGG_STATUS"]
pub mod mss_tpcc_a_intagg_status;
#[doc = "MSS_TPCC_A_INTAGG_STATUS_RAW (rw) register accessor: MSS_TPCC_A_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_a_intagg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_a_intagg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tpcc_a_intagg_status_raw`]
module"]
#[doc(alias = "MSS_TPCC_A_INTAGG_STATUS_RAW")]
pub type MssTpccAIntaggStatusRaw =
    crate::Reg<mss_tpcc_a_intagg_status_raw::MssTpccAIntaggStatusRawSpec>;
#[doc = "MSS_TPCC_A_INTAGG_STATUS_RAW"]
pub mod mss_tpcc_a_intagg_status_raw;
#[doc = "MSS_BUS_SAFETY_CTRL (rw) register accessor: MSS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_BUS_SAFETY_CTRL")]
pub type MssBusSafetyCtrl = crate::Reg<mss_bus_safety_ctrl::MssBusSafetyCtrlSpec>;
#[doc = "MSS_BUS_SAFETY_CTRL"]
pub mod mss_bus_safety_ctrl;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_CTRL (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_CTRL")]
pub type MssCr5aAxiRdBusSafetyCtrl =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_ctrl::MssCr5aAxiRdBusSafetyCtrlSpec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_CTRL"]
pub mod mss_cr5a_axi_rd_bus_safety_ctrl;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_FI (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_fi`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_FI")]
pub type MssCr5aAxiRdBusSafetyFi =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_fi::MssCr5aAxiRdBusSafetyFiSpec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_FI"]
pub mod mss_cr5a_axi_rd_bus_safety_fi;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_err`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR")]
pub type MssCr5aAxiRdBusSafetyErr =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_err::MssCr5aAxiRdBusSafetyErrSpec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR"]
pub mod mss_cr5a_axi_rd_bus_safety_err;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssCr5aAxiRdBusSafetyErrStatData0 =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_err_stat_data0::MssCr5aAxiRdBusSafetyErrStatData0Spec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_cr5a_axi_rd_bus_safety_err_stat_data0;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssCr5aAxiRdBusSafetyErrStatCmd =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_err_stat_cmd::MssCr5aAxiRdBusSafetyErrStatCmdSpec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_cr5a_axi_rd_bus_safety_err_stat_cmd;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_rd_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_rd_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_rd_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_READ")]
pub type MssCr5aAxiRdBusSafetyErrStatRead =
    crate::Reg<mss_cr5a_axi_rd_bus_safety_err_stat_read::MssCr5aAxiRdBusSafetyErrStatReadSpec>;
#[doc = "MSS_CR5A_AXI_RD_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_cr5a_axi_rd_bus_safety_err_stat_read;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_CTRL (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_CTRL")]
pub type MssCr5aAxiWrBusSafetyCtrl =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_ctrl::MssCr5aAxiWrBusSafetyCtrlSpec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_CTRL"]
pub mod mss_cr5a_axi_wr_bus_safety_ctrl;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_FI (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_fi`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_FI")]
pub type MssCr5aAxiWrBusSafetyFi =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_fi::MssCr5aAxiWrBusSafetyFiSpec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_FI"]
pub mod mss_cr5a_axi_wr_bus_safety_fi;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_err`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR")]
pub type MssCr5aAxiWrBusSafetyErr =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_err::MssCr5aAxiWrBusSafetyErrSpec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR"]
pub mod mss_cr5a_axi_wr_bus_safety_err;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssCr5aAxiWrBusSafetyErrStatData0 =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_err_stat_data0::MssCr5aAxiWrBusSafetyErrStatData0Spec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_cr5a_axi_wr_bus_safety_err_stat_data0;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssCr5aAxiWrBusSafetyErrStatCmd =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_err_stat_cmd::MssCr5aAxiWrBusSafetyErrStatCmdSpec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_cr5a_axi_wr_bus_safety_err_stat_cmd;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssCr5aAxiWrBusSafetyErrStatWrite =
    crate::Reg<mss_cr5a_axi_wr_bus_safety_err_stat_write::MssCr5aAxiWrBusSafetyErrStatWriteSpec>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_cr5a_axi_wr_bus_safety_err_stat_write;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_wr_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_wr_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_wr_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssCr5aAxiWrBusSafetyErrStatWriteresp = crate::Reg<
    mss_cr5a_axi_wr_bus_safety_err_stat_writeresp::MssCr5aAxiWrBusSafetyErrStatWriterespSpec,
>;
#[doc = "MSS_CR5A_AXI_WR_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_cr5a_axi_wr_bus_safety_err_stat_writeresp;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_CTRL (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_CTRL")]
pub type MssCr5aAxiSBusSafetyCtrl =
    crate::Reg<mss_cr5a_axi_s_bus_safety_ctrl::MssCr5aAxiSBusSafetyCtrlSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_CTRL"]
pub mod mss_cr5a_axi_s_bus_safety_ctrl;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_FI (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_fi`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_FI")]
pub type MssCr5aAxiSBusSafetyFi =
    crate::Reg<mss_cr5a_axi_s_bus_safety_fi::MssCr5aAxiSBusSafetyFiSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_FI"]
pub mod mss_cr5a_axi_s_bus_safety_fi;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR")]
pub type MssCr5aAxiSBusSafetyErr =
    crate::Reg<mss_cr5a_axi_s_bus_safety_err::MssCr5aAxiSBusSafetyErrSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR"]
pub mod mss_cr5a_axi_s_bus_safety_err;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssCr5aAxiSBusSafetyErrStatData0 =
    crate::Reg<mss_cr5a_axi_s_bus_safety_err_stat_data0::MssCr5aAxiSBusSafetyErrStatData0Spec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_cr5a_axi_s_bus_safety_err_stat_data0;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssCr5aAxiSBusSafetyErrStatCmd =
    crate::Reg<mss_cr5a_axi_s_bus_safety_err_stat_cmd::MssCr5aAxiSBusSafetyErrStatCmdSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_cr5a_axi_s_bus_safety_err_stat_cmd;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssCr5aAxiSBusSafetyErrStatWrite =
    crate::Reg<mss_cr5a_axi_s_bus_safety_err_stat_write::MssCr5aAxiSBusSafetyErrStatWriteSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_cr5a_axi_s_bus_safety_err_stat_write;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_READ")]
pub type MssCr5aAxiSBusSafetyErrStatRead =
    crate::Reg<mss_cr5a_axi_s_bus_safety_err_stat_read::MssCr5aAxiSBusSafetyErrStatReadSpec>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_cr5a_axi_s_bus_safety_err_stat_read;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_axi_s_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_axi_s_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_axi_s_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssCr5aAxiSBusSafetyErrStatWriteresp = crate::Reg<
    mss_cr5a_axi_s_bus_safety_err_stat_writeresp::MssCr5aAxiSBusSafetyErrStatWriterespSpec,
>;
#[doc = "MSS_CR5A_AXI_S_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_cr5a_axi_s_bus_safety_err_stat_writeresp;
#[doc = "MSS_L2_A_BUS_SAFETY_CTRL (rw) register accessor: MSS_L2_A_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_CTRL")]
pub type MssL2ABusSafetyCtrl = crate::Reg<mss_l2_a_bus_safety_ctrl::MssL2ABusSafetyCtrlSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_CTRL"]
pub mod mss_l2_a_bus_safety_ctrl;
#[doc = "MSS_L2_A_BUS_SAFETY_FI (rw) register accessor: MSS_L2_A_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_fi`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_FI")]
pub type MssL2ABusSafetyFi = crate::Reg<mss_l2_a_bus_safety_fi::MssL2ABusSafetyFiSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_FI"]
pub mod mss_l2_a_bus_safety_fi;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR")]
pub type MssL2ABusSafetyErr = crate::Reg<mss_l2_a_bus_safety_err::MssL2ABusSafetyErrSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR"]
pub mod mss_l2_a_bus_safety_err;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssL2ABusSafetyErrStatData0 =
    crate::Reg<mss_l2_a_bus_safety_err_stat_data0::MssL2ABusSafetyErrStatData0Spec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_l2_a_bus_safety_err_stat_data0;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssL2ABusSafetyErrStatCmd =
    crate::Reg<mss_l2_a_bus_safety_err_stat_cmd::MssL2ABusSafetyErrStatCmdSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_l2_a_bus_safety_err_stat_cmd;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssL2ABusSafetyErrStatWrite =
    crate::Reg<mss_l2_a_bus_safety_err_stat_write::MssL2ABusSafetyErrStatWriteSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_l2_a_bus_safety_err_stat_write;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR_STAT_READ")]
pub type MssL2ABusSafetyErrStatRead =
    crate::Reg<mss_l2_a_bus_safety_err_stat_read::MssL2ABusSafetyErrStatReadSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_l2_a_bus_safety_err_stat_read;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_a_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_a_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_a_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssL2ABusSafetyErrStatWriteresp =
    crate::Reg<mss_l2_a_bus_safety_err_stat_writeresp::MssL2ABusSafetyErrStatWriterespSpec>;
#[doc = "MSS_L2_A_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_l2_a_bus_safety_err_stat_writeresp;
#[doc = "MSS_L2_B_BUS_SAFETY_CTRL (rw) register accessor: MSS_L2_B_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_CTRL")]
pub type MssL2BBusSafetyCtrl = crate::Reg<mss_l2_b_bus_safety_ctrl::MssL2BBusSafetyCtrlSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_CTRL"]
pub mod mss_l2_b_bus_safety_ctrl;
#[doc = "MSS_L2_B_BUS_SAFETY_FI (rw) register accessor: MSS_L2_B_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_fi`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_FI")]
pub type MssL2BBusSafetyFi = crate::Reg<mss_l2_b_bus_safety_fi::MssL2BBusSafetyFiSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_FI"]
pub mod mss_l2_b_bus_safety_fi;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR")]
pub type MssL2BBusSafetyErr = crate::Reg<mss_l2_b_bus_safety_err::MssL2BBusSafetyErrSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR"]
pub mod mss_l2_b_bus_safety_err;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssL2BBusSafetyErrStatData0 =
    crate::Reg<mss_l2_b_bus_safety_err_stat_data0::MssL2BBusSafetyErrStatData0Spec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_l2_b_bus_safety_err_stat_data0;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssL2BBusSafetyErrStatCmd =
    crate::Reg<mss_l2_b_bus_safety_err_stat_cmd::MssL2BBusSafetyErrStatCmdSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_l2_b_bus_safety_err_stat_cmd;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssL2BBusSafetyErrStatWrite =
    crate::Reg<mss_l2_b_bus_safety_err_stat_write::MssL2BBusSafetyErrStatWriteSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_l2_b_bus_safety_err_stat_write;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR_STAT_READ")]
pub type MssL2BBusSafetyErrStatRead =
    crate::Reg<mss_l2_b_bus_safety_err_stat_read::MssL2BBusSafetyErrStatReadSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_l2_b_bus_safety_err_stat_read;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_b_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_b_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_b_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssL2BBusSafetyErrStatWriteresp =
    crate::Reg<mss_l2_b_bus_safety_err_stat_writeresp::MssL2BBusSafetyErrStatWriterespSpec>;
#[doc = "MSS_L2_B_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_l2_b_bus_safety_err_stat_writeresp;
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT0 (rw) register accessor: MSS_BUS_SAFETY_SEC_ERR_STAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_sec_err_stat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_sec_err_stat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_bus_safety_sec_err_stat0`]
module"]
#[doc(alias = "MSS_BUS_SAFETY_SEC_ERR_STAT0")]
pub type MssBusSafetySecErrStat0 =
    crate::Reg<mss_bus_safety_sec_err_stat0::MssBusSafetySecErrStat0Spec>;
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT0"]
pub mod mss_bus_safety_sec_err_stat0;
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT1 (rw) register accessor: MSS_BUS_SAFETY_SEC_ERR_STAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_sec_err_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_sec_err_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_bus_safety_sec_err_stat1`]
module"]
#[doc(alias = "MSS_BUS_SAFETY_SEC_ERR_STAT1")]
pub type MssBusSafetySecErrStat1 =
    crate::Reg<mss_bus_safety_sec_err_stat1::MssBusSafetySecErrStat1Spec>;
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT1"]
pub mod mss_bus_safety_sec_err_stat1;
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
#[doc = "HW_REG4 (rw) register accessor: HW_REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg4`]
module"]
#[doc(alias = "HW_REG4")]
pub type HwReg4 = crate::Reg<hw_reg4::HwReg4Spec>;
#[doc = "HW_REG4"]
pub mod hw_reg4;
#[doc = "HW_REG5 (rw) register accessor: HW_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg5`]
module"]
#[doc(alias = "HW_REG5")]
pub type HwReg5 = crate::Reg<hw_reg5::HwReg5Spec>;
#[doc = "HW_REG5"]
pub mod hw_reg5;
#[doc = "HW_REG6 (rw) register accessor: HW_REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg6`]
module"]
#[doc(alias = "HW_REG6")]
pub type HwReg6 = crate::Reg<hw_reg6::HwReg6Spec>;
#[doc = "HW_REG6"]
pub mod hw_reg6;
#[doc = "HW_REG7 (rw) register accessor: HW_REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg7`]
module"]
#[doc(alias = "HW_REG7")]
pub type HwReg7 = crate::Reg<hw_reg7::HwReg7Spec>;
#[doc = "HW_REG7"]
pub mod hw_reg7;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_CTRL (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_CTRL")]
pub type MssCr5aAhbBusSafetyCtrl =
    crate::Reg<mss_cr5a_ahb_bus_safety_ctrl::MssCr5aAhbBusSafetyCtrlSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_CTRL"]
pub mod mss_cr5a_ahb_bus_safety_ctrl;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_FI (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_fi`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_FI")]
pub type MssCr5aAhbBusSafetyFi = crate::Reg<mss_cr5a_ahb_bus_safety_fi::MssCr5aAhbBusSafetyFiSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_FI"]
pub mod mss_cr5a_ahb_bus_safety_fi;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR")]
pub type MssCr5aAhbBusSafetyErr =
    crate::Reg<mss_cr5a_ahb_bus_safety_err::MssCr5aAhbBusSafetyErrSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR"]
pub mod mss_cr5a_ahb_bus_safety_err;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssCr5aAhbBusSafetyErrStatData0 =
    crate::Reg<mss_cr5a_ahb_bus_safety_err_stat_data0::MssCr5aAhbBusSafetyErrStatData0Spec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_cr5a_ahb_bus_safety_err_stat_data0;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssCr5aAhbBusSafetyErrStatCmd =
    crate::Reg<mss_cr5a_ahb_bus_safety_err_stat_cmd::MssCr5aAhbBusSafetyErrStatCmdSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_cr5a_ahb_bus_safety_err_stat_cmd;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssCr5aAhbBusSafetyErrStatWrite =
    crate::Reg<mss_cr5a_ahb_bus_safety_err_stat_write::MssCr5aAhbBusSafetyErrStatWriteSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_cr5a_ahb_bus_safety_err_stat_write;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_READ")]
pub type MssCr5aAhbBusSafetyErrStatRead =
    crate::Reg<mss_cr5a_ahb_bus_safety_err_stat_read::MssCr5aAhbBusSafetyErrStatReadSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_cr5a_ahb_bus_safety_err_stat_read;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_ahb_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_ahb_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_ahb_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssCr5aAhbBusSafetyErrStatWriteresp =
    crate::Reg<mss_cr5a_ahb_bus_safety_err_stat_writeresp::MssCr5aAhbBusSafetyErrStatWriterespSpec>;
#[doc = "MSS_CR5A_AHB_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_cr5a_ahb_bus_safety_err_stat_writeresp;
#[doc = "DMM_CTRL_REG (rw) register accessor: DMM_CTRL_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`dmm_ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmm_ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmm_ctrl_reg`]
module"]
#[doc(alias = "DMM_CTRL_REG")]
pub type DmmCtrlReg = crate::Reg<dmm_ctrl_reg::DmmCtrlRegSpec>;
#[doc = "DMM_CTRL_REG"]
pub mod dmm_ctrl_reg;
#[doc = "MSS_CR5A_MBOX_WRITE_DONE (rw) register accessor: MSS_CR5A_MBOX_WRITE_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_mbox_write_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_mbox_write_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_mbox_write_done`]
module"]
#[doc(alias = "MSS_CR5A_MBOX_WRITE_DONE")]
pub type MssCr5aMboxWriteDone = crate::Reg<mss_cr5a_mbox_write_done::MssCr5aMboxWriteDoneSpec>;
#[doc = "MSS_CR5A_MBOX_WRITE_DONE"]
pub mod mss_cr5a_mbox_write_done;
#[doc = "MSS_CR5A_MBOX_READ_REQ (rw) register accessor: MSS_CR5A_MBOX_READ_REQ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_mbox_read_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_mbox_read_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_mbox_read_req`]
module"]
#[doc(alias = "MSS_CR5A_MBOX_READ_REQ")]
pub type MssCr5aMboxReadReq = crate::Reg<mss_cr5a_mbox_read_req::MssCr5aMboxReadReqSpec>;
#[doc = "MSS_CR5A_MBOX_READ_REQ"]
pub mod mss_cr5a_mbox_read_req;
#[doc = "MSS_CR5A_MBOX_READ_DONE (rw) register accessor: MSS_CR5A_MBOX_READ_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_mbox_read_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_mbox_read_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cr5a_mbox_read_done`]
module"]
#[doc(alias = "MSS_CR5A_MBOX_READ_DONE")]
pub type MssCr5aMboxReadDone = crate::Reg<mss_cr5a_mbox_read_done::MssCr5aMboxReadDoneSpec>;
#[doc = "MSS_CR5A_MBOX_READ_DONE"]
pub mod mss_cr5a_mbox_read_done;
#[doc = "MSS_PBIST_KEY_RST (rw) register accessor: MSS_PBIST_KEY_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_key_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_key_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pbist_key_rst`]
module"]
#[doc(alias = "MSS_PBIST_KEY_RST")]
pub type MssPbistKeyRst = crate::Reg<mss_pbist_key_rst::MssPbistKeyRstSpec>;
#[doc = "MSS_PBIST_KEY_RST"]
pub mod mss_pbist_key_rst;
#[doc = "MSS_PBIST_REG0 (rw) register accessor: MSS_PBIST_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pbist_reg0`]
module"]
#[doc(alias = "MSS_PBIST_REG0")]
pub type MssPbistReg0 = crate::Reg<mss_pbist_reg0::MssPbistReg0Spec>;
#[doc = "MSS_PBIST_REG0"]
pub mod mss_pbist_reg0;
#[doc = "MSS_PBIST_REG1 (rw) register accessor: MSS_PBIST_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pbist_reg1`]
module"]
#[doc(alias = "MSS_PBIST_REG1")]
pub type MssPbistReg1 = crate::Reg<mss_pbist_reg1::MssPbistReg1Spec>;
#[doc = "MSS_PBIST_REG1"]
pub mod mss_pbist_reg1;
#[doc = "MSS_PBIST_REG2 (rw) register accessor: MSS_PBIST_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_pbist_reg2`]
module"]
#[doc(alias = "MSS_PBIST_REG2")]
pub type MssPbistReg2 = crate::Reg<mss_pbist_reg2::MssPbistReg2Spec>;
#[doc = "MSS_PBIST_REG2"]
pub mod mss_pbist_reg2;
#[doc = "MSS_QSPI_CONFIG (rw) register accessor: MSS_QSPI_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_config`]
module"]
#[doc(alias = "MSS_QSPI_CONFIG")]
pub type MssQspiConfig = crate::Reg<mss_qspi_config::MssQspiConfigSpec>;
#[doc = "MSS_QSPI_CONFIG"]
pub mod mss_qspi_config;
#[doc = "MSS_STC_CONTROL (rw) register accessor: MSS_STC_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_stc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_stc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_stc_control`]
module"]
#[doc(alias = "MSS_STC_CONTROL")]
pub type MssStcControl = crate::Reg<mss_stc_control::MssStcControlSpec>;
#[doc = "MSS_STC_CONTROL"]
pub mod mss_stc_control;
#[doc = "MSS_CTI_TRIG_SEL (rw) register accessor: MSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cti_trig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cti_trig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_cti_trig_sel`]
module"]
#[doc(alias = "MSS_CTI_TRIG_SEL")]
pub type MssCtiTrigSel = crate::Reg<mss_cti_trig_sel::MssCtiTrigSelSpec>;
#[doc = "MSS_CTI_TRIG_SEL"]
pub mod mss_cti_trig_sel;
#[doc = "MSS_DBGSS_CTI_TRIG_SEL (rw) register accessor: MSS_DBGSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbgss_cti_trig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbgss_cti_trig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dbgss_cti_trig_sel`]
module"]
#[doc(alias = "MSS_DBGSS_CTI_TRIG_SEL")]
pub type MssDbgssCtiTrigSel = crate::Reg<mss_dbgss_cti_trig_sel::MssDbgssCtiTrigSelSpec>;
#[doc = "MSS_DBGSS_CTI_TRIG_SEL"]
pub mod mss_dbgss_cti_trig_sel;
#[doc = "MSS_BOOT_INFO_REG0 (rw) register accessor: MSS_BOOT_INFO_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg0`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG0")]
pub type MssBootInfoReg0 = crate::Reg<mss_boot_info_reg0::MssBootInfoReg0Spec>;
#[doc = "MSS_BOOT_INFO_REG0"]
pub mod mss_boot_info_reg0;
#[doc = "MSS_BOOT_INFO_REG1 (rw) register accessor: MSS_BOOT_INFO_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg1`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG1")]
pub type MssBootInfoReg1 = crate::Reg<mss_boot_info_reg1::MssBootInfoReg1Spec>;
#[doc = "MSS_BOOT_INFO_REG1"]
pub mod mss_boot_info_reg1;
#[doc = "MSS_BOOT_INFO_REG2 (rw) register accessor: MSS_BOOT_INFO_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg2`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG2")]
pub type MssBootInfoReg2 = crate::Reg<mss_boot_info_reg2::MssBootInfoReg2Spec>;
#[doc = "MSS_BOOT_INFO_REG2"]
pub mod mss_boot_info_reg2;
#[doc = "MSS_BOOT_INFO_REG3 (rw) register accessor: MSS_BOOT_INFO_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg3`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG3")]
pub type MssBootInfoReg3 = crate::Reg<mss_boot_info_reg3::MssBootInfoReg3Spec>;
#[doc = "MSS_BOOT_INFO_REG3"]
pub mod mss_boot_info_reg3;
#[doc = "MSS_BOOT_INFO_REG4 (rw) register accessor: MSS_BOOT_INFO_REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg4`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG4")]
pub type MssBootInfoReg4 = crate::Reg<mss_boot_info_reg4::MssBootInfoReg4Spec>;
#[doc = "MSS_BOOT_INFO_REG4"]
pub mod mss_boot_info_reg4;
#[doc = "MSS_BOOT_INFO_REG5 (rw) register accessor: MSS_BOOT_INFO_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg5`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG5")]
pub type MssBootInfoReg5 = crate::Reg<mss_boot_info_reg5::MssBootInfoReg5Spec>;
#[doc = "MSS_BOOT_INFO_REG5"]
pub mod mss_boot_info_reg5;
#[doc = "MSS_BOOT_INFO_REG6 (rw) register accessor: MSS_BOOT_INFO_REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg6`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG6")]
pub type MssBootInfoReg6 = crate::Reg<mss_boot_info_reg6::MssBootInfoReg6Spec>;
#[doc = "MSS_BOOT_INFO_REG6"]
pub mod mss_boot_info_reg6;
#[doc = "MSS_BOOT_INFO_REG7 (rw) register accessor: MSS_BOOT_INFO_REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_boot_info_reg7`]
module"]
#[doc(alias = "MSS_BOOT_INFO_REG7")]
pub type MssBootInfoReg7 = crate::Reg<mss_boot_info_reg7::MssBootInfoReg7Spec>;
#[doc = "MSS_BOOT_INFO_REG7"]
pub mod mss_boot_info_reg7;
#[doc = "MSS_TPTC_ECCAGGR_CLK_CNTRL (rw) register accessor: MSS_TPTC_ECCAGGR_CLK_CNTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tptc_eccaggr_clk_cntrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tptc_eccaggr_clk_cntrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tptc_eccaggr_clk_cntrl`]
module"]
#[doc(alias = "MSS_TPTC_ECCAGGR_CLK_CNTRL")]
pub type MssTptcEccaggrClkCntrl =
    crate::Reg<mss_tptc_eccaggr_clk_cntrl::MssTptcEccaggrClkCntrlSpec>;
#[doc = "MSS_TPTC_ECCAGGR_CLK_CNTRL"]
pub mod mss_tptc_eccaggr_clk_cntrl;
#[doc = "MSS_PERIPH_ERRAGG_MASK0 (rw) register accessor: MSS_PERIPH_ERRAGG_MASK0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_mask0`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_MASK0")]
pub type MssPeriphErraggMask0 = crate::Reg<mss_periph_erragg_mask0::MssPeriphErraggMask0Spec>;
#[doc = "MSS_PERIPH_ERRAGG_MASK0"]
pub mod mss_periph_erragg_mask0;
#[doc = "MSS_PERIPH_ERRAGG_STATUS0 (rw) register accessor: MSS_PERIPH_ERRAGG_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_status0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_status0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_status0`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_STATUS0")]
pub type MssPeriphErraggStatus0 = crate::Reg<mss_periph_erragg_status0::MssPeriphErraggStatus0Spec>;
#[doc = "MSS_PERIPH_ERRAGG_STATUS0"]
pub mod mss_periph_erragg_status0;
#[doc = "MSS_PERIPH_ERRAGG_STATUS_RAW0 (rw) register accessor: MSS_PERIPH_ERRAGG_STATUS_RAW0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_status_raw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_status_raw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_status_raw0`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_STATUS_RAW0")]
pub type MssPeriphErraggStatusRaw0 =
    crate::Reg<mss_periph_erragg_status_raw0::MssPeriphErraggStatusRaw0Spec>;
#[doc = "MSS_PERIPH_ERRAGG_STATUS_RAW0"]
pub mod mss_periph_erragg_status_raw0;
#[doc = "MSS_PERIPH_ERRAGG_MASK1 (rw) register accessor: MSS_PERIPH_ERRAGG_MASK1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_mask1`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_MASK1")]
pub type MssPeriphErraggMask1 = crate::Reg<mss_periph_erragg_mask1::MssPeriphErraggMask1Spec>;
#[doc = "MSS_PERIPH_ERRAGG_MASK1"]
pub mod mss_periph_erragg_mask1;
#[doc = "MSS_PERIPH_ERRAGG_STATUS1 (rw) register accessor: MSS_PERIPH_ERRAGG_STATUS1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_status1`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_STATUS1")]
pub type MssPeriphErraggStatus1 = crate::Reg<mss_periph_erragg_status1::MssPeriphErraggStatus1Spec>;
#[doc = "MSS_PERIPH_ERRAGG_STATUS1"]
pub mod mss_periph_erragg_status1;
#[doc = "MSS_PERIPH_ERRAGG_STATUS_RAW1 (rw) register accessor: MSS_PERIPH_ERRAGG_STATUS_RAW1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_status_raw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_status_raw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_periph_erragg_status_raw1`]
module"]
#[doc(alias = "MSS_PERIPH_ERRAGG_STATUS_RAW1")]
pub type MssPeriphErraggStatusRaw1 =
    crate::Reg<mss_periph_erragg_status_raw1::MssPeriphErraggStatusRaw1Spec>;
#[doc = "MSS_PERIPH_ERRAGG_STATUS_RAW1"]
pub mod mss_periph_erragg_status_raw1;
#[doc = "MSS_DMM_EVENT0_REG (rw) register accessor: MSS_DMM_EVENT0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event0_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event0_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event0_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT0_REG")]
pub type MssDmmEvent0Reg = crate::Reg<mss_dmm_event0_reg::MssDmmEvent0RegSpec>;
#[doc = "MSS_DMM_EVENT0_REG"]
pub mod mss_dmm_event0_reg;
#[doc = "MSS_DMM_EVENT1_REG (rw) register accessor: MSS_DMM_EVENT1_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event1_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event1_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event1_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT1_REG")]
pub type MssDmmEvent1Reg = crate::Reg<mss_dmm_event1_reg::MssDmmEvent1RegSpec>;
#[doc = "MSS_DMM_EVENT1_REG"]
pub mod mss_dmm_event1_reg;
#[doc = "MSS_DMM_EVENT2_REG (rw) register accessor: MSS_DMM_EVENT2_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event2_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT2_REG")]
pub type MssDmmEvent2Reg = crate::Reg<mss_dmm_event2_reg::MssDmmEvent2RegSpec>;
#[doc = "MSS_DMM_EVENT2_REG"]
pub mod mss_dmm_event2_reg;
#[doc = "MSS_DMM_EVENT3_REG (rw) register accessor: MSS_DMM_EVENT3_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event3_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event3_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event3_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT3_REG")]
pub type MssDmmEvent3Reg = crate::Reg<mss_dmm_event3_reg::MssDmmEvent3RegSpec>;
#[doc = "MSS_DMM_EVENT3_REG"]
pub mod mss_dmm_event3_reg;
#[doc = "MSS_DMM_EVENT4_REG (rw) register accessor: MSS_DMM_EVENT4_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event4_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event4_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event4_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT4_REG")]
pub type MssDmmEvent4Reg = crate::Reg<mss_dmm_event4_reg::MssDmmEvent4RegSpec>;
#[doc = "MSS_DMM_EVENT4_REG"]
pub mod mss_dmm_event4_reg;
#[doc = "MSS_DMM_EVENT5_REG (rw) register accessor: MSS_DMM_EVENT5_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event5_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event5_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event5_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT5_REG")]
pub type MssDmmEvent5Reg = crate::Reg<mss_dmm_event5_reg::MssDmmEvent5RegSpec>;
#[doc = "MSS_DMM_EVENT5_REG"]
pub mod mss_dmm_event5_reg;
#[doc = "MSS_DMM_EVENT6_REG (rw) register accessor: MSS_DMM_EVENT6_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event6_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event6_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event6_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT6_REG")]
pub type MssDmmEvent6Reg = crate::Reg<mss_dmm_event6_reg::MssDmmEvent6RegSpec>;
#[doc = "MSS_DMM_EVENT6_REG"]
pub mod mss_dmm_event6_reg;
#[doc = "MSS_DMM_EVENT7_REG (rw) register accessor: MSS_DMM_EVENT7_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event7_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event7_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event7_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT7_REG")]
pub type MssDmmEvent7Reg = crate::Reg<mss_dmm_event7_reg::MssDmmEvent7RegSpec>;
#[doc = "MSS_DMM_EVENT7_REG"]
pub mod mss_dmm_event7_reg;
#[doc = "MSS_DMM_EVENT8_REG (rw) register accessor: MSS_DMM_EVENT8_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event8_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event8_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event8_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT8_REG")]
pub type MssDmmEvent8Reg = crate::Reg<mss_dmm_event8_reg::MssDmmEvent8RegSpec>;
#[doc = "MSS_DMM_EVENT8_REG"]
pub mod mss_dmm_event8_reg;
#[doc = "MSS_DMM_EVENT9_REG (rw) register accessor: MSS_DMM_EVENT9_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event9_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event9_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event9_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT9_REG")]
pub type MssDmmEvent9Reg = crate::Reg<mss_dmm_event9_reg::MssDmmEvent9RegSpec>;
#[doc = "MSS_DMM_EVENT9_REG"]
pub mod mss_dmm_event9_reg;
#[doc = "MSS_DMM_EVENT10_REG (rw) register accessor: MSS_DMM_EVENT10_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event10_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event10_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event10_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT10_REG")]
pub type MssDmmEvent10Reg = crate::Reg<mss_dmm_event10_reg::MssDmmEvent10RegSpec>;
#[doc = "MSS_DMM_EVENT10_REG"]
pub mod mss_dmm_event10_reg;
#[doc = "MSS_DMM_EVENT11_REG (rw) register accessor: MSS_DMM_EVENT11_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event11_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event11_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event11_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT11_REG")]
pub type MssDmmEvent11Reg = crate::Reg<mss_dmm_event11_reg::MssDmmEvent11RegSpec>;
#[doc = "MSS_DMM_EVENT11_REG"]
pub mod mss_dmm_event11_reg;
#[doc = "MSS_DMM_EVENT12_REG (rw) register accessor: MSS_DMM_EVENT12_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event12_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event12_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event12_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT12_REG")]
pub type MssDmmEvent12Reg = crate::Reg<mss_dmm_event12_reg::MssDmmEvent12RegSpec>;
#[doc = "MSS_DMM_EVENT12_REG"]
pub mod mss_dmm_event12_reg;
#[doc = "MSS_DMM_EVENT13_REG (rw) register accessor: MSS_DMM_EVENT13_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event13_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event13_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event13_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT13_REG")]
pub type MssDmmEvent13Reg = crate::Reg<mss_dmm_event13_reg::MssDmmEvent13RegSpec>;
#[doc = "MSS_DMM_EVENT13_REG"]
pub mod mss_dmm_event13_reg;
#[doc = "MSS_DMM_EVENT14_REG (rw) register accessor: MSS_DMM_EVENT14_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event14_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event14_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event14_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT14_REG")]
pub type MssDmmEvent14Reg = crate::Reg<mss_dmm_event14_reg::MssDmmEvent14RegSpec>;
#[doc = "MSS_DMM_EVENT14_REG"]
pub mod mss_dmm_event14_reg;
#[doc = "MSS_DMM_EVENT15_REG (rw) register accessor: MSS_DMM_EVENT15_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event15_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event15_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_event15_reg`]
module"]
#[doc(alias = "MSS_DMM_EVENT15_REG")]
pub type MssDmmEvent15Reg = crate::Reg<mss_dmm_event15_reg::MssDmmEvent15RegSpec>;
#[doc = "MSS_DMM_EVENT15_REG"]
pub mod mss_dmm_event15_reg;
#[doc = "MSS_TPTC_BOUNDARY_CFG (rw) register accessor: MSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tptc_boundary_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tptc_boundary_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tptc_boundary_cfg`]
module"]
#[doc(alias = "MSS_TPTC_BOUNDARY_CFG")]
pub type MssTptcBoundaryCfg = crate::Reg<mss_tptc_boundary_cfg::MssTptcBoundaryCfgSpec>;
#[doc = "MSS_TPTC_BOUNDARY_CFG"]
pub mod mss_tptc_boundary_cfg;
#[doc = "MSS_TPTC_XID_REORDER_CFG (rw) register accessor: MSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tptc_xid_reorder_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tptc_xid_reorder_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_tptc_xid_reorder_cfg`]
module"]
#[doc(alias = "MSS_TPTC_XID_REORDER_CFG")]
pub type MssTptcXidReorderCfg = crate::Reg<mss_tptc_xid_reorder_cfg::MssTptcXidReorderCfgSpec>;
#[doc = "MSS_TPTC_XID_REORDER_CFG"]
pub mod mss_tptc_xid_reorder_cfg;
#[doc = "GPADC_CTRL (rw) register accessor: GPADC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_ctrl`]
module"]
#[doc(alias = "GPADC_CTRL")]
pub type GpadcCtrl = crate::Reg<gpadc_ctrl::GpadcCtrlSpec>;
#[doc = "GPADC_CTRL"]
pub mod gpadc_ctrl;
#[doc = "HW_Sync_FE_CTRL (rw) register accessor: HW_Sync_FE_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_sync_fe_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_sync_fe_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_sync_fe_ctrl`]
module"]
#[doc(alias = "HW_Sync_FE_CTRL")]
pub type HwSyncFeCtrl = crate::Reg<hw_sync_fe_ctrl::HwSyncFeCtrlSpec>;
#[doc = "HW_Sync_FE_CTRL"]
pub mod hw_sync_fe_ctrl;
#[doc = "DEBUGSS_CSETB_FLUSH (rw) register accessor: DEBUGSS_CSETB_FLUSH\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_csetb_flush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_csetb_flush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_csetb_flush`]
module"]
#[doc(alias = "DEBUGSS_CSETB_FLUSH")]
pub type DebugssCsetbFlush = crate::Reg<debugss_csetb_flush::DebugssCsetbFlushSpec>;
#[doc = "DEBUGSS_CSETB_FLUSH"]
pub mod debugss_csetb_flush;
#[doc = "ANALOG_WU_STATUS_REG_POLARITY_INV (rw) register accessor: ANALOG_WU_STATUS_REG_POLARITY_INV\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_wu_status_reg_polarity_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_wu_status_reg_polarity_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_wu_status_reg_polarity_inv`]
module"]
#[doc(alias = "ANALOG_WU_STATUS_REG_POLARITY_INV")]
pub type AnalogWuStatusRegPolarityInv =
    crate::Reg<analog_wu_status_reg_polarity_inv::AnalogWuStatusRegPolarityInvSpec>;
#[doc = "ANALOG_WU_STATUS_REG_POLARITY_INV"]
pub mod analog_wu_status_reg_polarity_inv;
#[doc = "ANALOG_CLK_STATUS_REG_POLARITY_INV (rw) register accessor: ANALOG_CLK_STATUS_REG_POLARITY_INV\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_clk_status_reg_polarity_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_clk_status_reg_polarity_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_clk_status_reg_polarity_inv`]
module"]
#[doc(alias = "ANALOG_CLK_STATUS_REG_POLARITY_INV")]
pub type AnalogClkStatusRegPolarityInv =
    crate::Reg<analog_clk_status_reg_polarity_inv::AnalogClkStatusRegPolarityInvSpec>;
#[doc = "ANALOG_CLK_STATUS_REG_POLARITY_INV"]
pub mod analog_clk_status_reg_polarity_inv;
#[doc = "ANALOG_WU_STATUS_REG_GRP1_MASK (rw) register accessor: ANALOG_WU_STATUS_REG_GRP1_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_wu_status_reg_grp1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_wu_status_reg_grp1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_wu_status_reg_grp1_mask`]
module"]
#[doc(alias = "ANALOG_WU_STATUS_REG_GRP1_MASK")]
pub type AnalogWuStatusRegGrp1Mask =
    crate::Reg<analog_wu_status_reg_grp1_mask::AnalogWuStatusRegGrp1MaskSpec>;
#[doc = "ANALOG_WU_STATUS_REG_GRP1_MASK"]
pub mod analog_wu_status_reg_grp1_mask;
#[doc = "ANALOG_CLK_STATUS_REG_GRP1_MASK (rw) register accessor: ANALOG_CLK_STATUS_REG_GRP1_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_clk_status_reg_grp1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_clk_status_reg_grp1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_clk_status_reg_grp1_mask`]
module"]
#[doc(alias = "ANALOG_CLK_STATUS_REG_GRP1_MASK")]
pub type AnalogClkStatusRegGrp1Mask =
    crate::Reg<analog_clk_status_reg_grp1_mask::AnalogClkStatusRegGrp1MaskSpec>;
#[doc = "ANALOG_CLK_STATUS_REG_GRP1_MASK"]
pub mod analog_clk_status_reg_grp1_mask;
#[doc = "ANALOG_WU_STATUS_REG_GRP2_MASK (rw) register accessor: ANALOG_WU_STATUS_REG_GRP2_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_wu_status_reg_grp2_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_wu_status_reg_grp2_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_wu_status_reg_grp2_mask`]
module"]
#[doc(alias = "ANALOG_WU_STATUS_REG_GRP2_MASK")]
pub type AnalogWuStatusRegGrp2Mask =
    crate::Reg<analog_wu_status_reg_grp2_mask::AnalogWuStatusRegGrp2MaskSpec>;
#[doc = "ANALOG_WU_STATUS_REG_GRP2_MASK"]
pub mod analog_wu_status_reg_grp2_mask;
#[doc = "ANALOG_CLK_STATUS_REG_GRP2_MASK (rw) register accessor: ANALOG_CLK_STATUS_REG_GRP2_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_clk_status_reg_grp2_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_clk_status_reg_grp2_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_clk_status_reg_grp2_mask`]
module"]
#[doc(alias = "ANALOG_CLK_STATUS_REG_GRP2_MASK")]
pub type AnalogClkStatusRegGrp2Mask =
    crate::Reg<analog_clk_status_reg_grp2_mask::AnalogClkStatusRegGrp2MaskSpec>;
#[doc = "ANALOG_CLK_STATUS_REG_GRP2_MASK"]
pub mod analog_clk_status_reg_grp2_mask;
#[doc = "NERROR_MASK (rw) register accessor: NERROR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`nerror_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nerror_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nerror_mask`]
module"]
#[doc(alias = "NERROR_MASK")]
pub type NerrorMask = crate::Reg<nerror_mask::NerrorMaskSpec>;
#[doc = "NERROR_MASK"]
pub mod nerror_mask;
#[doc = "MSS2R5SS_BUS_SAFETY_CTRL (rw) register accessor: MSS2R5SS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_CTRL")]
pub type Mss2r5ssBusSafetyCtrl = crate::Reg<mss2r5ss_bus_safety_ctrl::Mss2r5ssBusSafetyCtrlSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_CTRL"]
pub mod mss2r5ss_bus_safety_ctrl;
#[doc = "MSS2R5SS_BUS_SAFETY_FI (rw) register accessor: MSS2R5SS_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_fi`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_FI")]
pub type Mss2r5ssBusSafetyFi = crate::Reg<mss2r5ss_bus_safety_fi::Mss2r5ssBusSafetyFiSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_FI"]
pub mod mss2r5ss_bus_safety_fi;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR")]
pub type Mss2r5ssBusSafetyErr = crate::Reg<mss2r5ss_bus_safety_err::Mss2r5ssBusSafetyErrSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR"]
pub mod mss2r5ss_bus_safety_err;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0")]
pub type Mss2r5ssBusSafetyErrStatData0 =
    crate::Reg<mss2r5ss_bus_safety_err_stat_data0::Mss2r5ssBusSafetyErrStatData0Spec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss2r5ss_bus_safety_err_stat_data0;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR_STAT_CMD")]
pub type Mss2r5ssBusSafetyErrStatCmd =
    crate::Reg<mss2r5ss_bus_safety_err_stat_cmd::Mss2r5ssBusSafetyErrStatCmdSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss2r5ss_bus_safety_err_stat_cmd;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE")]
pub type Mss2r5ssBusSafetyErrStatWrite =
    crate::Reg<mss2r5ss_bus_safety_err_stat_write::Mss2r5ssBusSafetyErrStatWriteSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss2r5ss_bus_safety_err_stat_write;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR_STAT_READ")]
pub type Mss2r5ssBusSafetyErrStatRead =
    crate::Reg<mss2r5ss_bus_safety_err_stat_read::Mss2r5ssBusSafetyErrStatReadSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss2r5ss_bus_safety_err_stat_read;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss2r5ss_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss2r5ss_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss2r5ss_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type Mss2r5ssBusSafetyErrStatWriteresp =
    crate::Reg<mss2r5ss_bus_safety_err_stat_writeresp::Mss2r5ssBusSafetyErrStatWriterespSpec>;
#[doc = "MSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss2r5ss_bus_safety_err_stat_writeresp;
#[doc = "DSS2R5SS_BUS_SAFETY_CTRL (rw) register accessor: DSS2R5SS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_ctrl`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_CTRL")]
pub type Dss2r5ssBusSafetyCtrl = crate::Reg<dss2r5ss_bus_safety_ctrl::Dss2r5ssBusSafetyCtrlSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_CTRL"]
pub mod dss2r5ss_bus_safety_ctrl;
#[doc = "DSS2R5SS_BUS_SAFETY_FI (rw) register accessor: DSS2R5SS_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_fi`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_FI")]
pub type Dss2r5ssBusSafetyFi = crate::Reg<dss2r5ss_bus_safety_fi::Dss2r5ssBusSafetyFiSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_FI"]
pub mod dss2r5ss_bus_safety_fi;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR")]
pub type Dss2r5ssBusSafetyErr = crate::Reg<dss2r5ss_bus_safety_err::Dss2r5ssBusSafetyErrSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR"]
pub mod dss2r5ss_bus_safety_err;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0")]
pub type Dss2r5ssBusSafetyErrStatData0 =
    crate::Reg<dss2r5ss_bus_safety_err_stat_data0::Dss2r5ssBusSafetyErrStatData0Spec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod dss2r5ss_bus_safety_err_stat_data0;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD")]
pub type Dss2r5ssBusSafetyErrStatCmd =
    crate::Reg<dss2r5ss_bus_safety_err_stat_cmd::Dss2r5ssBusSafetyErrStatCmdSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_CMD"]
pub mod dss2r5ss_bus_safety_err_stat_cmd;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err_stat_write`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE")]
pub type Dss2r5ssBusSafetyErrStatWrite =
    crate::Reg<dss2r5ss_bus_safety_err_stat_write::Dss2r5ssBusSafetyErrStatWriteSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod dss2r5ss_bus_safety_err_stat_write;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err_stat_read`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR_STAT_READ")]
pub type Dss2r5ssBusSafetyErrStatRead =
    crate::Reg<dss2r5ss_bus_safety_err_stat_read::Dss2r5ssBusSafetyErrStatReadSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_READ"]
pub mod dss2r5ss_bus_safety_err_stat_read;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`dss2r5ss_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss2r5ss_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dss2r5ss_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type Dss2r5ssBusSafetyErrStatWriteresp =
    crate::Reg<dss2r5ss_bus_safety_err_stat_writeresp::Dss2r5ssBusSafetyErrStatWriterespSpec>;
#[doc = "DSS2R5SS_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod dss2r5ss_bus_safety_err_stat_writeresp;
#[doc = "MSS_DMM_ACCESS_MODE (rw) register accessor: MSS_DMM_ACCESS_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_access_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_access_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_dmm_access_mode`]
module"]
#[doc(alias = "MSS_DMM_ACCESS_MODE")]
pub type MssDmmAccessMode = crate::Reg<mss_dmm_access_mode::MssDmmAccessModeSpec>;
#[doc = "MSS_DMM_ACCESS_MODE"]
pub mod mss_dmm_access_mode;
#[doc = "CPSW_HW_TRIG_CTRL (rw) register accessor: CPSW_HW_TRIG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_hw_trig_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_hw_trig_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_hw_trig_ctrl`]
module"]
#[doc(alias = "CPSW_HW_TRIG_CTRL")]
pub type CpswHwTrigCtrl = crate::Reg<cpsw_hw_trig_ctrl::CpswHwTrigCtrlSpec>;
#[doc = "CPSW_HW_TRIG_CTRL"]
pub mod cpsw_hw_trig_ctrl;
#[doc = "CPSW_HW_TRIG_VAL (rw) register accessor: CPSW_HW_TRIG_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_hw_trig_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_hw_trig_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_hw_trig_val`]
module"]
#[doc(alias = "CPSW_HW_TRIG_VAL")]
pub type CpswHwTrigVal = crate::Reg<cpsw_hw_trig_val::CpswHwTrigValSpec>;
#[doc = "CPSW_HW_TRIG_VAL"]
pub mod cpsw_hw_trig_val;
#[doc = "CPSW_TRIG_CAPTURE_COUNT (rw) register accessor: CPSW_TRIG_CAPTURE_COUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_trig_capture_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_trig_capture_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_trig_capture_count`]
module"]
#[doc(alias = "CPSW_TRIG_CAPTURE_COUNT")]
pub type CpswTrigCaptureCount = crate::Reg<cpsw_trig_capture_count::CpswTrigCaptureCountSpec>;
#[doc = "CPSW_TRIG_CAPTURE_COUNT"]
pub mod cpsw_trig_capture_count;
#[doc = "MSS_L2_C_BUS_SAFETY_CTRL (rw) register accessor: MSS_L2_C_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_ctrl`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_CTRL")]
pub type MssL2CBusSafetyCtrl = crate::Reg<mss_l2_c_bus_safety_ctrl::MssL2CBusSafetyCtrlSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_CTRL"]
pub mod mss_l2_c_bus_safety_ctrl;
#[doc = "MSS_L2_C_BUS_SAFETY_FI (rw) register accessor: MSS_L2_C_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_fi`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_FI")]
pub type MssL2CBusSafetyFi = crate::Reg<mss_l2_c_bus_safety_fi::MssL2CBusSafetyFiSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_FI"]
pub mod mss_l2_c_bus_safety_fi;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR")]
pub type MssL2CBusSafetyErr = crate::Reg<mss_l2_c_bus_safety_err::MssL2CBusSafetyErrSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR"]
pub mod mss_l2_c_bus_safety_err;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR_STAT_DATA0")]
pub type MssL2CBusSafetyErrStatData0 =
    crate::Reg<mss_l2_c_bus_safety_err_stat_data0::MssL2CBusSafetyErrStatData0Spec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod mss_l2_c_bus_safety_err_stat_data0;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR_STAT_CMD")]
pub type MssL2CBusSafetyErrStatCmd =
    crate::Reg<mss_l2_c_bus_safety_err_stat_cmd::MssL2CBusSafetyErrStatCmdSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_CMD"]
pub mod mss_l2_c_bus_safety_err_stat_cmd;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err_stat_write`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE")]
pub type MssL2CBusSafetyErrStatWrite =
    crate::Reg<mss_l2_c_bus_safety_err_stat_write::MssL2CBusSafetyErrStatWriteSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod mss_l2_c_bus_safety_err_stat_write;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err_stat_read`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR_STAT_READ")]
pub type MssL2CBusSafetyErrStatRead =
    crate::Reg<mss_l2_c_bus_safety_err_stat_read::MssL2CBusSafetyErrStatReadSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_READ"]
pub mod mss_l2_c_bus_safety_err_stat_read;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_c_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_c_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_l2_c_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type MssL2CBusSafetyErrStatWriteresp =
    crate::Reg<mss_l2_c_bus_safety_err_stat_writeresp::MssL2CBusSafetyErrStatWriterespSpec>;
#[doc = "MSS_L2_C_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod mss_l2_c_bus_safety_err_stat_writeresp;
#[doc = "R5SS2DSS_BUS_SAFETY_CTRL (rw) register accessor: R5SS2DSS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_ctrl`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_CTRL")]
pub type R5ss2dssBusSafetyCtrl = crate::Reg<r5ss2dss_bus_safety_ctrl::R5ss2dssBusSafetyCtrlSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_CTRL"]
pub mod r5ss2dss_bus_safety_ctrl;
#[doc = "R5SS2DSS_BUS_SAFETY_FI (rw) register accessor: R5SS2DSS_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_fi`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_FI")]
pub type R5ss2dssBusSafetyFi = crate::Reg<r5ss2dss_bus_safety_fi::R5ss2dssBusSafetyFiSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_FI"]
pub mod r5ss2dss_bus_safety_fi;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR")]
pub type R5ss2dssBusSafetyErr = crate::Reg<r5ss2dss_bus_safety_err::R5ss2dssBusSafetyErrSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR"]
pub mod r5ss2dss_bus_safety_err;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR_STAT_DATA0")]
pub type R5ss2dssBusSafetyErrStatData0 =
    crate::Reg<r5ss2dss_bus_safety_err_stat_data0::R5ss2dssBusSafetyErrStatData0Spec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod r5ss2dss_bus_safety_err_stat_data0;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD")]
pub type R5ss2dssBusSafetyErrStatCmd =
    crate::Reg<r5ss2dss_bus_safety_err_stat_cmd::R5ss2dssBusSafetyErrStatCmdSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_CMD"]
pub mod r5ss2dss_bus_safety_err_stat_cmd;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err_stat_write`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITE")]
pub type R5ss2dssBusSafetyErrStatWrite =
    crate::Reg<r5ss2dss_bus_safety_err_stat_write::R5ss2dssBusSafetyErrStatWriteSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod r5ss2dss_bus_safety_err_stat_write;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err_stat_read`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR_STAT_READ")]
pub type R5ss2dssBusSafetyErrStatRead =
    crate::Reg<r5ss2dss_bus_safety_err_stat_read::R5ss2dssBusSafetyErrStatReadSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_READ"]
pub mod r5ss2dss_bus_safety_err_stat_read;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2dss_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2dss_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2dss_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type R5ss2dssBusSafetyErrStatWriteresp =
    crate::Reg<r5ss2dss_bus_safety_err_stat_writeresp::R5ss2dssBusSafetyErrStatWriterespSpec>;
#[doc = "R5SS2DSS_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod r5ss2dss_bus_safety_err_stat_writeresp;
#[doc = "R5SS2MSS_BUS_SAFETY_CTRL (rw) register accessor: R5SS2MSS_BUS_SAFETY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_ctrl`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_CTRL")]
pub type R5ss2mssBusSafetyCtrl = crate::Reg<r5ss2mss_bus_safety_ctrl::R5ss2mssBusSafetyCtrlSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_CTRL"]
pub mod r5ss2mss_bus_safety_ctrl;
#[doc = "R5SS2MSS_BUS_SAFETY_FI (rw) register accessor: R5SS2MSS_BUS_SAFETY_FI\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_fi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_fi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_fi`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_FI")]
pub type R5ss2mssBusSafetyFi = crate::Reg<r5ss2mss_bus_safety_fi::R5ss2mssBusSafetyFiSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_FI"]
pub mod r5ss2mss_bus_safety_fi;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR")]
pub type R5ss2mssBusSafetyErr = crate::Reg<r5ss2mss_bus_safety_err::R5ss2mssBusSafetyErrSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR"]
pub mod r5ss2mss_bus_safety_err;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_DATA0 (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR_STAT_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err_stat_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err_stat_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err_stat_data0`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR_STAT_DATA0")]
pub type R5ss2mssBusSafetyErrStatData0 =
    crate::Reg<r5ss2mss_bus_safety_err_stat_data0::R5ss2mssBusSafetyErrStatData0Spec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_DATA0"]
pub mod r5ss2mss_bus_safety_err_stat_data0;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_CMD (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR_STAT_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err_stat_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err_stat_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err_stat_cmd`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR_STAT_CMD")]
pub type R5ss2mssBusSafetyErrStatCmd =
    crate::Reg<r5ss2mss_bus_safety_err_stat_cmd::R5ss2mssBusSafetyErrStatCmdSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_CMD"]
pub mod r5ss2mss_bus_safety_err_stat_cmd;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITE (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err_stat_write::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err_stat_write::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err_stat_write`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITE")]
pub type R5ss2mssBusSafetyErrStatWrite =
    crate::Reg<r5ss2mss_bus_safety_err_stat_write::R5ss2mssBusSafetyErrStatWriteSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITE"]
pub mod r5ss2mss_bus_safety_err_stat_write;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_READ (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR_STAT_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err_stat_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err_stat_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err_stat_read`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR_STAT_READ")]
pub type R5ss2mssBusSafetyErrStatRead =
    crate::Reg<r5ss2mss_bus_safety_err_stat_read::R5ss2mssBusSafetyErrStatReadSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_READ"]
pub mod r5ss2mss_bus_safety_err_stat_read;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITERESP (rw) register accessor: R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITERESP\n\nYou can [`read`](crate::Reg::read) this register and get [`r5ss2mss_bus_safety_err_stat_writeresp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5ss2mss_bus_safety_err_stat_writeresp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5ss2mss_bus_safety_err_stat_writeresp`]
module"]
#[doc(alias = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITERESP")]
pub type R5ss2mssBusSafetyErrStatWriteresp =
    crate::Reg<r5ss2mss_bus_safety_err_stat_writeresp::R5ss2mssBusSafetyErrStatWriterespSpec>;
#[doc = "R5SS2MSS_BUS_SAFETY_ERR_STAT_WRITERESP"]
pub mod r5ss2mss_bus_safety_err_stat_writeresp;
#[doc = "NW_PACKET_COUNT (rw) register accessor: NW_PACKET_COUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`nw_packet_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nw_packet_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nw_packet_count`]
module"]
#[doc(alias = "NW_PACKET_COUNT")]
pub type NwPacketCount = crate::Reg<nw_packet_count::NwPacketCountSpec>;
#[doc = "NW_PACKET_COUNT"]
pub mod nw_packet_count;
#[doc = "NW_PACKET_COUNT_RESET (rw) register accessor: NW_PACKET_COUNT_RESET\n\nYou can [`read`](crate::Reg::read) this register and get [`nw_packet_count_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nw_packet_count_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nw_packet_count_reset`]
module"]
#[doc(alias = "NW_PACKET_COUNT_RESET")]
pub type NwPacketCountReset = crate::Reg<nw_packet_count_reset::NwPacketCountResetSpec>;
#[doc = "NW_PACKET_COUNT_RESET"]
pub mod nw_packet_count_reset;
#[doc = "CPSW_CRC_PING_ADDR (rw) register accessor: CPSW_CRC_PING_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_crc_ping_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_crc_ping_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_crc_ping_addr`]
module"]
#[doc(alias = "CPSW_CRC_PING_ADDR")]
pub type CpswCrcPingAddr = crate::Reg<cpsw_crc_ping_addr::CpswCrcPingAddrSpec>;
#[doc = "CPSW_CRC_PING_ADDR"]
pub mod cpsw_crc_ping_addr;
#[doc = "CPSW_CRC_PONG_ADDR (rw) register accessor: CPSW_CRC_PONG_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_crc_pong_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_crc_pong_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_crc_pong_addr`]
module"]
#[doc(alias = "CPSW_CRC_PONG_ADDR")]
pub type CpswCrcPongAddr = crate::Reg<cpsw_crc_pong_addr::CpswCrcPongAddrSpec>;
#[doc = "CPSW_CRC_PONG_ADDR"]
pub mod cpsw_crc_pong_addr;
#[doc = "R5_CONTROL (rw) register accessor: R5_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_control`]
module"]
#[doc(alias = "R5_CONTROL")]
pub type R5Control = crate::Reg<r5_control::R5ControlSpec>;
#[doc = "R5_CONTROL"]
pub mod r5_control;
#[doc = "R5_ROM_ECLIPSE (rw) register accessor: R5_ROM_ECLIPSE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_rom_eclipse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_rom_eclipse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_rom_eclipse`]
module"]
#[doc(alias = "R5_ROM_ECLIPSE")]
pub type R5RomEclipse = crate::Reg<r5_rom_eclipse::R5RomEclipseSpec>;
#[doc = "R5_ROM_ECLIPSE"]
pub mod r5_rom_eclipse;
#[doc = "R5_COREA_HALT (rw) register accessor: R5_COREA_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_corea_halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_corea_halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_corea_halt`]
module"]
#[doc(alias = "R5_COREA_HALT")]
pub type R5CoreaHalt = crate::Reg<r5_corea_halt::R5CoreaHaltSpec>;
#[doc = "R5_COREA_HALT"]
pub mod r5_corea_halt;
#[doc = "R5_COREB_HALT (rw) register accessor: R5_COREB_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_coreb_halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_coreb_halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_coreb_halt`]
module"]
#[doc(alias = "R5_COREB_HALT")]
pub type R5CorebHalt = crate::Reg<r5_coreb_halt::R5CorebHaltSpec>;
#[doc = "R5_COREB_HALT"]
pub mod r5_coreb_halt;
#[doc = "R5_STATUS_REG (rw) register accessor: R5_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5_status_reg`]
module"]
#[doc(alias = "R5_STATUS_REG")]
pub type R5StatusReg = crate::Reg<r5_status_reg::R5StatusRegSpec>;
#[doc = "R5_STATUS_REG"]
pub mod r5_status_reg;
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
