#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    _reserved1: [u8; 0x04],
    rss_tpcc_a_erragg_mask: RssTpccAErraggMask,
    rss_tpcc_a_erragg_status: RssTpccAErraggStatus,
    rss_tpcc_a_erragg_status_raw: RssTpccAErraggStatusRaw,
    rss_tpcc_a_intagg_mask: RssTpccAIntaggMask,
    rss_tpcc_a_intagg_status: RssTpccAIntaggStatus,
    rss_tpcc_a_intagg_status_raw: RssTpccAIntaggStatusRaw,
    rss_tpcc_meminit_start: RssTpccMeminitStart,
    rss_tpcc_meminit_done: RssTpccMeminitDone,
    rss_tpcc_meminit_status: RssTpccMeminitStatus,
    tptc_dbs_cfg: TptcDbsCfg,
    rss_tpcc_a_parity_ctrl: RssTpccAParityCtrl,
    rss_tpcc_a_parity_status: RssTpccAParityStatus,
    _reserved13: [u8; 0x023c],
    rss_tptc_boundary_cfg: RssTptcBoundaryCfg,
    rss_tptc_xid_reorder_cfg: RssTptcXidReorderCfg,
    dbg_ack_cpu_ctrl: DbgAckCpuCtrl,
    rss_adcbuf_ping_meminit: RssAdcbufPingMeminit,
    rss_adcbuf_ping_meminit_done: RssAdcbufPingMeminitDone,
    rss_adcbuf_ping_meminit_status: RssAdcbufPingMeminitStatus,
    rss_adcbuf_pong_meminit: RssAdcbufPongMeminit,
    rss_adcbuf_pong_meminit_done: RssAdcbufPongMeminitDone,
    rss_adcbuf_pong_meminit_status: RssAdcbufPongMeminitStatus,
    _reserved22: [u8; 0x30],
    soc_to_bss_sw_int: SocToBssSwInt,
    rss_dbg_ack_ctl0: RssDbgAckCtl0,
    dmmswint1: Dmmswint1,
    rss_shared_mem_meminit: RssSharedMemMeminit,
    rss_shared_mem_meminit_done: RssSharedMemMeminitDone,
    rss_shared_mem_meminit_status: RssSharedMemMeminitStatus,
    _reserved28: [u8; 0x0120],
    bss_control: BssControl,
    bss_tcm_meminit: BssTcmMeminit,
    bss_tcm_meminit_done: BssTcmMeminitDone,
    bss_tcm_meminit_status: BssTcmMeminitStatus,
    bss_vim_meminit: BssVimMeminit,
    bss_vim_meminit_done: BssVimMeminitDone,
    bss_vim_meminit_status: BssVimMeminitStatus,
    bss_dfe_meminit: BssDfeMeminit,
    bss_dfe_meminit_done: BssDfeMeminitDone,
    bss_dfe_meminit_status: BssDfeMeminitStatus,
    bss_rampgen_meminit: BssRampgenMeminit,
    bss_rampgen_meminit_done: BssRampgenMeminitDone,
    bss_rampgen_meminit_status: BssRampgenMeminitStatus,
    bss_dss_l3_sticky: BssDssL3Sticky,
    bss_dss_l3_access: BssDssL3Access,
    _reserved43: [u8; 0x03c4],
    testpatternrx1icfg: Testpatternrx1icfg,
    testpatternrx2icfg: Testpatternrx2icfg,
    testpatternrx3icfg: Testpatternrx3icfg,
    testpatternrx4icfg: Testpatternrx4icfg,
    testpatternrx1qcfg: Testpatternrx1qcfg,
    testpatternrx2qcfg: Testpatternrx2qcfg,
    testpatternrx3qcfg: Testpatternrx3qcfg,
    testpatternrx4qcfg: Testpatternrx4qcfg,
    testpatternvldcfg: Testpatternvldcfg,
    adcbufcfg1: Adcbufcfg1,
    adcbufcfg1_extd: Adcbufcfg1Extd,
    adcbufcfg2: Adcbufcfg2,
    adcbufcfg3: Adcbufcfg3,
    adcbufcfg4: Adcbufcfg4,
    adcbufintgenditherdly: Adcbufintgenditherdly,
    cbuff_frame_start_sel: CbuffFrameStartSel,
    _reserved59: [u8; 0x03c0],
    cqcfg1: Cqcfg1,
    cqcfg2: Cqcfg2,
    cpreg0: Cpreg0,
    cpreg1: Cpreg1,
    cpreg2: Cpreg2,
    cpreg3: Cpreg3,
    cpreg4: Cpreg4,
    cpreg5: Cpreg5,
    cpreg6: Cpreg6,
    cpreg7: Cpreg7,
    cpreg8: Cpreg8,
    cpreg9: Cpreg9,
    cpreg10: Cpreg10,
    cpreg11: Cpreg11,
    cpreg12: Cpreg12,
    cpreg13: Cpreg13,
    cpreg14: Cpreg14,
    cpreg15: Cpreg15,
    ch0cpreg0: Ch0cpreg0,
    ch0cpreg1: Ch0cpreg1,
    ch0cpreg2: Ch0cpreg2,
    ch0cpreg3: Ch0cpreg3,
    ch0cpreg4: Ch0cpreg4,
    ch0cpreg5: Ch0cpreg5,
    ch0cpreg6: Ch0cpreg6,
    ch0cpreg7: Ch0cpreg7,
    ch0cpreg8: Ch0cpreg8,
    ch0cpreg9: Ch0cpreg9,
    ch0cpreg10: Ch0cpreg10,
    ch0cpreg11: Ch0cpreg11,
    ch0cpreg12: Ch0cpreg12,
    ch0cpreg13: Ch0cpreg13,
    ch0cpreg14: Ch0cpreg14,
    ch0cpreg15: Ch0cpreg15,
    ch1cpreg0: Ch1cpreg0,
    ch1cpreg1: Ch1cpreg1,
    ch1cpreg2: Ch1cpreg2,
    ch1cpreg3: Ch1cpreg3,
    ch1cpreg4: Ch1cpreg4,
    ch1cpreg5: Ch1cpreg5,
    ch1cpreg6: Ch1cpreg6,
    ch1cpreg7: Ch1cpreg7,
    ch1cpreg8: Ch1cpreg8,
    ch1cpreg9: Ch1cpreg9,
    ch1cpreg10: Ch1cpreg10,
    ch1cpreg11: Ch1cpreg11,
    ch1cpreg12: Ch1cpreg12,
    ch1cpreg13: Ch1cpreg13,
    ch1cpreg14: Ch1cpreg14,
    ch1cpreg15: Ch1cpreg15,
    ch2cpreg0: Ch2cpreg0,
    ch2cpreg1: Ch2cpreg1,
    ch2cpreg2: Ch2cpreg2,
    ch2cpreg3: Ch2cpreg3,
    ch2cpreg4: Ch2cpreg4,
    ch2cpreg5: Ch2cpreg5,
    ch2cpreg6: Ch2cpreg6,
    ch2cpreg7: Ch2cpreg7,
    ch2cpreg8: Ch2cpreg8,
    ch2cpreg9: Ch2cpreg9,
    ch2cpreg10: Ch2cpreg10,
    ch2cpreg11: Ch2cpreg11,
    ch2cpreg12: Ch2cpreg12,
    ch2cpreg13: Ch2cpreg13,
    ch2cpreg14: Ch2cpreg14,
    ch2cpreg15: Ch2cpreg15,
    ch3cpreg0: Ch3cpreg0,
    ch3cpreg1: Ch3cpreg1,
    ch3cpreg2: Ch3cpreg2,
    ch3cpreg3: Ch3cpreg3,
    ch3cpreg4: Ch3cpreg4,
    ch3cpreg5: Ch3cpreg5,
    ch3cpreg6: Ch3cpreg6,
    ch3cpreg7: Ch3cpreg7,
    ch3cpreg8: Ch3cpreg8,
    ch3cpreg9: Ch3cpreg9,
    ch3cpreg10: Ch3cpreg10,
    ch3cpreg11: Ch3cpreg11,
    ch3cpreg12: Ch3cpreg12,
    ch3cpreg13: Ch3cpreg13,
    ch3cpreg14: Ch3cpreg14,
    ch3cpreg15: Ch3cpreg15,
    ch4cpreg0: Ch4cpreg0,
    ch4cpreg1: Ch4cpreg1,
    ch4cpreg2: Ch4cpreg2,
    ch4cpreg3: Ch4cpreg3,
    ch4cpreg4: Ch4cpreg4,
    ch4cpreg5: Ch4cpreg5,
    ch4cpreg6: Ch4cpreg6,
    ch4cpreg7: Ch4cpreg7,
    ch4cpreg8: Ch4cpreg8,
    ch4cpreg9: Ch4cpreg9,
    ch4cpreg10: Ch4cpreg10,
    ch4cpreg11: Ch4cpreg11,
    ch4cpreg12: Ch4cpreg12,
    ch4cpreg13: Ch4cpreg13,
    ch4cpreg14: Ch4cpreg14,
    ch4cpreg15: Ch4cpreg15,
    ch5cpreg0: Ch5cpreg0,
    ch5cpreg1: Ch5cpreg1,
    ch5cpreg2: Ch5cpreg2,
    ch5cpreg3: Ch5cpreg3,
    ch5cpreg4: Ch5cpreg4,
    ch5cpreg5: Ch5cpreg5,
    ch5cpreg6: Ch5cpreg6,
    ch5cpreg7: Ch5cpreg7,
    ch5cpreg8: Ch5cpreg8,
    ch5cpreg9: Ch5cpreg9,
    ch5cpreg10: Ch5cpreg10,
    ch5cpreg11: Ch5cpreg11,
    ch5cpreg12: Ch5cpreg12,
    ch5cpreg13: Ch5cpreg13,
    ch5cpreg14: Ch5cpreg14,
    ch5cpreg15: Ch5cpreg15,
    ch6cpreg0: Ch6cpreg0,
    ch6cpreg1: Ch6cpreg1,
    ch6cpreg2: Ch6cpreg2,
    ch6cpreg3: Ch6cpreg3,
    ch6cpreg4: Ch6cpreg4,
    ch6cpreg5: Ch6cpreg5,
    ch6cpreg6: Ch6cpreg6,
    ch6cpreg7: Ch6cpreg7,
    ch6cpreg8: Ch6cpreg8,
    ch6cpreg9: Ch6cpreg9,
    ch6cpreg10: Ch6cpreg10,
    ch6cpreg11: Ch6cpreg11,
    ch6cpreg12: Ch6cpreg12,
    ch6cpreg13: Ch6cpreg13,
    ch6cpreg14: Ch6cpreg14,
    ch6cpreg15: Ch6cpreg15,
    ch7cpreg0: Ch7cpreg0,
    ch7cpreg1: Ch7cpreg1,
    ch7cpreg2: Ch7cpreg2,
    ch7cpreg3: Ch7cpreg3,
    ch7cpreg4: Ch7cpreg4,
    ch7cpreg5: Ch7cpreg5,
    ch7cpreg6: Ch7cpreg6,
    ch7cpreg7: Ch7cpreg7,
    ch7cpreg8: Ch7cpreg8,
    ch7cpreg9: Ch7cpreg9,
    ch7cpreg10: Ch7cpreg10,
    ch7cpreg11: Ch7cpreg11,
    ch7cpreg12: Ch7cpreg12,
    ch7cpreg13: Ch7cpreg13,
    ch7cpreg14: Ch7cpreg14,
    ch7cpreg15: Ch7cpreg15,
    ch01_hil_cp_override: Ch01HilCpOverride,
    ch23_hil_cp_override: Ch23HilCpOverride,
    ch45_hil_cp_override: Ch45HilCpOverride,
    ch67_hil_cp_override: Ch67HilCpOverride,
    ch_hil_cp_override: ChHilCpOverride,
    rss_bookkeeping_ctrl: RssBookkeepingCtrl,
    rss_app_gp: RssAppGp,
    rss_bookkeeping_seq_num: RssBookkeepingSeqNum,
    rss_bookkeeping_frm_cnt: RssBookkeepingFrmCnt,
    rss_bookkeeping_chrp_cnt: RssBookkeepingChrpCnt,
    _reserved215: [u8; 0x0160],
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
    _reserved225: [u8; 0x10],
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
    #[doc = "0x08 - RSS_TPCC_A_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn rss_tpcc_a_erragg_mask(&self) -> &RssTpccAErraggMask {
        &self.rss_tpcc_a_erragg_mask
    }
    #[doc = "0x0c - RSS_TPCC_A_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn rss_tpcc_a_erragg_status(&self) -> &RssTpccAErraggStatus {
        &self.rss_tpcc_a_erragg_status
    }
    #[doc = "0x10 - RSS_TPCC_A_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn rss_tpcc_a_erragg_status_raw(&self) -> &RssTpccAErraggStatusRaw {
        &self.rss_tpcc_a_erragg_status_raw
    }
    #[doc = "0x14 - RSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    pub const fn rss_tpcc_a_intagg_mask(&self) -> &RssTpccAIntaggMask {
        &self.rss_tpcc_a_intagg_mask
    }
    #[doc = "0x18 - RSS_TPCC_A_INTAGG_STATUS"]
    #[inline(always)]
    pub const fn rss_tpcc_a_intagg_status(&self) -> &RssTpccAIntaggStatus {
        &self.rss_tpcc_a_intagg_status
    }
    #[doc = "0x1c - RSS_TPCC_A_INTAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn rss_tpcc_a_intagg_status_raw(&self) -> &RssTpccAIntaggStatusRaw {
        &self.rss_tpcc_a_intagg_status_raw
    }
    #[doc = "0x20 - RSS_TPCC_MEMINIT_START"]
    #[inline(always)]
    pub const fn rss_tpcc_meminit_start(&self) -> &RssTpccMeminitStart {
        &self.rss_tpcc_meminit_start
    }
    #[doc = "0x24 - RSS_TPCC_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn rss_tpcc_meminit_done(&self) -> &RssTpccMeminitDone {
        &self.rss_tpcc_meminit_done
    }
    #[doc = "0x28 - RSS_TPCC_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn rss_tpcc_meminit_status(&self) -> &RssTpccMeminitStatus {
        &self.rss_tpcc_meminit_status
    }
    #[doc = "0x2c - TPTC_DBS_CFG"]
    #[inline(always)]
    pub const fn tptc_dbs_cfg(&self) -> &TptcDbsCfg {
        &self.tptc_dbs_cfg
    }
    #[doc = "0x30 - RSS_TPCC_A_PARITY_CTRL"]
    #[inline(always)]
    pub const fn rss_tpcc_a_parity_ctrl(&self) -> &RssTpccAParityCtrl {
        &self.rss_tpcc_a_parity_ctrl
    }
    #[doc = "0x34 - RSS_TPCC_A_PARITY_STATUS"]
    #[inline(always)]
    pub const fn rss_tpcc_a_parity_status(&self) -> &RssTpccAParityStatus {
        &self.rss_tpcc_a_parity_status
    }
    #[doc = "0x274 - RSS_TPTC_BOUNDARY_CFG"]
    #[inline(always)]
    pub const fn rss_tptc_boundary_cfg(&self) -> &RssTptcBoundaryCfg {
        &self.rss_tptc_boundary_cfg
    }
    #[doc = "0x278 - RSS_TPTC_XID_REORDER_CFG"]
    #[inline(always)]
    pub const fn rss_tptc_xid_reorder_cfg(&self) -> &RssTptcXidReorderCfg {
        &self.rss_tptc_xid_reorder_cfg
    }
    #[doc = "0x27c - DBG_ACK_CPU_CTRL"]
    #[inline(always)]
    pub const fn dbg_ack_cpu_ctrl(&self) -> &DbgAckCpuCtrl {
        &self.dbg_ack_cpu_ctrl
    }
    #[doc = "0x280 - RSS_ADCBUF_PING_MEMINIT"]
    #[inline(always)]
    pub const fn rss_adcbuf_ping_meminit(&self) -> &RssAdcbufPingMeminit {
        &self.rss_adcbuf_ping_meminit
    }
    #[doc = "0x284 - RSS_ADCBUF_PING_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn rss_adcbuf_ping_meminit_done(&self) -> &RssAdcbufPingMeminitDone {
        &self.rss_adcbuf_ping_meminit_done
    }
    #[doc = "0x288 - RSS_ADCBUF_PING_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn rss_adcbuf_ping_meminit_status(&self) -> &RssAdcbufPingMeminitStatus {
        &self.rss_adcbuf_ping_meminit_status
    }
    #[doc = "0x28c - RSS_ADCBUF_PONG_MEMINIT"]
    #[inline(always)]
    pub const fn rss_adcbuf_pong_meminit(&self) -> &RssAdcbufPongMeminit {
        &self.rss_adcbuf_pong_meminit
    }
    #[doc = "0x290 - RSS_ADCBUF_PONG_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn rss_adcbuf_pong_meminit_done(&self) -> &RssAdcbufPongMeminitDone {
        &self.rss_adcbuf_pong_meminit_done
    }
    #[doc = "0x294 - RSS_ADCBUF_PONG_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn rss_adcbuf_pong_meminit_status(&self) -> &RssAdcbufPongMeminitStatus {
        &self.rss_adcbuf_pong_meminit_status
    }
    #[doc = "0x2c8 - SOC_TO_BSS_SW_INT"]
    #[inline(always)]
    pub const fn soc_to_bss_sw_int(&self) -> &SocToBssSwInt {
        &self.soc_to_bss_sw_int
    }
    #[doc = "0x2cc - RSS_DBG_ACK_CTL0"]
    #[inline(always)]
    pub const fn rss_dbg_ack_ctl0(&self) -> &RssDbgAckCtl0 {
        &self.rss_dbg_ack_ctl0
    }
    #[doc = "0x2d0 - DMMSWINT1"]
    #[inline(always)]
    pub const fn dmmswint1(&self) -> &Dmmswint1 {
        &self.dmmswint1
    }
    #[doc = "0x2d4 - RSS_SHARED_MEM_MEMINIT"]
    #[inline(always)]
    pub const fn rss_shared_mem_meminit(&self) -> &RssSharedMemMeminit {
        &self.rss_shared_mem_meminit
    }
    #[doc = "0x2d8 - RSS_SHARED_MEM_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn rss_shared_mem_meminit_done(&self) -> &RssSharedMemMeminitDone {
        &self.rss_shared_mem_meminit_done
    }
    #[doc = "0x2dc - RSS_SHARED_MEM_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn rss_shared_mem_meminit_status(&self) -> &RssSharedMemMeminitStatus {
        &self.rss_shared_mem_meminit_status
    }
    #[doc = "0x400 - BSS_CONTROL"]
    #[inline(always)]
    pub const fn bss_control(&self) -> &BssControl {
        &self.bss_control
    }
    #[doc = "0x404 - BSS_TCM_MEMINIT"]
    #[inline(always)]
    pub const fn bss_tcm_meminit(&self) -> &BssTcmMeminit {
        &self.bss_tcm_meminit
    }
    #[doc = "0x408 - BSS_TCM_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn bss_tcm_meminit_done(&self) -> &BssTcmMeminitDone {
        &self.bss_tcm_meminit_done
    }
    #[doc = "0x40c - BSS_TCM_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn bss_tcm_meminit_status(&self) -> &BssTcmMeminitStatus {
        &self.bss_tcm_meminit_status
    }
    #[doc = "0x410 - BSS_VIM_MEMINIT"]
    #[inline(always)]
    pub const fn bss_vim_meminit(&self) -> &BssVimMeminit {
        &self.bss_vim_meminit
    }
    #[doc = "0x414 - BSS_VIM_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn bss_vim_meminit_done(&self) -> &BssVimMeminitDone {
        &self.bss_vim_meminit_done
    }
    #[doc = "0x418 - BSS_VIM_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn bss_vim_meminit_status(&self) -> &BssVimMeminitStatus {
        &self.bss_vim_meminit_status
    }
    #[doc = "0x41c - BSS_DFE_MEMINIT"]
    #[inline(always)]
    pub const fn bss_dfe_meminit(&self) -> &BssDfeMeminit {
        &self.bss_dfe_meminit
    }
    #[doc = "0x420 - BSS_DFE_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn bss_dfe_meminit_done(&self) -> &BssDfeMeminitDone {
        &self.bss_dfe_meminit_done
    }
    #[doc = "0x424 - BSS_DFE_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn bss_dfe_meminit_status(&self) -> &BssDfeMeminitStatus {
        &self.bss_dfe_meminit_status
    }
    #[doc = "0x428 - BSS_RAMPGEN_MEMINIT"]
    #[inline(always)]
    pub const fn bss_rampgen_meminit(&self) -> &BssRampgenMeminit {
        &self.bss_rampgen_meminit
    }
    #[doc = "0x42c - BSS_RAMPGEN_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn bss_rampgen_meminit_done(&self) -> &BssRampgenMeminitDone {
        &self.bss_rampgen_meminit_done
    }
    #[doc = "0x430 - BSS_RAMPGEN_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn bss_rampgen_meminit_status(&self) -> &BssRampgenMeminitStatus {
        &self.bss_rampgen_meminit_status
    }
    #[doc = "0x434 - BSS_DSS_L3_STICKY"]
    #[inline(always)]
    pub const fn bss_dss_l3_sticky(&self) -> &BssDssL3Sticky {
        &self.bss_dss_l3_sticky
    }
    #[doc = "0x438 - BSS_DSS_L3_ACCESS"]
    #[inline(always)]
    pub const fn bss_dss_l3_access(&self) -> &BssDssL3Access {
        &self.bss_dss_l3_access
    }
    #[doc = "0x800 - TESTPATTERNRX1ICFG"]
    #[inline(always)]
    pub const fn testpatternrx1icfg(&self) -> &Testpatternrx1icfg {
        &self.testpatternrx1icfg
    }
    #[doc = "0x804 - TESTPATTERNRX2ICFG"]
    #[inline(always)]
    pub const fn testpatternrx2icfg(&self) -> &Testpatternrx2icfg {
        &self.testpatternrx2icfg
    }
    #[doc = "0x808 - TESTPATTERNRX3ICFG"]
    #[inline(always)]
    pub const fn testpatternrx3icfg(&self) -> &Testpatternrx3icfg {
        &self.testpatternrx3icfg
    }
    #[doc = "0x80c - TESTPATTERNRX4ICFG"]
    #[inline(always)]
    pub const fn testpatternrx4icfg(&self) -> &Testpatternrx4icfg {
        &self.testpatternrx4icfg
    }
    #[doc = "0x810 - TESTPATTERNRX1QCFG"]
    #[inline(always)]
    pub const fn testpatternrx1qcfg(&self) -> &Testpatternrx1qcfg {
        &self.testpatternrx1qcfg
    }
    #[doc = "0x814 - TESTPATTERNRX2QCFG"]
    #[inline(always)]
    pub const fn testpatternrx2qcfg(&self) -> &Testpatternrx2qcfg {
        &self.testpatternrx2qcfg
    }
    #[doc = "0x818 - TESTPATTERNRX3QCFG"]
    #[inline(always)]
    pub const fn testpatternrx3qcfg(&self) -> &Testpatternrx3qcfg {
        &self.testpatternrx3qcfg
    }
    #[doc = "0x81c - TESTPATTERNRX4QCFG"]
    #[inline(always)]
    pub const fn testpatternrx4qcfg(&self) -> &Testpatternrx4qcfg {
        &self.testpatternrx4qcfg
    }
    #[doc = "0x820 - TESTPATTERNVLDCFG"]
    #[inline(always)]
    pub const fn testpatternvldcfg(&self) -> &Testpatternvldcfg {
        &self.testpatternvldcfg
    }
    #[doc = "0x824 - ADCBUFCFG1"]
    #[inline(always)]
    pub const fn adcbufcfg1(&self) -> &Adcbufcfg1 {
        &self.adcbufcfg1
    }
    #[doc = "0x828 - ADCBUFCFG1_EXTD"]
    #[inline(always)]
    pub const fn adcbufcfg1_extd(&self) -> &Adcbufcfg1Extd {
        &self.adcbufcfg1_extd
    }
    #[doc = "0x82c - ADCBUFCFG2"]
    #[inline(always)]
    pub const fn adcbufcfg2(&self) -> &Adcbufcfg2 {
        &self.adcbufcfg2
    }
    #[doc = "0x830 - ADCBUFCFG3"]
    #[inline(always)]
    pub const fn adcbufcfg3(&self) -> &Adcbufcfg3 {
        &self.adcbufcfg3
    }
    #[doc = "0x834 - ADCBUFCFG4"]
    #[inline(always)]
    pub const fn adcbufcfg4(&self) -> &Adcbufcfg4 {
        &self.adcbufcfg4
    }
    #[doc = "0x838 - ADCBUFINTGENDITHERDLY"]
    #[inline(always)]
    pub const fn adcbufintgenditherdly(&self) -> &Adcbufintgenditherdly {
        &self.adcbufintgenditherdly
    }
    #[doc = "0x83c - CBUFF_FRAME_START_SEL"]
    #[inline(always)]
    pub const fn cbuff_frame_start_sel(&self) -> &CbuffFrameStartSel {
        &self.cbuff_frame_start_sel
    }
    #[doc = "0xc00 - CQCFG1"]
    #[inline(always)]
    pub const fn cqcfg1(&self) -> &Cqcfg1 {
        &self.cqcfg1
    }
    #[doc = "0xc04 - CQCFG2"]
    #[inline(always)]
    pub const fn cqcfg2(&self) -> &Cqcfg2 {
        &self.cqcfg2
    }
    #[doc = "0xc08 - CPREG0"]
    #[inline(always)]
    pub const fn cpreg0(&self) -> &Cpreg0 {
        &self.cpreg0
    }
    #[doc = "0xc0c - CPREG1"]
    #[inline(always)]
    pub const fn cpreg1(&self) -> &Cpreg1 {
        &self.cpreg1
    }
    #[doc = "0xc10 - CPREG2"]
    #[inline(always)]
    pub const fn cpreg2(&self) -> &Cpreg2 {
        &self.cpreg2
    }
    #[doc = "0xc14 - CPREG3"]
    #[inline(always)]
    pub const fn cpreg3(&self) -> &Cpreg3 {
        &self.cpreg3
    }
    #[doc = "0xc18 - CPREG4"]
    #[inline(always)]
    pub const fn cpreg4(&self) -> &Cpreg4 {
        &self.cpreg4
    }
    #[doc = "0xc1c - CPREG5"]
    #[inline(always)]
    pub const fn cpreg5(&self) -> &Cpreg5 {
        &self.cpreg5
    }
    #[doc = "0xc20 - CPREG6"]
    #[inline(always)]
    pub const fn cpreg6(&self) -> &Cpreg6 {
        &self.cpreg6
    }
    #[doc = "0xc24 - CPREG7"]
    #[inline(always)]
    pub const fn cpreg7(&self) -> &Cpreg7 {
        &self.cpreg7
    }
    #[doc = "0xc28 - CPREG8"]
    #[inline(always)]
    pub const fn cpreg8(&self) -> &Cpreg8 {
        &self.cpreg8
    }
    #[doc = "0xc2c - CPREG9"]
    #[inline(always)]
    pub const fn cpreg9(&self) -> &Cpreg9 {
        &self.cpreg9
    }
    #[doc = "0xc30 - CPREG10"]
    #[inline(always)]
    pub const fn cpreg10(&self) -> &Cpreg10 {
        &self.cpreg10
    }
    #[doc = "0xc34 - CPREG11"]
    #[inline(always)]
    pub const fn cpreg11(&self) -> &Cpreg11 {
        &self.cpreg11
    }
    #[doc = "0xc38 - CPREG12"]
    #[inline(always)]
    pub const fn cpreg12(&self) -> &Cpreg12 {
        &self.cpreg12
    }
    #[doc = "0xc3c - CPREG13"]
    #[inline(always)]
    pub const fn cpreg13(&self) -> &Cpreg13 {
        &self.cpreg13
    }
    #[doc = "0xc40 - CPREG14"]
    #[inline(always)]
    pub const fn cpreg14(&self) -> &Cpreg14 {
        &self.cpreg14
    }
    #[doc = "0xc44 - CPREG15"]
    #[inline(always)]
    pub const fn cpreg15(&self) -> &Cpreg15 {
        &self.cpreg15
    }
    #[doc = "0xc48 - CH0CPREG0"]
    #[inline(always)]
    pub const fn ch0cpreg0(&self) -> &Ch0cpreg0 {
        &self.ch0cpreg0
    }
    #[doc = "0xc4c - CH0CPREG1"]
    #[inline(always)]
    pub const fn ch0cpreg1(&self) -> &Ch0cpreg1 {
        &self.ch0cpreg1
    }
    #[doc = "0xc50 - CH0CPREG2"]
    #[inline(always)]
    pub const fn ch0cpreg2(&self) -> &Ch0cpreg2 {
        &self.ch0cpreg2
    }
    #[doc = "0xc54 - CH0CPREG3"]
    #[inline(always)]
    pub const fn ch0cpreg3(&self) -> &Ch0cpreg3 {
        &self.ch0cpreg3
    }
    #[doc = "0xc58 - CH0CPREG4"]
    #[inline(always)]
    pub const fn ch0cpreg4(&self) -> &Ch0cpreg4 {
        &self.ch0cpreg4
    }
    #[doc = "0xc5c - CH0CPREG5"]
    #[inline(always)]
    pub const fn ch0cpreg5(&self) -> &Ch0cpreg5 {
        &self.ch0cpreg5
    }
    #[doc = "0xc60 - CH0CPREG6"]
    #[inline(always)]
    pub const fn ch0cpreg6(&self) -> &Ch0cpreg6 {
        &self.ch0cpreg6
    }
    #[doc = "0xc64 - CH0CPREG7"]
    #[inline(always)]
    pub const fn ch0cpreg7(&self) -> &Ch0cpreg7 {
        &self.ch0cpreg7
    }
    #[doc = "0xc68 - CH0CPREG8"]
    #[inline(always)]
    pub const fn ch0cpreg8(&self) -> &Ch0cpreg8 {
        &self.ch0cpreg8
    }
    #[doc = "0xc6c - CH0CPREG9"]
    #[inline(always)]
    pub const fn ch0cpreg9(&self) -> &Ch0cpreg9 {
        &self.ch0cpreg9
    }
    #[doc = "0xc70 - CH0CPREG10"]
    #[inline(always)]
    pub const fn ch0cpreg10(&self) -> &Ch0cpreg10 {
        &self.ch0cpreg10
    }
    #[doc = "0xc74 - CH0CPREG11"]
    #[inline(always)]
    pub const fn ch0cpreg11(&self) -> &Ch0cpreg11 {
        &self.ch0cpreg11
    }
    #[doc = "0xc78 - CH0CPREG12"]
    #[inline(always)]
    pub const fn ch0cpreg12(&self) -> &Ch0cpreg12 {
        &self.ch0cpreg12
    }
    #[doc = "0xc7c - CH0CPREG13"]
    #[inline(always)]
    pub const fn ch0cpreg13(&self) -> &Ch0cpreg13 {
        &self.ch0cpreg13
    }
    #[doc = "0xc80 - CH0CPREG14"]
    #[inline(always)]
    pub const fn ch0cpreg14(&self) -> &Ch0cpreg14 {
        &self.ch0cpreg14
    }
    #[doc = "0xc84 - CH0CPREG15"]
    #[inline(always)]
    pub const fn ch0cpreg15(&self) -> &Ch0cpreg15 {
        &self.ch0cpreg15
    }
    #[doc = "0xc88 - CH1CPREG0"]
    #[inline(always)]
    pub const fn ch1cpreg0(&self) -> &Ch1cpreg0 {
        &self.ch1cpreg0
    }
    #[doc = "0xc8c - CH1CPREG1"]
    #[inline(always)]
    pub const fn ch1cpreg1(&self) -> &Ch1cpreg1 {
        &self.ch1cpreg1
    }
    #[doc = "0xc90 - CH1CPREG2"]
    #[inline(always)]
    pub const fn ch1cpreg2(&self) -> &Ch1cpreg2 {
        &self.ch1cpreg2
    }
    #[doc = "0xc94 - CH1CPREG3"]
    #[inline(always)]
    pub const fn ch1cpreg3(&self) -> &Ch1cpreg3 {
        &self.ch1cpreg3
    }
    #[doc = "0xc98 - CH1CPREG4"]
    #[inline(always)]
    pub const fn ch1cpreg4(&self) -> &Ch1cpreg4 {
        &self.ch1cpreg4
    }
    #[doc = "0xc9c - CH1CPREG5"]
    #[inline(always)]
    pub const fn ch1cpreg5(&self) -> &Ch1cpreg5 {
        &self.ch1cpreg5
    }
    #[doc = "0xca0 - CH1CPREG6"]
    #[inline(always)]
    pub const fn ch1cpreg6(&self) -> &Ch1cpreg6 {
        &self.ch1cpreg6
    }
    #[doc = "0xca4 - CH1CPREG7"]
    #[inline(always)]
    pub const fn ch1cpreg7(&self) -> &Ch1cpreg7 {
        &self.ch1cpreg7
    }
    #[doc = "0xca8 - CH1CPREG8"]
    #[inline(always)]
    pub const fn ch1cpreg8(&self) -> &Ch1cpreg8 {
        &self.ch1cpreg8
    }
    #[doc = "0xcac - CH1CPREG9"]
    #[inline(always)]
    pub const fn ch1cpreg9(&self) -> &Ch1cpreg9 {
        &self.ch1cpreg9
    }
    #[doc = "0xcb0 - CH1CPREG10"]
    #[inline(always)]
    pub const fn ch1cpreg10(&self) -> &Ch1cpreg10 {
        &self.ch1cpreg10
    }
    #[doc = "0xcb4 - CH1CPREG11"]
    #[inline(always)]
    pub const fn ch1cpreg11(&self) -> &Ch1cpreg11 {
        &self.ch1cpreg11
    }
    #[doc = "0xcb8 - CH1CPREG12"]
    #[inline(always)]
    pub const fn ch1cpreg12(&self) -> &Ch1cpreg12 {
        &self.ch1cpreg12
    }
    #[doc = "0xcbc - CH1CPREG13"]
    #[inline(always)]
    pub const fn ch1cpreg13(&self) -> &Ch1cpreg13 {
        &self.ch1cpreg13
    }
    #[doc = "0xcc0 - CH1CPREG14"]
    #[inline(always)]
    pub const fn ch1cpreg14(&self) -> &Ch1cpreg14 {
        &self.ch1cpreg14
    }
    #[doc = "0xcc4 - CH1CPREG15"]
    #[inline(always)]
    pub const fn ch1cpreg15(&self) -> &Ch1cpreg15 {
        &self.ch1cpreg15
    }
    #[doc = "0xcc8 - CH2CPREG0"]
    #[inline(always)]
    pub const fn ch2cpreg0(&self) -> &Ch2cpreg0 {
        &self.ch2cpreg0
    }
    #[doc = "0xccc - CH2CPREG1"]
    #[inline(always)]
    pub const fn ch2cpreg1(&self) -> &Ch2cpreg1 {
        &self.ch2cpreg1
    }
    #[doc = "0xcd0 - CH2CPREG2"]
    #[inline(always)]
    pub const fn ch2cpreg2(&self) -> &Ch2cpreg2 {
        &self.ch2cpreg2
    }
    #[doc = "0xcd4 - CH2CPREG3"]
    #[inline(always)]
    pub const fn ch2cpreg3(&self) -> &Ch2cpreg3 {
        &self.ch2cpreg3
    }
    #[doc = "0xcd8 - CH2CPREG4"]
    #[inline(always)]
    pub const fn ch2cpreg4(&self) -> &Ch2cpreg4 {
        &self.ch2cpreg4
    }
    #[doc = "0xcdc - CH2CPREG5"]
    #[inline(always)]
    pub const fn ch2cpreg5(&self) -> &Ch2cpreg5 {
        &self.ch2cpreg5
    }
    #[doc = "0xce0 - CH2CPREG6"]
    #[inline(always)]
    pub const fn ch2cpreg6(&self) -> &Ch2cpreg6 {
        &self.ch2cpreg6
    }
    #[doc = "0xce4 - CH2CPREG7"]
    #[inline(always)]
    pub const fn ch2cpreg7(&self) -> &Ch2cpreg7 {
        &self.ch2cpreg7
    }
    #[doc = "0xce8 - CH2CPREG8"]
    #[inline(always)]
    pub const fn ch2cpreg8(&self) -> &Ch2cpreg8 {
        &self.ch2cpreg8
    }
    #[doc = "0xcec - CH2CPREG9"]
    #[inline(always)]
    pub const fn ch2cpreg9(&self) -> &Ch2cpreg9 {
        &self.ch2cpreg9
    }
    #[doc = "0xcf0 - CH2CPREG10"]
    #[inline(always)]
    pub const fn ch2cpreg10(&self) -> &Ch2cpreg10 {
        &self.ch2cpreg10
    }
    #[doc = "0xcf4 - CH2CPREG11"]
    #[inline(always)]
    pub const fn ch2cpreg11(&self) -> &Ch2cpreg11 {
        &self.ch2cpreg11
    }
    #[doc = "0xcf8 - CH2CPREG12"]
    #[inline(always)]
    pub const fn ch2cpreg12(&self) -> &Ch2cpreg12 {
        &self.ch2cpreg12
    }
    #[doc = "0xcfc - CH2CPREG13"]
    #[inline(always)]
    pub const fn ch2cpreg13(&self) -> &Ch2cpreg13 {
        &self.ch2cpreg13
    }
    #[doc = "0xd00 - CH2CPREG14"]
    #[inline(always)]
    pub const fn ch2cpreg14(&self) -> &Ch2cpreg14 {
        &self.ch2cpreg14
    }
    #[doc = "0xd04 - CH2CPREG15"]
    #[inline(always)]
    pub const fn ch2cpreg15(&self) -> &Ch2cpreg15 {
        &self.ch2cpreg15
    }
    #[doc = "0xd08 - CH3CPREG0"]
    #[inline(always)]
    pub const fn ch3cpreg0(&self) -> &Ch3cpreg0 {
        &self.ch3cpreg0
    }
    #[doc = "0xd0c - CH3CPREG1"]
    #[inline(always)]
    pub const fn ch3cpreg1(&self) -> &Ch3cpreg1 {
        &self.ch3cpreg1
    }
    #[doc = "0xd10 - CH3CPREG2"]
    #[inline(always)]
    pub const fn ch3cpreg2(&self) -> &Ch3cpreg2 {
        &self.ch3cpreg2
    }
    #[doc = "0xd14 - CH3CPREG3"]
    #[inline(always)]
    pub const fn ch3cpreg3(&self) -> &Ch3cpreg3 {
        &self.ch3cpreg3
    }
    #[doc = "0xd18 - CH3CPREG4"]
    #[inline(always)]
    pub const fn ch3cpreg4(&self) -> &Ch3cpreg4 {
        &self.ch3cpreg4
    }
    #[doc = "0xd1c - CH3CPREG5"]
    #[inline(always)]
    pub const fn ch3cpreg5(&self) -> &Ch3cpreg5 {
        &self.ch3cpreg5
    }
    #[doc = "0xd20 - CH3CPREG6"]
    #[inline(always)]
    pub const fn ch3cpreg6(&self) -> &Ch3cpreg6 {
        &self.ch3cpreg6
    }
    #[doc = "0xd24 - CH3CPREG7"]
    #[inline(always)]
    pub const fn ch3cpreg7(&self) -> &Ch3cpreg7 {
        &self.ch3cpreg7
    }
    #[doc = "0xd28 - CH3CPREG8"]
    #[inline(always)]
    pub const fn ch3cpreg8(&self) -> &Ch3cpreg8 {
        &self.ch3cpreg8
    }
    #[doc = "0xd2c - CH3CPREG9"]
    #[inline(always)]
    pub const fn ch3cpreg9(&self) -> &Ch3cpreg9 {
        &self.ch3cpreg9
    }
    #[doc = "0xd30 - CH3CPREG10"]
    #[inline(always)]
    pub const fn ch3cpreg10(&self) -> &Ch3cpreg10 {
        &self.ch3cpreg10
    }
    #[doc = "0xd34 - CH3CPREG11"]
    #[inline(always)]
    pub const fn ch3cpreg11(&self) -> &Ch3cpreg11 {
        &self.ch3cpreg11
    }
    #[doc = "0xd38 - CH3CPREG12"]
    #[inline(always)]
    pub const fn ch3cpreg12(&self) -> &Ch3cpreg12 {
        &self.ch3cpreg12
    }
    #[doc = "0xd3c - CH3CPREG13"]
    #[inline(always)]
    pub const fn ch3cpreg13(&self) -> &Ch3cpreg13 {
        &self.ch3cpreg13
    }
    #[doc = "0xd40 - CH3CPREG14"]
    #[inline(always)]
    pub const fn ch3cpreg14(&self) -> &Ch3cpreg14 {
        &self.ch3cpreg14
    }
    #[doc = "0xd44 - CH3CPREG15"]
    #[inline(always)]
    pub const fn ch3cpreg15(&self) -> &Ch3cpreg15 {
        &self.ch3cpreg15
    }
    #[doc = "0xd48 - CH4CPREG0"]
    #[inline(always)]
    pub const fn ch4cpreg0(&self) -> &Ch4cpreg0 {
        &self.ch4cpreg0
    }
    #[doc = "0xd4c - CH4CPREG1"]
    #[inline(always)]
    pub const fn ch4cpreg1(&self) -> &Ch4cpreg1 {
        &self.ch4cpreg1
    }
    #[doc = "0xd50 - CH4CPREG2"]
    #[inline(always)]
    pub const fn ch4cpreg2(&self) -> &Ch4cpreg2 {
        &self.ch4cpreg2
    }
    #[doc = "0xd54 - CH4CPREG3"]
    #[inline(always)]
    pub const fn ch4cpreg3(&self) -> &Ch4cpreg3 {
        &self.ch4cpreg3
    }
    #[doc = "0xd58 - CH4CPREG4"]
    #[inline(always)]
    pub const fn ch4cpreg4(&self) -> &Ch4cpreg4 {
        &self.ch4cpreg4
    }
    #[doc = "0xd5c - CH4CPREG5"]
    #[inline(always)]
    pub const fn ch4cpreg5(&self) -> &Ch4cpreg5 {
        &self.ch4cpreg5
    }
    #[doc = "0xd60 - CH4CPREG6"]
    #[inline(always)]
    pub const fn ch4cpreg6(&self) -> &Ch4cpreg6 {
        &self.ch4cpreg6
    }
    #[doc = "0xd64 - CH4CPREG7"]
    #[inline(always)]
    pub const fn ch4cpreg7(&self) -> &Ch4cpreg7 {
        &self.ch4cpreg7
    }
    #[doc = "0xd68 - CH4CPREG8"]
    #[inline(always)]
    pub const fn ch4cpreg8(&self) -> &Ch4cpreg8 {
        &self.ch4cpreg8
    }
    #[doc = "0xd6c - CH4CPREG9"]
    #[inline(always)]
    pub const fn ch4cpreg9(&self) -> &Ch4cpreg9 {
        &self.ch4cpreg9
    }
    #[doc = "0xd70 - CH4CPREG10"]
    #[inline(always)]
    pub const fn ch4cpreg10(&self) -> &Ch4cpreg10 {
        &self.ch4cpreg10
    }
    #[doc = "0xd74 - CH4CPREG11"]
    #[inline(always)]
    pub const fn ch4cpreg11(&self) -> &Ch4cpreg11 {
        &self.ch4cpreg11
    }
    #[doc = "0xd78 - CH4CPREG12"]
    #[inline(always)]
    pub const fn ch4cpreg12(&self) -> &Ch4cpreg12 {
        &self.ch4cpreg12
    }
    #[doc = "0xd7c - CH4CPREG13"]
    #[inline(always)]
    pub const fn ch4cpreg13(&self) -> &Ch4cpreg13 {
        &self.ch4cpreg13
    }
    #[doc = "0xd80 - CH4CPREG14"]
    #[inline(always)]
    pub const fn ch4cpreg14(&self) -> &Ch4cpreg14 {
        &self.ch4cpreg14
    }
    #[doc = "0xd84 - CH4CPREG15"]
    #[inline(always)]
    pub const fn ch4cpreg15(&self) -> &Ch4cpreg15 {
        &self.ch4cpreg15
    }
    #[doc = "0xd88 - CH5CPREG0"]
    #[inline(always)]
    pub const fn ch5cpreg0(&self) -> &Ch5cpreg0 {
        &self.ch5cpreg0
    }
    #[doc = "0xd8c - CH5CPREG1"]
    #[inline(always)]
    pub const fn ch5cpreg1(&self) -> &Ch5cpreg1 {
        &self.ch5cpreg1
    }
    #[doc = "0xd90 - CH5CPREG2"]
    #[inline(always)]
    pub const fn ch5cpreg2(&self) -> &Ch5cpreg2 {
        &self.ch5cpreg2
    }
    #[doc = "0xd94 - CH5CPREG3"]
    #[inline(always)]
    pub const fn ch5cpreg3(&self) -> &Ch5cpreg3 {
        &self.ch5cpreg3
    }
    #[doc = "0xd98 - CH5CPREG4"]
    #[inline(always)]
    pub const fn ch5cpreg4(&self) -> &Ch5cpreg4 {
        &self.ch5cpreg4
    }
    #[doc = "0xd9c - CH5CPREG5"]
    #[inline(always)]
    pub const fn ch5cpreg5(&self) -> &Ch5cpreg5 {
        &self.ch5cpreg5
    }
    #[doc = "0xda0 - CH5CPREG6"]
    #[inline(always)]
    pub const fn ch5cpreg6(&self) -> &Ch5cpreg6 {
        &self.ch5cpreg6
    }
    #[doc = "0xda4 - CH5CPREG7"]
    #[inline(always)]
    pub const fn ch5cpreg7(&self) -> &Ch5cpreg7 {
        &self.ch5cpreg7
    }
    #[doc = "0xda8 - CH5CPREG8"]
    #[inline(always)]
    pub const fn ch5cpreg8(&self) -> &Ch5cpreg8 {
        &self.ch5cpreg8
    }
    #[doc = "0xdac - CH5CPREG9"]
    #[inline(always)]
    pub const fn ch5cpreg9(&self) -> &Ch5cpreg9 {
        &self.ch5cpreg9
    }
    #[doc = "0xdb0 - CH5CPREG10"]
    #[inline(always)]
    pub const fn ch5cpreg10(&self) -> &Ch5cpreg10 {
        &self.ch5cpreg10
    }
    #[doc = "0xdb4 - CH5CPREG11"]
    #[inline(always)]
    pub const fn ch5cpreg11(&self) -> &Ch5cpreg11 {
        &self.ch5cpreg11
    }
    #[doc = "0xdb8 - CH5CPREG12"]
    #[inline(always)]
    pub const fn ch5cpreg12(&self) -> &Ch5cpreg12 {
        &self.ch5cpreg12
    }
    #[doc = "0xdbc - CH5CPREG13"]
    #[inline(always)]
    pub const fn ch5cpreg13(&self) -> &Ch5cpreg13 {
        &self.ch5cpreg13
    }
    #[doc = "0xdc0 - CH5CPREG14"]
    #[inline(always)]
    pub const fn ch5cpreg14(&self) -> &Ch5cpreg14 {
        &self.ch5cpreg14
    }
    #[doc = "0xdc4 - CH5CPREG15"]
    #[inline(always)]
    pub const fn ch5cpreg15(&self) -> &Ch5cpreg15 {
        &self.ch5cpreg15
    }
    #[doc = "0xdc8 - CH6CPREG0"]
    #[inline(always)]
    pub const fn ch6cpreg0(&self) -> &Ch6cpreg0 {
        &self.ch6cpreg0
    }
    #[doc = "0xdcc - CH6CPREG1"]
    #[inline(always)]
    pub const fn ch6cpreg1(&self) -> &Ch6cpreg1 {
        &self.ch6cpreg1
    }
    #[doc = "0xdd0 - CH6CPREG2"]
    #[inline(always)]
    pub const fn ch6cpreg2(&self) -> &Ch6cpreg2 {
        &self.ch6cpreg2
    }
    #[doc = "0xdd4 - CH6CPREG3"]
    #[inline(always)]
    pub const fn ch6cpreg3(&self) -> &Ch6cpreg3 {
        &self.ch6cpreg3
    }
    #[doc = "0xdd8 - CH6CPREG4"]
    #[inline(always)]
    pub const fn ch6cpreg4(&self) -> &Ch6cpreg4 {
        &self.ch6cpreg4
    }
    #[doc = "0xddc - CH6CPREG5"]
    #[inline(always)]
    pub const fn ch6cpreg5(&self) -> &Ch6cpreg5 {
        &self.ch6cpreg5
    }
    #[doc = "0xde0 - CH6CPREG6"]
    #[inline(always)]
    pub const fn ch6cpreg6(&self) -> &Ch6cpreg6 {
        &self.ch6cpreg6
    }
    #[doc = "0xde4 - CH6CPREG7"]
    #[inline(always)]
    pub const fn ch6cpreg7(&self) -> &Ch6cpreg7 {
        &self.ch6cpreg7
    }
    #[doc = "0xde8 - CH6CPREG8"]
    #[inline(always)]
    pub const fn ch6cpreg8(&self) -> &Ch6cpreg8 {
        &self.ch6cpreg8
    }
    #[doc = "0xdec - CH6CPREG9"]
    #[inline(always)]
    pub const fn ch6cpreg9(&self) -> &Ch6cpreg9 {
        &self.ch6cpreg9
    }
    #[doc = "0xdf0 - CH6CPREG10"]
    #[inline(always)]
    pub const fn ch6cpreg10(&self) -> &Ch6cpreg10 {
        &self.ch6cpreg10
    }
    #[doc = "0xdf4 - CH6CPREG11"]
    #[inline(always)]
    pub const fn ch6cpreg11(&self) -> &Ch6cpreg11 {
        &self.ch6cpreg11
    }
    #[doc = "0xdf8 - CH6CPREG12"]
    #[inline(always)]
    pub const fn ch6cpreg12(&self) -> &Ch6cpreg12 {
        &self.ch6cpreg12
    }
    #[doc = "0xdfc - CH6CPREG13"]
    #[inline(always)]
    pub const fn ch6cpreg13(&self) -> &Ch6cpreg13 {
        &self.ch6cpreg13
    }
    #[doc = "0xe00 - CH6CPREG14"]
    #[inline(always)]
    pub const fn ch6cpreg14(&self) -> &Ch6cpreg14 {
        &self.ch6cpreg14
    }
    #[doc = "0xe04 - CH6CPREG15"]
    #[inline(always)]
    pub const fn ch6cpreg15(&self) -> &Ch6cpreg15 {
        &self.ch6cpreg15
    }
    #[doc = "0xe08 - CH7CPREG0"]
    #[inline(always)]
    pub const fn ch7cpreg0(&self) -> &Ch7cpreg0 {
        &self.ch7cpreg0
    }
    #[doc = "0xe0c - CH7CPREG1"]
    #[inline(always)]
    pub const fn ch7cpreg1(&self) -> &Ch7cpreg1 {
        &self.ch7cpreg1
    }
    #[doc = "0xe10 - CH7CPREG2"]
    #[inline(always)]
    pub const fn ch7cpreg2(&self) -> &Ch7cpreg2 {
        &self.ch7cpreg2
    }
    #[doc = "0xe14 - CH7CPREG3"]
    #[inline(always)]
    pub const fn ch7cpreg3(&self) -> &Ch7cpreg3 {
        &self.ch7cpreg3
    }
    #[doc = "0xe18 - CH7CPREG4"]
    #[inline(always)]
    pub const fn ch7cpreg4(&self) -> &Ch7cpreg4 {
        &self.ch7cpreg4
    }
    #[doc = "0xe1c - CH7CPREG5"]
    #[inline(always)]
    pub const fn ch7cpreg5(&self) -> &Ch7cpreg5 {
        &self.ch7cpreg5
    }
    #[doc = "0xe20 - CH7CPREG6"]
    #[inline(always)]
    pub const fn ch7cpreg6(&self) -> &Ch7cpreg6 {
        &self.ch7cpreg6
    }
    #[doc = "0xe24 - CH7CPREG7"]
    #[inline(always)]
    pub const fn ch7cpreg7(&self) -> &Ch7cpreg7 {
        &self.ch7cpreg7
    }
    #[doc = "0xe28 - CH7CPREG8"]
    #[inline(always)]
    pub const fn ch7cpreg8(&self) -> &Ch7cpreg8 {
        &self.ch7cpreg8
    }
    #[doc = "0xe2c - CH7CPREG9"]
    #[inline(always)]
    pub const fn ch7cpreg9(&self) -> &Ch7cpreg9 {
        &self.ch7cpreg9
    }
    #[doc = "0xe30 - CH7CPREG10"]
    #[inline(always)]
    pub const fn ch7cpreg10(&self) -> &Ch7cpreg10 {
        &self.ch7cpreg10
    }
    #[doc = "0xe34 - CH7CPREG11"]
    #[inline(always)]
    pub const fn ch7cpreg11(&self) -> &Ch7cpreg11 {
        &self.ch7cpreg11
    }
    #[doc = "0xe38 - CH7CPREG12"]
    #[inline(always)]
    pub const fn ch7cpreg12(&self) -> &Ch7cpreg12 {
        &self.ch7cpreg12
    }
    #[doc = "0xe3c - CH7CPREG13"]
    #[inline(always)]
    pub const fn ch7cpreg13(&self) -> &Ch7cpreg13 {
        &self.ch7cpreg13
    }
    #[doc = "0xe40 - CH7CPREG14"]
    #[inline(always)]
    pub const fn ch7cpreg14(&self) -> &Ch7cpreg14 {
        &self.ch7cpreg14
    }
    #[doc = "0xe44 - CH7CPREG15"]
    #[inline(always)]
    pub const fn ch7cpreg15(&self) -> &Ch7cpreg15 {
        &self.ch7cpreg15
    }
    #[doc = "0xe48 - CH01_HIL_CP_OVERRIDE"]
    #[inline(always)]
    pub const fn ch01_hil_cp_override(&self) -> &Ch01HilCpOverride {
        &self.ch01_hil_cp_override
    }
    #[doc = "0xe4c - CH23_HIL_CP_OVERRIDE"]
    #[inline(always)]
    pub const fn ch23_hil_cp_override(&self) -> &Ch23HilCpOverride {
        &self.ch23_hil_cp_override
    }
    #[doc = "0xe50 - CH45_HIL_CP_OVERRIDE"]
    #[inline(always)]
    pub const fn ch45_hil_cp_override(&self) -> &Ch45HilCpOverride {
        &self.ch45_hil_cp_override
    }
    #[doc = "0xe54 - CH67_HIL_CP_OVERRIDE"]
    #[inline(always)]
    pub const fn ch67_hil_cp_override(&self) -> &Ch67HilCpOverride {
        &self.ch67_hil_cp_override
    }
    #[doc = "0xe58 - CH_HIL_CP_OVERRIDE"]
    #[inline(always)]
    pub const fn ch_hil_cp_override(&self) -> &ChHilCpOverride {
        &self.ch_hil_cp_override
    }
    #[doc = "0xe5c - RSS_BOOKKEEPING_CTRL"]
    #[inline(always)]
    pub const fn rss_bookkeeping_ctrl(&self) -> &RssBookkeepingCtrl {
        &self.rss_bookkeeping_ctrl
    }
    #[doc = "0xe60 - RSS_APP_GP"]
    #[inline(always)]
    pub const fn rss_app_gp(&self) -> &RssAppGp {
        &self.rss_app_gp
    }
    #[doc = "0xe64 - RSS_BOOKKEEPING_SEQ_NUM"]
    #[inline(always)]
    pub const fn rss_bookkeeping_seq_num(&self) -> &RssBookkeepingSeqNum {
        &self.rss_bookkeeping_seq_num
    }
    #[doc = "0xe68 - RSS_BOOKKEEPING_FRM_CNT"]
    #[inline(always)]
    pub const fn rss_bookkeeping_frm_cnt(&self) -> &RssBookkeepingFrmCnt {
        &self.rss_bookkeeping_frm_cnt
    }
    #[doc = "0xe6c - RSS_BOOKKEEPING_CHRP_CNT"]
    #[inline(always)]
    pub const fn rss_bookkeeping_chrp_cnt(&self) -> &RssBookkeepingChrpCnt {
        &self.rss_bookkeeping_chrp_cnt
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
#[doc = "RSS_TPCC_A_ERRAGG_MASK (rw) register accessor: RSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_erragg_mask`]
module"]
#[doc(alias = "RSS_TPCC_A_ERRAGG_MASK")]
pub type RssTpccAErraggMask = crate::Reg<rss_tpcc_a_erragg_mask::RssTpccAErraggMaskSpec>;
#[doc = "RSS_TPCC_A_ERRAGG_MASK"]
pub mod rss_tpcc_a_erragg_mask;
#[doc = "RSS_TPCC_A_ERRAGG_STATUS (rw) register accessor: RSS_TPCC_A_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_erragg_status`]
module"]
#[doc(alias = "RSS_TPCC_A_ERRAGG_STATUS")]
pub type RssTpccAErraggStatus = crate::Reg<rss_tpcc_a_erragg_status::RssTpccAErraggStatusSpec>;
#[doc = "RSS_TPCC_A_ERRAGG_STATUS"]
pub mod rss_tpcc_a_erragg_status;
#[doc = "RSS_TPCC_A_ERRAGG_STATUS_RAW (rw) register accessor: RSS_TPCC_A_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_erragg_status_raw`]
module"]
#[doc(alias = "RSS_TPCC_A_ERRAGG_STATUS_RAW")]
pub type RssTpccAErraggStatusRaw =
    crate::Reg<rss_tpcc_a_erragg_status_raw::RssTpccAErraggStatusRawSpec>;
#[doc = "RSS_TPCC_A_ERRAGG_STATUS_RAW"]
pub mod rss_tpcc_a_erragg_status_raw;
#[doc = "RSS_TPCC_A_INTAGG_MASK (rw) register accessor: RSS_TPCC_A_INTAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_intagg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_intagg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_intagg_mask`]
module"]
#[doc(alias = "RSS_TPCC_A_INTAGG_MASK")]
pub type RssTpccAIntaggMask = crate::Reg<rss_tpcc_a_intagg_mask::RssTpccAIntaggMaskSpec>;
#[doc = "RSS_TPCC_A_INTAGG_MASK"]
pub mod rss_tpcc_a_intagg_mask;
#[doc = "RSS_TPCC_A_INTAGG_STATUS (rw) register accessor: RSS_TPCC_A_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_intagg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_intagg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_intagg_status`]
module"]
#[doc(alias = "RSS_TPCC_A_INTAGG_STATUS")]
pub type RssTpccAIntaggStatus = crate::Reg<rss_tpcc_a_intagg_status::RssTpccAIntaggStatusSpec>;
#[doc = "RSS_TPCC_A_INTAGG_STATUS"]
pub mod rss_tpcc_a_intagg_status;
#[doc = "RSS_TPCC_A_INTAGG_STATUS_RAW (rw) register accessor: RSS_TPCC_A_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_intagg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_intagg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_intagg_status_raw`]
module"]
#[doc(alias = "RSS_TPCC_A_INTAGG_STATUS_RAW")]
pub type RssTpccAIntaggStatusRaw =
    crate::Reg<rss_tpcc_a_intagg_status_raw::RssTpccAIntaggStatusRawSpec>;
#[doc = "RSS_TPCC_A_INTAGG_STATUS_RAW"]
pub mod rss_tpcc_a_intagg_status_raw;
#[doc = "RSS_TPCC_MEMINIT_START (rw) register accessor: RSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_meminit_start`]
module"]
#[doc(alias = "RSS_TPCC_MEMINIT_START")]
pub type RssTpccMeminitStart = crate::Reg<rss_tpcc_meminit_start::RssTpccMeminitStartSpec>;
#[doc = "RSS_TPCC_MEMINIT_START"]
pub mod rss_tpcc_meminit_start;
#[doc = "RSS_TPCC_MEMINIT_DONE (rw) register accessor: RSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_meminit_done`]
module"]
#[doc(alias = "RSS_TPCC_MEMINIT_DONE")]
pub type RssTpccMeminitDone = crate::Reg<rss_tpcc_meminit_done::RssTpccMeminitDoneSpec>;
#[doc = "RSS_TPCC_MEMINIT_DONE"]
pub mod rss_tpcc_meminit_done;
#[doc = "RSS_TPCC_MEMINIT_STATUS (rw) register accessor: RSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_meminit_status`]
module"]
#[doc(alias = "RSS_TPCC_MEMINIT_STATUS")]
pub type RssTpccMeminitStatus = crate::Reg<rss_tpcc_meminit_status::RssTpccMeminitStatusSpec>;
#[doc = "RSS_TPCC_MEMINIT_STATUS"]
pub mod rss_tpcc_meminit_status;
#[doc = "TPTC_DBS_CFG (rw) register accessor: TPTC_DBS_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tptc_dbs_cfg`]
module"]
#[doc(alias = "TPTC_DBS_CFG")]
pub type TptcDbsCfg = crate::Reg<tptc_dbs_cfg::TptcDbsCfgSpec>;
#[doc = "TPTC_DBS_CFG"]
pub mod tptc_dbs_cfg;
#[doc = "RSS_TPCC_A_PARITY_CTRL (rw) register accessor: RSS_TPCC_A_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_parity_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_parity_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_parity_ctrl`]
module"]
#[doc(alias = "RSS_TPCC_A_PARITY_CTRL")]
pub type RssTpccAParityCtrl = crate::Reg<rss_tpcc_a_parity_ctrl::RssTpccAParityCtrlSpec>;
#[doc = "RSS_TPCC_A_PARITY_CTRL"]
pub mod rss_tpcc_a_parity_ctrl;
#[doc = "RSS_TPCC_A_PARITY_STATUS (rw) register accessor: RSS_TPCC_A_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_parity_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_parity_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tpcc_a_parity_status`]
module"]
#[doc(alias = "RSS_TPCC_A_PARITY_STATUS")]
pub type RssTpccAParityStatus = crate::Reg<rss_tpcc_a_parity_status::RssTpccAParityStatusSpec>;
#[doc = "RSS_TPCC_A_PARITY_STATUS"]
pub mod rss_tpcc_a_parity_status;
#[doc = "RSS_TPTC_BOUNDARY_CFG (rw) register accessor: RSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tptc_boundary_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tptc_boundary_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tptc_boundary_cfg`]
module"]
#[doc(alias = "RSS_TPTC_BOUNDARY_CFG")]
pub type RssTptcBoundaryCfg = crate::Reg<rss_tptc_boundary_cfg::RssTptcBoundaryCfgSpec>;
#[doc = "RSS_TPTC_BOUNDARY_CFG"]
pub mod rss_tptc_boundary_cfg;
#[doc = "RSS_TPTC_XID_REORDER_CFG (rw) register accessor: RSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tptc_xid_reorder_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tptc_xid_reorder_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_tptc_xid_reorder_cfg`]
module"]
#[doc(alias = "RSS_TPTC_XID_REORDER_CFG")]
pub type RssTptcXidReorderCfg = crate::Reg<rss_tptc_xid_reorder_cfg::RssTptcXidReorderCfgSpec>;
#[doc = "RSS_TPTC_XID_REORDER_CFG"]
pub mod rss_tptc_xid_reorder_cfg;
#[doc = "DBG_ACK_CPU_CTRL (rw) register accessor: DBG_ACK_CPU_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_cpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_cpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_ack_cpu_ctrl`]
module"]
#[doc(alias = "DBG_ACK_CPU_CTRL")]
pub type DbgAckCpuCtrl = crate::Reg<dbg_ack_cpu_ctrl::DbgAckCpuCtrlSpec>;
#[doc = "DBG_ACK_CPU_CTRL"]
pub mod dbg_ack_cpu_ctrl;
#[doc = "RSS_ADCBUF_PING_MEMINIT (rw) register accessor: RSS_ADCBUF_PING_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_ping_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_ping_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_ping_meminit`]
module"]
#[doc(alias = "RSS_ADCBUF_PING_MEMINIT")]
pub type RssAdcbufPingMeminit = crate::Reg<rss_adcbuf_ping_meminit::RssAdcbufPingMeminitSpec>;
#[doc = "RSS_ADCBUF_PING_MEMINIT"]
pub mod rss_adcbuf_ping_meminit;
#[doc = "RSS_ADCBUF_PING_MEMINIT_DONE (rw) register accessor: RSS_ADCBUF_PING_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_ping_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_ping_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_ping_meminit_done`]
module"]
#[doc(alias = "RSS_ADCBUF_PING_MEMINIT_DONE")]
pub type RssAdcbufPingMeminitDone =
    crate::Reg<rss_adcbuf_ping_meminit_done::RssAdcbufPingMeminitDoneSpec>;
#[doc = "RSS_ADCBUF_PING_MEMINIT_DONE"]
pub mod rss_adcbuf_ping_meminit_done;
#[doc = "RSS_ADCBUF_PING_MEMINIT_STATUS (rw) register accessor: RSS_ADCBUF_PING_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_ping_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_ping_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_ping_meminit_status`]
module"]
#[doc(alias = "RSS_ADCBUF_PING_MEMINIT_STATUS")]
pub type RssAdcbufPingMeminitStatus =
    crate::Reg<rss_adcbuf_ping_meminit_status::RssAdcbufPingMeminitStatusSpec>;
#[doc = "RSS_ADCBUF_PING_MEMINIT_STATUS"]
pub mod rss_adcbuf_ping_meminit_status;
#[doc = "RSS_ADCBUF_PONG_MEMINIT (rw) register accessor: RSS_ADCBUF_PONG_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_pong_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_pong_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_pong_meminit`]
module"]
#[doc(alias = "RSS_ADCBUF_PONG_MEMINIT")]
pub type RssAdcbufPongMeminit = crate::Reg<rss_adcbuf_pong_meminit::RssAdcbufPongMeminitSpec>;
#[doc = "RSS_ADCBUF_PONG_MEMINIT"]
pub mod rss_adcbuf_pong_meminit;
#[doc = "RSS_ADCBUF_PONG_MEMINIT_DONE (rw) register accessor: RSS_ADCBUF_PONG_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_pong_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_pong_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_pong_meminit_done`]
module"]
#[doc(alias = "RSS_ADCBUF_PONG_MEMINIT_DONE")]
pub type RssAdcbufPongMeminitDone =
    crate::Reg<rss_adcbuf_pong_meminit_done::RssAdcbufPongMeminitDoneSpec>;
#[doc = "RSS_ADCBUF_PONG_MEMINIT_DONE"]
pub mod rss_adcbuf_pong_meminit_done;
#[doc = "RSS_ADCBUF_PONG_MEMINIT_STATUS (rw) register accessor: RSS_ADCBUF_PONG_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_pong_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_pong_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_adcbuf_pong_meminit_status`]
module"]
#[doc(alias = "RSS_ADCBUF_PONG_MEMINIT_STATUS")]
pub type RssAdcbufPongMeminitStatus =
    crate::Reg<rss_adcbuf_pong_meminit_status::RssAdcbufPongMeminitStatusSpec>;
#[doc = "RSS_ADCBUF_PONG_MEMINIT_STATUS"]
pub mod rss_adcbuf_pong_meminit_status;
#[doc = "SOC_TO_BSS_SW_INT (rw) register accessor: SOC_TO_BSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_to_bss_sw_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_to_bss_sw_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_to_bss_sw_int`]
module"]
#[doc(alias = "SOC_TO_BSS_SW_INT")]
pub type SocToBssSwInt = crate::Reg<soc_to_bss_sw_int::SocToBssSwIntSpec>;
#[doc = "SOC_TO_BSS_SW_INT"]
pub mod soc_to_bss_sw_int;
#[doc = "RSS_DBG_ACK_CTL0 (rw) register accessor: RSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_dbg_ack_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_dbg_ack_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_dbg_ack_ctl0`]
module"]
#[doc(alias = "RSS_DBG_ACK_CTL0")]
pub type RssDbgAckCtl0 = crate::Reg<rss_dbg_ack_ctl0::RssDbgAckCtl0Spec>;
#[doc = "RSS_DBG_ACK_CTL0"]
pub mod rss_dbg_ack_ctl0;
#[doc = "DMMSWINT1 (rw) register accessor: DMMSWINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmswint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmswint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmmswint1`]
module"]
#[doc(alias = "DMMSWINT1")]
pub type Dmmswint1 = crate::Reg<dmmswint1::Dmmswint1Spec>;
#[doc = "DMMSWINT1"]
pub mod dmmswint1;
#[doc = "RSS_SHARED_MEM_MEMINIT (rw) register accessor: RSS_SHARED_MEM_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_shared_mem_meminit`]
module"]
#[doc(alias = "RSS_SHARED_MEM_MEMINIT")]
pub type RssSharedMemMeminit = crate::Reg<rss_shared_mem_meminit::RssSharedMemMeminitSpec>;
#[doc = "RSS_SHARED_MEM_MEMINIT"]
pub mod rss_shared_mem_meminit;
#[doc = "RSS_SHARED_MEM_MEMINIT_DONE (rw) register accessor: RSS_SHARED_MEM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_shared_mem_meminit_done`]
module"]
#[doc(alias = "RSS_SHARED_MEM_MEMINIT_DONE")]
pub type RssSharedMemMeminitDone =
    crate::Reg<rss_shared_mem_meminit_done::RssSharedMemMeminitDoneSpec>;
#[doc = "RSS_SHARED_MEM_MEMINIT_DONE"]
pub mod rss_shared_mem_meminit_done;
#[doc = "RSS_SHARED_MEM_MEMINIT_STATUS (rw) register accessor: RSS_SHARED_MEM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_shared_mem_meminit_status`]
module"]
#[doc(alias = "RSS_SHARED_MEM_MEMINIT_STATUS")]
pub type RssSharedMemMeminitStatus =
    crate::Reg<rss_shared_mem_meminit_status::RssSharedMemMeminitStatusSpec>;
#[doc = "RSS_SHARED_MEM_MEMINIT_STATUS"]
pub mod rss_shared_mem_meminit_status;
#[doc = "BSS_CONTROL (rw) register accessor: BSS_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_control`]
module"]
#[doc(alias = "BSS_CONTROL")]
pub type BssControl = crate::Reg<bss_control::BssControlSpec>;
#[doc = "BSS_CONTROL"]
pub mod bss_control;
#[doc = "BSS_TCM_MEMINIT (rw) register accessor: BSS_TCM_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_tcm_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_tcm_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_tcm_meminit`]
module"]
#[doc(alias = "BSS_TCM_MEMINIT")]
pub type BssTcmMeminit = crate::Reg<bss_tcm_meminit::BssTcmMeminitSpec>;
#[doc = "BSS_TCM_MEMINIT"]
pub mod bss_tcm_meminit;
#[doc = "BSS_TCM_MEMINIT_DONE (rw) register accessor: BSS_TCM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_tcm_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_tcm_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_tcm_meminit_done`]
module"]
#[doc(alias = "BSS_TCM_MEMINIT_DONE")]
pub type BssTcmMeminitDone = crate::Reg<bss_tcm_meminit_done::BssTcmMeminitDoneSpec>;
#[doc = "BSS_TCM_MEMINIT_DONE"]
pub mod bss_tcm_meminit_done;
#[doc = "BSS_TCM_MEMINIT_STATUS (rw) register accessor: BSS_TCM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_tcm_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_tcm_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_tcm_meminit_status`]
module"]
#[doc(alias = "BSS_TCM_MEMINIT_STATUS")]
pub type BssTcmMeminitStatus = crate::Reg<bss_tcm_meminit_status::BssTcmMeminitStatusSpec>;
#[doc = "BSS_TCM_MEMINIT_STATUS"]
pub mod bss_tcm_meminit_status;
#[doc = "BSS_VIM_MEMINIT (rw) register accessor: BSS_VIM_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_vim_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_vim_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_vim_meminit`]
module"]
#[doc(alias = "BSS_VIM_MEMINIT")]
pub type BssVimMeminit = crate::Reg<bss_vim_meminit::BssVimMeminitSpec>;
#[doc = "BSS_VIM_MEMINIT"]
pub mod bss_vim_meminit;
#[doc = "BSS_VIM_MEMINIT_DONE (rw) register accessor: BSS_VIM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_vim_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_vim_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_vim_meminit_done`]
module"]
#[doc(alias = "BSS_VIM_MEMINIT_DONE")]
pub type BssVimMeminitDone = crate::Reg<bss_vim_meminit_done::BssVimMeminitDoneSpec>;
#[doc = "BSS_VIM_MEMINIT_DONE"]
pub mod bss_vim_meminit_done;
#[doc = "BSS_VIM_MEMINIT_STATUS (rw) register accessor: BSS_VIM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_vim_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_vim_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_vim_meminit_status`]
module"]
#[doc(alias = "BSS_VIM_MEMINIT_STATUS")]
pub type BssVimMeminitStatus = crate::Reg<bss_vim_meminit_status::BssVimMeminitStatusSpec>;
#[doc = "BSS_VIM_MEMINIT_STATUS"]
pub mod bss_vim_meminit_status;
#[doc = "BSS_DFE_MEMINIT (rw) register accessor: BSS_DFE_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dfe_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dfe_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_dfe_meminit`]
module"]
#[doc(alias = "BSS_DFE_MEMINIT")]
pub type BssDfeMeminit = crate::Reg<bss_dfe_meminit::BssDfeMeminitSpec>;
#[doc = "BSS_DFE_MEMINIT"]
pub mod bss_dfe_meminit;
#[doc = "BSS_DFE_MEMINIT_DONE (rw) register accessor: BSS_DFE_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dfe_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dfe_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_dfe_meminit_done`]
module"]
#[doc(alias = "BSS_DFE_MEMINIT_DONE")]
pub type BssDfeMeminitDone = crate::Reg<bss_dfe_meminit_done::BssDfeMeminitDoneSpec>;
#[doc = "BSS_DFE_MEMINIT_DONE"]
pub mod bss_dfe_meminit_done;
#[doc = "BSS_DFE_MEMINIT_STATUS (rw) register accessor: BSS_DFE_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dfe_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dfe_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_dfe_meminit_status`]
module"]
#[doc(alias = "BSS_DFE_MEMINIT_STATUS")]
pub type BssDfeMeminitStatus = crate::Reg<bss_dfe_meminit_status::BssDfeMeminitStatusSpec>;
#[doc = "BSS_DFE_MEMINIT_STATUS"]
pub mod bss_dfe_meminit_status;
#[doc = "BSS_RAMPGEN_MEMINIT (rw) register accessor: BSS_RAMPGEN_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_rampgen_meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_rampgen_meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_rampgen_meminit`]
module"]
#[doc(alias = "BSS_RAMPGEN_MEMINIT")]
pub type BssRampgenMeminit = crate::Reg<bss_rampgen_meminit::BssRampgenMeminitSpec>;
#[doc = "BSS_RAMPGEN_MEMINIT"]
pub mod bss_rampgen_meminit;
#[doc = "BSS_RAMPGEN_MEMINIT_DONE (rw) register accessor: BSS_RAMPGEN_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_rampgen_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_rampgen_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_rampgen_meminit_done`]
module"]
#[doc(alias = "BSS_RAMPGEN_MEMINIT_DONE")]
pub type BssRampgenMeminitDone = crate::Reg<bss_rampgen_meminit_done::BssRampgenMeminitDoneSpec>;
#[doc = "BSS_RAMPGEN_MEMINIT_DONE"]
pub mod bss_rampgen_meminit_done;
#[doc = "BSS_RAMPGEN_MEMINIT_STATUS (rw) register accessor: BSS_RAMPGEN_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_rampgen_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_rampgen_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_rampgen_meminit_status`]
module"]
#[doc(alias = "BSS_RAMPGEN_MEMINIT_STATUS")]
pub type BssRampgenMeminitStatus =
    crate::Reg<bss_rampgen_meminit_status::BssRampgenMeminitStatusSpec>;
#[doc = "BSS_RAMPGEN_MEMINIT_STATUS"]
pub mod bss_rampgen_meminit_status;
#[doc = "BSS_DSS_L3_STICKY (rw) register accessor: BSS_DSS_L3_STICKY\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dss_l3_sticky::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dss_l3_sticky::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_dss_l3_sticky`]
module"]
#[doc(alias = "BSS_DSS_L3_STICKY")]
pub type BssDssL3Sticky = crate::Reg<bss_dss_l3_sticky::BssDssL3StickySpec>;
#[doc = "BSS_DSS_L3_STICKY"]
pub mod bss_dss_l3_sticky;
#[doc = "BSS_DSS_L3_ACCESS (rw) register accessor: BSS_DSS_L3_ACCESS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dss_l3_access::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dss_l3_access::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bss_dss_l3_access`]
module"]
#[doc(alias = "BSS_DSS_L3_ACCESS")]
pub type BssDssL3Access = crate::Reg<bss_dss_l3_access::BssDssL3AccessSpec>;
#[doc = "BSS_DSS_L3_ACCESS"]
pub mod bss_dss_l3_access;
#[doc = "TESTPATTERNRX1ICFG (rw) register accessor: TESTPATTERNRX1ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx1icfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx1icfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx1icfg`]
module"]
#[doc(alias = "TESTPATTERNRX1ICFG")]
pub type Testpatternrx1icfg = crate::Reg<testpatternrx1icfg::Testpatternrx1icfgSpec>;
#[doc = "TESTPATTERNRX1ICFG"]
pub mod testpatternrx1icfg;
#[doc = "TESTPATTERNRX2ICFG (rw) register accessor: TESTPATTERNRX2ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx2icfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx2icfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx2icfg`]
module"]
#[doc(alias = "TESTPATTERNRX2ICFG")]
pub type Testpatternrx2icfg = crate::Reg<testpatternrx2icfg::Testpatternrx2icfgSpec>;
#[doc = "TESTPATTERNRX2ICFG"]
pub mod testpatternrx2icfg;
#[doc = "TESTPATTERNRX3ICFG (rw) register accessor: TESTPATTERNRX3ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx3icfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx3icfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx3icfg`]
module"]
#[doc(alias = "TESTPATTERNRX3ICFG")]
pub type Testpatternrx3icfg = crate::Reg<testpatternrx3icfg::Testpatternrx3icfgSpec>;
#[doc = "TESTPATTERNRX3ICFG"]
pub mod testpatternrx3icfg;
#[doc = "TESTPATTERNRX4ICFG (rw) register accessor: TESTPATTERNRX4ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx4icfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx4icfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx4icfg`]
module"]
#[doc(alias = "TESTPATTERNRX4ICFG")]
pub type Testpatternrx4icfg = crate::Reg<testpatternrx4icfg::Testpatternrx4icfgSpec>;
#[doc = "TESTPATTERNRX4ICFG"]
pub mod testpatternrx4icfg;
#[doc = "TESTPATTERNRX1QCFG (rw) register accessor: TESTPATTERNRX1QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx1qcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx1qcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx1qcfg`]
module"]
#[doc(alias = "TESTPATTERNRX1QCFG")]
pub type Testpatternrx1qcfg = crate::Reg<testpatternrx1qcfg::Testpatternrx1qcfgSpec>;
#[doc = "TESTPATTERNRX1QCFG"]
pub mod testpatternrx1qcfg;
#[doc = "TESTPATTERNRX2QCFG (rw) register accessor: TESTPATTERNRX2QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx2qcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx2qcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx2qcfg`]
module"]
#[doc(alias = "TESTPATTERNRX2QCFG")]
pub type Testpatternrx2qcfg = crate::Reg<testpatternrx2qcfg::Testpatternrx2qcfgSpec>;
#[doc = "TESTPATTERNRX2QCFG"]
pub mod testpatternrx2qcfg;
#[doc = "TESTPATTERNRX3QCFG (rw) register accessor: TESTPATTERNRX3QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx3qcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx3qcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx3qcfg`]
module"]
#[doc(alias = "TESTPATTERNRX3QCFG")]
pub type Testpatternrx3qcfg = crate::Reg<testpatternrx3qcfg::Testpatternrx3qcfgSpec>;
#[doc = "TESTPATTERNRX3QCFG"]
pub mod testpatternrx3qcfg;
#[doc = "TESTPATTERNRX4QCFG (rw) register accessor: TESTPATTERNRX4QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx4qcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx4qcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternrx4qcfg`]
module"]
#[doc(alias = "TESTPATTERNRX4QCFG")]
pub type Testpatternrx4qcfg = crate::Reg<testpatternrx4qcfg::Testpatternrx4qcfgSpec>;
#[doc = "TESTPATTERNRX4QCFG"]
pub mod testpatternrx4qcfg;
#[doc = "TESTPATTERNVLDCFG (rw) register accessor: TESTPATTERNVLDCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternvldcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternvldcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testpatternvldcfg`]
module"]
#[doc(alias = "TESTPATTERNVLDCFG")]
pub type Testpatternvldcfg = crate::Reg<testpatternvldcfg::TestpatternvldcfgSpec>;
#[doc = "TESTPATTERNVLDCFG"]
pub mod testpatternvldcfg;
#[doc = "ADCBUFCFG1 (rw) register accessor: ADCBUFCFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg1`]
module"]
#[doc(alias = "ADCBUFCFG1")]
pub type Adcbufcfg1 = crate::Reg<adcbufcfg1::Adcbufcfg1Spec>;
#[doc = "ADCBUFCFG1"]
pub mod adcbufcfg1;
#[doc = "ADCBUFCFG1_EXTD (rw) register accessor: ADCBUFCFG1_EXTD\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1_extd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1_extd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg1_extd`]
module"]
#[doc(alias = "ADCBUFCFG1_EXTD")]
pub type Adcbufcfg1Extd = crate::Reg<adcbufcfg1_extd::Adcbufcfg1ExtdSpec>;
#[doc = "ADCBUFCFG1_EXTD"]
pub mod adcbufcfg1_extd;
#[doc = "ADCBUFCFG2 (rw) register accessor: ADCBUFCFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg2`]
module"]
#[doc(alias = "ADCBUFCFG2")]
pub type Adcbufcfg2 = crate::Reg<adcbufcfg2::Adcbufcfg2Spec>;
#[doc = "ADCBUFCFG2"]
pub mod adcbufcfg2;
#[doc = "ADCBUFCFG3 (rw) register accessor: ADCBUFCFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg3`]
module"]
#[doc(alias = "ADCBUFCFG3")]
pub type Adcbufcfg3 = crate::Reg<adcbufcfg3::Adcbufcfg3Spec>;
#[doc = "ADCBUFCFG3"]
pub mod adcbufcfg3;
#[doc = "ADCBUFCFG4 (rw) register accessor: ADCBUFCFG4\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg4`]
module"]
#[doc(alias = "ADCBUFCFG4")]
pub type Adcbufcfg4 = crate::Reg<adcbufcfg4::Adcbufcfg4Spec>;
#[doc = "ADCBUFCFG4"]
pub mod adcbufcfg4;
#[doc = "ADCBUFINTGENDITHERDLY (rw) register accessor: ADCBUFINTGENDITHERDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufintgenditherdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufintgenditherdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufintgenditherdly`]
module"]
#[doc(alias = "ADCBUFINTGENDITHERDLY")]
pub type Adcbufintgenditherdly = crate::Reg<adcbufintgenditherdly::AdcbufintgenditherdlySpec>;
#[doc = "ADCBUFINTGENDITHERDLY"]
pub mod adcbufintgenditherdly;
#[doc = "CBUFF_FRAME_START_SEL (rw) register accessor: CBUFF_FRAME_START_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuff_frame_start_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuff_frame_start_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbuff_frame_start_sel`]
module"]
#[doc(alias = "CBUFF_FRAME_START_SEL")]
pub type CbuffFrameStartSel = crate::Reg<cbuff_frame_start_sel::CbuffFrameStartSelSpec>;
#[doc = "CBUFF_FRAME_START_SEL"]
pub mod cbuff_frame_start_sel;
#[doc = "CQCFG1 (rw) register accessor: CQCFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg1`]
module"]
#[doc(alias = "CQCFG1")]
pub type Cqcfg1 = crate::Reg<cqcfg1::Cqcfg1Spec>;
#[doc = "CQCFG1"]
pub mod cqcfg1;
#[doc = "CQCFG2 (rw) register accessor: CQCFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg2`]
module"]
#[doc(alias = "CQCFG2")]
pub type Cqcfg2 = crate::Reg<cqcfg2::Cqcfg2Spec>;
#[doc = "CQCFG2"]
pub mod cqcfg2;
#[doc = "CPREG0 (rw) register accessor: CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg0`]
module"]
#[doc(alias = "CPREG0")]
pub type Cpreg0 = crate::Reg<cpreg0::Cpreg0Spec>;
#[doc = "CPREG0"]
pub mod cpreg0;
#[doc = "CPREG1 (rw) register accessor: CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg1`]
module"]
#[doc(alias = "CPREG1")]
pub type Cpreg1 = crate::Reg<cpreg1::Cpreg1Spec>;
#[doc = "CPREG1"]
pub mod cpreg1;
#[doc = "CPREG2 (rw) register accessor: CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg2`]
module"]
#[doc(alias = "CPREG2")]
pub type Cpreg2 = crate::Reg<cpreg2::Cpreg2Spec>;
#[doc = "CPREG2"]
pub mod cpreg2;
#[doc = "CPREG3 (rw) register accessor: CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg3`]
module"]
#[doc(alias = "CPREG3")]
pub type Cpreg3 = crate::Reg<cpreg3::Cpreg3Spec>;
#[doc = "CPREG3"]
pub mod cpreg3;
#[doc = "CPREG4 (rw) register accessor: CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg4`]
module"]
#[doc(alias = "CPREG4")]
pub type Cpreg4 = crate::Reg<cpreg4::Cpreg4Spec>;
#[doc = "CPREG4"]
pub mod cpreg4;
#[doc = "CPREG5 (rw) register accessor: CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg5`]
module"]
#[doc(alias = "CPREG5")]
pub type Cpreg5 = crate::Reg<cpreg5::Cpreg5Spec>;
#[doc = "CPREG5"]
pub mod cpreg5;
#[doc = "CPREG6 (rw) register accessor: CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg6`]
module"]
#[doc(alias = "CPREG6")]
pub type Cpreg6 = crate::Reg<cpreg6::Cpreg6Spec>;
#[doc = "CPREG6"]
pub mod cpreg6;
#[doc = "CPREG7 (rw) register accessor: CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg7`]
module"]
#[doc(alias = "CPREG7")]
pub type Cpreg7 = crate::Reg<cpreg7::Cpreg7Spec>;
#[doc = "CPREG7"]
pub mod cpreg7;
#[doc = "CPREG8 (rw) register accessor: CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg8`]
module"]
#[doc(alias = "CPREG8")]
pub type Cpreg8 = crate::Reg<cpreg8::Cpreg8Spec>;
#[doc = "CPREG8"]
pub mod cpreg8;
#[doc = "CPREG9 (rw) register accessor: CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg9`]
module"]
#[doc(alias = "CPREG9")]
pub type Cpreg9 = crate::Reg<cpreg9::Cpreg9Spec>;
#[doc = "CPREG9"]
pub mod cpreg9;
#[doc = "CPREG10 (rw) register accessor: CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg10`]
module"]
#[doc(alias = "CPREG10")]
pub type Cpreg10 = crate::Reg<cpreg10::Cpreg10Spec>;
#[doc = "CPREG10"]
pub mod cpreg10;
#[doc = "CPREG11 (rw) register accessor: CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg11`]
module"]
#[doc(alias = "CPREG11")]
pub type Cpreg11 = crate::Reg<cpreg11::Cpreg11Spec>;
#[doc = "CPREG11"]
pub mod cpreg11;
#[doc = "CPREG12 (rw) register accessor: CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg12`]
module"]
#[doc(alias = "CPREG12")]
pub type Cpreg12 = crate::Reg<cpreg12::Cpreg12Spec>;
#[doc = "CPREG12"]
pub mod cpreg12;
#[doc = "CPREG13 (rw) register accessor: CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg13`]
module"]
#[doc(alias = "CPREG13")]
pub type Cpreg13 = crate::Reg<cpreg13::Cpreg13Spec>;
#[doc = "CPREG13"]
pub mod cpreg13;
#[doc = "CPREG14 (rw) register accessor: CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg14`]
module"]
#[doc(alias = "CPREG14")]
pub type Cpreg14 = crate::Reg<cpreg14::Cpreg14Spec>;
#[doc = "CPREG14"]
pub mod cpreg14;
#[doc = "CPREG15 (rw) register accessor: CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpreg15`]
module"]
#[doc(alias = "CPREG15")]
pub type Cpreg15 = crate::Reg<cpreg15::Cpreg15Spec>;
#[doc = "CPREG15"]
pub mod cpreg15;
#[doc = "CH0CPREG0 (rw) register accessor: CH0CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg0`]
module"]
#[doc(alias = "CH0CPREG0")]
pub type Ch0cpreg0 = crate::Reg<ch0cpreg0::Ch0cpreg0Spec>;
#[doc = "CH0CPREG0"]
pub mod ch0cpreg0;
#[doc = "CH0CPREG1 (rw) register accessor: CH0CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg1`]
module"]
#[doc(alias = "CH0CPREG1")]
pub type Ch0cpreg1 = crate::Reg<ch0cpreg1::Ch0cpreg1Spec>;
#[doc = "CH0CPREG1"]
pub mod ch0cpreg1;
#[doc = "CH0CPREG2 (rw) register accessor: CH0CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg2`]
module"]
#[doc(alias = "CH0CPREG2")]
pub type Ch0cpreg2 = crate::Reg<ch0cpreg2::Ch0cpreg2Spec>;
#[doc = "CH0CPREG2"]
pub mod ch0cpreg2;
#[doc = "CH0CPREG3 (rw) register accessor: CH0CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg3`]
module"]
#[doc(alias = "CH0CPREG3")]
pub type Ch0cpreg3 = crate::Reg<ch0cpreg3::Ch0cpreg3Spec>;
#[doc = "CH0CPREG3"]
pub mod ch0cpreg3;
#[doc = "CH0CPREG4 (rw) register accessor: CH0CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg4`]
module"]
#[doc(alias = "CH0CPREG4")]
pub type Ch0cpreg4 = crate::Reg<ch0cpreg4::Ch0cpreg4Spec>;
#[doc = "CH0CPREG4"]
pub mod ch0cpreg4;
#[doc = "CH0CPREG5 (rw) register accessor: CH0CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg5`]
module"]
#[doc(alias = "CH0CPREG5")]
pub type Ch0cpreg5 = crate::Reg<ch0cpreg5::Ch0cpreg5Spec>;
#[doc = "CH0CPREG5"]
pub mod ch0cpreg5;
#[doc = "CH0CPREG6 (rw) register accessor: CH0CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg6`]
module"]
#[doc(alias = "CH0CPREG6")]
pub type Ch0cpreg6 = crate::Reg<ch0cpreg6::Ch0cpreg6Spec>;
#[doc = "CH0CPREG6"]
pub mod ch0cpreg6;
#[doc = "CH0CPREG7 (rw) register accessor: CH0CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg7`]
module"]
#[doc(alias = "CH0CPREG7")]
pub type Ch0cpreg7 = crate::Reg<ch0cpreg7::Ch0cpreg7Spec>;
#[doc = "CH0CPREG7"]
pub mod ch0cpreg7;
#[doc = "CH0CPREG8 (rw) register accessor: CH0CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg8`]
module"]
#[doc(alias = "CH0CPREG8")]
pub type Ch0cpreg8 = crate::Reg<ch0cpreg8::Ch0cpreg8Spec>;
#[doc = "CH0CPREG8"]
pub mod ch0cpreg8;
#[doc = "CH0CPREG9 (rw) register accessor: CH0CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg9`]
module"]
#[doc(alias = "CH0CPREG9")]
pub type Ch0cpreg9 = crate::Reg<ch0cpreg9::Ch0cpreg9Spec>;
#[doc = "CH0CPREG9"]
pub mod ch0cpreg9;
#[doc = "CH0CPREG10 (rw) register accessor: CH0CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg10`]
module"]
#[doc(alias = "CH0CPREG10")]
pub type Ch0cpreg10 = crate::Reg<ch0cpreg10::Ch0cpreg10Spec>;
#[doc = "CH0CPREG10"]
pub mod ch0cpreg10;
#[doc = "CH0CPREG11 (rw) register accessor: CH0CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg11`]
module"]
#[doc(alias = "CH0CPREG11")]
pub type Ch0cpreg11 = crate::Reg<ch0cpreg11::Ch0cpreg11Spec>;
#[doc = "CH0CPREG11"]
pub mod ch0cpreg11;
#[doc = "CH0CPREG12 (rw) register accessor: CH0CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg12`]
module"]
#[doc(alias = "CH0CPREG12")]
pub type Ch0cpreg12 = crate::Reg<ch0cpreg12::Ch0cpreg12Spec>;
#[doc = "CH0CPREG12"]
pub mod ch0cpreg12;
#[doc = "CH0CPREG13 (rw) register accessor: CH0CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg13`]
module"]
#[doc(alias = "CH0CPREG13")]
pub type Ch0cpreg13 = crate::Reg<ch0cpreg13::Ch0cpreg13Spec>;
#[doc = "CH0CPREG13"]
pub mod ch0cpreg13;
#[doc = "CH0CPREG14 (rw) register accessor: CH0CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg14`]
module"]
#[doc(alias = "CH0CPREG14")]
pub type Ch0cpreg14 = crate::Reg<ch0cpreg14::Ch0cpreg14Spec>;
#[doc = "CH0CPREG14"]
pub mod ch0cpreg14;
#[doc = "CH0CPREG15 (rw) register accessor: CH0CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cpreg15`]
module"]
#[doc(alias = "CH0CPREG15")]
pub type Ch0cpreg15 = crate::Reg<ch0cpreg15::Ch0cpreg15Spec>;
#[doc = "CH0CPREG15"]
pub mod ch0cpreg15;
#[doc = "CH1CPREG0 (rw) register accessor: CH1CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg0`]
module"]
#[doc(alias = "CH1CPREG0")]
pub type Ch1cpreg0 = crate::Reg<ch1cpreg0::Ch1cpreg0Spec>;
#[doc = "CH1CPREG0"]
pub mod ch1cpreg0;
#[doc = "CH1CPREG1 (rw) register accessor: CH1CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg1`]
module"]
#[doc(alias = "CH1CPREG1")]
pub type Ch1cpreg1 = crate::Reg<ch1cpreg1::Ch1cpreg1Spec>;
#[doc = "CH1CPREG1"]
pub mod ch1cpreg1;
#[doc = "CH1CPREG2 (rw) register accessor: CH1CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg2`]
module"]
#[doc(alias = "CH1CPREG2")]
pub type Ch1cpreg2 = crate::Reg<ch1cpreg2::Ch1cpreg2Spec>;
#[doc = "CH1CPREG2"]
pub mod ch1cpreg2;
#[doc = "CH1CPREG3 (rw) register accessor: CH1CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg3`]
module"]
#[doc(alias = "CH1CPREG3")]
pub type Ch1cpreg3 = crate::Reg<ch1cpreg3::Ch1cpreg3Spec>;
#[doc = "CH1CPREG3"]
pub mod ch1cpreg3;
#[doc = "CH1CPREG4 (rw) register accessor: CH1CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg4`]
module"]
#[doc(alias = "CH1CPREG4")]
pub type Ch1cpreg4 = crate::Reg<ch1cpreg4::Ch1cpreg4Spec>;
#[doc = "CH1CPREG4"]
pub mod ch1cpreg4;
#[doc = "CH1CPREG5 (rw) register accessor: CH1CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg5`]
module"]
#[doc(alias = "CH1CPREG5")]
pub type Ch1cpreg5 = crate::Reg<ch1cpreg5::Ch1cpreg5Spec>;
#[doc = "CH1CPREG5"]
pub mod ch1cpreg5;
#[doc = "CH1CPREG6 (rw) register accessor: CH1CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg6`]
module"]
#[doc(alias = "CH1CPREG6")]
pub type Ch1cpreg6 = crate::Reg<ch1cpreg6::Ch1cpreg6Spec>;
#[doc = "CH1CPREG6"]
pub mod ch1cpreg6;
#[doc = "CH1CPREG7 (rw) register accessor: CH1CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg7`]
module"]
#[doc(alias = "CH1CPREG7")]
pub type Ch1cpreg7 = crate::Reg<ch1cpreg7::Ch1cpreg7Spec>;
#[doc = "CH1CPREG7"]
pub mod ch1cpreg7;
#[doc = "CH1CPREG8 (rw) register accessor: CH1CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg8`]
module"]
#[doc(alias = "CH1CPREG8")]
pub type Ch1cpreg8 = crate::Reg<ch1cpreg8::Ch1cpreg8Spec>;
#[doc = "CH1CPREG8"]
pub mod ch1cpreg8;
#[doc = "CH1CPREG9 (rw) register accessor: CH1CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg9`]
module"]
#[doc(alias = "CH1CPREG9")]
pub type Ch1cpreg9 = crate::Reg<ch1cpreg9::Ch1cpreg9Spec>;
#[doc = "CH1CPREG9"]
pub mod ch1cpreg9;
#[doc = "CH1CPREG10 (rw) register accessor: CH1CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg10`]
module"]
#[doc(alias = "CH1CPREG10")]
pub type Ch1cpreg10 = crate::Reg<ch1cpreg10::Ch1cpreg10Spec>;
#[doc = "CH1CPREG10"]
pub mod ch1cpreg10;
#[doc = "CH1CPREG11 (rw) register accessor: CH1CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg11`]
module"]
#[doc(alias = "CH1CPREG11")]
pub type Ch1cpreg11 = crate::Reg<ch1cpreg11::Ch1cpreg11Spec>;
#[doc = "CH1CPREG11"]
pub mod ch1cpreg11;
#[doc = "CH1CPREG12 (rw) register accessor: CH1CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg12`]
module"]
#[doc(alias = "CH1CPREG12")]
pub type Ch1cpreg12 = crate::Reg<ch1cpreg12::Ch1cpreg12Spec>;
#[doc = "CH1CPREG12"]
pub mod ch1cpreg12;
#[doc = "CH1CPREG13 (rw) register accessor: CH1CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg13`]
module"]
#[doc(alias = "CH1CPREG13")]
pub type Ch1cpreg13 = crate::Reg<ch1cpreg13::Ch1cpreg13Spec>;
#[doc = "CH1CPREG13"]
pub mod ch1cpreg13;
#[doc = "CH1CPREG14 (rw) register accessor: CH1CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg14`]
module"]
#[doc(alias = "CH1CPREG14")]
pub type Ch1cpreg14 = crate::Reg<ch1cpreg14::Ch1cpreg14Spec>;
#[doc = "CH1CPREG14"]
pub mod ch1cpreg14;
#[doc = "CH1CPREG15 (rw) register accessor: CH1CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cpreg15`]
module"]
#[doc(alias = "CH1CPREG15")]
pub type Ch1cpreg15 = crate::Reg<ch1cpreg15::Ch1cpreg15Spec>;
#[doc = "CH1CPREG15"]
pub mod ch1cpreg15;
#[doc = "CH2CPREG0 (rw) register accessor: CH2CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg0`]
module"]
#[doc(alias = "CH2CPREG0")]
pub type Ch2cpreg0 = crate::Reg<ch2cpreg0::Ch2cpreg0Spec>;
#[doc = "CH2CPREG0"]
pub mod ch2cpreg0;
#[doc = "CH2CPREG1 (rw) register accessor: CH2CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg1`]
module"]
#[doc(alias = "CH2CPREG1")]
pub type Ch2cpreg1 = crate::Reg<ch2cpreg1::Ch2cpreg1Spec>;
#[doc = "CH2CPREG1"]
pub mod ch2cpreg1;
#[doc = "CH2CPREG2 (rw) register accessor: CH2CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg2`]
module"]
#[doc(alias = "CH2CPREG2")]
pub type Ch2cpreg2 = crate::Reg<ch2cpreg2::Ch2cpreg2Spec>;
#[doc = "CH2CPREG2"]
pub mod ch2cpreg2;
#[doc = "CH2CPREG3 (rw) register accessor: CH2CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg3`]
module"]
#[doc(alias = "CH2CPREG3")]
pub type Ch2cpreg3 = crate::Reg<ch2cpreg3::Ch2cpreg3Spec>;
#[doc = "CH2CPREG3"]
pub mod ch2cpreg3;
#[doc = "CH2CPREG4 (rw) register accessor: CH2CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg4`]
module"]
#[doc(alias = "CH2CPREG4")]
pub type Ch2cpreg4 = crate::Reg<ch2cpreg4::Ch2cpreg4Spec>;
#[doc = "CH2CPREG4"]
pub mod ch2cpreg4;
#[doc = "CH2CPREG5 (rw) register accessor: CH2CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg5`]
module"]
#[doc(alias = "CH2CPREG5")]
pub type Ch2cpreg5 = crate::Reg<ch2cpreg5::Ch2cpreg5Spec>;
#[doc = "CH2CPREG5"]
pub mod ch2cpreg5;
#[doc = "CH2CPREG6 (rw) register accessor: CH2CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg6`]
module"]
#[doc(alias = "CH2CPREG6")]
pub type Ch2cpreg6 = crate::Reg<ch2cpreg6::Ch2cpreg6Spec>;
#[doc = "CH2CPREG6"]
pub mod ch2cpreg6;
#[doc = "CH2CPREG7 (rw) register accessor: CH2CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg7`]
module"]
#[doc(alias = "CH2CPREG7")]
pub type Ch2cpreg7 = crate::Reg<ch2cpreg7::Ch2cpreg7Spec>;
#[doc = "CH2CPREG7"]
pub mod ch2cpreg7;
#[doc = "CH2CPREG8 (rw) register accessor: CH2CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg8`]
module"]
#[doc(alias = "CH2CPREG8")]
pub type Ch2cpreg8 = crate::Reg<ch2cpreg8::Ch2cpreg8Spec>;
#[doc = "CH2CPREG8"]
pub mod ch2cpreg8;
#[doc = "CH2CPREG9 (rw) register accessor: CH2CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg9`]
module"]
#[doc(alias = "CH2CPREG9")]
pub type Ch2cpreg9 = crate::Reg<ch2cpreg9::Ch2cpreg9Spec>;
#[doc = "CH2CPREG9"]
pub mod ch2cpreg9;
#[doc = "CH2CPREG10 (rw) register accessor: CH2CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg10`]
module"]
#[doc(alias = "CH2CPREG10")]
pub type Ch2cpreg10 = crate::Reg<ch2cpreg10::Ch2cpreg10Spec>;
#[doc = "CH2CPREG10"]
pub mod ch2cpreg10;
#[doc = "CH2CPREG11 (rw) register accessor: CH2CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg11`]
module"]
#[doc(alias = "CH2CPREG11")]
pub type Ch2cpreg11 = crate::Reg<ch2cpreg11::Ch2cpreg11Spec>;
#[doc = "CH2CPREG11"]
pub mod ch2cpreg11;
#[doc = "CH2CPREG12 (rw) register accessor: CH2CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg12`]
module"]
#[doc(alias = "CH2CPREG12")]
pub type Ch2cpreg12 = crate::Reg<ch2cpreg12::Ch2cpreg12Spec>;
#[doc = "CH2CPREG12"]
pub mod ch2cpreg12;
#[doc = "CH2CPREG13 (rw) register accessor: CH2CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg13`]
module"]
#[doc(alias = "CH2CPREG13")]
pub type Ch2cpreg13 = crate::Reg<ch2cpreg13::Ch2cpreg13Spec>;
#[doc = "CH2CPREG13"]
pub mod ch2cpreg13;
#[doc = "CH2CPREG14 (rw) register accessor: CH2CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg14`]
module"]
#[doc(alias = "CH2CPREG14")]
pub type Ch2cpreg14 = crate::Reg<ch2cpreg14::Ch2cpreg14Spec>;
#[doc = "CH2CPREG14"]
pub mod ch2cpreg14;
#[doc = "CH2CPREG15 (rw) register accessor: CH2CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cpreg15`]
module"]
#[doc(alias = "CH2CPREG15")]
pub type Ch2cpreg15 = crate::Reg<ch2cpreg15::Ch2cpreg15Spec>;
#[doc = "CH2CPREG15"]
pub mod ch2cpreg15;
#[doc = "CH3CPREG0 (rw) register accessor: CH3CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg0`]
module"]
#[doc(alias = "CH3CPREG0")]
pub type Ch3cpreg0 = crate::Reg<ch3cpreg0::Ch3cpreg0Spec>;
#[doc = "CH3CPREG0"]
pub mod ch3cpreg0;
#[doc = "CH3CPREG1 (rw) register accessor: CH3CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg1`]
module"]
#[doc(alias = "CH3CPREG1")]
pub type Ch3cpreg1 = crate::Reg<ch3cpreg1::Ch3cpreg1Spec>;
#[doc = "CH3CPREG1"]
pub mod ch3cpreg1;
#[doc = "CH3CPREG2 (rw) register accessor: CH3CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg2`]
module"]
#[doc(alias = "CH3CPREG2")]
pub type Ch3cpreg2 = crate::Reg<ch3cpreg2::Ch3cpreg2Spec>;
#[doc = "CH3CPREG2"]
pub mod ch3cpreg2;
#[doc = "CH3CPREG3 (rw) register accessor: CH3CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg3`]
module"]
#[doc(alias = "CH3CPREG3")]
pub type Ch3cpreg3 = crate::Reg<ch3cpreg3::Ch3cpreg3Spec>;
#[doc = "CH3CPREG3"]
pub mod ch3cpreg3;
#[doc = "CH3CPREG4 (rw) register accessor: CH3CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg4`]
module"]
#[doc(alias = "CH3CPREG4")]
pub type Ch3cpreg4 = crate::Reg<ch3cpreg4::Ch3cpreg4Spec>;
#[doc = "CH3CPREG4"]
pub mod ch3cpreg4;
#[doc = "CH3CPREG5 (rw) register accessor: CH3CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg5`]
module"]
#[doc(alias = "CH3CPREG5")]
pub type Ch3cpreg5 = crate::Reg<ch3cpreg5::Ch3cpreg5Spec>;
#[doc = "CH3CPREG5"]
pub mod ch3cpreg5;
#[doc = "CH3CPREG6 (rw) register accessor: CH3CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg6`]
module"]
#[doc(alias = "CH3CPREG6")]
pub type Ch3cpreg6 = crate::Reg<ch3cpreg6::Ch3cpreg6Spec>;
#[doc = "CH3CPREG6"]
pub mod ch3cpreg6;
#[doc = "CH3CPREG7 (rw) register accessor: CH3CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg7`]
module"]
#[doc(alias = "CH3CPREG7")]
pub type Ch3cpreg7 = crate::Reg<ch3cpreg7::Ch3cpreg7Spec>;
#[doc = "CH3CPREG7"]
pub mod ch3cpreg7;
#[doc = "CH3CPREG8 (rw) register accessor: CH3CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg8`]
module"]
#[doc(alias = "CH3CPREG8")]
pub type Ch3cpreg8 = crate::Reg<ch3cpreg8::Ch3cpreg8Spec>;
#[doc = "CH3CPREG8"]
pub mod ch3cpreg8;
#[doc = "CH3CPREG9 (rw) register accessor: CH3CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg9`]
module"]
#[doc(alias = "CH3CPREG9")]
pub type Ch3cpreg9 = crate::Reg<ch3cpreg9::Ch3cpreg9Spec>;
#[doc = "CH3CPREG9"]
pub mod ch3cpreg9;
#[doc = "CH3CPREG10 (rw) register accessor: CH3CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg10`]
module"]
#[doc(alias = "CH3CPREG10")]
pub type Ch3cpreg10 = crate::Reg<ch3cpreg10::Ch3cpreg10Spec>;
#[doc = "CH3CPREG10"]
pub mod ch3cpreg10;
#[doc = "CH3CPREG11 (rw) register accessor: CH3CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg11`]
module"]
#[doc(alias = "CH3CPREG11")]
pub type Ch3cpreg11 = crate::Reg<ch3cpreg11::Ch3cpreg11Spec>;
#[doc = "CH3CPREG11"]
pub mod ch3cpreg11;
#[doc = "CH3CPREG12 (rw) register accessor: CH3CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg12`]
module"]
#[doc(alias = "CH3CPREG12")]
pub type Ch3cpreg12 = crate::Reg<ch3cpreg12::Ch3cpreg12Spec>;
#[doc = "CH3CPREG12"]
pub mod ch3cpreg12;
#[doc = "CH3CPREG13 (rw) register accessor: CH3CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg13`]
module"]
#[doc(alias = "CH3CPREG13")]
pub type Ch3cpreg13 = crate::Reg<ch3cpreg13::Ch3cpreg13Spec>;
#[doc = "CH3CPREG13"]
pub mod ch3cpreg13;
#[doc = "CH3CPREG14 (rw) register accessor: CH3CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg14`]
module"]
#[doc(alias = "CH3CPREG14")]
pub type Ch3cpreg14 = crate::Reg<ch3cpreg14::Ch3cpreg14Spec>;
#[doc = "CH3CPREG14"]
pub mod ch3cpreg14;
#[doc = "CH3CPREG15 (rw) register accessor: CH3CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cpreg15`]
module"]
#[doc(alias = "CH3CPREG15")]
pub type Ch3cpreg15 = crate::Reg<ch3cpreg15::Ch3cpreg15Spec>;
#[doc = "CH3CPREG15"]
pub mod ch3cpreg15;
#[doc = "CH4CPREG0 (rw) register accessor: CH4CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg0`]
module"]
#[doc(alias = "CH4CPREG0")]
pub type Ch4cpreg0 = crate::Reg<ch4cpreg0::Ch4cpreg0Spec>;
#[doc = "CH4CPREG0"]
pub mod ch4cpreg0;
#[doc = "CH4CPREG1 (rw) register accessor: CH4CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg1`]
module"]
#[doc(alias = "CH4CPREG1")]
pub type Ch4cpreg1 = crate::Reg<ch4cpreg1::Ch4cpreg1Spec>;
#[doc = "CH4CPREG1"]
pub mod ch4cpreg1;
#[doc = "CH4CPREG2 (rw) register accessor: CH4CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg2`]
module"]
#[doc(alias = "CH4CPREG2")]
pub type Ch4cpreg2 = crate::Reg<ch4cpreg2::Ch4cpreg2Spec>;
#[doc = "CH4CPREG2"]
pub mod ch4cpreg2;
#[doc = "CH4CPREG3 (rw) register accessor: CH4CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg3`]
module"]
#[doc(alias = "CH4CPREG3")]
pub type Ch4cpreg3 = crate::Reg<ch4cpreg3::Ch4cpreg3Spec>;
#[doc = "CH4CPREG3"]
pub mod ch4cpreg3;
#[doc = "CH4CPREG4 (rw) register accessor: CH4CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg4`]
module"]
#[doc(alias = "CH4CPREG4")]
pub type Ch4cpreg4 = crate::Reg<ch4cpreg4::Ch4cpreg4Spec>;
#[doc = "CH4CPREG4"]
pub mod ch4cpreg4;
#[doc = "CH4CPREG5 (rw) register accessor: CH4CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg5`]
module"]
#[doc(alias = "CH4CPREG5")]
pub type Ch4cpreg5 = crate::Reg<ch4cpreg5::Ch4cpreg5Spec>;
#[doc = "CH4CPREG5"]
pub mod ch4cpreg5;
#[doc = "CH4CPREG6 (rw) register accessor: CH4CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg6`]
module"]
#[doc(alias = "CH4CPREG6")]
pub type Ch4cpreg6 = crate::Reg<ch4cpreg6::Ch4cpreg6Spec>;
#[doc = "CH4CPREG6"]
pub mod ch4cpreg6;
#[doc = "CH4CPREG7 (rw) register accessor: CH4CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg7`]
module"]
#[doc(alias = "CH4CPREG7")]
pub type Ch4cpreg7 = crate::Reg<ch4cpreg7::Ch4cpreg7Spec>;
#[doc = "CH4CPREG7"]
pub mod ch4cpreg7;
#[doc = "CH4CPREG8 (rw) register accessor: CH4CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg8`]
module"]
#[doc(alias = "CH4CPREG8")]
pub type Ch4cpreg8 = crate::Reg<ch4cpreg8::Ch4cpreg8Spec>;
#[doc = "CH4CPREG8"]
pub mod ch4cpreg8;
#[doc = "CH4CPREG9 (rw) register accessor: CH4CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg9`]
module"]
#[doc(alias = "CH4CPREG9")]
pub type Ch4cpreg9 = crate::Reg<ch4cpreg9::Ch4cpreg9Spec>;
#[doc = "CH4CPREG9"]
pub mod ch4cpreg9;
#[doc = "CH4CPREG10 (rw) register accessor: CH4CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg10`]
module"]
#[doc(alias = "CH4CPREG10")]
pub type Ch4cpreg10 = crate::Reg<ch4cpreg10::Ch4cpreg10Spec>;
#[doc = "CH4CPREG10"]
pub mod ch4cpreg10;
#[doc = "CH4CPREG11 (rw) register accessor: CH4CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg11`]
module"]
#[doc(alias = "CH4CPREG11")]
pub type Ch4cpreg11 = crate::Reg<ch4cpreg11::Ch4cpreg11Spec>;
#[doc = "CH4CPREG11"]
pub mod ch4cpreg11;
#[doc = "CH4CPREG12 (rw) register accessor: CH4CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg12`]
module"]
#[doc(alias = "CH4CPREG12")]
pub type Ch4cpreg12 = crate::Reg<ch4cpreg12::Ch4cpreg12Spec>;
#[doc = "CH4CPREG12"]
pub mod ch4cpreg12;
#[doc = "CH4CPREG13 (rw) register accessor: CH4CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg13`]
module"]
#[doc(alias = "CH4CPREG13")]
pub type Ch4cpreg13 = crate::Reg<ch4cpreg13::Ch4cpreg13Spec>;
#[doc = "CH4CPREG13"]
pub mod ch4cpreg13;
#[doc = "CH4CPREG14 (rw) register accessor: CH4CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg14`]
module"]
#[doc(alias = "CH4CPREG14")]
pub type Ch4cpreg14 = crate::Reg<ch4cpreg14::Ch4cpreg14Spec>;
#[doc = "CH4CPREG14"]
pub mod ch4cpreg14;
#[doc = "CH4CPREG15 (rw) register accessor: CH4CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cpreg15`]
module"]
#[doc(alias = "CH4CPREG15")]
pub type Ch4cpreg15 = crate::Reg<ch4cpreg15::Ch4cpreg15Spec>;
#[doc = "CH4CPREG15"]
pub mod ch4cpreg15;
#[doc = "CH5CPREG0 (rw) register accessor: CH5CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg0`]
module"]
#[doc(alias = "CH5CPREG0")]
pub type Ch5cpreg0 = crate::Reg<ch5cpreg0::Ch5cpreg0Spec>;
#[doc = "CH5CPREG0"]
pub mod ch5cpreg0;
#[doc = "CH5CPREG1 (rw) register accessor: CH5CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg1`]
module"]
#[doc(alias = "CH5CPREG1")]
pub type Ch5cpreg1 = crate::Reg<ch5cpreg1::Ch5cpreg1Spec>;
#[doc = "CH5CPREG1"]
pub mod ch5cpreg1;
#[doc = "CH5CPREG2 (rw) register accessor: CH5CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg2`]
module"]
#[doc(alias = "CH5CPREG2")]
pub type Ch5cpreg2 = crate::Reg<ch5cpreg2::Ch5cpreg2Spec>;
#[doc = "CH5CPREG2"]
pub mod ch5cpreg2;
#[doc = "CH5CPREG3 (rw) register accessor: CH5CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg3`]
module"]
#[doc(alias = "CH5CPREG3")]
pub type Ch5cpreg3 = crate::Reg<ch5cpreg3::Ch5cpreg3Spec>;
#[doc = "CH5CPREG3"]
pub mod ch5cpreg3;
#[doc = "CH5CPREG4 (rw) register accessor: CH5CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg4`]
module"]
#[doc(alias = "CH5CPREG4")]
pub type Ch5cpreg4 = crate::Reg<ch5cpreg4::Ch5cpreg4Spec>;
#[doc = "CH5CPREG4"]
pub mod ch5cpreg4;
#[doc = "CH5CPREG5 (rw) register accessor: CH5CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg5`]
module"]
#[doc(alias = "CH5CPREG5")]
pub type Ch5cpreg5 = crate::Reg<ch5cpreg5::Ch5cpreg5Spec>;
#[doc = "CH5CPREG5"]
pub mod ch5cpreg5;
#[doc = "CH5CPREG6 (rw) register accessor: CH5CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg6`]
module"]
#[doc(alias = "CH5CPREG6")]
pub type Ch5cpreg6 = crate::Reg<ch5cpreg6::Ch5cpreg6Spec>;
#[doc = "CH5CPREG6"]
pub mod ch5cpreg6;
#[doc = "CH5CPREG7 (rw) register accessor: CH5CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg7`]
module"]
#[doc(alias = "CH5CPREG7")]
pub type Ch5cpreg7 = crate::Reg<ch5cpreg7::Ch5cpreg7Spec>;
#[doc = "CH5CPREG7"]
pub mod ch5cpreg7;
#[doc = "CH5CPREG8 (rw) register accessor: CH5CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg8`]
module"]
#[doc(alias = "CH5CPREG8")]
pub type Ch5cpreg8 = crate::Reg<ch5cpreg8::Ch5cpreg8Spec>;
#[doc = "CH5CPREG8"]
pub mod ch5cpreg8;
#[doc = "CH5CPREG9 (rw) register accessor: CH5CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg9`]
module"]
#[doc(alias = "CH5CPREG9")]
pub type Ch5cpreg9 = crate::Reg<ch5cpreg9::Ch5cpreg9Spec>;
#[doc = "CH5CPREG9"]
pub mod ch5cpreg9;
#[doc = "CH5CPREG10 (rw) register accessor: CH5CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg10`]
module"]
#[doc(alias = "CH5CPREG10")]
pub type Ch5cpreg10 = crate::Reg<ch5cpreg10::Ch5cpreg10Spec>;
#[doc = "CH5CPREG10"]
pub mod ch5cpreg10;
#[doc = "CH5CPREG11 (rw) register accessor: CH5CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg11`]
module"]
#[doc(alias = "CH5CPREG11")]
pub type Ch5cpreg11 = crate::Reg<ch5cpreg11::Ch5cpreg11Spec>;
#[doc = "CH5CPREG11"]
pub mod ch5cpreg11;
#[doc = "CH5CPREG12 (rw) register accessor: CH5CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg12`]
module"]
#[doc(alias = "CH5CPREG12")]
pub type Ch5cpreg12 = crate::Reg<ch5cpreg12::Ch5cpreg12Spec>;
#[doc = "CH5CPREG12"]
pub mod ch5cpreg12;
#[doc = "CH5CPREG13 (rw) register accessor: CH5CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg13`]
module"]
#[doc(alias = "CH5CPREG13")]
pub type Ch5cpreg13 = crate::Reg<ch5cpreg13::Ch5cpreg13Spec>;
#[doc = "CH5CPREG13"]
pub mod ch5cpreg13;
#[doc = "CH5CPREG14 (rw) register accessor: CH5CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg14`]
module"]
#[doc(alias = "CH5CPREG14")]
pub type Ch5cpreg14 = crate::Reg<ch5cpreg14::Ch5cpreg14Spec>;
#[doc = "CH5CPREG14"]
pub mod ch5cpreg14;
#[doc = "CH5CPREG15 (rw) register accessor: CH5CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cpreg15`]
module"]
#[doc(alias = "CH5CPREG15")]
pub type Ch5cpreg15 = crate::Reg<ch5cpreg15::Ch5cpreg15Spec>;
#[doc = "CH5CPREG15"]
pub mod ch5cpreg15;
#[doc = "CH6CPREG0 (rw) register accessor: CH6CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg0`]
module"]
#[doc(alias = "CH6CPREG0")]
pub type Ch6cpreg0 = crate::Reg<ch6cpreg0::Ch6cpreg0Spec>;
#[doc = "CH6CPREG0"]
pub mod ch6cpreg0;
#[doc = "CH6CPREG1 (rw) register accessor: CH6CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg1`]
module"]
#[doc(alias = "CH6CPREG1")]
pub type Ch6cpreg1 = crate::Reg<ch6cpreg1::Ch6cpreg1Spec>;
#[doc = "CH6CPREG1"]
pub mod ch6cpreg1;
#[doc = "CH6CPREG2 (rw) register accessor: CH6CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg2`]
module"]
#[doc(alias = "CH6CPREG2")]
pub type Ch6cpreg2 = crate::Reg<ch6cpreg2::Ch6cpreg2Spec>;
#[doc = "CH6CPREG2"]
pub mod ch6cpreg2;
#[doc = "CH6CPREG3 (rw) register accessor: CH6CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg3`]
module"]
#[doc(alias = "CH6CPREG3")]
pub type Ch6cpreg3 = crate::Reg<ch6cpreg3::Ch6cpreg3Spec>;
#[doc = "CH6CPREG3"]
pub mod ch6cpreg3;
#[doc = "CH6CPREG4 (rw) register accessor: CH6CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg4`]
module"]
#[doc(alias = "CH6CPREG4")]
pub type Ch6cpreg4 = crate::Reg<ch6cpreg4::Ch6cpreg4Spec>;
#[doc = "CH6CPREG4"]
pub mod ch6cpreg4;
#[doc = "CH6CPREG5 (rw) register accessor: CH6CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg5`]
module"]
#[doc(alias = "CH6CPREG5")]
pub type Ch6cpreg5 = crate::Reg<ch6cpreg5::Ch6cpreg5Spec>;
#[doc = "CH6CPREG5"]
pub mod ch6cpreg5;
#[doc = "CH6CPREG6 (rw) register accessor: CH6CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg6`]
module"]
#[doc(alias = "CH6CPREG6")]
pub type Ch6cpreg6 = crate::Reg<ch6cpreg6::Ch6cpreg6Spec>;
#[doc = "CH6CPREG6"]
pub mod ch6cpreg6;
#[doc = "CH6CPREG7 (rw) register accessor: CH6CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg7`]
module"]
#[doc(alias = "CH6CPREG7")]
pub type Ch6cpreg7 = crate::Reg<ch6cpreg7::Ch6cpreg7Spec>;
#[doc = "CH6CPREG7"]
pub mod ch6cpreg7;
#[doc = "CH6CPREG8 (rw) register accessor: CH6CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg8`]
module"]
#[doc(alias = "CH6CPREG8")]
pub type Ch6cpreg8 = crate::Reg<ch6cpreg8::Ch6cpreg8Spec>;
#[doc = "CH6CPREG8"]
pub mod ch6cpreg8;
#[doc = "CH6CPREG9 (rw) register accessor: CH6CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg9`]
module"]
#[doc(alias = "CH6CPREG9")]
pub type Ch6cpreg9 = crate::Reg<ch6cpreg9::Ch6cpreg9Spec>;
#[doc = "CH6CPREG9"]
pub mod ch6cpreg9;
#[doc = "CH6CPREG10 (rw) register accessor: CH6CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg10`]
module"]
#[doc(alias = "CH6CPREG10")]
pub type Ch6cpreg10 = crate::Reg<ch6cpreg10::Ch6cpreg10Spec>;
#[doc = "CH6CPREG10"]
pub mod ch6cpreg10;
#[doc = "CH6CPREG11 (rw) register accessor: CH6CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg11`]
module"]
#[doc(alias = "CH6CPREG11")]
pub type Ch6cpreg11 = crate::Reg<ch6cpreg11::Ch6cpreg11Spec>;
#[doc = "CH6CPREG11"]
pub mod ch6cpreg11;
#[doc = "CH6CPREG12 (rw) register accessor: CH6CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg12`]
module"]
#[doc(alias = "CH6CPREG12")]
pub type Ch6cpreg12 = crate::Reg<ch6cpreg12::Ch6cpreg12Spec>;
#[doc = "CH6CPREG12"]
pub mod ch6cpreg12;
#[doc = "CH6CPREG13 (rw) register accessor: CH6CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg13`]
module"]
#[doc(alias = "CH6CPREG13")]
pub type Ch6cpreg13 = crate::Reg<ch6cpreg13::Ch6cpreg13Spec>;
#[doc = "CH6CPREG13"]
pub mod ch6cpreg13;
#[doc = "CH6CPREG14 (rw) register accessor: CH6CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg14`]
module"]
#[doc(alias = "CH6CPREG14")]
pub type Ch6cpreg14 = crate::Reg<ch6cpreg14::Ch6cpreg14Spec>;
#[doc = "CH6CPREG14"]
pub mod ch6cpreg14;
#[doc = "CH6CPREG15 (rw) register accessor: CH6CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cpreg15`]
module"]
#[doc(alias = "CH6CPREG15")]
pub type Ch6cpreg15 = crate::Reg<ch6cpreg15::Ch6cpreg15Spec>;
#[doc = "CH6CPREG15"]
pub mod ch6cpreg15;
#[doc = "CH7CPREG0 (rw) register accessor: CH7CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg0`]
module"]
#[doc(alias = "CH7CPREG0")]
pub type Ch7cpreg0 = crate::Reg<ch7cpreg0::Ch7cpreg0Spec>;
#[doc = "CH7CPREG0"]
pub mod ch7cpreg0;
#[doc = "CH7CPREG1 (rw) register accessor: CH7CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg1`]
module"]
#[doc(alias = "CH7CPREG1")]
pub type Ch7cpreg1 = crate::Reg<ch7cpreg1::Ch7cpreg1Spec>;
#[doc = "CH7CPREG1"]
pub mod ch7cpreg1;
#[doc = "CH7CPREG2 (rw) register accessor: CH7CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg2`]
module"]
#[doc(alias = "CH7CPREG2")]
pub type Ch7cpreg2 = crate::Reg<ch7cpreg2::Ch7cpreg2Spec>;
#[doc = "CH7CPREG2"]
pub mod ch7cpreg2;
#[doc = "CH7CPREG3 (rw) register accessor: CH7CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg3`]
module"]
#[doc(alias = "CH7CPREG3")]
pub type Ch7cpreg3 = crate::Reg<ch7cpreg3::Ch7cpreg3Spec>;
#[doc = "CH7CPREG3"]
pub mod ch7cpreg3;
#[doc = "CH7CPREG4 (rw) register accessor: CH7CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg4`]
module"]
#[doc(alias = "CH7CPREG4")]
pub type Ch7cpreg4 = crate::Reg<ch7cpreg4::Ch7cpreg4Spec>;
#[doc = "CH7CPREG4"]
pub mod ch7cpreg4;
#[doc = "CH7CPREG5 (rw) register accessor: CH7CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg5`]
module"]
#[doc(alias = "CH7CPREG5")]
pub type Ch7cpreg5 = crate::Reg<ch7cpreg5::Ch7cpreg5Spec>;
#[doc = "CH7CPREG5"]
pub mod ch7cpreg5;
#[doc = "CH7CPREG6 (rw) register accessor: CH7CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg6`]
module"]
#[doc(alias = "CH7CPREG6")]
pub type Ch7cpreg6 = crate::Reg<ch7cpreg6::Ch7cpreg6Spec>;
#[doc = "CH7CPREG6"]
pub mod ch7cpreg6;
#[doc = "CH7CPREG7 (rw) register accessor: CH7CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg7`]
module"]
#[doc(alias = "CH7CPREG7")]
pub type Ch7cpreg7 = crate::Reg<ch7cpreg7::Ch7cpreg7Spec>;
#[doc = "CH7CPREG7"]
pub mod ch7cpreg7;
#[doc = "CH7CPREG8 (rw) register accessor: CH7CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg8`]
module"]
#[doc(alias = "CH7CPREG8")]
pub type Ch7cpreg8 = crate::Reg<ch7cpreg8::Ch7cpreg8Spec>;
#[doc = "CH7CPREG8"]
pub mod ch7cpreg8;
#[doc = "CH7CPREG9 (rw) register accessor: CH7CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg9`]
module"]
#[doc(alias = "CH7CPREG9")]
pub type Ch7cpreg9 = crate::Reg<ch7cpreg9::Ch7cpreg9Spec>;
#[doc = "CH7CPREG9"]
pub mod ch7cpreg9;
#[doc = "CH7CPREG10 (rw) register accessor: CH7CPREG10\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg10`]
module"]
#[doc(alias = "CH7CPREG10")]
pub type Ch7cpreg10 = crate::Reg<ch7cpreg10::Ch7cpreg10Spec>;
#[doc = "CH7CPREG10"]
pub mod ch7cpreg10;
#[doc = "CH7CPREG11 (rw) register accessor: CH7CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg11`]
module"]
#[doc(alias = "CH7CPREG11")]
pub type Ch7cpreg11 = crate::Reg<ch7cpreg11::Ch7cpreg11Spec>;
#[doc = "CH7CPREG11"]
pub mod ch7cpreg11;
#[doc = "CH7CPREG12 (rw) register accessor: CH7CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg12`]
module"]
#[doc(alias = "CH7CPREG12")]
pub type Ch7cpreg12 = crate::Reg<ch7cpreg12::Ch7cpreg12Spec>;
#[doc = "CH7CPREG12"]
pub mod ch7cpreg12;
#[doc = "CH7CPREG13 (rw) register accessor: CH7CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg13`]
module"]
#[doc(alias = "CH7CPREG13")]
pub type Ch7cpreg13 = crate::Reg<ch7cpreg13::Ch7cpreg13Spec>;
#[doc = "CH7CPREG13"]
pub mod ch7cpreg13;
#[doc = "CH7CPREG14 (rw) register accessor: CH7CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg14`]
module"]
#[doc(alias = "CH7CPREG14")]
pub type Ch7cpreg14 = crate::Reg<ch7cpreg14::Ch7cpreg14Spec>;
#[doc = "CH7CPREG14"]
pub mod ch7cpreg14;
#[doc = "CH7CPREG15 (rw) register accessor: CH7CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cpreg15`]
module"]
#[doc(alias = "CH7CPREG15")]
pub type Ch7cpreg15 = crate::Reg<ch7cpreg15::Ch7cpreg15Spec>;
#[doc = "CH7CPREG15"]
pub mod ch7cpreg15;
#[doc = "CH01_HIL_CP_OVERRIDE (rw) register accessor: CH01_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch01_hil_cp_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch01_hil_cp_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch01_hil_cp_override`]
module"]
#[doc(alias = "CH01_HIL_CP_OVERRIDE")]
pub type Ch01HilCpOverride = crate::Reg<ch01_hil_cp_override::Ch01HilCpOverrideSpec>;
#[doc = "CH01_HIL_CP_OVERRIDE"]
pub mod ch01_hil_cp_override;
#[doc = "CH23_HIL_CP_OVERRIDE (rw) register accessor: CH23_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_hil_cp_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_hil_cp_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_hil_cp_override`]
module"]
#[doc(alias = "CH23_HIL_CP_OVERRIDE")]
pub type Ch23HilCpOverride = crate::Reg<ch23_hil_cp_override::Ch23HilCpOverrideSpec>;
#[doc = "CH23_HIL_CP_OVERRIDE"]
pub mod ch23_hil_cp_override;
#[doc = "CH45_HIL_CP_OVERRIDE (rw) register accessor: CH45_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch45_hil_cp_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch45_hil_cp_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch45_hil_cp_override`]
module"]
#[doc(alias = "CH45_HIL_CP_OVERRIDE")]
pub type Ch45HilCpOverride = crate::Reg<ch45_hil_cp_override::Ch45HilCpOverrideSpec>;
#[doc = "CH45_HIL_CP_OVERRIDE"]
pub mod ch45_hil_cp_override;
#[doc = "CH67_HIL_CP_OVERRIDE (rw) register accessor: CH67_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch67_hil_cp_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch67_hil_cp_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch67_hil_cp_override`]
module"]
#[doc(alias = "CH67_HIL_CP_OVERRIDE")]
pub type Ch67HilCpOverride = crate::Reg<ch67_hil_cp_override::Ch67HilCpOverrideSpec>;
#[doc = "CH67_HIL_CP_OVERRIDE"]
pub mod ch67_hil_cp_override;
#[doc = "CH_HIL_CP_OVERRIDE (rw) register accessor: CH_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_hil_cp_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_hil_cp_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_hil_cp_override`]
module"]
#[doc(alias = "CH_HIL_CP_OVERRIDE")]
pub type ChHilCpOverride = crate::Reg<ch_hil_cp_override::ChHilCpOverrideSpec>;
#[doc = "CH_HIL_CP_OVERRIDE"]
pub mod ch_hil_cp_override;
#[doc = "RSS_BOOKKEEPING_CTRL (rw) register accessor: RSS_BOOKKEEPING_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_bookkeeping_ctrl`]
module"]
#[doc(alias = "RSS_BOOKKEEPING_CTRL")]
pub type RssBookkeepingCtrl = crate::Reg<rss_bookkeeping_ctrl::RssBookkeepingCtrlSpec>;
#[doc = "RSS_BOOKKEEPING_CTRL"]
pub mod rss_bookkeeping_ctrl;
#[doc = "RSS_APP_GP (rw) register accessor: RSS_APP_GP\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_app_gp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_app_gp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_app_gp`]
module"]
#[doc(alias = "RSS_APP_GP")]
pub type RssAppGp = crate::Reg<rss_app_gp::RssAppGpSpec>;
#[doc = "RSS_APP_GP"]
pub mod rss_app_gp;
#[doc = "RSS_BOOKKEEPING_SEQ_NUM (rw) register accessor: RSS_BOOKKEEPING_SEQ_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_seq_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_seq_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_bookkeeping_seq_num`]
module"]
#[doc(alias = "RSS_BOOKKEEPING_SEQ_NUM")]
pub type RssBookkeepingSeqNum = crate::Reg<rss_bookkeeping_seq_num::RssBookkeepingSeqNumSpec>;
#[doc = "RSS_BOOKKEEPING_SEQ_NUM"]
pub mod rss_bookkeeping_seq_num;
#[doc = "RSS_BOOKKEEPING_FRM_CNT (rw) register accessor: RSS_BOOKKEEPING_FRM_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_frm_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_frm_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_bookkeeping_frm_cnt`]
module"]
#[doc(alias = "RSS_BOOKKEEPING_FRM_CNT")]
pub type RssBookkeepingFrmCnt = crate::Reg<rss_bookkeeping_frm_cnt::RssBookkeepingFrmCntSpec>;
#[doc = "RSS_BOOKKEEPING_FRM_CNT"]
pub mod rss_bookkeeping_frm_cnt;
#[doc = "RSS_BOOKKEEPING_CHRP_CNT (rw) register accessor: RSS_BOOKKEEPING_CHRP_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_chrp_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_chrp_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rss_bookkeeping_chrp_cnt`]
module"]
#[doc(alias = "RSS_BOOKKEEPING_CHRP_CNT")]
pub type RssBookkeepingChrpCnt = crate::Reg<rss_bookkeeping_chrp_cnt::RssBookkeepingChrpCntSpec>;
#[doc = "RSS_BOOKKEEPING_CHRP_CNT"]
pub mod rss_bookkeeping_chrp_cnt;
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
