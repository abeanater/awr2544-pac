#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    param_ram_idx: ParamRamIdx,
    param_ram_loop: ParamRamLoop,
    _reserved3: [u8; 0x08],
    previous_name: PreviousName,
    _reserved4: [u8; 0x04],
    fw2dma_trig: Fw2dmaTrig,
    dma2hwa_trig: Dma2hwaTrig,
    sigdmach0done: Sigdmach0done,
    sigdmach1done: Sigdmach1done,
    sigdmach2done: Sigdmach2done,
    sigdmach3done: Sigdmach3done,
    sigdmach4done: Sigdmach4done,
    sigdmach5done: Sigdmach5done,
    sigdmach6done: Sigdmach6done,
    sigdmach7done: Sigdmach7done,
    sigdmach8done: Sigdmach8done,
    sigdmach9done: Sigdmach9done,
    sigdmach10done: Sigdmach10done,
    sigdmach11done: Sigdmach11done,
    sigdmach12done: Sigdmach12done,
    sigdmach13done: Sigdmach13done,
    sigdmach14done: Sigdmach14done,
    sigdmach15done: Sigdmach15done,
    sigdmach16done: Sigdmach16done,
    sigdmach17done: Sigdmach17done,
    sigdmach18done: Sigdmach18done,
    sigdmach19done: Sigdmach19done,
    sigdmach20done: Sigdmach20done,
    sigdmach21done: Sigdmach21done,
    sigdmach22done: Sigdmach22done,
    sigdmach23done: Sigdmach23done,
    sigdmach24done: Sigdmach24done,
    sigdmach25done: Sigdmach25done,
    sigdmach26done: Sigdmach26done,
    sigdmach27done: Sigdmach27done,
    sigdmach28done: Sigdmach28done,
    sigdmach29done: Sigdmach29done,
    sigdmach30done: Sigdmach30done,
    sigdmach31done: Sigdmach31done,
    fw2hwa_trig_0: Fw2hwaTrig0,
    fw2hwa_trig_1: Fw2hwaTrig1,
    _reserved40: [u8; 0x04],
    bpm_pattern_0: BpmPattern0,
    bpm_pattern_1: BpmPattern1,
    bpm_pattern_2: BpmPattern2,
    bpm_pattern_3: BpmPattern3,
    bpm_pattern_4: BpmPattern4,
    bpm_pattern_5: BpmPattern5,
    bpm_pattern_6: BpmPattern6,
    bpm_pattern_7: BpmPattern7,
    bpm_rate: BpmRate,
    param_done_set_status_0: ParamDoneSetStatus0,
    param_done_set_status_1: ParamDoneSetStatus1,
    param_done_clr_0: ParamDoneClr0,
    param_done_clr_1: ParamDoneClr1,
    trigger_set_status_0: TriggerSetStatus0,
    trigger_set_status_1: TriggerSetStatus1,
    trigger_set_in_clr_0: TriggerSetInClr0,
    trigger_set_in_clr_1: TriggerSetInClr1,
    dc_est_reset_sw: DcEstResetSw,
    dc_est_ctrl: DcEstCtrl,
    dc_est_i_0_val: DcEstI0Val,
    dc_est_i_1_val: DcEstI1Val,
    dc_est_i_2_val: DcEstI2Val,
    dc_est_i_3_val: DcEstI3Val,
    dc_est_i_4_val: DcEstI4Val,
    dc_est_i_5_val: DcEstI5Val,
    dc_est_i_6_val: DcEstI6Val,
    dc_est_i_7_val: DcEstI7Val,
    _reserved67: [u8; 0x10],
    dc_est_q_0_val: DcEstQ0Val,
    dc_est_q_1_val: DcEstQ1Val,
    dc_est_q_2_val: DcEstQ2Val,
    dc_est_q_3_val: DcEstQ3Val,
    dc_est_q_4_val: DcEstQ4Val,
    dc_est_q_5_val: DcEstQ5Val,
    dc_est_q_6_val: DcEstQ6Val,
    dc_est_q_7_val: DcEstQ7Val,
    _reserved75: [u8; 0x10],
    dc_acc_i_0_val_lsb: DcAccI0ValLsb,
    dc_acc_i_0_val_msb: DcAccI0ValMsb,
    dc_acc_i_1_val_lsb: DcAccI1ValLsb,
    dc_acc_i_1_val_msb: DcAccI1ValMsb,
    dc_acc_i_2_val_lsb: DcAccI2ValLsb,
    dc_acc_i_2_val_msb: DcAccI2ValMsb,
    dc_acc_i_3_val_lsb: DcAccI3ValLsb,
    dc_acc_i_3_val_msb: DcAccI3ValMsb,
    dc_acc_i_4_val_lsb: DcAccI4ValLsb,
    dc_acc_i_4_val_msb: DcAccI4ValMsb,
    dc_acc_i_5_val_lsb: DcAccI5ValLsb,
    dc_acc_i_5_val_msb: DcAccI5ValMsb,
    dc_acc_i_6_val_lsb: DcAccI6ValLsb,
    dc_acc_i_6_val_msb: DcAccI6ValMsb,
    dc_acc_i_7_val_lsb: DcAccI7ValLsb,
    dc_acc_i_7_val_msb: DcAccI7ValMsb,
    _reserved91: [u8; 0x20],
    dc_acc_q_0_val_lsb: DcAccQ0ValLsb,
    dc_acc_q_0_val_msb: DcAccQ0ValMsb,
    dc_acc_q_1_val_lsb: DcAccQ1ValLsb,
    dc_acc_q_1_val_msb: DcAccQ1ValMsb,
    dc_acc_q_2_val_lsb: DcAccQ2ValLsb,
    dc_acc_q_2_val_msb: DcAccQ2ValMsb,
    dc_acc_q_3_val_lsb: DcAccQ3ValLsb,
    dc_acc_q_3_val_msb: DcAccQ3ValMsb,
    dc_acc_q_4_val_lsb: DcAccQ4ValLsb,
    dc_acc_q_4_val_msb: DcAccQ4ValMsb,
    dc_acc_q_5_val_lsb: DcAccQ5ValLsb,
    dc_acc_q_5_val_msb: DcAccQ5ValMsb,
    dc_acc_q_6_val_lsb: DcAccQ6ValLsb,
    dc_acc_q_6_val_msb: DcAccQ6ValMsb,
    dc_acc_q_7_val_lsb: DcAccQ7ValLsb,
    dc_acc_q_7_val_msb: DcAccQ7ValMsb,
    _reserved107: [u8; 0x20],
    dc_acc_clip_status: DcAccClipStatus,
    dc_est_clip_status: DcEstClipStatus,
    dc_i0_sw: DcI0Sw,
    dc_i1_sw: DcI1Sw,
    dc_i2_sw: DcI2Sw,
    dc_i3_sw: DcI3Sw,
    dc_i4_sw: DcI4Sw,
    dc_i5_sw: DcI5Sw,
    dc_i6_sw: DcI6Sw,
    dc_i7_sw: DcI7Sw,
    _reserved117: [u8; 0x10],
    dc_q0_sw: DcQ0Sw,
    dc_q1_sw: DcQ1Sw,
    dc_q2_sw: DcQ2Sw,
    dc_q3_sw: DcQ3Sw,
    dc_q4_sw: DcQ4Sw,
    dc_q5_sw: DcQ5Sw,
    dc_q6_sw: DcQ6Sw,
    dc_q7_sw: DcQ7Sw,
    _reserved125: [u8; 0x10],
    dc_sub_clip: DcSubClip,
    dc_est_ctrl_profile2: DcEstCtrlProfile2,
    dc_reserved_3: DcReserved3,
    dc_reserved_4: DcReserved4,
    dc_reserved_5: DcReserved5,
    intf_stats_reset_sw: IntfStatsResetSw,
    intf_stats_ctrl: IntfStatsCtrl,
    intf_loc_thresh_mag0_val: IntfLocThreshMag0Val,
    intf_loc_thresh_mag1_val: IntfLocThreshMag1Val,
    intf_loc_thresh_mag2_val: IntfLocThreshMag2Val,
    intf_loc_thresh_mag3_val: IntfLocThreshMag3Val,
    _reserved136: [u8; 0x20],
    intf_loc_thresh_magdiff0_val: IntfLocThreshMagdiff0Val,
    intf_loc_thresh_magdiff1_val: IntfLocThreshMagdiff1Val,
    intf_loc_thresh_magdiff2_val: IntfLocThreshMagdiff2Val,
    intf_loc_thresh_magdiff3_val: IntfLocThreshMagdiff3Val,
    _reserved140: [u8; 0x20],
    intf_loc_count_all_chirp: IntfLocCountAllChirp,
    intf_loc_count_all_frame: IntfLocCountAllFrame,
    intf_stats_mag_acc_0_lsb: IntfStatsMagAcc0Lsb,
    intf_stats_mag_acc_0_msb: IntfStatsMagAcc0Msb,
    intf_stats_mag_acc_1_lsb: IntfStatsMagAcc1Lsb,
    intf_stats_mag_acc_1_msb: IntfStatsMagAcc1Msb,
    intf_stats_mag_acc_2_lsb: IntfStatsMagAcc2Lsb,
    intf_stats_mag_acc_2_msb: IntfStatsMagAcc2Msb,
    intf_stats_mag_acc_3_lsb: IntfStatsMagAcc3Lsb,
    intf_stats_mag_acc_3_msb: IntfStatsMagAcc3Msb,
    _reserved150: [u8; 0x40],
    intf_stats_magdiff_acc_0_lsb: IntfStatsMagdiffAcc0Lsb,
    intf_stats_magdiff_acc_0_msb: IntfStatsMagdiffAcc0Msb,
    intf_stats_magdiff_acc_1_lsb: IntfStatsMagdiffAcc1Lsb,
    intf_stats_magdiff_acc_1_msb: IntfStatsMagdiffAcc1Msb,
    intf_stats_magdiff_acc_2_lsb: IntfStatsMagdiffAcc2Lsb,
    intf_stats_magdiff_acc_2_msb: IntfStatsMagdiffAcc2Msb,
    intf_stats_magdiff_acc_3_lsb: IntfStatsMagdiffAcc3Lsb,
    intf_stats_magdiff_acc_3_msb: IntfStatsMagdiffAcc3Msb,
    _reserved158: [u8; 0x40],
    intf_loc_thresh_mag0_sw: IntfLocThreshMag0Sw,
    intf_loc_thresh_mag1_sw: IntfLocThreshMag1Sw,
    intf_loc_thresh_mag2_sw: IntfLocThreshMag2Sw,
    intf_loc_thresh_mag3_sw: IntfLocThreshMag3Sw,
    _reserved162: [u8; 0x20],
    intf_loc_thresh_magdiff0_sw: IntfLocThreshMagdiff0Sw,
    intf_loc_thresh_magdiff1_sw: IntfLocThreshMagdiff1Sw,
    intf_loc_thresh_magdiff2_sw: IntfLocThreshMagdiff2Sw,
    intf_loc_thresh_magdiff3_sw: IntfLocThreshMagdiff3Sw,
    _reserved166: [u8; 0x20],
    intf_stats_acc_clip_status: IntfStatsAccClipStatus,
    intf_stats_thresh_clip_status: IntfStatsThreshClipStatus,
    intf_mitg_window_param_0: IntfMitgWindowParam0,
    intf_mitg_window_param_1: IntfMitgWindowParam1,
    intf_mitg_window_param_2: IntfMitgWindowParam2,
    intf_mitg_window_param_3: IntfMitgWindowParam3,
    intf_mitg_window_param_4: IntfMitgWindowParam4,
    intf_stats_sum_mag_val: IntfStatsSumMagVal,
    intf_stats_sum_mag_val_clip_status: IntfStatsSumMagValClipStatus,
    intf_stats_sum_magdiff_val: IntfStatsSumMagdiffVal,
    intf_stats_sum_magdiff_val_clip_status: IntfStatsSumMagdiffValClipStatus,
    interf_reserved_5: InterfReserved5,
    icmult_scale0: IcmultScale0,
    icmult_scale1: IcmultScale1,
    icmult_scale2: IcmultScale2,
    icmult_scale3: IcmultScale3,
    _reserved182: [u8; 0x20],
    qcmult_scale0: QcmultScale0,
    qcmult_scale1: QcmultScale1,
    qcmult_scale2: QcmultScale2,
    qcmult_scale3: QcmultScale3,
    _reserved186: [u8; 0x20],
    twid_incr_delta_frac: TwidIncrDeltaFrac,
    _reserved187: [u8; 0x04],
    twid_incr_delta_frac_reset_sw: TwidIncrDeltaFracResetSw,
    twid_incr_delta_frac_clip_status: TwidIncrDeltaFracClipStatus,
    _reserved189: [u8; 0x04],
    cmult_reserved_2: CmultReserved2,
    _reserved190: [u8; 0x5c],
    lfsr_seed: LfsrSeed,
    lfsr_load: LfsrLoad,
    dither_twid_en: DitherTwidEn,
    fft_clip: FftClip,
    clr_fftclip: ClrFftclip,
    clr_clip_misc: ClrClipMisc,
    ip_op_formatter_clip_status: IpOpFormatterClipStatus,
    fft_reserved_1: FftReserved1,
    fft_reserved_2: FftReserved2,
    fft_reserved_3: FftReserved3,
    _reserved200: [u8; 0xa4],
    cmp_ege_k0123: CmpEgeK0123,
    cmp_ege_k4567: CmpEgeK4567,
    mem_init_start: MemInitStart,
    mem_init_done: MemInitDone,
    mem_init_status: MemInitStatus,
    _reserved205: [u8; 0x08],
    hwa_safety_en: HwaSafetyEn,
    hwa_safety_err_mask: HwaSafetyErrMask,
    hwa_safety_err_status: HwaSafetyErrStatus,
    hwa_safety_err_status_raw: HwaSafetyErrStatusRaw,
    hwa_safety_dmem0_err_addr: HwaSafetyDmem0ErrAddr,
    hwa_safety_dmem1_err_addr: HwaSafetyDmem1ErrAddr,
    hwa_safety_dmem2_err_addr: HwaSafetyDmem2ErrAddr,
    hwa_safety_dmem3_err_addr: HwaSafetyDmem3ErrAddr,
    _reserved213: [u8; 0x10],
    hwa_safety_window_ram_err_addr: HwaSafetyWindowRamErrAddr,
    mem_access_err_status: MemAccessErrStatus,
    loop_cnt: LoopCnt,
    paramaddr: Paramaddr,
    paramaddr_cpuintr0: ParamaddrCpuintr0,
    _reserved218: [u8; 0x04],
    fsm_state: FsmState,
    single_step_en: SingleStepEn,
    single_step_trig: SingleStepTrig,
    _reserved221: [u8; 0x2c],
    reg_cmp_lfsrseed_0: RegCmpLfsrseed0,
    reg_cmp_lfsrseed_1: RegCmpLfsrseed1,
    reg_cmp_lfsrload_0: RegCmpLfsrload0,
    reg_cmp_lfsrload_1: RegCmpLfsrload1,
    _reserved225: [u8; 0x092c],
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
    _reserved235: [u8; 0x10],
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
    #[doc = "0x04 - PARAM_RAM_IDX"]
    #[inline(always)]
    pub const fn param_ram_idx(&self) -> &ParamRamIdx {
        &self.param_ram_idx
    }
    #[doc = "0x08 - PARAM_RAM_LOOP"]
    #[inline(always)]
    pub const fn param_ram_loop(&self) -> &ParamRamLoop {
        &self.param_ram_loop
    }
    #[doc = "0x14 - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x1c - FW2DMA_TRIG"]
    #[inline(always)]
    pub const fn fw2dma_trig(&self) -> &Fw2dmaTrig {
        &self.fw2dma_trig
    }
    #[doc = "0x20 - DMA2HWA_TRIG"]
    #[inline(always)]
    pub const fn dma2hwa_trig(&self) -> &Dma2hwaTrig {
        &self.dma2hwa_trig
    }
    #[doc = "0x24 - SIGDMACH0DONE"]
    #[inline(always)]
    pub const fn sigdmach0done(&self) -> &Sigdmach0done {
        &self.sigdmach0done
    }
    #[doc = "0x28 - SIGDMACH1DONE"]
    #[inline(always)]
    pub const fn sigdmach1done(&self) -> &Sigdmach1done {
        &self.sigdmach1done
    }
    #[doc = "0x2c - SIGDMACH2DONE"]
    #[inline(always)]
    pub const fn sigdmach2done(&self) -> &Sigdmach2done {
        &self.sigdmach2done
    }
    #[doc = "0x30 - SIGDMACH3DONE"]
    #[inline(always)]
    pub const fn sigdmach3done(&self) -> &Sigdmach3done {
        &self.sigdmach3done
    }
    #[doc = "0x34 - SIGDMACH4DONE"]
    #[inline(always)]
    pub const fn sigdmach4done(&self) -> &Sigdmach4done {
        &self.sigdmach4done
    }
    #[doc = "0x38 - SIGDMACH5DONE"]
    #[inline(always)]
    pub const fn sigdmach5done(&self) -> &Sigdmach5done {
        &self.sigdmach5done
    }
    #[doc = "0x3c - SIGDMACH6DONE"]
    #[inline(always)]
    pub const fn sigdmach6done(&self) -> &Sigdmach6done {
        &self.sigdmach6done
    }
    #[doc = "0x40 - SIGDMACH7DONE"]
    #[inline(always)]
    pub const fn sigdmach7done(&self) -> &Sigdmach7done {
        &self.sigdmach7done
    }
    #[doc = "0x44 - SIGDMACH8DONE"]
    #[inline(always)]
    pub const fn sigdmach8done(&self) -> &Sigdmach8done {
        &self.sigdmach8done
    }
    #[doc = "0x48 - SIGDMACH9DONE"]
    #[inline(always)]
    pub const fn sigdmach9done(&self) -> &Sigdmach9done {
        &self.sigdmach9done
    }
    #[doc = "0x4c - SIGDMACH10DONE"]
    #[inline(always)]
    pub const fn sigdmach10done(&self) -> &Sigdmach10done {
        &self.sigdmach10done
    }
    #[doc = "0x50 - SIGDMACH11DONE"]
    #[inline(always)]
    pub const fn sigdmach11done(&self) -> &Sigdmach11done {
        &self.sigdmach11done
    }
    #[doc = "0x54 - SIGDMACH12DONE"]
    #[inline(always)]
    pub const fn sigdmach12done(&self) -> &Sigdmach12done {
        &self.sigdmach12done
    }
    #[doc = "0x58 - SIGDMACH13DONE"]
    #[inline(always)]
    pub const fn sigdmach13done(&self) -> &Sigdmach13done {
        &self.sigdmach13done
    }
    #[doc = "0x5c - SIGDMACH14DONE"]
    #[inline(always)]
    pub const fn sigdmach14done(&self) -> &Sigdmach14done {
        &self.sigdmach14done
    }
    #[doc = "0x60 - SIGDMACH15DONE"]
    #[inline(always)]
    pub const fn sigdmach15done(&self) -> &Sigdmach15done {
        &self.sigdmach15done
    }
    #[doc = "0x64 - SIGDMACH16DONE"]
    #[inline(always)]
    pub const fn sigdmach16done(&self) -> &Sigdmach16done {
        &self.sigdmach16done
    }
    #[doc = "0x68 - SIGDMACH17DONE"]
    #[inline(always)]
    pub const fn sigdmach17done(&self) -> &Sigdmach17done {
        &self.sigdmach17done
    }
    #[doc = "0x6c - SIGDMACH18DONE"]
    #[inline(always)]
    pub const fn sigdmach18done(&self) -> &Sigdmach18done {
        &self.sigdmach18done
    }
    #[doc = "0x70 - SIGDMACH19DONE"]
    #[inline(always)]
    pub const fn sigdmach19done(&self) -> &Sigdmach19done {
        &self.sigdmach19done
    }
    #[doc = "0x74 - SIGDMACH20DONE"]
    #[inline(always)]
    pub const fn sigdmach20done(&self) -> &Sigdmach20done {
        &self.sigdmach20done
    }
    #[doc = "0x78 - SIGDMACH21DONE"]
    #[inline(always)]
    pub const fn sigdmach21done(&self) -> &Sigdmach21done {
        &self.sigdmach21done
    }
    #[doc = "0x7c - SIGDMACH22DONE"]
    #[inline(always)]
    pub const fn sigdmach22done(&self) -> &Sigdmach22done {
        &self.sigdmach22done
    }
    #[doc = "0x80 - SIGDMACH23DONE"]
    #[inline(always)]
    pub const fn sigdmach23done(&self) -> &Sigdmach23done {
        &self.sigdmach23done
    }
    #[doc = "0x84 - SIGDMACH24DONE"]
    #[inline(always)]
    pub const fn sigdmach24done(&self) -> &Sigdmach24done {
        &self.sigdmach24done
    }
    #[doc = "0x88 - SIGDMACH25DONE"]
    #[inline(always)]
    pub const fn sigdmach25done(&self) -> &Sigdmach25done {
        &self.sigdmach25done
    }
    #[doc = "0x8c - SIGDMACH26DONE"]
    #[inline(always)]
    pub const fn sigdmach26done(&self) -> &Sigdmach26done {
        &self.sigdmach26done
    }
    #[doc = "0x90 - SIGDMACH27DONE"]
    #[inline(always)]
    pub const fn sigdmach27done(&self) -> &Sigdmach27done {
        &self.sigdmach27done
    }
    #[doc = "0x94 - SIGDMACH28DONE"]
    #[inline(always)]
    pub const fn sigdmach28done(&self) -> &Sigdmach28done {
        &self.sigdmach28done
    }
    #[doc = "0x98 - SIGDMACH29DONE"]
    #[inline(always)]
    pub const fn sigdmach29done(&self) -> &Sigdmach29done {
        &self.sigdmach29done
    }
    #[doc = "0x9c - SIGDMACH30DONE"]
    #[inline(always)]
    pub const fn sigdmach30done(&self) -> &Sigdmach30done {
        &self.sigdmach30done
    }
    #[doc = "0xa0 - SIGDMACH31DONE"]
    #[inline(always)]
    pub const fn sigdmach31done(&self) -> &Sigdmach31done {
        &self.sigdmach31done
    }
    #[doc = "0xa4 - FW2HWA_TRIG_0"]
    #[inline(always)]
    pub const fn fw2hwa_trig_0(&self) -> &Fw2hwaTrig0 {
        &self.fw2hwa_trig_0
    }
    #[doc = "0xa8 - FW2HWA_TRIG_1"]
    #[inline(always)]
    pub const fn fw2hwa_trig_1(&self) -> &Fw2hwaTrig1 {
        &self.fw2hwa_trig_1
    }
    #[doc = "0xb0 - BPM_PATTERN_0"]
    #[inline(always)]
    pub const fn bpm_pattern_0(&self) -> &BpmPattern0 {
        &self.bpm_pattern_0
    }
    #[doc = "0xb4 - BPM_PATTERN_1"]
    #[inline(always)]
    pub const fn bpm_pattern_1(&self) -> &BpmPattern1 {
        &self.bpm_pattern_1
    }
    #[doc = "0xb8 - BPM_PATTERN_2"]
    #[inline(always)]
    pub const fn bpm_pattern_2(&self) -> &BpmPattern2 {
        &self.bpm_pattern_2
    }
    #[doc = "0xbc - BPM_PATTERN_3"]
    #[inline(always)]
    pub const fn bpm_pattern_3(&self) -> &BpmPattern3 {
        &self.bpm_pattern_3
    }
    #[doc = "0xc0 - BPM_PATTERN_4"]
    #[inline(always)]
    pub const fn bpm_pattern_4(&self) -> &BpmPattern4 {
        &self.bpm_pattern_4
    }
    #[doc = "0xc4 - BPM_PATTERN_5"]
    #[inline(always)]
    pub const fn bpm_pattern_5(&self) -> &BpmPattern5 {
        &self.bpm_pattern_5
    }
    #[doc = "0xc8 - BPM_PATTERN_6"]
    #[inline(always)]
    pub const fn bpm_pattern_6(&self) -> &BpmPattern6 {
        &self.bpm_pattern_6
    }
    #[doc = "0xcc - BPM_PATTERN_7"]
    #[inline(always)]
    pub const fn bpm_pattern_7(&self) -> &BpmPattern7 {
        &self.bpm_pattern_7
    }
    #[doc = "0xd0 - BPM_RATE"]
    #[inline(always)]
    pub const fn bpm_rate(&self) -> &BpmRate {
        &self.bpm_rate
    }
    #[doc = "0xd4 - PARAM_DONE_SET_STATUS_0"]
    #[inline(always)]
    pub const fn param_done_set_status_0(&self) -> &ParamDoneSetStatus0 {
        &self.param_done_set_status_0
    }
    #[doc = "0xd8 - PARAM_DONE_SET_STATUS_1"]
    #[inline(always)]
    pub const fn param_done_set_status_1(&self) -> &ParamDoneSetStatus1 {
        &self.param_done_set_status_1
    }
    #[doc = "0xdc - PARAM_DONE_CLR_0"]
    #[inline(always)]
    pub const fn param_done_clr_0(&self) -> &ParamDoneClr0 {
        &self.param_done_clr_0
    }
    #[doc = "0xe0 - PARAM_DONE_CLR_1"]
    #[inline(always)]
    pub const fn param_done_clr_1(&self) -> &ParamDoneClr1 {
        &self.param_done_clr_1
    }
    #[doc = "0xe4 - TRIGGER_SET_STATUS_0"]
    #[inline(always)]
    pub const fn trigger_set_status_0(&self) -> &TriggerSetStatus0 {
        &self.trigger_set_status_0
    }
    #[doc = "0xe8 - TRIGGER_SET_STATUS_1"]
    #[inline(always)]
    pub const fn trigger_set_status_1(&self) -> &TriggerSetStatus1 {
        &self.trigger_set_status_1
    }
    #[doc = "0xec - TRIGGER_SET_IN_CLR_0"]
    #[inline(always)]
    pub const fn trigger_set_in_clr_0(&self) -> &TriggerSetInClr0 {
        &self.trigger_set_in_clr_0
    }
    #[doc = "0xf0 - TRIGGER_SET_IN_CLR_1"]
    #[inline(always)]
    pub const fn trigger_set_in_clr_1(&self) -> &TriggerSetInClr1 {
        &self.trigger_set_in_clr_1
    }
    #[doc = "0xf4 - DC_EST_RESET_SW"]
    #[inline(always)]
    pub const fn dc_est_reset_sw(&self) -> &DcEstResetSw {
        &self.dc_est_reset_sw
    }
    #[doc = "0xf8 - DC_EST_CTRL"]
    #[inline(always)]
    pub const fn dc_est_ctrl(&self) -> &DcEstCtrl {
        &self.dc_est_ctrl
    }
    #[doc = "0xfc - DC_EST_I_0_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_0_val(&self) -> &DcEstI0Val {
        &self.dc_est_i_0_val
    }
    #[doc = "0x100 - DC_EST_I_1_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_1_val(&self) -> &DcEstI1Val {
        &self.dc_est_i_1_val
    }
    #[doc = "0x104 - DC_EST_I_2_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_2_val(&self) -> &DcEstI2Val {
        &self.dc_est_i_2_val
    }
    #[doc = "0x108 - DC_EST_I_3_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_3_val(&self) -> &DcEstI3Val {
        &self.dc_est_i_3_val
    }
    #[doc = "0x10c - DC_EST_I_4_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_4_val(&self) -> &DcEstI4Val {
        &self.dc_est_i_4_val
    }
    #[doc = "0x110 - DC_EST_I_5_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_5_val(&self) -> &DcEstI5Val {
        &self.dc_est_i_5_val
    }
    #[doc = "0x114 - DC_EST_I_6_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_6_val(&self) -> &DcEstI6Val {
        &self.dc_est_i_6_val
    }
    #[doc = "0x118 - DC_EST_I_7_VAL"]
    #[inline(always)]
    pub const fn dc_est_i_7_val(&self) -> &DcEstI7Val {
        &self.dc_est_i_7_val
    }
    #[doc = "0x12c - DC_EST_Q_0_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_0_val(&self) -> &DcEstQ0Val {
        &self.dc_est_q_0_val
    }
    #[doc = "0x130 - DC_EST_Q_1_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_1_val(&self) -> &DcEstQ1Val {
        &self.dc_est_q_1_val
    }
    #[doc = "0x134 - DC_EST_Q_2_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_2_val(&self) -> &DcEstQ2Val {
        &self.dc_est_q_2_val
    }
    #[doc = "0x138 - DC_EST_Q_3_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_3_val(&self) -> &DcEstQ3Val {
        &self.dc_est_q_3_val
    }
    #[doc = "0x13c - DC_EST_Q_4_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_4_val(&self) -> &DcEstQ4Val {
        &self.dc_est_q_4_val
    }
    #[doc = "0x140 - DC_EST_Q_5_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_5_val(&self) -> &DcEstQ5Val {
        &self.dc_est_q_5_val
    }
    #[doc = "0x144 - DC_EST_Q_6_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_6_val(&self) -> &DcEstQ6Val {
        &self.dc_est_q_6_val
    }
    #[doc = "0x148 - DC_EST_Q_7_VAL"]
    #[inline(always)]
    pub const fn dc_est_q_7_val(&self) -> &DcEstQ7Val {
        &self.dc_est_q_7_val
    }
    #[doc = "0x15c - DC_ACC_I_0_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_0_val_lsb(&self) -> &DcAccI0ValLsb {
        &self.dc_acc_i_0_val_lsb
    }
    #[doc = "0x160 - DC_ACC_I_0_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_0_val_msb(&self) -> &DcAccI0ValMsb {
        &self.dc_acc_i_0_val_msb
    }
    #[doc = "0x164 - DC_ACC_I_1_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_1_val_lsb(&self) -> &DcAccI1ValLsb {
        &self.dc_acc_i_1_val_lsb
    }
    #[doc = "0x168 - DC_ACC_I_1_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_1_val_msb(&self) -> &DcAccI1ValMsb {
        &self.dc_acc_i_1_val_msb
    }
    #[doc = "0x16c - DC_ACC_I_2_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_2_val_lsb(&self) -> &DcAccI2ValLsb {
        &self.dc_acc_i_2_val_lsb
    }
    #[doc = "0x170 - DC_ACC_I_2_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_2_val_msb(&self) -> &DcAccI2ValMsb {
        &self.dc_acc_i_2_val_msb
    }
    #[doc = "0x174 - DC_ACC_I_3_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_3_val_lsb(&self) -> &DcAccI3ValLsb {
        &self.dc_acc_i_3_val_lsb
    }
    #[doc = "0x178 - DC_ACC_I_3_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_3_val_msb(&self) -> &DcAccI3ValMsb {
        &self.dc_acc_i_3_val_msb
    }
    #[doc = "0x17c - DC_ACC_I_4_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_4_val_lsb(&self) -> &DcAccI4ValLsb {
        &self.dc_acc_i_4_val_lsb
    }
    #[doc = "0x180 - DC_ACC_I_4_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_4_val_msb(&self) -> &DcAccI4ValMsb {
        &self.dc_acc_i_4_val_msb
    }
    #[doc = "0x184 - DC_ACC_I_5_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_5_val_lsb(&self) -> &DcAccI5ValLsb {
        &self.dc_acc_i_5_val_lsb
    }
    #[doc = "0x188 - DC_ACC_I_5_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_5_val_msb(&self) -> &DcAccI5ValMsb {
        &self.dc_acc_i_5_val_msb
    }
    #[doc = "0x18c - DC_ACC_I_6_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_6_val_lsb(&self) -> &DcAccI6ValLsb {
        &self.dc_acc_i_6_val_lsb
    }
    #[doc = "0x190 - DC_ACC_I_6_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_6_val_msb(&self) -> &DcAccI6ValMsb {
        &self.dc_acc_i_6_val_msb
    }
    #[doc = "0x194 - DC_ACC_I_7_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_i_7_val_lsb(&self) -> &DcAccI7ValLsb {
        &self.dc_acc_i_7_val_lsb
    }
    #[doc = "0x198 - DC_ACC_I_7_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_i_7_val_msb(&self) -> &DcAccI7ValMsb {
        &self.dc_acc_i_7_val_msb
    }
    #[doc = "0x1bc - DC_ACC_Q_0_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_0_val_lsb(&self) -> &DcAccQ0ValLsb {
        &self.dc_acc_q_0_val_lsb
    }
    #[doc = "0x1c0 - DC_ACC_Q_0_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_0_val_msb(&self) -> &DcAccQ0ValMsb {
        &self.dc_acc_q_0_val_msb
    }
    #[doc = "0x1c4 - DC_ACC_Q_1_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_1_val_lsb(&self) -> &DcAccQ1ValLsb {
        &self.dc_acc_q_1_val_lsb
    }
    #[doc = "0x1c8 - DC_ACC_Q_1_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_1_val_msb(&self) -> &DcAccQ1ValMsb {
        &self.dc_acc_q_1_val_msb
    }
    #[doc = "0x1cc - DC_ACC_Q_2_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_2_val_lsb(&self) -> &DcAccQ2ValLsb {
        &self.dc_acc_q_2_val_lsb
    }
    #[doc = "0x1d0 - DC_ACC_Q_2_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_2_val_msb(&self) -> &DcAccQ2ValMsb {
        &self.dc_acc_q_2_val_msb
    }
    #[doc = "0x1d4 - DC_ACC_Q_3_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_3_val_lsb(&self) -> &DcAccQ3ValLsb {
        &self.dc_acc_q_3_val_lsb
    }
    #[doc = "0x1d8 - DC_ACC_Q_3_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_3_val_msb(&self) -> &DcAccQ3ValMsb {
        &self.dc_acc_q_3_val_msb
    }
    #[doc = "0x1dc - DC_ACC_Q_4_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_4_val_lsb(&self) -> &DcAccQ4ValLsb {
        &self.dc_acc_q_4_val_lsb
    }
    #[doc = "0x1e0 - DC_ACC_Q_4_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_4_val_msb(&self) -> &DcAccQ4ValMsb {
        &self.dc_acc_q_4_val_msb
    }
    #[doc = "0x1e4 - DC_ACC_Q_5_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_5_val_lsb(&self) -> &DcAccQ5ValLsb {
        &self.dc_acc_q_5_val_lsb
    }
    #[doc = "0x1e8 - DC_ACC_Q_5_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_5_val_msb(&self) -> &DcAccQ5ValMsb {
        &self.dc_acc_q_5_val_msb
    }
    #[doc = "0x1ec - DC_ACC_Q_6_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_6_val_lsb(&self) -> &DcAccQ6ValLsb {
        &self.dc_acc_q_6_val_lsb
    }
    #[doc = "0x1f0 - DC_ACC_Q_6_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_6_val_msb(&self) -> &DcAccQ6ValMsb {
        &self.dc_acc_q_6_val_msb
    }
    #[doc = "0x1f4 - DC_ACC_Q_7_VAL_LSB"]
    #[inline(always)]
    pub const fn dc_acc_q_7_val_lsb(&self) -> &DcAccQ7ValLsb {
        &self.dc_acc_q_7_val_lsb
    }
    #[doc = "0x1f8 - DC_ACC_Q_7_VAL_MSB"]
    #[inline(always)]
    pub const fn dc_acc_q_7_val_msb(&self) -> &DcAccQ7ValMsb {
        &self.dc_acc_q_7_val_msb
    }
    #[doc = "0x21c - DC_ACC_CLIP_STATUS"]
    #[inline(always)]
    pub const fn dc_acc_clip_status(&self) -> &DcAccClipStatus {
        &self.dc_acc_clip_status
    }
    #[doc = "0x220 - DC_EST_CLIP_STATUS"]
    #[inline(always)]
    pub const fn dc_est_clip_status(&self) -> &DcEstClipStatus {
        &self.dc_est_clip_status
    }
    #[doc = "0x224 - DC_I0_SW"]
    #[inline(always)]
    pub const fn dc_i0_sw(&self) -> &DcI0Sw {
        &self.dc_i0_sw
    }
    #[doc = "0x228 - DC_I1_SW"]
    #[inline(always)]
    pub const fn dc_i1_sw(&self) -> &DcI1Sw {
        &self.dc_i1_sw
    }
    #[doc = "0x22c - DC_I2_SW"]
    #[inline(always)]
    pub const fn dc_i2_sw(&self) -> &DcI2Sw {
        &self.dc_i2_sw
    }
    #[doc = "0x230 - DC_I3_SW"]
    #[inline(always)]
    pub const fn dc_i3_sw(&self) -> &DcI3Sw {
        &self.dc_i3_sw
    }
    #[doc = "0x234 - DC_I4_SW"]
    #[inline(always)]
    pub const fn dc_i4_sw(&self) -> &DcI4Sw {
        &self.dc_i4_sw
    }
    #[doc = "0x238 - DC_I5_SW"]
    #[inline(always)]
    pub const fn dc_i5_sw(&self) -> &DcI5Sw {
        &self.dc_i5_sw
    }
    #[doc = "0x23c - DC_I6_SW"]
    #[inline(always)]
    pub const fn dc_i6_sw(&self) -> &DcI6Sw {
        &self.dc_i6_sw
    }
    #[doc = "0x240 - DC_I7_SW"]
    #[inline(always)]
    pub const fn dc_i7_sw(&self) -> &DcI7Sw {
        &self.dc_i7_sw
    }
    #[doc = "0x254 - DC_Q0_SW"]
    #[inline(always)]
    pub const fn dc_q0_sw(&self) -> &DcQ0Sw {
        &self.dc_q0_sw
    }
    #[doc = "0x258 - DC_Q1_SW"]
    #[inline(always)]
    pub const fn dc_q1_sw(&self) -> &DcQ1Sw {
        &self.dc_q1_sw
    }
    #[doc = "0x25c - DC_Q2_SW"]
    #[inline(always)]
    pub const fn dc_q2_sw(&self) -> &DcQ2Sw {
        &self.dc_q2_sw
    }
    #[doc = "0x260 - DC_Q3_SW"]
    #[inline(always)]
    pub const fn dc_q3_sw(&self) -> &DcQ3Sw {
        &self.dc_q3_sw
    }
    #[doc = "0x264 - DC_Q4_SW"]
    #[inline(always)]
    pub const fn dc_q4_sw(&self) -> &DcQ4Sw {
        &self.dc_q4_sw
    }
    #[doc = "0x268 - DC_Q5_SW"]
    #[inline(always)]
    pub const fn dc_q5_sw(&self) -> &DcQ5Sw {
        &self.dc_q5_sw
    }
    #[doc = "0x26c - DC_Q6_SW"]
    #[inline(always)]
    pub const fn dc_q6_sw(&self) -> &DcQ6Sw {
        &self.dc_q6_sw
    }
    #[doc = "0x270 - DC_Q7_SW"]
    #[inline(always)]
    pub const fn dc_q7_sw(&self) -> &DcQ7Sw {
        &self.dc_q7_sw
    }
    #[doc = "0x284 - DC_SUB_CLIP"]
    #[inline(always)]
    pub const fn dc_sub_clip(&self) -> &DcSubClip {
        &self.dc_sub_clip
    }
    #[doc = "0x288 - DC_EST_CTRL_PROFILE2"]
    #[inline(always)]
    pub const fn dc_est_ctrl_profile2(&self) -> &DcEstCtrlProfile2 {
        &self.dc_est_ctrl_profile2
    }
    #[doc = "0x28c - DC_RESERVED_3"]
    #[inline(always)]
    pub const fn dc_reserved_3(&self) -> &DcReserved3 {
        &self.dc_reserved_3
    }
    #[doc = "0x290 - DC_RESERVED_4"]
    #[inline(always)]
    pub const fn dc_reserved_4(&self) -> &DcReserved4 {
        &self.dc_reserved_4
    }
    #[doc = "0x294 - DC_RESERVED_5"]
    #[inline(always)]
    pub const fn dc_reserved_5(&self) -> &DcReserved5 {
        &self.dc_reserved_5
    }
    #[doc = "0x298 - INTF_STATS_RESET_SW"]
    #[inline(always)]
    pub const fn intf_stats_reset_sw(&self) -> &IntfStatsResetSw {
        &self.intf_stats_reset_sw
    }
    #[doc = "0x29c - INTF_STATS_CTRL"]
    #[inline(always)]
    pub const fn intf_stats_ctrl(&self) -> &IntfStatsCtrl {
        &self.intf_stats_ctrl
    }
    #[doc = "0x2a0 - INTF_LOC_THRESH_MAG0_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag0_val(&self) -> &IntfLocThreshMag0Val {
        &self.intf_loc_thresh_mag0_val
    }
    #[doc = "0x2a4 - INTF_LOC_THRESH_MAG1_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag1_val(&self) -> &IntfLocThreshMag1Val {
        &self.intf_loc_thresh_mag1_val
    }
    #[doc = "0x2a8 - INTF_LOC_THRESH_MAG2_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag2_val(&self) -> &IntfLocThreshMag2Val {
        &self.intf_loc_thresh_mag2_val
    }
    #[doc = "0x2ac - INTF_LOC_THRESH_MAG3_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag3_val(&self) -> &IntfLocThreshMag3Val {
        &self.intf_loc_thresh_mag3_val
    }
    #[doc = "0x2d0 - INTF_LOC_THRESH_MAGDIFF0_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff0_val(&self) -> &IntfLocThreshMagdiff0Val {
        &self.intf_loc_thresh_magdiff0_val
    }
    #[doc = "0x2d4 - INTF_LOC_THRESH_MAGDIFF1_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff1_val(&self) -> &IntfLocThreshMagdiff1Val {
        &self.intf_loc_thresh_magdiff1_val
    }
    #[doc = "0x2d8 - INTF_LOC_THRESH_MAGDIFF2_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff2_val(&self) -> &IntfLocThreshMagdiff2Val {
        &self.intf_loc_thresh_magdiff2_val
    }
    #[doc = "0x2dc - INTF_LOC_THRESH_MAGDIFF3_VAL"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff3_val(&self) -> &IntfLocThreshMagdiff3Val {
        &self.intf_loc_thresh_magdiff3_val
    }
    #[doc = "0x300 - INTF_LOC_COUNT_ALL_CHIRP"]
    #[inline(always)]
    pub const fn intf_loc_count_all_chirp(&self) -> &IntfLocCountAllChirp {
        &self.intf_loc_count_all_chirp
    }
    #[doc = "0x304 - INTF_LOC_COUNT_ALL_FRAME"]
    #[inline(always)]
    pub const fn intf_loc_count_all_frame(&self) -> &IntfLocCountAllFrame {
        &self.intf_loc_count_all_frame
    }
    #[doc = "0x308 - INTF_STATS_MAG_ACC_0_LSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_0_lsb(&self) -> &IntfStatsMagAcc0Lsb {
        &self.intf_stats_mag_acc_0_lsb
    }
    #[doc = "0x30c - INTF_STATS_MAG_ACC_0_MSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_0_msb(&self) -> &IntfStatsMagAcc0Msb {
        &self.intf_stats_mag_acc_0_msb
    }
    #[doc = "0x310 - INTF_STATS_MAG_ACC_1_LSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_1_lsb(&self) -> &IntfStatsMagAcc1Lsb {
        &self.intf_stats_mag_acc_1_lsb
    }
    #[doc = "0x314 - INTF_STATS_MAG_ACC_1_MSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_1_msb(&self) -> &IntfStatsMagAcc1Msb {
        &self.intf_stats_mag_acc_1_msb
    }
    #[doc = "0x318 - INTF_STATS_MAG_ACC_2_LSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_2_lsb(&self) -> &IntfStatsMagAcc2Lsb {
        &self.intf_stats_mag_acc_2_lsb
    }
    #[doc = "0x31c - INTF_STATS_MAG_ACC_2_MSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_2_msb(&self) -> &IntfStatsMagAcc2Msb {
        &self.intf_stats_mag_acc_2_msb
    }
    #[doc = "0x320 - INTF_STATS_MAG_ACC_3_LSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_3_lsb(&self) -> &IntfStatsMagAcc3Lsb {
        &self.intf_stats_mag_acc_3_lsb
    }
    #[doc = "0x324 - INTF_STATS_MAG_ACC_3_MSB"]
    #[inline(always)]
    pub const fn intf_stats_mag_acc_3_msb(&self) -> &IntfStatsMagAcc3Msb {
        &self.intf_stats_mag_acc_3_msb
    }
    #[doc = "0x368 - INTF_STATS_MAGDIFF_ACC_0_LSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_0_lsb(&self) -> &IntfStatsMagdiffAcc0Lsb {
        &self.intf_stats_magdiff_acc_0_lsb
    }
    #[doc = "0x36c - INTF_STATS_MAGDIFF_ACC_0_MSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_0_msb(&self) -> &IntfStatsMagdiffAcc0Msb {
        &self.intf_stats_magdiff_acc_0_msb
    }
    #[doc = "0x370 - INTF_STATS_MAGDIFF_ACC_1_LSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_1_lsb(&self) -> &IntfStatsMagdiffAcc1Lsb {
        &self.intf_stats_magdiff_acc_1_lsb
    }
    #[doc = "0x374 - INTF_STATS_MAGDIFF_ACC_1_MSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_1_msb(&self) -> &IntfStatsMagdiffAcc1Msb {
        &self.intf_stats_magdiff_acc_1_msb
    }
    #[doc = "0x378 - INTF_STATS_MAGDIFF_ACC_2_LSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_2_lsb(&self) -> &IntfStatsMagdiffAcc2Lsb {
        &self.intf_stats_magdiff_acc_2_lsb
    }
    #[doc = "0x37c - INTF_STATS_MAGDIFF_ACC_2_MSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_2_msb(&self) -> &IntfStatsMagdiffAcc2Msb {
        &self.intf_stats_magdiff_acc_2_msb
    }
    #[doc = "0x380 - INTF_STATS_MAGDIFF_ACC_3_LSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_3_lsb(&self) -> &IntfStatsMagdiffAcc3Lsb {
        &self.intf_stats_magdiff_acc_3_lsb
    }
    #[doc = "0x384 - INTF_STATS_MAGDIFF_ACC_3_MSB"]
    #[inline(always)]
    pub const fn intf_stats_magdiff_acc_3_msb(&self) -> &IntfStatsMagdiffAcc3Msb {
        &self.intf_stats_magdiff_acc_3_msb
    }
    #[doc = "0x3c8 - INTF_LOC_THRESH_MAG0_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag0_sw(&self) -> &IntfLocThreshMag0Sw {
        &self.intf_loc_thresh_mag0_sw
    }
    #[doc = "0x3cc - INTF_LOC_THRESH_MAG1_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag1_sw(&self) -> &IntfLocThreshMag1Sw {
        &self.intf_loc_thresh_mag1_sw
    }
    #[doc = "0x3d0 - INTF_LOC_THRESH_MAG2_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag2_sw(&self) -> &IntfLocThreshMag2Sw {
        &self.intf_loc_thresh_mag2_sw
    }
    #[doc = "0x3d4 - INTF_LOC_THRESH_MAG3_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_mag3_sw(&self) -> &IntfLocThreshMag3Sw {
        &self.intf_loc_thresh_mag3_sw
    }
    #[doc = "0x3f8 - INTF_LOC_THRESH_MAGDIFF0_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff0_sw(&self) -> &IntfLocThreshMagdiff0Sw {
        &self.intf_loc_thresh_magdiff0_sw
    }
    #[doc = "0x3fc - INTF_LOC_THRESH_MAGDIFF1_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff1_sw(&self) -> &IntfLocThreshMagdiff1Sw {
        &self.intf_loc_thresh_magdiff1_sw
    }
    #[doc = "0x400 - INTF_LOC_THRESH_MAGDIFF2_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff2_sw(&self) -> &IntfLocThreshMagdiff2Sw {
        &self.intf_loc_thresh_magdiff2_sw
    }
    #[doc = "0x404 - INTF_LOC_THRESH_MAGDIFF3_SW"]
    #[inline(always)]
    pub const fn intf_loc_thresh_magdiff3_sw(&self) -> &IntfLocThreshMagdiff3Sw {
        &self.intf_loc_thresh_magdiff3_sw
    }
    #[doc = "0x428 - INTF_STATS_ACC_CLIP_STATUS"]
    #[inline(always)]
    pub const fn intf_stats_acc_clip_status(&self) -> &IntfStatsAccClipStatus {
        &self.intf_stats_acc_clip_status
    }
    #[doc = "0x42c - INTF_STATS_THRESH_CLIP_STATUS"]
    #[inline(always)]
    pub const fn intf_stats_thresh_clip_status(&self) -> &IntfStatsThreshClipStatus {
        &self.intf_stats_thresh_clip_status
    }
    #[doc = "0x430 - INTF_MITG_WINDOW_PARAM_0"]
    #[inline(always)]
    pub const fn intf_mitg_window_param_0(&self) -> &IntfMitgWindowParam0 {
        &self.intf_mitg_window_param_0
    }
    #[doc = "0x434 - INTF_MITG_WINDOW_PARAM_1"]
    #[inline(always)]
    pub const fn intf_mitg_window_param_1(&self) -> &IntfMitgWindowParam1 {
        &self.intf_mitg_window_param_1
    }
    #[doc = "0x438 - INTF_MITG_WINDOW_PARAM_2"]
    #[inline(always)]
    pub const fn intf_mitg_window_param_2(&self) -> &IntfMitgWindowParam2 {
        &self.intf_mitg_window_param_2
    }
    #[doc = "0x43c - INTF_MITG_WINDOW_PARAM_3"]
    #[inline(always)]
    pub const fn intf_mitg_window_param_3(&self) -> &IntfMitgWindowParam3 {
        &self.intf_mitg_window_param_3
    }
    #[doc = "0x440 - INTF_MITG_WINDOW_PARAM_4"]
    #[inline(always)]
    pub const fn intf_mitg_window_param_4(&self) -> &IntfMitgWindowParam4 {
        &self.intf_mitg_window_param_4
    }
    #[doc = "0x444 - INTF_STATS_SUM_MAG_VAL"]
    #[inline(always)]
    pub const fn intf_stats_sum_mag_val(&self) -> &IntfStatsSumMagVal {
        &self.intf_stats_sum_mag_val
    }
    #[doc = "0x448 - INTF_STATS_SUM_MAG_VAL_CLIP_STATUS"]
    #[inline(always)]
    pub const fn intf_stats_sum_mag_val_clip_status(&self) -> &IntfStatsSumMagValClipStatus {
        &self.intf_stats_sum_mag_val_clip_status
    }
    #[doc = "0x44c - INTF_STATS_SUM_MAGDIFF_VAL"]
    #[inline(always)]
    pub const fn intf_stats_sum_magdiff_val(&self) -> &IntfStatsSumMagdiffVal {
        &self.intf_stats_sum_magdiff_val
    }
    #[doc = "0x450 - INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS"]
    #[inline(always)]
    pub const fn intf_stats_sum_magdiff_val_clip_status(
        &self,
    ) -> &IntfStatsSumMagdiffValClipStatus {
        &self.intf_stats_sum_magdiff_val_clip_status
    }
    #[doc = "0x454 - INTERF_RESERVED_5"]
    #[inline(always)]
    pub const fn interf_reserved_5(&self) -> &InterfReserved5 {
        &self.interf_reserved_5
    }
    #[doc = "0x458 - ICMULT_SCALE0"]
    #[inline(always)]
    pub const fn icmult_scale0(&self) -> &IcmultScale0 {
        &self.icmult_scale0
    }
    #[doc = "0x45c - ICMULT_SCALE1"]
    #[inline(always)]
    pub const fn icmult_scale1(&self) -> &IcmultScale1 {
        &self.icmult_scale1
    }
    #[doc = "0x460 - ICMULT_SCALE2"]
    #[inline(always)]
    pub const fn icmult_scale2(&self) -> &IcmultScale2 {
        &self.icmult_scale2
    }
    #[doc = "0x464 - ICMULT_SCALE3"]
    #[inline(always)]
    pub const fn icmult_scale3(&self) -> &IcmultScale3 {
        &self.icmult_scale3
    }
    #[doc = "0x488 - QCMULT_SCALE0"]
    #[inline(always)]
    pub const fn qcmult_scale0(&self) -> &QcmultScale0 {
        &self.qcmult_scale0
    }
    #[doc = "0x48c - QCMULT_SCALE1"]
    #[inline(always)]
    pub const fn qcmult_scale1(&self) -> &QcmultScale1 {
        &self.qcmult_scale1
    }
    #[doc = "0x490 - QCMULT_SCALE2"]
    #[inline(always)]
    pub const fn qcmult_scale2(&self) -> &QcmultScale2 {
        &self.qcmult_scale2
    }
    #[doc = "0x494 - QCMULT_SCALE3"]
    #[inline(always)]
    pub const fn qcmult_scale3(&self) -> &QcmultScale3 {
        &self.qcmult_scale3
    }
    #[doc = "0x4b8 - TWID_INCR_DELTA_FRAC"]
    #[inline(always)]
    pub const fn twid_incr_delta_frac(&self) -> &TwidIncrDeltaFrac {
        &self.twid_incr_delta_frac
    }
    #[doc = "0x4c0 - TWID_INCR_DELTA_FRAC_RESET_SW"]
    #[inline(always)]
    pub const fn twid_incr_delta_frac_reset_sw(&self) -> &TwidIncrDeltaFracResetSw {
        &self.twid_incr_delta_frac_reset_sw
    }
    #[doc = "0x4c4 - TWID_INCR_DELTA_FRAC_CLIP_STATUS"]
    #[inline(always)]
    pub const fn twid_incr_delta_frac_clip_status(&self) -> &TwidIncrDeltaFracClipStatus {
        &self.twid_incr_delta_frac_clip_status
    }
    #[doc = "0x4cc - CMULT_RESERVED_2"]
    #[inline(always)]
    pub const fn cmult_reserved_2(&self) -> &CmultReserved2 {
        &self.cmult_reserved_2
    }
    #[doc = "0x52c - LFSR_SEED"]
    #[inline(always)]
    pub const fn lfsr_seed(&self) -> &LfsrSeed {
        &self.lfsr_seed
    }
    #[doc = "0x530 - LFSR_LOAD"]
    #[inline(always)]
    pub const fn lfsr_load(&self) -> &LfsrLoad {
        &self.lfsr_load
    }
    #[doc = "0x534 - DITHER_TWID_EN"]
    #[inline(always)]
    pub const fn dither_twid_en(&self) -> &DitherTwidEn {
        &self.dither_twid_en
    }
    #[doc = "0x538 - FFT_CLIP"]
    #[inline(always)]
    pub const fn fft_clip(&self) -> &FftClip {
        &self.fft_clip
    }
    #[doc = "0x53c - CLR_FFTCLIP"]
    #[inline(always)]
    pub const fn clr_fftclip(&self) -> &ClrFftclip {
        &self.clr_fftclip
    }
    #[doc = "0x540 - CLR_CLIP_MISC"]
    #[inline(always)]
    pub const fn clr_clip_misc(&self) -> &ClrClipMisc {
        &self.clr_clip_misc
    }
    #[doc = "0x544 - IP_OP_FORMATTER_CLIP_STATUS"]
    #[inline(always)]
    pub const fn ip_op_formatter_clip_status(&self) -> &IpOpFormatterClipStatus {
        &self.ip_op_formatter_clip_status
    }
    #[doc = "0x548 - FFT_RESERVED_1"]
    #[inline(always)]
    pub const fn fft_reserved_1(&self) -> &FftReserved1 {
        &self.fft_reserved_1
    }
    #[doc = "0x54c - FFT_RESERVED_2"]
    #[inline(always)]
    pub const fn fft_reserved_2(&self) -> &FftReserved2 {
        &self.fft_reserved_2
    }
    #[doc = "0x550 - FFT_RESERVED_3"]
    #[inline(always)]
    pub const fn fft_reserved_3(&self) -> &FftReserved3 {
        &self.fft_reserved_3
    }
    #[doc = "0x5f8 - CMP_EGE_K0123"]
    #[inline(always)]
    pub const fn cmp_ege_k0123(&self) -> &CmpEgeK0123 {
        &self.cmp_ege_k0123
    }
    #[doc = "0x5fc - CMP_EGE_K4567"]
    #[inline(always)]
    pub const fn cmp_ege_k4567(&self) -> &CmpEgeK4567 {
        &self.cmp_ege_k4567
    }
    #[doc = "0x600 - MEM_INIT_START"]
    #[inline(always)]
    pub const fn mem_init_start(&self) -> &MemInitStart {
        &self.mem_init_start
    }
    #[doc = "0x604 - MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn mem_init_done(&self) -> &MemInitDone {
        &self.mem_init_done
    }
    #[doc = "0x608 - MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn mem_init_status(&self) -> &MemInitStatus {
        &self.mem_init_status
    }
    #[doc = "0x614 - HWA_SAFETY_EN"]
    #[inline(always)]
    pub const fn hwa_safety_en(&self) -> &HwaSafetyEn {
        &self.hwa_safety_en
    }
    #[doc = "0x618 - HWA_SAFETY_ERR_MASK"]
    #[inline(always)]
    pub const fn hwa_safety_err_mask(&self) -> &HwaSafetyErrMask {
        &self.hwa_safety_err_mask
    }
    #[doc = "0x61c - HWA_SAFETY_ERR_STATUS"]
    #[inline(always)]
    pub const fn hwa_safety_err_status(&self) -> &HwaSafetyErrStatus {
        &self.hwa_safety_err_status
    }
    #[doc = "0x620 - HWA_SAFETY_ERR_STATUS_RAW"]
    #[inline(always)]
    pub const fn hwa_safety_err_status_raw(&self) -> &HwaSafetyErrStatusRaw {
        &self.hwa_safety_err_status_raw
    }
    #[doc = "0x624 - HWA_SAFETY_DMEM0_ERR_ADDR"]
    #[inline(always)]
    pub const fn hwa_safety_dmem0_err_addr(&self) -> &HwaSafetyDmem0ErrAddr {
        &self.hwa_safety_dmem0_err_addr
    }
    #[doc = "0x628 - HWA_SAFETY_DMEM1_ERR_ADDR"]
    #[inline(always)]
    pub const fn hwa_safety_dmem1_err_addr(&self) -> &HwaSafetyDmem1ErrAddr {
        &self.hwa_safety_dmem1_err_addr
    }
    #[doc = "0x62c - HWA_SAFETY_DMEM2_ERR_ADDR"]
    #[inline(always)]
    pub const fn hwa_safety_dmem2_err_addr(&self) -> &HwaSafetyDmem2ErrAddr {
        &self.hwa_safety_dmem2_err_addr
    }
    #[doc = "0x630 - HWA_SAFETY_DMEM3_ERR_ADDR"]
    #[inline(always)]
    pub const fn hwa_safety_dmem3_err_addr(&self) -> &HwaSafetyDmem3ErrAddr {
        &self.hwa_safety_dmem3_err_addr
    }
    #[doc = "0x644 - HWA_SAFETY_WINDOW_RAM_ERR_ADDR"]
    #[inline(always)]
    pub const fn hwa_safety_window_ram_err_addr(&self) -> &HwaSafetyWindowRamErrAddr {
        &self.hwa_safety_window_ram_err_addr
    }
    #[doc = "0x648 - MEM_ACCESS_ERR_STATUS"]
    #[inline(always)]
    pub const fn mem_access_err_status(&self) -> &MemAccessErrStatus {
        &self.mem_access_err_status
    }
    #[doc = "0x64c - LOOP_CNT"]
    #[inline(always)]
    pub const fn loop_cnt(&self) -> &LoopCnt {
        &self.loop_cnt
    }
    #[doc = "0x650 - PARAMADDR"]
    #[inline(always)]
    pub const fn paramaddr(&self) -> &Paramaddr {
        &self.paramaddr
    }
    #[doc = "0x654 - PARAMADDR_CPUINTR0"]
    #[inline(always)]
    pub const fn paramaddr_cpuintr0(&self) -> &ParamaddrCpuintr0 {
        &self.paramaddr_cpuintr0
    }
    #[doc = "0x65c - FSM_STATE"]
    #[inline(always)]
    pub const fn fsm_state(&self) -> &FsmState {
        &self.fsm_state
    }
    #[doc = "0x660 - SINGLE_STEP_EN"]
    #[inline(always)]
    pub const fn single_step_en(&self) -> &SingleStepEn {
        &self.single_step_en
    }
    #[doc = "0x664 - SINGLE_STEP_TRIG"]
    #[inline(always)]
    pub const fn single_step_trig(&self) -> &SingleStepTrig {
        &self.single_step_trig
    }
    #[doc = "0x694 - REG_CMP_LFSRSEED_0"]
    #[inline(always)]
    pub const fn reg_cmp_lfsrseed_0(&self) -> &RegCmpLfsrseed0 {
        &self.reg_cmp_lfsrseed_0
    }
    #[doc = "0x698 - REG_CMP_LFSRSEED_1"]
    #[inline(always)]
    pub const fn reg_cmp_lfsrseed_1(&self) -> &RegCmpLfsrseed1 {
        &self.reg_cmp_lfsrseed_1
    }
    #[doc = "0x69c - REG_CMP_LFSRLOAD_0"]
    #[inline(always)]
    pub const fn reg_cmp_lfsrload_0(&self) -> &RegCmpLfsrload0 {
        &self.reg_cmp_lfsrload_0
    }
    #[doc = "0x6a0 - REG_CMP_LFSRLOAD_1"]
    #[inline(always)]
    pub const fn reg_cmp_lfsrload_1(&self) -> &RegCmpLfsrload1 {
        &self.reg_cmp_lfsrload_1
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
#[doc = "PARAM_RAM_IDX (rw) register accessor: PARAM_RAM_IDX\n\nYou can [`read`](crate::Reg::read) this register and get [`param_ram_idx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_ram_idx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_ram_idx`]
module"]
#[doc(alias = "PARAM_RAM_IDX")]
pub type ParamRamIdx = crate::Reg<param_ram_idx::ParamRamIdxSpec>;
#[doc = "PARAM_RAM_IDX"]
pub mod param_ram_idx;
#[doc = "PARAM_RAM_LOOP (rw) register accessor: PARAM_RAM_LOOP\n\nYou can [`read`](crate::Reg::read) this register and get [`param_ram_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_ram_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_ram_loop`]
module"]
#[doc(alias = "PARAM_RAM_LOOP")]
pub type ParamRamLoop = crate::Reg<param_ram_loop::ParamRamLoopSpec>;
#[doc = "PARAM_RAM_LOOP"]
pub mod param_ram_loop;
#[doc = "PREVIOUS_NAME (rw) register accessor: PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@previous_name`]
module"]
#[doc(alias = "PREVIOUS_NAME")]
pub type PreviousName = crate::Reg<previous_name::PreviousNameSpec>;
#[doc = "PREVIOUS_NAME"]
pub mod previous_name;
#[doc = "FW2DMA_TRIG (rw) register accessor: FW2DMA_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`fw2dma_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fw2dma_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fw2dma_trig`]
module"]
#[doc(alias = "FW2DMA_TRIG")]
pub type Fw2dmaTrig = crate::Reg<fw2dma_trig::Fw2dmaTrigSpec>;
#[doc = "FW2DMA_TRIG"]
pub mod fw2dma_trig;
#[doc = "DMA2HWA_TRIG (rw) register accessor: DMA2HWA_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2hwa_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2hwa_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2hwa_trig`]
module"]
#[doc(alias = "DMA2HWA_TRIG")]
pub type Dma2hwaTrig = crate::Reg<dma2hwa_trig::Dma2hwaTrigSpec>;
#[doc = "DMA2HWA_TRIG"]
pub mod dma2hwa_trig;
#[doc = "SIGDMACH0DONE (rw) register accessor: SIGDMACH0DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach0done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach0done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach0done`]
module"]
#[doc(alias = "SIGDMACH0DONE")]
pub type Sigdmach0done = crate::Reg<sigdmach0done::Sigdmach0doneSpec>;
#[doc = "SIGDMACH0DONE"]
pub mod sigdmach0done;
#[doc = "SIGDMACH1DONE (rw) register accessor: SIGDMACH1DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach1done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach1done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach1done`]
module"]
#[doc(alias = "SIGDMACH1DONE")]
pub type Sigdmach1done = crate::Reg<sigdmach1done::Sigdmach1doneSpec>;
#[doc = "SIGDMACH1DONE"]
pub mod sigdmach1done;
#[doc = "SIGDMACH2DONE (rw) register accessor: SIGDMACH2DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach2done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach2done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach2done`]
module"]
#[doc(alias = "SIGDMACH2DONE")]
pub type Sigdmach2done = crate::Reg<sigdmach2done::Sigdmach2doneSpec>;
#[doc = "SIGDMACH2DONE"]
pub mod sigdmach2done;
#[doc = "SIGDMACH3DONE (rw) register accessor: SIGDMACH3DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach3done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach3done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach3done`]
module"]
#[doc(alias = "SIGDMACH3DONE")]
pub type Sigdmach3done = crate::Reg<sigdmach3done::Sigdmach3doneSpec>;
#[doc = "SIGDMACH3DONE"]
pub mod sigdmach3done;
#[doc = "SIGDMACH4DONE (rw) register accessor: SIGDMACH4DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach4done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach4done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach4done`]
module"]
#[doc(alias = "SIGDMACH4DONE")]
pub type Sigdmach4done = crate::Reg<sigdmach4done::Sigdmach4doneSpec>;
#[doc = "SIGDMACH4DONE"]
pub mod sigdmach4done;
#[doc = "SIGDMACH5DONE (rw) register accessor: SIGDMACH5DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach5done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach5done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach5done`]
module"]
#[doc(alias = "SIGDMACH5DONE")]
pub type Sigdmach5done = crate::Reg<sigdmach5done::Sigdmach5doneSpec>;
#[doc = "SIGDMACH5DONE"]
pub mod sigdmach5done;
#[doc = "SIGDMACH6DONE (rw) register accessor: SIGDMACH6DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach6done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach6done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach6done`]
module"]
#[doc(alias = "SIGDMACH6DONE")]
pub type Sigdmach6done = crate::Reg<sigdmach6done::Sigdmach6doneSpec>;
#[doc = "SIGDMACH6DONE"]
pub mod sigdmach6done;
#[doc = "SIGDMACH7DONE (rw) register accessor: SIGDMACH7DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach7done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach7done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach7done`]
module"]
#[doc(alias = "SIGDMACH7DONE")]
pub type Sigdmach7done = crate::Reg<sigdmach7done::Sigdmach7doneSpec>;
#[doc = "SIGDMACH7DONE"]
pub mod sigdmach7done;
#[doc = "SIGDMACH8DONE (rw) register accessor: SIGDMACH8DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach8done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach8done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach8done`]
module"]
#[doc(alias = "SIGDMACH8DONE")]
pub type Sigdmach8done = crate::Reg<sigdmach8done::Sigdmach8doneSpec>;
#[doc = "SIGDMACH8DONE"]
pub mod sigdmach8done;
#[doc = "SIGDMACH9DONE (rw) register accessor: SIGDMACH9DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach9done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach9done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach9done`]
module"]
#[doc(alias = "SIGDMACH9DONE")]
pub type Sigdmach9done = crate::Reg<sigdmach9done::Sigdmach9doneSpec>;
#[doc = "SIGDMACH9DONE"]
pub mod sigdmach9done;
#[doc = "SIGDMACH10DONE (rw) register accessor: SIGDMACH10DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach10done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach10done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach10done`]
module"]
#[doc(alias = "SIGDMACH10DONE")]
pub type Sigdmach10done = crate::Reg<sigdmach10done::Sigdmach10doneSpec>;
#[doc = "SIGDMACH10DONE"]
pub mod sigdmach10done;
#[doc = "SIGDMACH11DONE (rw) register accessor: SIGDMACH11DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach11done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach11done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach11done`]
module"]
#[doc(alias = "SIGDMACH11DONE")]
pub type Sigdmach11done = crate::Reg<sigdmach11done::Sigdmach11doneSpec>;
#[doc = "SIGDMACH11DONE"]
pub mod sigdmach11done;
#[doc = "SIGDMACH12DONE (rw) register accessor: SIGDMACH12DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach12done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach12done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach12done`]
module"]
#[doc(alias = "SIGDMACH12DONE")]
pub type Sigdmach12done = crate::Reg<sigdmach12done::Sigdmach12doneSpec>;
#[doc = "SIGDMACH12DONE"]
pub mod sigdmach12done;
#[doc = "SIGDMACH13DONE (rw) register accessor: SIGDMACH13DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach13done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach13done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach13done`]
module"]
#[doc(alias = "SIGDMACH13DONE")]
pub type Sigdmach13done = crate::Reg<sigdmach13done::Sigdmach13doneSpec>;
#[doc = "SIGDMACH13DONE"]
pub mod sigdmach13done;
#[doc = "SIGDMACH14DONE (rw) register accessor: SIGDMACH14DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach14done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach14done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach14done`]
module"]
#[doc(alias = "SIGDMACH14DONE")]
pub type Sigdmach14done = crate::Reg<sigdmach14done::Sigdmach14doneSpec>;
#[doc = "SIGDMACH14DONE"]
pub mod sigdmach14done;
#[doc = "SIGDMACH15DONE (rw) register accessor: SIGDMACH15DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach15done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach15done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach15done`]
module"]
#[doc(alias = "SIGDMACH15DONE")]
pub type Sigdmach15done = crate::Reg<sigdmach15done::Sigdmach15doneSpec>;
#[doc = "SIGDMACH15DONE"]
pub mod sigdmach15done;
#[doc = "SIGDMACH16DONE (rw) register accessor: SIGDMACH16DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach16done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach16done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach16done`]
module"]
#[doc(alias = "SIGDMACH16DONE")]
pub type Sigdmach16done = crate::Reg<sigdmach16done::Sigdmach16doneSpec>;
#[doc = "SIGDMACH16DONE"]
pub mod sigdmach16done;
#[doc = "SIGDMACH17DONE (rw) register accessor: SIGDMACH17DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach17done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach17done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach17done`]
module"]
#[doc(alias = "SIGDMACH17DONE")]
pub type Sigdmach17done = crate::Reg<sigdmach17done::Sigdmach17doneSpec>;
#[doc = "SIGDMACH17DONE"]
pub mod sigdmach17done;
#[doc = "SIGDMACH18DONE (rw) register accessor: SIGDMACH18DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach18done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach18done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach18done`]
module"]
#[doc(alias = "SIGDMACH18DONE")]
pub type Sigdmach18done = crate::Reg<sigdmach18done::Sigdmach18doneSpec>;
#[doc = "SIGDMACH18DONE"]
pub mod sigdmach18done;
#[doc = "SIGDMACH19DONE (rw) register accessor: SIGDMACH19DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach19done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach19done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach19done`]
module"]
#[doc(alias = "SIGDMACH19DONE")]
pub type Sigdmach19done = crate::Reg<sigdmach19done::Sigdmach19doneSpec>;
#[doc = "SIGDMACH19DONE"]
pub mod sigdmach19done;
#[doc = "SIGDMACH20DONE (rw) register accessor: SIGDMACH20DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach20done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach20done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach20done`]
module"]
#[doc(alias = "SIGDMACH20DONE")]
pub type Sigdmach20done = crate::Reg<sigdmach20done::Sigdmach20doneSpec>;
#[doc = "SIGDMACH20DONE"]
pub mod sigdmach20done;
#[doc = "SIGDMACH21DONE (rw) register accessor: SIGDMACH21DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach21done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach21done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach21done`]
module"]
#[doc(alias = "SIGDMACH21DONE")]
pub type Sigdmach21done = crate::Reg<sigdmach21done::Sigdmach21doneSpec>;
#[doc = "SIGDMACH21DONE"]
pub mod sigdmach21done;
#[doc = "SIGDMACH22DONE (rw) register accessor: SIGDMACH22DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach22done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach22done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach22done`]
module"]
#[doc(alias = "SIGDMACH22DONE")]
pub type Sigdmach22done = crate::Reg<sigdmach22done::Sigdmach22doneSpec>;
#[doc = "SIGDMACH22DONE"]
pub mod sigdmach22done;
#[doc = "SIGDMACH23DONE (rw) register accessor: SIGDMACH23DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach23done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach23done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach23done`]
module"]
#[doc(alias = "SIGDMACH23DONE")]
pub type Sigdmach23done = crate::Reg<sigdmach23done::Sigdmach23doneSpec>;
#[doc = "SIGDMACH23DONE"]
pub mod sigdmach23done;
#[doc = "SIGDMACH24DONE (rw) register accessor: SIGDMACH24DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach24done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach24done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach24done`]
module"]
#[doc(alias = "SIGDMACH24DONE")]
pub type Sigdmach24done = crate::Reg<sigdmach24done::Sigdmach24doneSpec>;
#[doc = "SIGDMACH24DONE"]
pub mod sigdmach24done;
#[doc = "SIGDMACH25DONE (rw) register accessor: SIGDMACH25DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach25done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach25done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach25done`]
module"]
#[doc(alias = "SIGDMACH25DONE")]
pub type Sigdmach25done = crate::Reg<sigdmach25done::Sigdmach25doneSpec>;
#[doc = "SIGDMACH25DONE"]
pub mod sigdmach25done;
#[doc = "SIGDMACH26DONE (rw) register accessor: SIGDMACH26DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach26done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach26done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach26done`]
module"]
#[doc(alias = "SIGDMACH26DONE")]
pub type Sigdmach26done = crate::Reg<sigdmach26done::Sigdmach26doneSpec>;
#[doc = "SIGDMACH26DONE"]
pub mod sigdmach26done;
#[doc = "SIGDMACH27DONE (rw) register accessor: SIGDMACH27DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach27done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach27done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach27done`]
module"]
#[doc(alias = "SIGDMACH27DONE")]
pub type Sigdmach27done = crate::Reg<sigdmach27done::Sigdmach27doneSpec>;
#[doc = "SIGDMACH27DONE"]
pub mod sigdmach27done;
#[doc = "SIGDMACH28DONE (rw) register accessor: SIGDMACH28DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach28done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach28done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach28done`]
module"]
#[doc(alias = "SIGDMACH28DONE")]
pub type Sigdmach28done = crate::Reg<sigdmach28done::Sigdmach28doneSpec>;
#[doc = "SIGDMACH28DONE"]
pub mod sigdmach28done;
#[doc = "SIGDMACH29DONE (rw) register accessor: SIGDMACH29DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach29done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach29done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach29done`]
module"]
#[doc(alias = "SIGDMACH29DONE")]
pub type Sigdmach29done = crate::Reg<sigdmach29done::Sigdmach29doneSpec>;
#[doc = "SIGDMACH29DONE"]
pub mod sigdmach29done;
#[doc = "SIGDMACH30DONE (rw) register accessor: SIGDMACH30DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach30done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach30done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach30done`]
module"]
#[doc(alias = "SIGDMACH30DONE")]
pub type Sigdmach30done = crate::Reg<sigdmach30done::Sigdmach30doneSpec>;
#[doc = "SIGDMACH30DONE"]
pub mod sigdmach30done;
#[doc = "SIGDMACH31DONE (rw) register accessor: SIGDMACH31DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach31done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach31done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach31done`]
module"]
#[doc(alias = "SIGDMACH31DONE")]
pub type Sigdmach31done = crate::Reg<sigdmach31done::Sigdmach31doneSpec>;
#[doc = "SIGDMACH31DONE"]
pub mod sigdmach31done;
#[doc = "FW2HWA_TRIG_0 (rw) register accessor: FW2HWA_TRIG_0\n\nYou can [`read`](crate::Reg::read) this register and get [`fw2hwa_trig_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fw2hwa_trig_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fw2hwa_trig_0`]
module"]
#[doc(alias = "FW2HWA_TRIG_0")]
pub type Fw2hwaTrig0 = crate::Reg<fw2hwa_trig_0::Fw2hwaTrig0Spec>;
#[doc = "FW2HWA_TRIG_0"]
pub mod fw2hwa_trig_0;
#[doc = "FW2HWA_TRIG_1 (rw) register accessor: FW2HWA_TRIG_1\n\nYou can [`read`](crate::Reg::read) this register and get [`fw2hwa_trig_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fw2hwa_trig_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fw2hwa_trig_1`]
module"]
#[doc(alias = "FW2HWA_TRIG_1")]
pub type Fw2hwaTrig1 = crate::Reg<fw2hwa_trig_1::Fw2hwaTrig1Spec>;
#[doc = "FW2HWA_TRIG_1"]
pub mod fw2hwa_trig_1;
#[doc = "BPM_PATTERN_0 (rw) register accessor: BPM_PATTERN_0\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_0`]
module"]
#[doc(alias = "BPM_PATTERN_0")]
pub type BpmPattern0 = crate::Reg<bpm_pattern_0::BpmPattern0Spec>;
#[doc = "BPM_PATTERN_0"]
pub mod bpm_pattern_0;
#[doc = "BPM_PATTERN_1 (rw) register accessor: BPM_PATTERN_1\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_1`]
module"]
#[doc(alias = "BPM_PATTERN_1")]
pub type BpmPattern1 = crate::Reg<bpm_pattern_1::BpmPattern1Spec>;
#[doc = "BPM_PATTERN_1"]
pub mod bpm_pattern_1;
#[doc = "BPM_PATTERN_2 (rw) register accessor: BPM_PATTERN_2\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_2`]
module"]
#[doc(alias = "BPM_PATTERN_2")]
pub type BpmPattern2 = crate::Reg<bpm_pattern_2::BpmPattern2Spec>;
#[doc = "BPM_PATTERN_2"]
pub mod bpm_pattern_2;
#[doc = "BPM_PATTERN_3 (rw) register accessor: BPM_PATTERN_3\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_3`]
module"]
#[doc(alias = "BPM_PATTERN_3")]
pub type BpmPattern3 = crate::Reg<bpm_pattern_3::BpmPattern3Spec>;
#[doc = "BPM_PATTERN_3"]
pub mod bpm_pattern_3;
#[doc = "BPM_PATTERN_4 (rw) register accessor: BPM_PATTERN_4\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_4`]
module"]
#[doc(alias = "BPM_PATTERN_4")]
pub type BpmPattern4 = crate::Reg<bpm_pattern_4::BpmPattern4Spec>;
#[doc = "BPM_PATTERN_4"]
pub mod bpm_pattern_4;
#[doc = "BPM_PATTERN_5 (rw) register accessor: BPM_PATTERN_5\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_5`]
module"]
#[doc(alias = "BPM_PATTERN_5")]
pub type BpmPattern5 = crate::Reg<bpm_pattern_5::BpmPattern5Spec>;
#[doc = "BPM_PATTERN_5"]
pub mod bpm_pattern_5;
#[doc = "BPM_PATTERN_6 (rw) register accessor: BPM_PATTERN_6\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_6`]
module"]
#[doc(alias = "BPM_PATTERN_6")]
pub type BpmPattern6 = crate::Reg<bpm_pattern_6::BpmPattern6Spec>;
#[doc = "BPM_PATTERN_6"]
pub mod bpm_pattern_6;
#[doc = "BPM_PATTERN_7 (rw) register accessor: BPM_PATTERN_7\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_pattern_7`]
module"]
#[doc(alias = "BPM_PATTERN_7")]
pub type BpmPattern7 = crate::Reg<bpm_pattern_7::BpmPattern7Spec>;
#[doc = "BPM_PATTERN_7"]
pub mod bpm_pattern_7;
#[doc = "BPM_RATE (rw) register accessor: BPM_RATE\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpm_rate`]
module"]
#[doc(alias = "BPM_RATE")]
pub type BpmRate = crate::Reg<bpm_rate::BpmRateSpec>;
#[doc = "BPM_RATE"]
pub mod bpm_rate;
#[doc = "PARAM_DONE_SET_STATUS_0 (rw) register accessor: PARAM_DONE_SET_STATUS_0\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_set_status_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_set_status_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_done_set_status_0`]
module"]
#[doc(alias = "PARAM_DONE_SET_STATUS_0")]
pub type ParamDoneSetStatus0 = crate::Reg<param_done_set_status_0::ParamDoneSetStatus0Spec>;
#[doc = "PARAM_DONE_SET_STATUS_0"]
pub mod param_done_set_status_0;
#[doc = "PARAM_DONE_SET_STATUS_1 (rw) register accessor: PARAM_DONE_SET_STATUS_1\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_set_status_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_set_status_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_done_set_status_1`]
module"]
#[doc(alias = "PARAM_DONE_SET_STATUS_1")]
pub type ParamDoneSetStatus1 = crate::Reg<param_done_set_status_1::ParamDoneSetStatus1Spec>;
#[doc = "PARAM_DONE_SET_STATUS_1"]
pub mod param_done_set_status_1;
#[doc = "PARAM_DONE_CLR_0 (rw) register accessor: PARAM_DONE_CLR_0\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_clr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_clr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_done_clr_0`]
module"]
#[doc(alias = "PARAM_DONE_CLR_0")]
pub type ParamDoneClr0 = crate::Reg<param_done_clr_0::ParamDoneClr0Spec>;
#[doc = "PARAM_DONE_CLR_0"]
pub mod param_done_clr_0;
#[doc = "PARAM_DONE_CLR_1 (rw) register accessor: PARAM_DONE_CLR_1\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_clr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_clr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param_done_clr_1`]
module"]
#[doc(alias = "PARAM_DONE_CLR_1")]
pub type ParamDoneClr1 = crate::Reg<param_done_clr_1::ParamDoneClr1Spec>;
#[doc = "PARAM_DONE_CLR_1"]
pub mod param_done_clr_1;
#[doc = "TRIGGER_SET_STATUS_0 (rw) register accessor: TRIGGER_SET_STATUS_0\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_status_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_status_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger_set_status_0`]
module"]
#[doc(alias = "TRIGGER_SET_STATUS_0")]
pub type TriggerSetStatus0 = crate::Reg<trigger_set_status_0::TriggerSetStatus0Spec>;
#[doc = "TRIGGER_SET_STATUS_0"]
pub mod trigger_set_status_0;
#[doc = "TRIGGER_SET_STATUS_1 (rw) register accessor: TRIGGER_SET_STATUS_1\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_status_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_status_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger_set_status_1`]
module"]
#[doc(alias = "TRIGGER_SET_STATUS_1")]
pub type TriggerSetStatus1 = crate::Reg<trigger_set_status_1::TriggerSetStatus1Spec>;
#[doc = "TRIGGER_SET_STATUS_1"]
pub mod trigger_set_status_1;
#[doc = "TRIGGER_SET_IN_CLR_0 (rw) register accessor: TRIGGER_SET_IN_CLR_0\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_in_clr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_in_clr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger_set_in_clr_0`]
module"]
#[doc(alias = "TRIGGER_SET_IN_CLR_0")]
pub type TriggerSetInClr0 = crate::Reg<trigger_set_in_clr_0::TriggerSetInClr0Spec>;
#[doc = "TRIGGER_SET_IN_CLR_0"]
pub mod trigger_set_in_clr_0;
#[doc = "TRIGGER_SET_IN_CLR_1 (rw) register accessor: TRIGGER_SET_IN_CLR_1\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_in_clr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_in_clr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger_set_in_clr_1`]
module"]
#[doc(alias = "TRIGGER_SET_IN_CLR_1")]
pub type TriggerSetInClr1 = crate::Reg<trigger_set_in_clr_1::TriggerSetInClr1Spec>;
#[doc = "TRIGGER_SET_IN_CLR_1"]
pub mod trigger_set_in_clr_1;
#[doc = "DC_EST_RESET_SW (rw) register accessor: DC_EST_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_reset_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_reset_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_reset_sw`]
module"]
#[doc(alias = "DC_EST_RESET_SW")]
pub type DcEstResetSw = crate::Reg<dc_est_reset_sw::DcEstResetSwSpec>;
#[doc = "DC_EST_RESET_SW"]
pub mod dc_est_reset_sw;
#[doc = "DC_EST_CTRL (rw) register accessor: DC_EST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_ctrl`]
module"]
#[doc(alias = "DC_EST_CTRL")]
pub type DcEstCtrl = crate::Reg<dc_est_ctrl::DcEstCtrlSpec>;
#[doc = "DC_EST_CTRL"]
pub mod dc_est_ctrl;
#[doc = "DC_EST_I_0_VAL (rw) register accessor: DC_EST_I_0_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_0_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_0_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_0_val`]
module"]
#[doc(alias = "DC_EST_I_0_VAL")]
pub type DcEstI0Val = crate::Reg<dc_est_i_0_val::DcEstI0ValSpec>;
#[doc = "DC_EST_I_0_VAL"]
pub mod dc_est_i_0_val;
#[doc = "DC_EST_I_1_VAL (rw) register accessor: DC_EST_I_1_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_1_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_1_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_1_val`]
module"]
#[doc(alias = "DC_EST_I_1_VAL")]
pub type DcEstI1Val = crate::Reg<dc_est_i_1_val::DcEstI1ValSpec>;
#[doc = "DC_EST_I_1_VAL"]
pub mod dc_est_i_1_val;
#[doc = "DC_EST_I_2_VAL (rw) register accessor: DC_EST_I_2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_2_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_2_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_2_val`]
module"]
#[doc(alias = "DC_EST_I_2_VAL")]
pub type DcEstI2Val = crate::Reg<dc_est_i_2_val::DcEstI2ValSpec>;
#[doc = "DC_EST_I_2_VAL"]
pub mod dc_est_i_2_val;
#[doc = "DC_EST_I_3_VAL (rw) register accessor: DC_EST_I_3_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_3_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_3_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_3_val`]
module"]
#[doc(alias = "DC_EST_I_3_VAL")]
pub type DcEstI3Val = crate::Reg<dc_est_i_3_val::DcEstI3ValSpec>;
#[doc = "DC_EST_I_3_VAL"]
pub mod dc_est_i_3_val;
#[doc = "DC_EST_I_4_VAL (rw) register accessor: DC_EST_I_4_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_4_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_4_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_4_val`]
module"]
#[doc(alias = "DC_EST_I_4_VAL")]
pub type DcEstI4Val = crate::Reg<dc_est_i_4_val::DcEstI4ValSpec>;
#[doc = "DC_EST_I_4_VAL"]
pub mod dc_est_i_4_val;
#[doc = "DC_EST_I_5_VAL (rw) register accessor: DC_EST_I_5_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_5_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_5_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_5_val`]
module"]
#[doc(alias = "DC_EST_I_5_VAL")]
pub type DcEstI5Val = crate::Reg<dc_est_i_5_val::DcEstI5ValSpec>;
#[doc = "DC_EST_I_5_VAL"]
pub mod dc_est_i_5_val;
#[doc = "DC_EST_I_6_VAL (rw) register accessor: DC_EST_I_6_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_6_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_6_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_6_val`]
module"]
#[doc(alias = "DC_EST_I_6_VAL")]
pub type DcEstI6Val = crate::Reg<dc_est_i_6_val::DcEstI6ValSpec>;
#[doc = "DC_EST_I_6_VAL"]
pub mod dc_est_i_6_val;
#[doc = "DC_EST_I_7_VAL (rw) register accessor: DC_EST_I_7_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_7_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_7_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_i_7_val`]
module"]
#[doc(alias = "DC_EST_I_7_VAL")]
pub type DcEstI7Val = crate::Reg<dc_est_i_7_val::DcEstI7ValSpec>;
#[doc = "DC_EST_I_7_VAL"]
pub mod dc_est_i_7_val;
#[doc = "DC_EST_Q_0_VAL (rw) register accessor: DC_EST_Q_0_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_0_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_0_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_0_val`]
module"]
#[doc(alias = "DC_EST_Q_0_VAL")]
pub type DcEstQ0Val = crate::Reg<dc_est_q_0_val::DcEstQ0ValSpec>;
#[doc = "DC_EST_Q_0_VAL"]
pub mod dc_est_q_0_val;
#[doc = "DC_EST_Q_1_VAL (rw) register accessor: DC_EST_Q_1_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_1_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_1_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_1_val`]
module"]
#[doc(alias = "DC_EST_Q_1_VAL")]
pub type DcEstQ1Val = crate::Reg<dc_est_q_1_val::DcEstQ1ValSpec>;
#[doc = "DC_EST_Q_1_VAL"]
pub mod dc_est_q_1_val;
#[doc = "DC_EST_Q_2_VAL (rw) register accessor: DC_EST_Q_2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_2_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_2_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_2_val`]
module"]
#[doc(alias = "DC_EST_Q_2_VAL")]
pub type DcEstQ2Val = crate::Reg<dc_est_q_2_val::DcEstQ2ValSpec>;
#[doc = "DC_EST_Q_2_VAL"]
pub mod dc_est_q_2_val;
#[doc = "DC_EST_Q_3_VAL (rw) register accessor: DC_EST_Q_3_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_3_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_3_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_3_val`]
module"]
#[doc(alias = "DC_EST_Q_3_VAL")]
pub type DcEstQ3Val = crate::Reg<dc_est_q_3_val::DcEstQ3ValSpec>;
#[doc = "DC_EST_Q_3_VAL"]
pub mod dc_est_q_3_val;
#[doc = "DC_EST_Q_4_VAL (rw) register accessor: DC_EST_Q_4_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_4_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_4_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_4_val`]
module"]
#[doc(alias = "DC_EST_Q_4_VAL")]
pub type DcEstQ4Val = crate::Reg<dc_est_q_4_val::DcEstQ4ValSpec>;
#[doc = "DC_EST_Q_4_VAL"]
pub mod dc_est_q_4_val;
#[doc = "DC_EST_Q_5_VAL (rw) register accessor: DC_EST_Q_5_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_5_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_5_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_5_val`]
module"]
#[doc(alias = "DC_EST_Q_5_VAL")]
pub type DcEstQ5Val = crate::Reg<dc_est_q_5_val::DcEstQ5ValSpec>;
#[doc = "DC_EST_Q_5_VAL"]
pub mod dc_est_q_5_val;
#[doc = "DC_EST_Q_6_VAL (rw) register accessor: DC_EST_Q_6_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_6_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_6_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_6_val`]
module"]
#[doc(alias = "DC_EST_Q_6_VAL")]
pub type DcEstQ6Val = crate::Reg<dc_est_q_6_val::DcEstQ6ValSpec>;
#[doc = "DC_EST_Q_6_VAL"]
pub mod dc_est_q_6_val;
#[doc = "DC_EST_Q_7_VAL (rw) register accessor: DC_EST_Q_7_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_7_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_7_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_q_7_val`]
module"]
#[doc(alias = "DC_EST_Q_7_VAL")]
pub type DcEstQ7Val = crate::Reg<dc_est_q_7_val::DcEstQ7ValSpec>;
#[doc = "DC_EST_Q_7_VAL"]
pub mod dc_est_q_7_val;
#[doc = "DC_ACC_I_0_VAL_LSB (rw) register accessor: DC_ACC_I_0_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_0_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_0_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_0_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_0_VAL_LSB")]
pub type DcAccI0ValLsb = crate::Reg<dc_acc_i_0_val_lsb::DcAccI0ValLsbSpec>;
#[doc = "DC_ACC_I_0_VAL_LSB"]
pub mod dc_acc_i_0_val_lsb;
#[doc = "DC_ACC_I_0_VAL_MSB (rw) register accessor: DC_ACC_I_0_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_0_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_0_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_0_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_0_VAL_MSB")]
pub type DcAccI0ValMsb = crate::Reg<dc_acc_i_0_val_msb::DcAccI0ValMsbSpec>;
#[doc = "DC_ACC_I_0_VAL_MSB"]
pub mod dc_acc_i_0_val_msb;
#[doc = "DC_ACC_I_1_VAL_LSB (rw) register accessor: DC_ACC_I_1_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_1_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_1_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_1_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_1_VAL_LSB")]
pub type DcAccI1ValLsb = crate::Reg<dc_acc_i_1_val_lsb::DcAccI1ValLsbSpec>;
#[doc = "DC_ACC_I_1_VAL_LSB"]
pub mod dc_acc_i_1_val_lsb;
#[doc = "DC_ACC_I_1_VAL_MSB (rw) register accessor: DC_ACC_I_1_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_1_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_1_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_1_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_1_VAL_MSB")]
pub type DcAccI1ValMsb = crate::Reg<dc_acc_i_1_val_msb::DcAccI1ValMsbSpec>;
#[doc = "DC_ACC_I_1_VAL_MSB"]
pub mod dc_acc_i_1_val_msb;
#[doc = "DC_ACC_I_2_VAL_LSB (rw) register accessor: DC_ACC_I_2_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_2_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_2_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_2_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_2_VAL_LSB")]
pub type DcAccI2ValLsb = crate::Reg<dc_acc_i_2_val_lsb::DcAccI2ValLsbSpec>;
#[doc = "DC_ACC_I_2_VAL_LSB"]
pub mod dc_acc_i_2_val_lsb;
#[doc = "DC_ACC_I_2_VAL_MSB (rw) register accessor: DC_ACC_I_2_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_2_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_2_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_2_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_2_VAL_MSB")]
pub type DcAccI2ValMsb = crate::Reg<dc_acc_i_2_val_msb::DcAccI2ValMsbSpec>;
#[doc = "DC_ACC_I_2_VAL_MSB"]
pub mod dc_acc_i_2_val_msb;
#[doc = "DC_ACC_I_3_VAL_LSB (rw) register accessor: DC_ACC_I_3_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_3_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_3_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_3_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_3_VAL_LSB")]
pub type DcAccI3ValLsb = crate::Reg<dc_acc_i_3_val_lsb::DcAccI3ValLsbSpec>;
#[doc = "DC_ACC_I_3_VAL_LSB"]
pub mod dc_acc_i_3_val_lsb;
#[doc = "DC_ACC_I_3_VAL_MSB (rw) register accessor: DC_ACC_I_3_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_3_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_3_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_3_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_3_VAL_MSB")]
pub type DcAccI3ValMsb = crate::Reg<dc_acc_i_3_val_msb::DcAccI3ValMsbSpec>;
#[doc = "DC_ACC_I_3_VAL_MSB"]
pub mod dc_acc_i_3_val_msb;
#[doc = "DC_ACC_I_4_VAL_LSB (rw) register accessor: DC_ACC_I_4_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_4_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_4_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_4_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_4_VAL_LSB")]
pub type DcAccI4ValLsb = crate::Reg<dc_acc_i_4_val_lsb::DcAccI4ValLsbSpec>;
#[doc = "DC_ACC_I_4_VAL_LSB"]
pub mod dc_acc_i_4_val_lsb;
#[doc = "DC_ACC_I_4_VAL_MSB (rw) register accessor: DC_ACC_I_4_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_4_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_4_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_4_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_4_VAL_MSB")]
pub type DcAccI4ValMsb = crate::Reg<dc_acc_i_4_val_msb::DcAccI4ValMsbSpec>;
#[doc = "DC_ACC_I_4_VAL_MSB"]
pub mod dc_acc_i_4_val_msb;
#[doc = "DC_ACC_I_5_VAL_LSB (rw) register accessor: DC_ACC_I_5_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_5_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_5_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_5_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_5_VAL_LSB")]
pub type DcAccI5ValLsb = crate::Reg<dc_acc_i_5_val_lsb::DcAccI5ValLsbSpec>;
#[doc = "DC_ACC_I_5_VAL_LSB"]
pub mod dc_acc_i_5_val_lsb;
#[doc = "DC_ACC_I_5_VAL_MSB (rw) register accessor: DC_ACC_I_5_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_5_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_5_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_5_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_5_VAL_MSB")]
pub type DcAccI5ValMsb = crate::Reg<dc_acc_i_5_val_msb::DcAccI5ValMsbSpec>;
#[doc = "DC_ACC_I_5_VAL_MSB"]
pub mod dc_acc_i_5_val_msb;
#[doc = "DC_ACC_I_6_VAL_LSB (rw) register accessor: DC_ACC_I_6_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_6_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_6_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_6_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_6_VAL_LSB")]
pub type DcAccI6ValLsb = crate::Reg<dc_acc_i_6_val_lsb::DcAccI6ValLsbSpec>;
#[doc = "DC_ACC_I_6_VAL_LSB"]
pub mod dc_acc_i_6_val_lsb;
#[doc = "DC_ACC_I_6_VAL_MSB (rw) register accessor: DC_ACC_I_6_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_6_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_6_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_6_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_6_VAL_MSB")]
pub type DcAccI6ValMsb = crate::Reg<dc_acc_i_6_val_msb::DcAccI6ValMsbSpec>;
#[doc = "DC_ACC_I_6_VAL_MSB"]
pub mod dc_acc_i_6_val_msb;
#[doc = "DC_ACC_I_7_VAL_LSB (rw) register accessor: DC_ACC_I_7_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_7_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_7_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_7_val_lsb`]
module"]
#[doc(alias = "DC_ACC_I_7_VAL_LSB")]
pub type DcAccI7ValLsb = crate::Reg<dc_acc_i_7_val_lsb::DcAccI7ValLsbSpec>;
#[doc = "DC_ACC_I_7_VAL_LSB"]
pub mod dc_acc_i_7_val_lsb;
#[doc = "DC_ACC_I_7_VAL_MSB (rw) register accessor: DC_ACC_I_7_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_i_7_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_i_7_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_i_7_val_msb`]
module"]
#[doc(alias = "DC_ACC_I_7_VAL_MSB")]
pub type DcAccI7ValMsb = crate::Reg<dc_acc_i_7_val_msb::DcAccI7ValMsbSpec>;
#[doc = "DC_ACC_I_7_VAL_MSB"]
pub mod dc_acc_i_7_val_msb;
#[doc = "DC_ACC_Q_0_VAL_LSB (rw) register accessor: DC_ACC_Q_0_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_0_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_0_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_0_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_0_VAL_LSB")]
pub type DcAccQ0ValLsb = crate::Reg<dc_acc_q_0_val_lsb::DcAccQ0ValLsbSpec>;
#[doc = "DC_ACC_Q_0_VAL_LSB"]
pub mod dc_acc_q_0_val_lsb;
#[doc = "DC_ACC_Q_0_VAL_MSB (rw) register accessor: DC_ACC_Q_0_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_0_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_0_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_0_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_0_VAL_MSB")]
pub type DcAccQ0ValMsb = crate::Reg<dc_acc_q_0_val_msb::DcAccQ0ValMsbSpec>;
#[doc = "DC_ACC_Q_0_VAL_MSB"]
pub mod dc_acc_q_0_val_msb;
#[doc = "DC_ACC_Q_1_VAL_LSB (rw) register accessor: DC_ACC_Q_1_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_1_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_1_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_1_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_1_VAL_LSB")]
pub type DcAccQ1ValLsb = crate::Reg<dc_acc_q_1_val_lsb::DcAccQ1ValLsbSpec>;
#[doc = "DC_ACC_Q_1_VAL_LSB"]
pub mod dc_acc_q_1_val_lsb;
#[doc = "DC_ACC_Q_1_VAL_MSB (rw) register accessor: DC_ACC_Q_1_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_1_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_1_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_1_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_1_VAL_MSB")]
pub type DcAccQ1ValMsb = crate::Reg<dc_acc_q_1_val_msb::DcAccQ1ValMsbSpec>;
#[doc = "DC_ACC_Q_1_VAL_MSB"]
pub mod dc_acc_q_1_val_msb;
#[doc = "DC_ACC_Q_2_VAL_LSB (rw) register accessor: DC_ACC_Q_2_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_2_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_2_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_2_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_2_VAL_LSB")]
pub type DcAccQ2ValLsb = crate::Reg<dc_acc_q_2_val_lsb::DcAccQ2ValLsbSpec>;
#[doc = "DC_ACC_Q_2_VAL_LSB"]
pub mod dc_acc_q_2_val_lsb;
#[doc = "DC_ACC_Q_2_VAL_MSB (rw) register accessor: DC_ACC_Q_2_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_2_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_2_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_2_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_2_VAL_MSB")]
pub type DcAccQ2ValMsb = crate::Reg<dc_acc_q_2_val_msb::DcAccQ2ValMsbSpec>;
#[doc = "DC_ACC_Q_2_VAL_MSB"]
pub mod dc_acc_q_2_val_msb;
#[doc = "DC_ACC_Q_3_VAL_LSB (rw) register accessor: DC_ACC_Q_3_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_3_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_3_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_3_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_3_VAL_LSB")]
pub type DcAccQ3ValLsb = crate::Reg<dc_acc_q_3_val_lsb::DcAccQ3ValLsbSpec>;
#[doc = "DC_ACC_Q_3_VAL_LSB"]
pub mod dc_acc_q_3_val_lsb;
#[doc = "DC_ACC_Q_3_VAL_MSB (rw) register accessor: DC_ACC_Q_3_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_3_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_3_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_3_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_3_VAL_MSB")]
pub type DcAccQ3ValMsb = crate::Reg<dc_acc_q_3_val_msb::DcAccQ3ValMsbSpec>;
#[doc = "DC_ACC_Q_3_VAL_MSB"]
pub mod dc_acc_q_3_val_msb;
#[doc = "DC_ACC_Q_4_VAL_LSB (rw) register accessor: DC_ACC_Q_4_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_4_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_4_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_4_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_4_VAL_LSB")]
pub type DcAccQ4ValLsb = crate::Reg<dc_acc_q_4_val_lsb::DcAccQ4ValLsbSpec>;
#[doc = "DC_ACC_Q_4_VAL_LSB"]
pub mod dc_acc_q_4_val_lsb;
#[doc = "DC_ACC_Q_4_VAL_MSB (rw) register accessor: DC_ACC_Q_4_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_4_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_4_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_4_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_4_VAL_MSB")]
pub type DcAccQ4ValMsb = crate::Reg<dc_acc_q_4_val_msb::DcAccQ4ValMsbSpec>;
#[doc = "DC_ACC_Q_4_VAL_MSB"]
pub mod dc_acc_q_4_val_msb;
#[doc = "DC_ACC_Q_5_VAL_LSB (rw) register accessor: DC_ACC_Q_5_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_5_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_5_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_5_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_5_VAL_LSB")]
pub type DcAccQ5ValLsb = crate::Reg<dc_acc_q_5_val_lsb::DcAccQ5ValLsbSpec>;
#[doc = "DC_ACC_Q_5_VAL_LSB"]
pub mod dc_acc_q_5_val_lsb;
#[doc = "DC_ACC_Q_5_VAL_MSB (rw) register accessor: DC_ACC_Q_5_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_5_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_5_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_5_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_5_VAL_MSB")]
pub type DcAccQ5ValMsb = crate::Reg<dc_acc_q_5_val_msb::DcAccQ5ValMsbSpec>;
#[doc = "DC_ACC_Q_5_VAL_MSB"]
pub mod dc_acc_q_5_val_msb;
#[doc = "DC_ACC_Q_6_VAL_LSB (rw) register accessor: DC_ACC_Q_6_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_6_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_6_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_6_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_6_VAL_LSB")]
pub type DcAccQ6ValLsb = crate::Reg<dc_acc_q_6_val_lsb::DcAccQ6ValLsbSpec>;
#[doc = "DC_ACC_Q_6_VAL_LSB"]
pub mod dc_acc_q_6_val_lsb;
#[doc = "DC_ACC_Q_6_VAL_MSB (rw) register accessor: DC_ACC_Q_6_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_6_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_6_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_6_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_6_VAL_MSB")]
pub type DcAccQ6ValMsb = crate::Reg<dc_acc_q_6_val_msb::DcAccQ6ValMsbSpec>;
#[doc = "DC_ACC_Q_6_VAL_MSB"]
pub mod dc_acc_q_6_val_msb;
#[doc = "DC_ACC_Q_7_VAL_LSB (rw) register accessor: DC_ACC_Q_7_VAL_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_7_val_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_7_val_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_7_val_lsb`]
module"]
#[doc(alias = "DC_ACC_Q_7_VAL_LSB")]
pub type DcAccQ7ValLsb = crate::Reg<dc_acc_q_7_val_lsb::DcAccQ7ValLsbSpec>;
#[doc = "DC_ACC_Q_7_VAL_LSB"]
pub mod dc_acc_q_7_val_lsb;
#[doc = "DC_ACC_Q_7_VAL_MSB (rw) register accessor: DC_ACC_Q_7_VAL_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_q_7_val_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_q_7_val_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_q_7_val_msb`]
module"]
#[doc(alias = "DC_ACC_Q_7_VAL_MSB")]
pub type DcAccQ7ValMsb = crate::Reg<dc_acc_q_7_val_msb::DcAccQ7ValMsbSpec>;
#[doc = "DC_ACC_Q_7_VAL_MSB"]
pub mod dc_acc_q_7_val_msb;
#[doc = "DC_ACC_CLIP_STATUS (rw) register accessor: DC_ACC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc_clip_status`]
module"]
#[doc(alias = "DC_ACC_CLIP_STATUS")]
pub type DcAccClipStatus = crate::Reg<dc_acc_clip_status::DcAccClipStatusSpec>;
#[doc = "DC_ACC_CLIP_STATUS"]
pub mod dc_acc_clip_status;
#[doc = "DC_EST_CLIP_STATUS (rw) register accessor: DC_EST_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_clip_status`]
module"]
#[doc(alias = "DC_EST_CLIP_STATUS")]
pub type DcEstClipStatus = crate::Reg<dc_est_clip_status::DcEstClipStatusSpec>;
#[doc = "DC_EST_CLIP_STATUS"]
pub mod dc_est_clip_status;
#[doc = "DC_I0_SW (rw) register accessor: DC_I0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i0_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i0_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i0_sw`]
module"]
#[doc(alias = "DC_I0_SW")]
pub type DcI0Sw = crate::Reg<dc_i0_sw::DcI0SwSpec>;
#[doc = "DC_I0_SW"]
pub mod dc_i0_sw;
#[doc = "DC_I1_SW (rw) register accessor: DC_I1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i1_sw`]
module"]
#[doc(alias = "DC_I1_SW")]
pub type DcI1Sw = crate::Reg<dc_i1_sw::DcI1SwSpec>;
#[doc = "DC_I1_SW"]
pub mod dc_i1_sw;
#[doc = "DC_I2_SW (rw) register accessor: DC_I2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i2_sw`]
module"]
#[doc(alias = "DC_I2_SW")]
pub type DcI2Sw = crate::Reg<dc_i2_sw::DcI2SwSpec>;
#[doc = "DC_I2_SW"]
pub mod dc_i2_sw;
#[doc = "DC_I3_SW (rw) register accessor: DC_I3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i3_sw`]
module"]
#[doc(alias = "DC_I3_SW")]
pub type DcI3Sw = crate::Reg<dc_i3_sw::DcI3SwSpec>;
#[doc = "DC_I3_SW"]
pub mod dc_i3_sw;
#[doc = "DC_I4_SW (rw) register accessor: DC_I4_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i4_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i4_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i4_sw`]
module"]
#[doc(alias = "DC_I4_SW")]
pub type DcI4Sw = crate::Reg<dc_i4_sw::DcI4SwSpec>;
#[doc = "DC_I4_SW"]
pub mod dc_i4_sw;
#[doc = "DC_I5_SW (rw) register accessor: DC_I5_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i5_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i5_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i5_sw`]
module"]
#[doc(alias = "DC_I5_SW")]
pub type DcI5Sw = crate::Reg<dc_i5_sw::DcI5SwSpec>;
#[doc = "DC_I5_SW"]
pub mod dc_i5_sw;
#[doc = "DC_I6_SW (rw) register accessor: DC_I6_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i6_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i6_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i6_sw`]
module"]
#[doc(alias = "DC_I6_SW")]
pub type DcI6Sw = crate::Reg<dc_i6_sw::DcI6SwSpec>;
#[doc = "DC_I6_SW"]
pub mod dc_i6_sw;
#[doc = "DC_I7_SW (rw) register accessor: DC_I7_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i7_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i7_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_i7_sw`]
module"]
#[doc(alias = "DC_I7_SW")]
pub type DcI7Sw = crate::Reg<dc_i7_sw::DcI7SwSpec>;
#[doc = "DC_I7_SW"]
pub mod dc_i7_sw;
#[doc = "DC_Q0_SW (rw) register accessor: DC_Q0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q0_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q0_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q0_sw`]
module"]
#[doc(alias = "DC_Q0_SW")]
pub type DcQ0Sw = crate::Reg<dc_q0_sw::DcQ0SwSpec>;
#[doc = "DC_Q0_SW"]
pub mod dc_q0_sw;
#[doc = "DC_Q1_SW (rw) register accessor: DC_Q1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q1_sw`]
module"]
#[doc(alias = "DC_Q1_SW")]
pub type DcQ1Sw = crate::Reg<dc_q1_sw::DcQ1SwSpec>;
#[doc = "DC_Q1_SW"]
pub mod dc_q1_sw;
#[doc = "DC_Q2_SW (rw) register accessor: DC_Q2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q2_sw`]
module"]
#[doc(alias = "DC_Q2_SW")]
pub type DcQ2Sw = crate::Reg<dc_q2_sw::DcQ2SwSpec>;
#[doc = "DC_Q2_SW"]
pub mod dc_q2_sw;
#[doc = "DC_Q3_SW (rw) register accessor: DC_Q3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q3_sw`]
module"]
#[doc(alias = "DC_Q3_SW")]
pub type DcQ3Sw = crate::Reg<dc_q3_sw::DcQ3SwSpec>;
#[doc = "DC_Q3_SW"]
pub mod dc_q3_sw;
#[doc = "DC_Q4_SW (rw) register accessor: DC_Q4_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q4_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q4_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q4_sw`]
module"]
#[doc(alias = "DC_Q4_SW")]
pub type DcQ4Sw = crate::Reg<dc_q4_sw::DcQ4SwSpec>;
#[doc = "DC_Q4_SW"]
pub mod dc_q4_sw;
#[doc = "DC_Q5_SW (rw) register accessor: DC_Q5_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q5_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q5_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q5_sw`]
module"]
#[doc(alias = "DC_Q5_SW")]
pub type DcQ5Sw = crate::Reg<dc_q5_sw::DcQ5SwSpec>;
#[doc = "DC_Q5_SW"]
pub mod dc_q5_sw;
#[doc = "DC_Q6_SW (rw) register accessor: DC_Q6_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q6_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q6_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q6_sw`]
module"]
#[doc(alias = "DC_Q6_SW")]
pub type DcQ6Sw = crate::Reg<dc_q6_sw::DcQ6SwSpec>;
#[doc = "DC_Q6_SW"]
pub mod dc_q6_sw;
#[doc = "DC_Q7_SW (rw) register accessor: DC_Q7_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_q7_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_q7_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_q7_sw`]
module"]
#[doc(alias = "DC_Q7_SW")]
pub type DcQ7Sw = crate::Reg<dc_q7_sw::DcQ7SwSpec>;
#[doc = "DC_Q7_SW"]
pub mod dc_q7_sw;
#[doc = "DC_SUB_CLIP (rw) register accessor: DC_SUB_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sub_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sub_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sub_clip`]
module"]
#[doc(alias = "DC_SUB_CLIP")]
pub type DcSubClip = crate::Reg<dc_sub_clip::DcSubClipSpec>;
#[doc = "DC_SUB_CLIP"]
pub mod dc_sub_clip;
#[doc = "DC_EST_CTRL_PROFILE2 (rw) register accessor: DC_EST_CTRL_PROFILE2\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_ctrl_profile2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_ctrl_profile2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_est_ctrl_profile2`]
module"]
#[doc(alias = "DC_EST_CTRL_PROFILE2")]
pub type DcEstCtrlProfile2 = crate::Reg<dc_est_ctrl_profile2::DcEstCtrlProfile2Spec>;
#[doc = "DC_EST_CTRL_PROFILE2"]
pub mod dc_est_ctrl_profile2;
#[doc = "DC_RESERVED_3 (rw) register accessor: DC_RESERVED_3\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_reserved_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_reserved_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_reserved_3`]
module"]
#[doc(alias = "DC_RESERVED_3")]
pub type DcReserved3 = crate::Reg<dc_reserved_3::DcReserved3Spec>;
#[doc = "DC_RESERVED_3"]
pub mod dc_reserved_3;
#[doc = "DC_RESERVED_4 (rw) register accessor: DC_RESERVED_4\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_reserved_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_reserved_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_reserved_4`]
module"]
#[doc(alias = "DC_RESERVED_4")]
pub type DcReserved4 = crate::Reg<dc_reserved_4::DcReserved4Spec>;
#[doc = "DC_RESERVED_4"]
pub mod dc_reserved_4;
#[doc = "DC_RESERVED_5 (rw) register accessor: DC_RESERVED_5\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_reserved_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_reserved_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_reserved_5`]
module"]
#[doc(alias = "DC_RESERVED_5")]
pub type DcReserved5 = crate::Reg<dc_reserved_5::DcReserved5Spec>;
#[doc = "DC_RESERVED_5"]
pub mod dc_reserved_5;
#[doc = "INTF_STATS_RESET_SW (rw) register accessor: INTF_STATS_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_reset_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_reset_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_reset_sw`]
module"]
#[doc(alias = "INTF_STATS_RESET_SW")]
pub type IntfStatsResetSw = crate::Reg<intf_stats_reset_sw::IntfStatsResetSwSpec>;
#[doc = "INTF_STATS_RESET_SW"]
pub mod intf_stats_reset_sw;
#[doc = "INTF_STATS_CTRL (rw) register accessor: INTF_STATS_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_ctrl`]
module"]
#[doc(alias = "INTF_STATS_CTRL")]
pub type IntfStatsCtrl = crate::Reg<intf_stats_ctrl::IntfStatsCtrlSpec>;
#[doc = "INTF_STATS_CTRL"]
pub mod intf_stats_ctrl;
#[doc = "INTF_LOC_THRESH_MAG0_VAL (rw) register accessor: INTF_LOC_THRESH_MAG0_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag0_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag0_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag0_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG0_VAL")]
pub type IntfLocThreshMag0Val = crate::Reg<intf_loc_thresh_mag0_val::IntfLocThreshMag0ValSpec>;
#[doc = "INTF_LOC_THRESH_MAG0_VAL"]
pub mod intf_loc_thresh_mag0_val;
#[doc = "INTF_LOC_THRESH_MAG1_VAL (rw) register accessor: INTF_LOC_THRESH_MAG1_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag1_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag1_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag1_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG1_VAL")]
pub type IntfLocThreshMag1Val = crate::Reg<intf_loc_thresh_mag1_val::IntfLocThreshMag1ValSpec>;
#[doc = "INTF_LOC_THRESH_MAG1_VAL"]
pub mod intf_loc_thresh_mag1_val;
#[doc = "INTF_LOC_THRESH_MAG2_VAL (rw) register accessor: INTF_LOC_THRESH_MAG2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag2_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag2_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag2_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG2_VAL")]
pub type IntfLocThreshMag2Val = crate::Reg<intf_loc_thresh_mag2_val::IntfLocThreshMag2ValSpec>;
#[doc = "INTF_LOC_THRESH_MAG2_VAL"]
pub mod intf_loc_thresh_mag2_val;
#[doc = "INTF_LOC_THRESH_MAG3_VAL (rw) register accessor: INTF_LOC_THRESH_MAG3_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag3_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag3_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag3_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG3_VAL")]
pub type IntfLocThreshMag3Val = crate::Reg<intf_loc_thresh_mag3_val::IntfLocThreshMag3ValSpec>;
#[doc = "INTF_LOC_THRESH_MAG3_VAL"]
pub mod intf_loc_thresh_mag3_val;
#[doc = "INTF_LOC_THRESH_MAGDIFF0_VAL (rw) register accessor: INTF_LOC_THRESH_MAGDIFF0_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff0_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff0_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff0_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF0_VAL")]
pub type IntfLocThreshMagdiff0Val =
    crate::Reg<intf_loc_thresh_magdiff0_val::IntfLocThreshMagdiff0ValSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF0_VAL"]
pub mod intf_loc_thresh_magdiff0_val;
#[doc = "INTF_LOC_THRESH_MAGDIFF1_VAL (rw) register accessor: INTF_LOC_THRESH_MAGDIFF1_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff1_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff1_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff1_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF1_VAL")]
pub type IntfLocThreshMagdiff1Val =
    crate::Reg<intf_loc_thresh_magdiff1_val::IntfLocThreshMagdiff1ValSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF1_VAL"]
pub mod intf_loc_thresh_magdiff1_val;
#[doc = "INTF_LOC_THRESH_MAGDIFF2_VAL (rw) register accessor: INTF_LOC_THRESH_MAGDIFF2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff2_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff2_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff2_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF2_VAL")]
pub type IntfLocThreshMagdiff2Val =
    crate::Reg<intf_loc_thresh_magdiff2_val::IntfLocThreshMagdiff2ValSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF2_VAL"]
pub mod intf_loc_thresh_magdiff2_val;
#[doc = "INTF_LOC_THRESH_MAGDIFF3_VAL (rw) register accessor: INTF_LOC_THRESH_MAGDIFF3_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff3_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff3_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff3_val`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF3_VAL")]
pub type IntfLocThreshMagdiff3Val =
    crate::Reg<intf_loc_thresh_magdiff3_val::IntfLocThreshMagdiff3ValSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF3_VAL"]
pub mod intf_loc_thresh_magdiff3_val;
#[doc = "INTF_LOC_COUNT_ALL_CHIRP (rw) register accessor: INTF_LOC_COUNT_ALL_CHIRP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_count_all_chirp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_count_all_chirp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_count_all_chirp`]
module"]
#[doc(alias = "INTF_LOC_COUNT_ALL_CHIRP")]
pub type IntfLocCountAllChirp = crate::Reg<intf_loc_count_all_chirp::IntfLocCountAllChirpSpec>;
#[doc = "INTF_LOC_COUNT_ALL_CHIRP"]
pub mod intf_loc_count_all_chirp;
#[doc = "INTF_LOC_COUNT_ALL_FRAME (rw) register accessor: INTF_LOC_COUNT_ALL_FRAME\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_count_all_frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_count_all_frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_count_all_frame`]
module"]
#[doc(alias = "INTF_LOC_COUNT_ALL_FRAME")]
pub type IntfLocCountAllFrame = crate::Reg<intf_loc_count_all_frame::IntfLocCountAllFrameSpec>;
#[doc = "INTF_LOC_COUNT_ALL_FRAME"]
pub mod intf_loc_count_all_frame;
#[doc = "INTF_STATS_MAG_ACC_0_LSB (rw) register accessor: INTF_STATS_MAG_ACC_0_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_0_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_0_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_0_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_0_LSB")]
pub type IntfStatsMagAcc0Lsb = crate::Reg<intf_stats_mag_acc_0_lsb::IntfStatsMagAcc0LsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_0_LSB"]
pub mod intf_stats_mag_acc_0_lsb;
#[doc = "INTF_STATS_MAG_ACC_0_MSB (rw) register accessor: INTF_STATS_MAG_ACC_0_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_0_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_0_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_0_msb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_0_MSB")]
pub type IntfStatsMagAcc0Msb = crate::Reg<intf_stats_mag_acc_0_msb::IntfStatsMagAcc0MsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_0_MSB"]
pub mod intf_stats_mag_acc_0_msb;
#[doc = "INTF_STATS_MAG_ACC_1_LSB (rw) register accessor: INTF_STATS_MAG_ACC_1_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_1_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_1_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_1_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_1_LSB")]
pub type IntfStatsMagAcc1Lsb = crate::Reg<intf_stats_mag_acc_1_lsb::IntfStatsMagAcc1LsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_1_LSB"]
pub mod intf_stats_mag_acc_1_lsb;
#[doc = "INTF_STATS_MAG_ACC_1_MSB (rw) register accessor: INTF_STATS_MAG_ACC_1_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_1_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_1_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_1_msb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_1_MSB")]
pub type IntfStatsMagAcc1Msb = crate::Reg<intf_stats_mag_acc_1_msb::IntfStatsMagAcc1MsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_1_MSB"]
pub mod intf_stats_mag_acc_1_msb;
#[doc = "INTF_STATS_MAG_ACC_2_LSB (rw) register accessor: INTF_STATS_MAG_ACC_2_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_2_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_2_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_2_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_2_LSB")]
pub type IntfStatsMagAcc2Lsb = crate::Reg<intf_stats_mag_acc_2_lsb::IntfStatsMagAcc2LsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_2_LSB"]
pub mod intf_stats_mag_acc_2_lsb;
#[doc = "INTF_STATS_MAG_ACC_2_MSB (rw) register accessor: INTF_STATS_MAG_ACC_2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_2_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_2_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_2_msb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_2_MSB")]
pub type IntfStatsMagAcc2Msb = crate::Reg<intf_stats_mag_acc_2_msb::IntfStatsMagAcc2MsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_2_MSB"]
pub mod intf_stats_mag_acc_2_msb;
#[doc = "INTF_STATS_MAG_ACC_3_LSB (rw) register accessor: INTF_STATS_MAG_ACC_3_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_3_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_3_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_3_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_3_LSB")]
pub type IntfStatsMagAcc3Lsb = crate::Reg<intf_stats_mag_acc_3_lsb::IntfStatsMagAcc3LsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_3_LSB"]
pub mod intf_stats_mag_acc_3_lsb;
#[doc = "INTF_STATS_MAG_ACC_3_MSB (rw) register accessor: INTF_STATS_MAG_ACC_3_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_mag_acc_3_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_mag_acc_3_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_mag_acc_3_msb`]
module"]
#[doc(alias = "INTF_STATS_MAG_ACC_3_MSB")]
pub type IntfStatsMagAcc3Msb = crate::Reg<intf_stats_mag_acc_3_msb::IntfStatsMagAcc3MsbSpec>;
#[doc = "INTF_STATS_MAG_ACC_3_MSB"]
pub mod intf_stats_mag_acc_3_msb;
#[doc = "INTF_STATS_MAGDIFF_ACC_0_LSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_0_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_0_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_0_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_0_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_0_LSB")]
pub type IntfStatsMagdiffAcc0Lsb =
    crate::Reg<intf_stats_magdiff_acc_0_lsb::IntfStatsMagdiffAcc0LsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_0_LSB"]
pub mod intf_stats_magdiff_acc_0_lsb;
#[doc = "INTF_STATS_MAGDIFF_ACC_0_MSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_0_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_0_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_0_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_0_msb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_0_MSB")]
pub type IntfStatsMagdiffAcc0Msb =
    crate::Reg<intf_stats_magdiff_acc_0_msb::IntfStatsMagdiffAcc0MsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_0_MSB"]
pub mod intf_stats_magdiff_acc_0_msb;
#[doc = "INTF_STATS_MAGDIFF_ACC_1_LSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_1_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_1_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_1_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_1_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_1_LSB")]
pub type IntfStatsMagdiffAcc1Lsb =
    crate::Reg<intf_stats_magdiff_acc_1_lsb::IntfStatsMagdiffAcc1LsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_1_LSB"]
pub mod intf_stats_magdiff_acc_1_lsb;
#[doc = "INTF_STATS_MAGDIFF_ACC_1_MSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_1_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_1_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_1_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_1_msb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_1_MSB")]
pub type IntfStatsMagdiffAcc1Msb =
    crate::Reg<intf_stats_magdiff_acc_1_msb::IntfStatsMagdiffAcc1MsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_1_MSB"]
pub mod intf_stats_magdiff_acc_1_msb;
#[doc = "INTF_STATS_MAGDIFF_ACC_2_LSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_2_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_2_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_2_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_2_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_2_LSB")]
pub type IntfStatsMagdiffAcc2Lsb =
    crate::Reg<intf_stats_magdiff_acc_2_lsb::IntfStatsMagdiffAcc2LsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_2_LSB"]
pub mod intf_stats_magdiff_acc_2_lsb;
#[doc = "INTF_STATS_MAGDIFF_ACC_2_MSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_2_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_2_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_2_msb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_2_MSB")]
pub type IntfStatsMagdiffAcc2Msb =
    crate::Reg<intf_stats_magdiff_acc_2_msb::IntfStatsMagdiffAcc2MsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_2_MSB"]
pub mod intf_stats_magdiff_acc_2_msb;
#[doc = "INTF_STATS_MAGDIFF_ACC_3_LSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_3_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_3_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_3_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_3_lsb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_3_LSB")]
pub type IntfStatsMagdiffAcc3Lsb =
    crate::Reg<intf_stats_magdiff_acc_3_lsb::IntfStatsMagdiffAcc3LsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_3_LSB"]
pub mod intf_stats_magdiff_acc_3_lsb;
#[doc = "INTF_STATS_MAGDIFF_ACC_3_MSB (rw) register accessor: INTF_STATS_MAGDIFF_ACC_3_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_magdiff_acc_3_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_magdiff_acc_3_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_magdiff_acc_3_msb`]
module"]
#[doc(alias = "INTF_STATS_MAGDIFF_ACC_3_MSB")]
pub type IntfStatsMagdiffAcc3Msb =
    crate::Reg<intf_stats_magdiff_acc_3_msb::IntfStatsMagdiffAcc3MsbSpec>;
#[doc = "INTF_STATS_MAGDIFF_ACC_3_MSB"]
pub mod intf_stats_magdiff_acc_3_msb;
#[doc = "INTF_LOC_THRESH_MAG0_SW (rw) register accessor: INTF_LOC_THRESH_MAG0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag0_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag0_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag0_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG0_SW")]
pub type IntfLocThreshMag0Sw = crate::Reg<intf_loc_thresh_mag0_sw::IntfLocThreshMag0SwSpec>;
#[doc = "INTF_LOC_THRESH_MAG0_SW"]
pub mod intf_loc_thresh_mag0_sw;
#[doc = "INTF_LOC_THRESH_MAG1_SW (rw) register accessor: INTF_LOC_THRESH_MAG1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag1_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG1_SW")]
pub type IntfLocThreshMag1Sw = crate::Reg<intf_loc_thresh_mag1_sw::IntfLocThreshMag1SwSpec>;
#[doc = "INTF_LOC_THRESH_MAG1_SW"]
pub mod intf_loc_thresh_mag1_sw;
#[doc = "INTF_LOC_THRESH_MAG2_SW (rw) register accessor: INTF_LOC_THRESH_MAG2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag2_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG2_SW")]
pub type IntfLocThreshMag2Sw = crate::Reg<intf_loc_thresh_mag2_sw::IntfLocThreshMag2SwSpec>;
#[doc = "INTF_LOC_THRESH_MAG2_SW"]
pub mod intf_loc_thresh_mag2_sw;
#[doc = "INTF_LOC_THRESH_MAG3_SW (rw) register accessor: INTF_LOC_THRESH_MAG3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_mag3_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAG3_SW")]
pub type IntfLocThreshMag3Sw = crate::Reg<intf_loc_thresh_mag3_sw::IntfLocThreshMag3SwSpec>;
#[doc = "INTF_LOC_THRESH_MAG3_SW"]
pub mod intf_loc_thresh_mag3_sw;
#[doc = "INTF_LOC_THRESH_MAGDIFF0_SW (rw) register accessor: INTF_LOC_THRESH_MAGDIFF0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff0_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff0_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff0_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF0_SW")]
pub type IntfLocThreshMagdiff0Sw =
    crate::Reg<intf_loc_thresh_magdiff0_sw::IntfLocThreshMagdiff0SwSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF0_SW"]
pub mod intf_loc_thresh_magdiff0_sw;
#[doc = "INTF_LOC_THRESH_MAGDIFF1_SW (rw) register accessor: INTF_LOC_THRESH_MAGDIFF1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff1_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF1_SW")]
pub type IntfLocThreshMagdiff1Sw =
    crate::Reg<intf_loc_thresh_magdiff1_sw::IntfLocThreshMagdiff1SwSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF1_SW"]
pub mod intf_loc_thresh_magdiff1_sw;
#[doc = "INTF_LOC_THRESH_MAGDIFF2_SW (rw) register accessor: INTF_LOC_THRESH_MAGDIFF2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff2_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF2_SW")]
pub type IntfLocThreshMagdiff2Sw =
    crate::Reg<intf_loc_thresh_magdiff2_sw::IntfLocThreshMagdiff2SwSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF2_SW"]
pub mod intf_loc_thresh_magdiff2_sw;
#[doc = "INTF_LOC_THRESH_MAGDIFF3_SW (rw) register accessor: INTF_LOC_THRESH_MAGDIFF3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_loc_thresh_magdiff3_sw`]
module"]
#[doc(alias = "INTF_LOC_THRESH_MAGDIFF3_SW")]
pub type IntfLocThreshMagdiff3Sw =
    crate::Reg<intf_loc_thresh_magdiff3_sw::IntfLocThreshMagdiff3SwSpec>;
#[doc = "INTF_LOC_THRESH_MAGDIFF3_SW"]
pub mod intf_loc_thresh_magdiff3_sw;
#[doc = "INTF_STATS_ACC_CLIP_STATUS (rw) register accessor: INTF_STATS_ACC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_acc_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_acc_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_acc_clip_status`]
module"]
#[doc(alias = "INTF_STATS_ACC_CLIP_STATUS")]
pub type IntfStatsAccClipStatus =
    crate::Reg<intf_stats_acc_clip_status::IntfStatsAccClipStatusSpec>;
#[doc = "INTF_STATS_ACC_CLIP_STATUS"]
pub mod intf_stats_acc_clip_status;
#[doc = "INTF_STATS_THRESH_CLIP_STATUS (rw) register accessor: INTF_STATS_THRESH_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_thresh_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_thresh_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_thresh_clip_status`]
module"]
#[doc(alias = "INTF_STATS_THRESH_CLIP_STATUS")]
pub type IntfStatsThreshClipStatus =
    crate::Reg<intf_stats_thresh_clip_status::IntfStatsThreshClipStatusSpec>;
#[doc = "INTF_STATS_THRESH_CLIP_STATUS"]
pub mod intf_stats_thresh_clip_status;
#[doc = "INTF_MITG_WINDOW_PARAM_0 (rw) register accessor: INTF_MITG_WINDOW_PARAM_0\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mitg_window_param_0`]
module"]
#[doc(alias = "INTF_MITG_WINDOW_PARAM_0")]
pub type IntfMitgWindowParam0 = crate::Reg<intf_mitg_window_param_0::IntfMitgWindowParam0Spec>;
#[doc = "INTF_MITG_WINDOW_PARAM_0"]
pub mod intf_mitg_window_param_0;
#[doc = "INTF_MITG_WINDOW_PARAM_1 (rw) register accessor: INTF_MITG_WINDOW_PARAM_1\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mitg_window_param_1`]
module"]
#[doc(alias = "INTF_MITG_WINDOW_PARAM_1")]
pub type IntfMitgWindowParam1 = crate::Reg<intf_mitg_window_param_1::IntfMitgWindowParam1Spec>;
#[doc = "INTF_MITG_WINDOW_PARAM_1"]
pub mod intf_mitg_window_param_1;
#[doc = "INTF_MITG_WINDOW_PARAM_2 (rw) register accessor: INTF_MITG_WINDOW_PARAM_2\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mitg_window_param_2`]
module"]
#[doc(alias = "INTF_MITG_WINDOW_PARAM_2")]
pub type IntfMitgWindowParam2 = crate::Reg<intf_mitg_window_param_2::IntfMitgWindowParam2Spec>;
#[doc = "INTF_MITG_WINDOW_PARAM_2"]
pub mod intf_mitg_window_param_2;
#[doc = "INTF_MITG_WINDOW_PARAM_3 (rw) register accessor: INTF_MITG_WINDOW_PARAM_3\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mitg_window_param_3`]
module"]
#[doc(alias = "INTF_MITG_WINDOW_PARAM_3")]
pub type IntfMitgWindowParam3 = crate::Reg<intf_mitg_window_param_3::IntfMitgWindowParam3Spec>;
#[doc = "INTF_MITG_WINDOW_PARAM_3"]
pub mod intf_mitg_window_param_3;
#[doc = "INTF_MITG_WINDOW_PARAM_4 (rw) register accessor: INTF_MITG_WINDOW_PARAM_4\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mitg_window_param_4`]
module"]
#[doc(alias = "INTF_MITG_WINDOW_PARAM_4")]
pub type IntfMitgWindowParam4 = crate::Reg<intf_mitg_window_param_4::IntfMitgWindowParam4Spec>;
#[doc = "INTF_MITG_WINDOW_PARAM_4"]
pub mod intf_mitg_window_param_4;
#[doc = "INTF_STATS_SUM_MAG_VAL (rw) register accessor: INTF_STATS_SUM_MAG_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_mag_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_mag_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_sum_mag_val`]
module"]
#[doc(alias = "INTF_STATS_SUM_MAG_VAL")]
pub type IntfStatsSumMagVal = crate::Reg<intf_stats_sum_mag_val::IntfStatsSumMagValSpec>;
#[doc = "INTF_STATS_SUM_MAG_VAL"]
pub mod intf_stats_sum_mag_val;
#[doc = "INTF_STATS_SUM_MAG_VAL_CLIP_STATUS (rw) register accessor: INTF_STATS_SUM_MAG_VAL_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_mag_val_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_mag_val_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_sum_mag_val_clip_status`]
module"]
#[doc(alias = "INTF_STATS_SUM_MAG_VAL_CLIP_STATUS")]
pub type IntfStatsSumMagValClipStatus =
    crate::Reg<intf_stats_sum_mag_val_clip_status::IntfStatsSumMagValClipStatusSpec>;
#[doc = "INTF_STATS_SUM_MAG_VAL_CLIP_STATUS"]
pub mod intf_stats_sum_mag_val_clip_status;
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL (rw) register accessor: INTF_STATS_SUM_MAGDIFF_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_magdiff_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_magdiff_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_sum_magdiff_val`]
module"]
#[doc(alias = "INTF_STATS_SUM_MAGDIFF_VAL")]
pub type IntfStatsSumMagdiffVal =
    crate::Reg<intf_stats_sum_magdiff_val::IntfStatsSumMagdiffValSpec>;
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL"]
pub mod intf_stats_sum_magdiff_val;
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS (rw) register accessor: INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_magdiff_val_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_magdiff_val_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_sum_magdiff_val_clip_status`]
module"]
#[doc(alias = "INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS")]
pub type IntfStatsSumMagdiffValClipStatus =
    crate::Reg<intf_stats_sum_magdiff_val_clip_status::IntfStatsSumMagdiffValClipStatusSpec>;
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS"]
pub mod intf_stats_sum_magdiff_val_clip_status;
#[doc = "INTERF_RESERVED_5 (rw) register accessor: INTERF_RESERVED_5\n\nYou can [`read`](crate::Reg::read) this register and get [`interf_reserved_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interf_reserved_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interf_reserved_5`]
module"]
#[doc(alias = "INTERF_RESERVED_5")]
pub type InterfReserved5 = crate::Reg<interf_reserved_5::InterfReserved5Spec>;
#[doc = "INTERF_RESERVED_5"]
pub mod interf_reserved_5;
#[doc = "ICMULT_SCALE0 (rw) register accessor: ICMULT_SCALE0\n\nYou can [`read`](crate::Reg::read) this register and get [`icmult_scale0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmult_scale0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmult_scale0`]
module"]
#[doc(alias = "ICMULT_SCALE0")]
pub type IcmultScale0 = crate::Reg<icmult_scale0::IcmultScale0Spec>;
#[doc = "ICMULT_SCALE0"]
pub mod icmult_scale0;
#[doc = "ICMULT_SCALE1 (rw) register accessor: ICMULT_SCALE1\n\nYou can [`read`](crate::Reg::read) this register and get [`icmult_scale1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmult_scale1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmult_scale1`]
module"]
#[doc(alias = "ICMULT_SCALE1")]
pub type IcmultScale1 = crate::Reg<icmult_scale1::IcmultScale1Spec>;
#[doc = "ICMULT_SCALE1"]
pub mod icmult_scale1;
#[doc = "ICMULT_SCALE2 (rw) register accessor: ICMULT_SCALE2\n\nYou can [`read`](crate::Reg::read) this register and get [`icmult_scale2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmult_scale2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmult_scale2`]
module"]
#[doc(alias = "ICMULT_SCALE2")]
pub type IcmultScale2 = crate::Reg<icmult_scale2::IcmultScale2Spec>;
#[doc = "ICMULT_SCALE2"]
pub mod icmult_scale2;
#[doc = "ICMULT_SCALE3 (rw) register accessor: ICMULT_SCALE3\n\nYou can [`read`](crate::Reg::read) this register and get [`icmult_scale3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmult_scale3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmult_scale3`]
module"]
#[doc(alias = "ICMULT_SCALE3")]
pub type IcmultScale3 = crate::Reg<icmult_scale3::IcmultScale3Spec>;
#[doc = "ICMULT_SCALE3"]
pub mod icmult_scale3;
#[doc = "QCMULT_SCALE0 (rw) register accessor: QCMULT_SCALE0\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcmult_scale0`]
module"]
#[doc(alias = "QCMULT_SCALE0")]
pub type QcmultScale0 = crate::Reg<qcmult_scale0::QcmultScale0Spec>;
#[doc = "QCMULT_SCALE0"]
pub mod qcmult_scale0;
#[doc = "QCMULT_SCALE1 (rw) register accessor: QCMULT_SCALE1\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcmult_scale1`]
module"]
#[doc(alias = "QCMULT_SCALE1")]
pub type QcmultScale1 = crate::Reg<qcmult_scale1::QcmultScale1Spec>;
#[doc = "QCMULT_SCALE1"]
pub mod qcmult_scale1;
#[doc = "QCMULT_SCALE2 (rw) register accessor: QCMULT_SCALE2\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcmult_scale2`]
module"]
#[doc(alias = "QCMULT_SCALE2")]
pub type QcmultScale2 = crate::Reg<qcmult_scale2::QcmultScale2Spec>;
#[doc = "QCMULT_SCALE2"]
pub mod qcmult_scale2;
#[doc = "QCMULT_SCALE3 (rw) register accessor: QCMULT_SCALE3\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcmult_scale3`]
module"]
#[doc(alias = "QCMULT_SCALE3")]
pub type QcmultScale3 = crate::Reg<qcmult_scale3::QcmultScale3Spec>;
#[doc = "QCMULT_SCALE3"]
pub mod qcmult_scale3;
#[doc = "TWID_INCR_DELTA_FRAC (rw) register accessor: TWID_INCR_DELTA_FRAC\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twid_incr_delta_frac`]
module"]
#[doc(alias = "TWID_INCR_DELTA_FRAC")]
pub type TwidIncrDeltaFrac = crate::Reg<twid_incr_delta_frac::TwidIncrDeltaFracSpec>;
#[doc = "TWID_INCR_DELTA_FRAC"]
pub mod twid_incr_delta_frac;
#[doc = "TWID_INCR_DELTA_FRAC_RESET_SW (rw) register accessor: TWID_INCR_DELTA_FRAC_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac_reset_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac_reset_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twid_incr_delta_frac_reset_sw`]
module"]
#[doc(alias = "TWID_INCR_DELTA_FRAC_RESET_SW")]
pub type TwidIncrDeltaFracResetSw =
    crate::Reg<twid_incr_delta_frac_reset_sw::TwidIncrDeltaFracResetSwSpec>;
#[doc = "TWID_INCR_DELTA_FRAC_RESET_SW"]
pub mod twid_incr_delta_frac_reset_sw;
#[doc = "TWID_INCR_DELTA_FRAC_CLIP_STATUS (rw) register accessor: TWID_INCR_DELTA_FRAC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twid_incr_delta_frac_clip_status`]
module"]
#[doc(alias = "TWID_INCR_DELTA_FRAC_CLIP_STATUS")]
pub type TwidIncrDeltaFracClipStatus =
    crate::Reg<twid_incr_delta_frac_clip_status::TwidIncrDeltaFracClipStatusSpec>;
#[doc = "TWID_INCR_DELTA_FRAC_CLIP_STATUS"]
pub mod twid_incr_delta_frac_clip_status;
#[doc = "CMULT_RESERVED_2 (rw) register accessor: CMULT_RESERVED_2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmult_reserved_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmult_reserved_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmult_reserved_2`]
module"]
#[doc(alias = "CMULT_RESERVED_2")]
pub type CmultReserved2 = crate::Reg<cmult_reserved_2::CmultReserved2Spec>;
#[doc = "CMULT_RESERVED_2"]
pub mod cmult_reserved_2;
#[doc = "LFSR_SEED (rw) register accessor: LFSR_SEED\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsr_seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr_seed`]
module"]
#[doc(alias = "LFSR_SEED")]
pub type LfsrSeed = crate::Reg<lfsr_seed::LfsrSeedSpec>;
#[doc = "LFSR_SEED"]
pub mod lfsr_seed;
#[doc = "LFSR_LOAD (rw) register accessor: LFSR_LOAD\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsr_load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr_load`]
module"]
#[doc(alias = "LFSR_LOAD")]
pub type LfsrLoad = crate::Reg<lfsr_load::LfsrLoadSpec>;
#[doc = "LFSR_LOAD"]
pub mod lfsr_load;
#[doc = "DITHER_TWID_EN (rw) register accessor: DITHER_TWID_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`dither_twid_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_twid_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dither_twid_en`]
module"]
#[doc(alias = "DITHER_TWID_EN")]
pub type DitherTwidEn = crate::Reg<dither_twid_en::DitherTwidEnSpec>;
#[doc = "DITHER_TWID_EN"]
pub mod dither_twid_en;
#[doc = "FFT_CLIP (rw) register accessor: FFT_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fft_clip`]
module"]
#[doc(alias = "FFT_CLIP")]
pub type FftClip = crate::Reg<fft_clip::FftClipSpec>;
#[doc = "FFT_CLIP"]
pub mod fft_clip;
#[doc = "CLR_FFTCLIP (rw) register accessor: CLR_FFTCLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_fftclip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_fftclip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_fftclip`]
module"]
#[doc(alias = "CLR_FFTCLIP")]
pub type ClrFftclip = crate::Reg<clr_fftclip::ClrFftclipSpec>;
#[doc = "CLR_FFTCLIP"]
pub mod clr_fftclip;
#[doc = "CLR_CLIP_MISC (rw) register accessor: CLR_CLIP_MISC\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_clip_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_clip_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_clip_misc`]
module"]
#[doc(alias = "CLR_CLIP_MISC")]
pub type ClrClipMisc = crate::Reg<clr_clip_misc::ClrClipMiscSpec>;
#[doc = "CLR_CLIP_MISC"]
pub mod clr_clip_misc;
#[doc = "IP_OP_FORMATTER_CLIP_STATUS (rw) register accessor: IP_OP_FORMATTER_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_op_formatter_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip_op_formatter_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_op_formatter_clip_status`]
module"]
#[doc(alias = "IP_OP_FORMATTER_CLIP_STATUS")]
pub type IpOpFormatterClipStatus =
    crate::Reg<ip_op_formatter_clip_status::IpOpFormatterClipStatusSpec>;
#[doc = "IP_OP_FORMATTER_CLIP_STATUS"]
pub mod ip_op_formatter_clip_status;
#[doc = "FFT_RESERVED_1 (rw) register accessor: FFT_RESERVED_1\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fft_reserved_1`]
module"]
#[doc(alias = "FFT_RESERVED_1")]
pub type FftReserved1 = crate::Reg<fft_reserved_1::FftReserved1Spec>;
#[doc = "FFT_RESERVED_1"]
pub mod fft_reserved_1;
#[doc = "FFT_RESERVED_2 (rw) register accessor: FFT_RESERVED_2\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fft_reserved_2`]
module"]
#[doc(alias = "FFT_RESERVED_2")]
pub type FftReserved2 = crate::Reg<fft_reserved_2::FftReserved2Spec>;
#[doc = "FFT_RESERVED_2"]
pub mod fft_reserved_2;
#[doc = "FFT_RESERVED_3 (rw) register accessor: FFT_RESERVED_3\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fft_reserved_3`]
module"]
#[doc(alias = "FFT_RESERVED_3")]
pub type FftReserved3 = crate::Reg<fft_reserved_3::FftReserved3Spec>;
#[doc = "FFT_RESERVED_3"]
pub mod fft_reserved_3;
#[doc = "CMP_EGE_K0123 (rw) register accessor: CMP_EGE_K0123\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k0123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k0123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ege_k0123`]
module"]
#[doc(alias = "CMP_EGE_K0123")]
pub type CmpEgeK0123 = crate::Reg<cmp_ege_k0123::CmpEgeK0123Spec>;
#[doc = "CMP_EGE_K0123"]
pub mod cmp_ege_k0123;
#[doc = "CMP_EGE_K4567 (rw) register accessor: CMP_EGE_K4567\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k4567::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k4567::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ege_k4567`]
module"]
#[doc(alias = "CMP_EGE_K4567")]
pub type CmpEgeK4567 = crate::Reg<cmp_ege_k4567::CmpEgeK4567Spec>;
#[doc = "CMP_EGE_K4567"]
pub mod cmp_ege_k4567;
#[doc = "MEM_INIT_START (rw) register accessor: MEM_INIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_init_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_init_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_init_start`]
module"]
#[doc(alias = "MEM_INIT_START")]
pub type MemInitStart = crate::Reg<mem_init_start::MemInitStartSpec>;
#[doc = "MEM_INIT_START"]
pub mod mem_init_start;
#[doc = "MEM_INIT_DONE (rw) register accessor: MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_init_done`]
module"]
#[doc(alias = "MEM_INIT_DONE")]
pub type MemInitDone = crate::Reg<mem_init_done::MemInitDoneSpec>;
#[doc = "MEM_INIT_DONE"]
pub mod mem_init_done;
#[doc = "MEM_INIT_STATUS (rw) register accessor: MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_init_status`]
module"]
#[doc(alias = "MEM_INIT_STATUS")]
pub type MemInitStatus = crate::Reg<mem_init_status::MemInitStatusSpec>;
#[doc = "MEM_INIT_STATUS"]
pub mod mem_init_status;
#[doc = "HWA_SAFETY_EN (rw) register accessor: HWA_SAFETY_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_en`]
module"]
#[doc(alias = "HWA_SAFETY_EN")]
pub type HwaSafetyEn = crate::Reg<hwa_safety_en::HwaSafetyEnSpec>;
#[doc = "HWA_SAFETY_EN"]
pub mod hwa_safety_en;
#[doc = "HWA_SAFETY_ERR_MASK (rw) register accessor: HWA_SAFETY_ERR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_mask`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_MASK")]
pub type HwaSafetyErrMask = crate::Reg<hwa_safety_err_mask::HwaSafetyErrMaskSpec>;
#[doc = "HWA_SAFETY_ERR_MASK"]
pub mod hwa_safety_err_mask;
#[doc = "HWA_SAFETY_ERR_STATUS (rw) register accessor: HWA_SAFETY_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_status`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_STATUS")]
pub type HwaSafetyErrStatus = crate::Reg<hwa_safety_err_status::HwaSafetyErrStatusSpec>;
#[doc = "HWA_SAFETY_ERR_STATUS"]
pub mod hwa_safety_err_status;
#[doc = "HWA_SAFETY_ERR_STATUS_RAW (rw) register accessor: HWA_SAFETY_ERR_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_status_raw`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_STATUS_RAW")]
pub type HwaSafetyErrStatusRaw = crate::Reg<hwa_safety_err_status_raw::HwaSafetyErrStatusRawSpec>;
#[doc = "HWA_SAFETY_ERR_STATUS_RAW"]
pub mod hwa_safety_err_status_raw;
#[doc = "HWA_SAFETY_DMEM0_ERR_ADDR (rw) register accessor: HWA_SAFETY_DMEM0_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_dmem0_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_dmem0_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_dmem0_err_addr`]
module"]
#[doc(alias = "HWA_SAFETY_DMEM0_ERR_ADDR")]
pub type HwaSafetyDmem0ErrAddr = crate::Reg<hwa_safety_dmem0_err_addr::HwaSafetyDmem0ErrAddrSpec>;
#[doc = "HWA_SAFETY_DMEM0_ERR_ADDR"]
pub mod hwa_safety_dmem0_err_addr;
#[doc = "HWA_SAFETY_DMEM1_ERR_ADDR (rw) register accessor: HWA_SAFETY_DMEM1_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_dmem1_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_dmem1_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_dmem1_err_addr`]
module"]
#[doc(alias = "HWA_SAFETY_DMEM1_ERR_ADDR")]
pub type HwaSafetyDmem1ErrAddr = crate::Reg<hwa_safety_dmem1_err_addr::HwaSafetyDmem1ErrAddrSpec>;
#[doc = "HWA_SAFETY_DMEM1_ERR_ADDR"]
pub mod hwa_safety_dmem1_err_addr;
#[doc = "HWA_SAFETY_DMEM2_ERR_ADDR (rw) register accessor: HWA_SAFETY_DMEM2_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_dmem2_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_dmem2_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_dmem2_err_addr`]
module"]
#[doc(alias = "HWA_SAFETY_DMEM2_ERR_ADDR")]
pub type HwaSafetyDmem2ErrAddr = crate::Reg<hwa_safety_dmem2_err_addr::HwaSafetyDmem2ErrAddrSpec>;
#[doc = "HWA_SAFETY_DMEM2_ERR_ADDR"]
pub mod hwa_safety_dmem2_err_addr;
#[doc = "HWA_SAFETY_DMEM3_ERR_ADDR (rw) register accessor: HWA_SAFETY_DMEM3_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_dmem3_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_dmem3_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_dmem3_err_addr`]
module"]
#[doc(alias = "HWA_SAFETY_DMEM3_ERR_ADDR")]
pub type HwaSafetyDmem3ErrAddr = crate::Reg<hwa_safety_dmem3_err_addr::HwaSafetyDmem3ErrAddrSpec>;
#[doc = "HWA_SAFETY_DMEM3_ERR_ADDR"]
pub mod hwa_safety_dmem3_err_addr;
#[doc = "HWA_SAFETY_WINDOW_RAM_ERR_ADDR (rw) register accessor: HWA_SAFETY_WINDOW_RAM_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_window_ram_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_window_ram_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_window_ram_err_addr`]
module"]
#[doc(alias = "HWA_SAFETY_WINDOW_RAM_ERR_ADDR")]
pub type HwaSafetyWindowRamErrAddr =
    crate::Reg<hwa_safety_window_ram_err_addr::HwaSafetyWindowRamErrAddrSpec>;
#[doc = "HWA_SAFETY_WINDOW_RAM_ERR_ADDR"]
pub mod hwa_safety_window_ram_err_addr;
#[doc = "MEM_ACCESS_ERR_STATUS (rw) register accessor: MEM_ACCESS_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_access_err_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_access_err_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_access_err_status`]
module"]
#[doc(alias = "MEM_ACCESS_ERR_STATUS")]
pub type MemAccessErrStatus = crate::Reg<mem_access_err_status::MemAccessErrStatusSpec>;
#[doc = "MEM_ACCESS_ERR_STATUS"]
pub mod mem_access_err_status;
#[doc = "LOOP_CNT (rw) register accessor: LOOP_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loop_cnt`]
module"]
#[doc(alias = "LOOP_CNT")]
pub type LoopCnt = crate::Reg<loop_cnt::LoopCntSpec>;
#[doc = "LOOP_CNT"]
pub mod loop_cnt;
#[doc = "PARAMADDR (rw) register accessor: PARAMADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`paramaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paramaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paramaddr`]
module"]
#[doc(alias = "PARAMADDR")]
pub type Paramaddr = crate::Reg<paramaddr::ParamaddrSpec>;
#[doc = "PARAMADDR"]
pub mod paramaddr;
#[doc = "PARAMADDR_CPUINTR0 (rw) register accessor: PARAMADDR_CPUINTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`paramaddr_cpuintr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paramaddr_cpuintr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paramaddr_cpuintr0`]
module"]
#[doc(alias = "PARAMADDR_CPUINTR0")]
pub type ParamaddrCpuintr0 = crate::Reg<paramaddr_cpuintr0::ParamaddrCpuintr0Spec>;
#[doc = "PARAMADDR_CPUINTR0"]
pub mod paramaddr_cpuintr0;
#[doc = "FSM_STATE (rw) register accessor: FSM_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_state`]
module"]
#[doc(alias = "FSM_STATE")]
pub type FsmState = crate::Reg<fsm_state::FsmStateSpec>;
#[doc = "FSM_STATE"]
pub mod fsm_state;
#[doc = "SINGLE_STEP_EN (rw) register accessor: SINGLE_STEP_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`single_step_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single_step_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_step_en`]
module"]
#[doc(alias = "SINGLE_STEP_EN")]
pub type SingleStepEn = crate::Reg<single_step_en::SingleStepEnSpec>;
#[doc = "SINGLE_STEP_EN"]
pub mod single_step_en;
#[doc = "SINGLE_STEP_TRIG (rw) register accessor: SINGLE_STEP_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`single_step_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single_step_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_step_trig`]
module"]
#[doc(alias = "SINGLE_STEP_TRIG")]
pub type SingleStepTrig = crate::Reg<single_step_trig::SingleStepTrigSpec>;
#[doc = "SINGLE_STEP_TRIG"]
pub mod single_step_trig;
#[doc = "REG_CMP_LFSRSEED_0 (rw) register accessor: REG_CMP_LFSRSEED_0\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrseed_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrseed_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmp_lfsrseed_0`]
module"]
#[doc(alias = "REG_CMP_LFSRSEED_0")]
pub type RegCmpLfsrseed0 = crate::Reg<reg_cmp_lfsrseed_0::RegCmpLfsrseed0Spec>;
#[doc = "REG_CMP_LFSRSEED_0"]
pub mod reg_cmp_lfsrseed_0;
#[doc = "REG_CMP_LFSRSEED_1 (rw) register accessor: REG_CMP_LFSRSEED_1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrseed_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrseed_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmp_lfsrseed_1`]
module"]
#[doc(alias = "REG_CMP_LFSRSEED_1")]
pub type RegCmpLfsrseed1 = crate::Reg<reg_cmp_lfsrseed_1::RegCmpLfsrseed1Spec>;
#[doc = "REG_CMP_LFSRSEED_1"]
pub mod reg_cmp_lfsrseed_1;
#[doc = "REG_CMP_LFSRLOAD_0 (rw) register accessor: REG_CMP_LFSRLOAD_0\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrload_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrload_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmp_lfsrload_0`]
module"]
#[doc(alias = "REG_CMP_LFSRLOAD_0")]
pub type RegCmpLfsrload0 = crate::Reg<reg_cmp_lfsrload_0::RegCmpLfsrload0Spec>;
#[doc = "REG_CMP_LFSRLOAD_0"]
pub mod reg_cmp_lfsrload_0;
#[doc = "REG_CMP_LFSRLOAD_1 (rw) register accessor: REG_CMP_LFSRLOAD_1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrload_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrload_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmp_lfsrload_1`]
module"]
#[doc(alias = "REG_CMP_LFSRLOAD_1")]
pub type RegCmpLfsrload1 = crate::Reg<reg_cmp_lfsrload_1::RegCmpLfsrload1Spec>;
#[doc = "REG_CMP_LFSRLOAD_1"]
pub mod reg_cmp_lfsrload_1;
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
