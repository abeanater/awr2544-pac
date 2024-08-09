#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    mdo_ctrl: MdoCtrl,
    probe_bus_sel0: ProbeBusSel0,
    probe_bus_sel1: ProbeBusSel1,
    rs232_sleep_clk_div: Rs232SleepClkDiv,
    rs232_sleep_clk_div_by2: Rs232SleepClkDivBy2,
    rs232_status: Rs232Status,
    _reserved7: [u8; 0x01e4],
    efuse_dieid0: EfuseDieid0,
    efuse_dieid1: EfuseDieid1,
    efuse_dieid2: EfuseDieid2,
    efuse_dieid3: EfuseDieid3,
    efuse_uid0: EfuseUid0,
    efuse_uid1: EfuseUid1,
    efuse_uid2: EfuseUid2,
    efuse_uid3: EfuseUid3,
    previous_name: PreviousName,
    efuse_from0_checksum: EfuseFrom0Checksum,
    efuse_rom_seq_update0: EfuseRomSeqUpdate0,
    efuse_rom_seq_update1: EfuseRomSeqUpdate1,
    efuse_rom_seq_update2: EfuseRomSeqUpdate2,
    efuse_rom_seq_update3: EfuseRomSeqUpdate3,
    efuse_rom_seq_update4: EfuseRomSeqUpdate4,
    efuse_rom_seq_update5: EfuseRomSeqUpdate5,
    efuse_rom_seq_update6: EfuseRomSeqUpdate6,
    efuse_rom_seq_update7: EfuseRomSeqUpdate7,
    efuse_rom_seq_update8: EfuseRomSeqUpdate8,
    _reserved26: [u8; 0x01b4],
    efuse0_row_61: Efuse0Row61,
    efuse0_row_62: Efuse0Row62,
    efuse0_row_63: Efuse0Row63,
    efuse1_row_5: Efuse1Row5,
    efuse1_row_6: Efuse1Row6,
    efuse1_row_7: Efuse1Row7,
    efuse1_row_8: Efuse1Row8,
    efuse1_row_9: Efuse1Row9,
    efuse1_row_10: Efuse1Row10,
    efuse1_row_11: Efuse1Row11,
    efuse1_row_12: Efuse1Row12,
    efuse1_row_13: Efuse1Row13,
    efuse1_row_14: Efuse1Row14,
    efuse1_row_15: Efuse1Row15,
    efuse1_row_16: Efuse1Row16,
    efuse1_row_17: Efuse1Row17,
    efuse1_row_18: Efuse1Row18,
    efuse1_row_19: Efuse1Row19,
    efuse1_row_20: Efuse1Row20,
    efuse1_row_21: Efuse1Row21,
    efuse1_row_22: Efuse1Row22,
    efuse1_row_23: Efuse1Row23,
    efuse1_row_24: Efuse1Row24,
    efuse1_row_25: Efuse1Row25,
    efuse1_row_26: Efuse1Row26,
    efuse1_row_27: Efuse1Row27,
    efuse1_row_28: Efuse1Row28,
    efuse1_row_29: Efuse1Row29,
    efuse1_row_30: Efuse1Row30,
    efuse1_row_31: Efuse1Row31,
    efuse1_row_32: Efuse1Row32,
    efuse1_row_33: Efuse1Row33,
    efuse1_row_34: Efuse1Row34,
    efuse1_row_35: Efuse1Row35,
    efuse1_row_36: Efuse1Row36,
    efuse1_row_37: Efuse1Row37,
    efuse1_row_38: Efuse1Row38,
    efuse1_row_39: Efuse1Row39,
    efuse1_row_40: Efuse1Row40,
    efuse1_row_41: Efuse1Row41,
    efuse1_row_42: Efuse1Row42,
    efuse1_row_43: Efuse1Row43,
    _reserved68: [u8; 0x0358],
    efuse_override_hsm_halt_on_rom_ecc_err_en: EfuseOverrideHsmHaltOnRomEccErrEn,
    efuse_override_mem_marginctrl: EfuseOverrideMemMarginctrl,
    efuse_override_lvds_bgap_trim: EfuseOverrideLvdsBgapTrim,
    efuse_override_xtal_stablization_wait: EfuseOverrideXtalStablizationWait,
    efuse_override_slicer_bias_rtrim: EfuseOverrideSlicerBiasRtrim,
    efuse_override_xo_output_drive: EfuseOverrideXoOutputDrive,
    efuse_override_rcosc_trim_code: EfuseOverrideRcoscTrimCode,
    efuse_override_ip1_bg1_rtrim: EfuseOverrideIp1Bg1Rtrim,
    efuse_override_ip1_bg1_slope: EfuseOverrideIp1Bg1Slope,
    efuse_override_ip1_bg1_mag: EfuseOverrideIp1Bg1Mag,
    efuse_override_rs232_clkmode: EfuseOverrideRs232Clkmode,
    efuse_override_vmon_vdd_ov_uv_trim: EfuseOverrideVmonVddOvUvTrim,
    efuse_override_vmon_vdds_3p3_uv_trim: EfuseOverrideVmonVdds3p3UvTrim,
    efuse_override_vmon_vdda_osc_trim: EfuseOverrideVmonVddaOscTrim,
    efuse_override_vdd_vt_det: EfuseOverrideVddVtDet,
    efuse_override_mask_cpu_clk_out_ctrl_lowv_val: EfuseOverrideMaskCpuClkOutCtrlLowvVal,
    efuse_override_mask_cpu_clk_out_ctrl_lowv_sel: EfuseOverrideMaskCpuClkOutCtrlLowvSel,
    efuse_override_en_vol_mon_func: EfuseOverrideEnVolMonFunc,
    efuse_override_en_vol_mon_func_: EfuseOverrideEnVolMonFunc_,
    efuse_override_spare_ana: EfuseOverrideSpareAna,
    efuse_override_slicer_dly_disable: EfuseOverrideSlicerDlyDisable,
    _reserved89: [u8; 0x077c],
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
    _reserved99: [u8; 0x10],
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
    #[doc = "0x04 - MDO_CTRL"]
    #[inline(always)]
    pub const fn mdo_ctrl(&self) -> &MdoCtrl {
        &self.mdo_ctrl
    }
    #[doc = "0x08 - PROBE_BUS_SEL0"]
    #[inline(always)]
    pub const fn probe_bus_sel0(&self) -> &ProbeBusSel0 {
        &self.probe_bus_sel0
    }
    #[doc = "0x0c - PROBE_BUS_SEL1"]
    #[inline(always)]
    pub const fn probe_bus_sel1(&self) -> &ProbeBusSel1 {
        &self.probe_bus_sel1
    }
    #[doc = "0x10 - RS232_SLEEP_CLK_DIV"]
    #[inline(always)]
    pub const fn rs232_sleep_clk_div(&self) -> &Rs232SleepClkDiv {
        &self.rs232_sleep_clk_div
    }
    #[doc = "0x14 - RS232_SLEEP_CLK_DIV_by2"]
    #[inline(always)]
    pub const fn rs232_sleep_clk_div_by2(&self) -> &Rs232SleepClkDivBy2 {
        &self.rs232_sleep_clk_div_by2
    }
    #[doc = "0x18 - RS232_STATUS"]
    #[inline(always)]
    pub const fn rs232_status(&self) -> &Rs232Status {
        &self.rs232_status
    }
    #[doc = "0x200 - EFUSE_DIEID0"]
    #[inline(always)]
    pub const fn efuse_dieid0(&self) -> &EfuseDieid0 {
        &self.efuse_dieid0
    }
    #[doc = "0x204 - EFUSE_DIEID1"]
    #[inline(always)]
    pub const fn efuse_dieid1(&self) -> &EfuseDieid1 {
        &self.efuse_dieid1
    }
    #[doc = "0x208 - EFUSE_DIEID2"]
    #[inline(always)]
    pub const fn efuse_dieid2(&self) -> &EfuseDieid2 {
        &self.efuse_dieid2
    }
    #[doc = "0x20c - EFUSE_DIEID3"]
    #[inline(always)]
    pub const fn efuse_dieid3(&self) -> &EfuseDieid3 {
        &self.efuse_dieid3
    }
    #[doc = "0x210 - EFUSE_UID0"]
    #[inline(always)]
    pub const fn efuse_uid0(&self) -> &EfuseUid0 {
        &self.efuse_uid0
    }
    #[doc = "0x214 - EFUSE_UID1"]
    #[inline(always)]
    pub const fn efuse_uid1(&self) -> &EfuseUid1 {
        &self.efuse_uid1
    }
    #[doc = "0x218 - EFUSE_UID2"]
    #[inline(always)]
    pub const fn efuse_uid2(&self) -> &EfuseUid2 {
        &self.efuse_uid2
    }
    #[doc = "0x21c - EFUSE_UID3"]
    #[inline(always)]
    pub const fn efuse_uid3(&self) -> &EfuseUid3 {
        &self.efuse_uid3
    }
    #[doc = "0x220 - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x224 - EFUSE_FROM0_CHECKSUM"]
    #[inline(always)]
    pub const fn efuse_from0_checksum(&self) -> &EfuseFrom0Checksum {
        &self.efuse_from0_checksum
    }
    #[doc = "0x228 - EFUSE_ROM_SEQ_UPDATE0"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update0(&self) -> &EfuseRomSeqUpdate0 {
        &self.efuse_rom_seq_update0
    }
    #[doc = "0x22c - EFUSE_ROM_SEQ_UPDATE1"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update1(&self) -> &EfuseRomSeqUpdate1 {
        &self.efuse_rom_seq_update1
    }
    #[doc = "0x230 - EFUSE_ROM_SEQ_UPDATE2"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update2(&self) -> &EfuseRomSeqUpdate2 {
        &self.efuse_rom_seq_update2
    }
    #[doc = "0x234 - EFUSE_ROM_SEQ_UPDATE3"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update3(&self) -> &EfuseRomSeqUpdate3 {
        &self.efuse_rom_seq_update3
    }
    #[doc = "0x238 - EFUSE_ROM_SEQ_UPDATE4"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update4(&self) -> &EfuseRomSeqUpdate4 {
        &self.efuse_rom_seq_update4
    }
    #[doc = "0x23c - EFUSE_ROM_SEQ_UPDATE5"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update5(&self) -> &EfuseRomSeqUpdate5 {
        &self.efuse_rom_seq_update5
    }
    #[doc = "0x240 - EFUSE_ROM_SEQ_UPDATE6"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update6(&self) -> &EfuseRomSeqUpdate6 {
        &self.efuse_rom_seq_update6
    }
    #[doc = "0x244 - EFUSE_ROM_SEQ_UPDATE7"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update7(&self) -> &EfuseRomSeqUpdate7 {
        &self.efuse_rom_seq_update7
    }
    #[doc = "0x248 - EFUSE_ROM_SEQ_UPDATE8"]
    #[inline(always)]
    pub const fn efuse_rom_seq_update8(&self) -> &EfuseRomSeqUpdate8 {
        &self.efuse_rom_seq_update8
    }
    #[doc = "0x400 - EFUSE0_ROW_61"]
    #[inline(always)]
    pub const fn efuse0_row_61(&self) -> &Efuse0Row61 {
        &self.efuse0_row_61
    }
    #[doc = "0x404 - EFUSE0_ROW_62"]
    #[inline(always)]
    pub const fn efuse0_row_62(&self) -> &Efuse0Row62 {
        &self.efuse0_row_62
    }
    #[doc = "0x408 - EFUSE0_ROW_63"]
    #[inline(always)]
    pub const fn efuse0_row_63(&self) -> &Efuse0Row63 {
        &self.efuse0_row_63
    }
    #[doc = "0x40c - EFUSE1_ROW_5"]
    #[inline(always)]
    pub const fn efuse1_row_5(&self) -> &Efuse1Row5 {
        &self.efuse1_row_5
    }
    #[doc = "0x410 - EFUSE1_ROW_6"]
    #[inline(always)]
    pub const fn efuse1_row_6(&self) -> &Efuse1Row6 {
        &self.efuse1_row_6
    }
    #[doc = "0x414 - EFUSE1_ROW_7"]
    #[inline(always)]
    pub const fn efuse1_row_7(&self) -> &Efuse1Row7 {
        &self.efuse1_row_7
    }
    #[doc = "0x418 - EFUSE1_ROW_8"]
    #[inline(always)]
    pub const fn efuse1_row_8(&self) -> &Efuse1Row8 {
        &self.efuse1_row_8
    }
    #[doc = "0x41c - EFUSE1_ROW_9"]
    #[inline(always)]
    pub const fn efuse1_row_9(&self) -> &Efuse1Row9 {
        &self.efuse1_row_9
    }
    #[doc = "0x420 - EFUSE1_ROW_10"]
    #[inline(always)]
    pub const fn efuse1_row_10(&self) -> &Efuse1Row10 {
        &self.efuse1_row_10
    }
    #[doc = "0x424 - EFUSE1_ROW_11"]
    #[inline(always)]
    pub const fn efuse1_row_11(&self) -> &Efuse1Row11 {
        &self.efuse1_row_11
    }
    #[doc = "0x428 - EFUSE1_ROW_12"]
    #[inline(always)]
    pub const fn efuse1_row_12(&self) -> &Efuse1Row12 {
        &self.efuse1_row_12
    }
    #[doc = "0x42c - EFUSE1_ROW_13"]
    #[inline(always)]
    pub const fn efuse1_row_13(&self) -> &Efuse1Row13 {
        &self.efuse1_row_13
    }
    #[doc = "0x430 - EFUSE1_ROW_14"]
    #[inline(always)]
    pub const fn efuse1_row_14(&self) -> &Efuse1Row14 {
        &self.efuse1_row_14
    }
    #[doc = "0x434 - EFUSE1_ROW_15"]
    #[inline(always)]
    pub const fn efuse1_row_15(&self) -> &Efuse1Row15 {
        &self.efuse1_row_15
    }
    #[doc = "0x438 - EFUSE1_ROW_16"]
    #[inline(always)]
    pub const fn efuse1_row_16(&self) -> &Efuse1Row16 {
        &self.efuse1_row_16
    }
    #[doc = "0x43c - EFUSE1_ROW_17"]
    #[inline(always)]
    pub const fn efuse1_row_17(&self) -> &Efuse1Row17 {
        &self.efuse1_row_17
    }
    #[doc = "0x440 - EFUSE1_ROW_18"]
    #[inline(always)]
    pub const fn efuse1_row_18(&self) -> &Efuse1Row18 {
        &self.efuse1_row_18
    }
    #[doc = "0x444 - EFUSE1_ROW_19"]
    #[inline(always)]
    pub const fn efuse1_row_19(&self) -> &Efuse1Row19 {
        &self.efuse1_row_19
    }
    #[doc = "0x448 - EFUSE1_ROW_20"]
    #[inline(always)]
    pub const fn efuse1_row_20(&self) -> &Efuse1Row20 {
        &self.efuse1_row_20
    }
    #[doc = "0x44c - EFUSE1_ROW_21"]
    #[inline(always)]
    pub const fn efuse1_row_21(&self) -> &Efuse1Row21 {
        &self.efuse1_row_21
    }
    #[doc = "0x450 - EFUSE1_ROW_22"]
    #[inline(always)]
    pub const fn efuse1_row_22(&self) -> &Efuse1Row22 {
        &self.efuse1_row_22
    }
    #[doc = "0x454 - EFUSE1_ROW_23"]
    #[inline(always)]
    pub const fn efuse1_row_23(&self) -> &Efuse1Row23 {
        &self.efuse1_row_23
    }
    #[doc = "0x458 - EFUSE1_ROW_24"]
    #[inline(always)]
    pub const fn efuse1_row_24(&self) -> &Efuse1Row24 {
        &self.efuse1_row_24
    }
    #[doc = "0x45c - EFUSE1_ROW_25"]
    #[inline(always)]
    pub const fn efuse1_row_25(&self) -> &Efuse1Row25 {
        &self.efuse1_row_25
    }
    #[doc = "0x460 - EFUSE1_ROW_26"]
    #[inline(always)]
    pub const fn efuse1_row_26(&self) -> &Efuse1Row26 {
        &self.efuse1_row_26
    }
    #[doc = "0x464 - EFUSE1_ROW_27"]
    #[inline(always)]
    pub const fn efuse1_row_27(&self) -> &Efuse1Row27 {
        &self.efuse1_row_27
    }
    #[doc = "0x468 - EFUSE1_ROW_28"]
    #[inline(always)]
    pub const fn efuse1_row_28(&self) -> &Efuse1Row28 {
        &self.efuse1_row_28
    }
    #[doc = "0x46c - EFUSE1_ROW_29"]
    #[inline(always)]
    pub const fn efuse1_row_29(&self) -> &Efuse1Row29 {
        &self.efuse1_row_29
    }
    #[doc = "0x470 - EFUSE1_ROW_30"]
    #[inline(always)]
    pub const fn efuse1_row_30(&self) -> &Efuse1Row30 {
        &self.efuse1_row_30
    }
    #[doc = "0x474 - EFUSE1_ROW_31"]
    #[inline(always)]
    pub const fn efuse1_row_31(&self) -> &Efuse1Row31 {
        &self.efuse1_row_31
    }
    #[doc = "0x478 - EFUSE1_ROW_32"]
    #[inline(always)]
    pub const fn efuse1_row_32(&self) -> &Efuse1Row32 {
        &self.efuse1_row_32
    }
    #[doc = "0x47c - EFUSE1_ROW_33"]
    #[inline(always)]
    pub const fn efuse1_row_33(&self) -> &Efuse1Row33 {
        &self.efuse1_row_33
    }
    #[doc = "0x480 - EFUSE1_ROW_34"]
    #[inline(always)]
    pub const fn efuse1_row_34(&self) -> &Efuse1Row34 {
        &self.efuse1_row_34
    }
    #[doc = "0x484 - EFUSE1_ROW_35"]
    #[inline(always)]
    pub const fn efuse1_row_35(&self) -> &Efuse1Row35 {
        &self.efuse1_row_35
    }
    #[doc = "0x488 - EFUSE1_ROW_36"]
    #[inline(always)]
    pub const fn efuse1_row_36(&self) -> &Efuse1Row36 {
        &self.efuse1_row_36
    }
    #[doc = "0x48c - EFUSE1_ROW_37"]
    #[inline(always)]
    pub const fn efuse1_row_37(&self) -> &Efuse1Row37 {
        &self.efuse1_row_37
    }
    #[doc = "0x490 - EFUSE1_ROW_38"]
    #[inline(always)]
    pub const fn efuse1_row_38(&self) -> &Efuse1Row38 {
        &self.efuse1_row_38
    }
    #[doc = "0x494 - EFUSE1_ROW_39"]
    #[inline(always)]
    pub const fn efuse1_row_39(&self) -> &Efuse1Row39 {
        &self.efuse1_row_39
    }
    #[doc = "0x498 - EFUSE1_ROW_40"]
    #[inline(always)]
    pub const fn efuse1_row_40(&self) -> &Efuse1Row40 {
        &self.efuse1_row_40
    }
    #[doc = "0x49c - EFUSE1_ROW_41"]
    #[inline(always)]
    pub const fn efuse1_row_41(&self) -> &Efuse1Row41 {
        &self.efuse1_row_41
    }
    #[doc = "0x4a0 - EFUSE1_ROW_42"]
    #[inline(always)]
    pub const fn efuse1_row_42(&self) -> &Efuse1Row42 {
        &self.efuse1_row_42
    }
    #[doc = "0x4a4 - EFUSE1_ROW_43"]
    #[inline(always)]
    pub const fn efuse1_row_43(&self) -> &Efuse1Row43 {
        &self.efuse1_row_43
    }
    #[doc = "0x800 - EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN"]
    #[inline(always)]
    pub const fn efuse_override_hsm_halt_on_rom_ecc_err_en(
        &self,
    ) -> &EfuseOverrideHsmHaltOnRomEccErrEn {
        &self.efuse_override_hsm_halt_on_rom_ecc_err_en
    }
    #[doc = "0x804 - EFUSE_OVERRIDE_MEM_MARGINCTRL"]
    #[inline(always)]
    pub const fn efuse_override_mem_marginctrl(&self) -> &EfuseOverrideMemMarginctrl {
        &self.efuse_override_mem_marginctrl
    }
    #[doc = "0x808 - EFUSE_OVERRIDE_LVDS_BGAP_TRIM"]
    #[inline(always)]
    pub const fn efuse_override_lvds_bgap_trim(&self) -> &EfuseOverrideLvdsBgapTrim {
        &self.efuse_override_lvds_bgap_trim
    }
    #[doc = "0x80c - EFUSE_OVERRIDE_XTAL_STABLIZATION_WAIT"]
    #[inline(always)]
    pub const fn efuse_override_xtal_stablization_wait(
        &self,
    ) -> &EfuseOverrideXtalStablizationWait {
        &self.efuse_override_xtal_stablization_wait
    }
    #[doc = "0x810 - EFUSE_OVERRIDE_SLICER_BIAS_RTRIM"]
    #[inline(always)]
    pub const fn efuse_override_slicer_bias_rtrim(&self) -> &EfuseOverrideSlicerBiasRtrim {
        &self.efuse_override_slicer_bias_rtrim
    }
    #[doc = "0x814 - EFUSE_OVERRIDE_XO_OUTPUT_DRIVE"]
    #[inline(always)]
    pub const fn efuse_override_xo_output_drive(&self) -> &EfuseOverrideXoOutputDrive {
        &self.efuse_override_xo_output_drive
    }
    #[doc = "0x818 - EFUSE_OVERRIDE_RCOSC_TRIM_CODE"]
    #[inline(always)]
    pub const fn efuse_override_rcosc_trim_code(&self) -> &EfuseOverrideRcoscTrimCode {
        &self.efuse_override_rcosc_trim_code
    }
    #[doc = "0x81c - EFUSE_OVERRIDE_IP1_BG1_RTRIM"]
    #[inline(always)]
    pub const fn efuse_override_ip1_bg1_rtrim(&self) -> &EfuseOverrideIp1Bg1Rtrim {
        &self.efuse_override_ip1_bg1_rtrim
    }
    #[doc = "0x820 - EFUSE_OVERRIDE_IP1_BG1_SLOPE"]
    #[inline(always)]
    pub const fn efuse_override_ip1_bg1_slope(&self) -> &EfuseOverrideIp1Bg1Slope {
        &self.efuse_override_ip1_bg1_slope
    }
    #[doc = "0x824 - EFUSE_OVERRIDE_IP1_BG1_MAG"]
    #[inline(always)]
    pub const fn efuse_override_ip1_bg1_mag(&self) -> &EfuseOverrideIp1Bg1Mag {
        &self.efuse_override_ip1_bg1_mag
    }
    #[doc = "0x828 - EFUSE_OVERRIDE_RS232_CLKMODE"]
    #[inline(always)]
    pub const fn efuse_override_rs232_clkmode(&self) -> &EfuseOverrideRs232Clkmode {
        &self.efuse_override_rs232_clkmode
    }
    #[doc = "0x82c - EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM"]
    #[inline(always)]
    pub const fn efuse_override_vmon_vdd_ov_uv_trim(&self) -> &EfuseOverrideVmonVddOvUvTrim {
        &self.efuse_override_vmon_vdd_ov_uv_trim
    }
    #[doc = "0x830 - EFUSE_OVERRIDE_VMON_VDDS_3P3_UV_TRIM"]
    #[inline(always)]
    pub const fn efuse_override_vmon_vdds_3p3_uv_trim(&self) -> &EfuseOverrideVmonVdds3p3UvTrim {
        &self.efuse_override_vmon_vdds_3p3_uv_trim
    }
    #[doc = "0x834 - EFUSE_OVERRIDE_VMON_VDDA_OSC_TRIM"]
    #[inline(always)]
    pub const fn efuse_override_vmon_vdda_osc_trim(&self) -> &EfuseOverrideVmonVddaOscTrim {
        &self.efuse_override_vmon_vdda_osc_trim
    }
    #[doc = "0x838 - EFUSE_OVERRIDE_VDD_VT_DET"]
    #[inline(always)]
    pub const fn efuse_override_vdd_vt_det(&self) -> &EfuseOverrideVddVtDet {
        &self.efuse_override_vdd_vt_det
    }
    #[doc = "0x83c - EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_VAL"]
    #[inline(always)]
    pub const fn efuse_override_mask_cpu_clk_out_ctrl_lowv_val(
        &self,
    ) -> &EfuseOverrideMaskCpuClkOutCtrlLowvVal {
        &self.efuse_override_mask_cpu_clk_out_ctrl_lowv_val
    }
    #[doc = "0x840 - EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_SEL"]
    #[inline(always)]
    pub const fn efuse_override_mask_cpu_clk_out_ctrl_lowv_sel(
        &self,
    ) -> &EfuseOverrideMaskCpuClkOutCtrlLowvSel {
        &self.efuse_override_mask_cpu_clk_out_ctrl_lowv_sel
    }
    #[doc = "0x844 - EFUSE_OVERRIDE_EN_VOL_MON_FUNC"]
    #[inline(always)]
    pub const fn efuse_override_en_vol_mon_func(&self) -> &EfuseOverrideEnVolMonFunc {
        &self.efuse_override_en_vol_mon_func
    }
    #[doc = "0x848 - EFUSE_OVERRIDE_EN_VOL_MON_FUNC"]
    #[inline(always)]
    pub const fn efuse_override_en_vol_mon_func_(&self) -> &EfuseOverrideEnVolMonFunc_ {
        &self.efuse_override_en_vol_mon_func_
    }
    #[doc = "0x84c - EFUSE_OVERRIDE_SPARE_ANA"]
    #[inline(always)]
    pub const fn efuse_override_spare_ana(&self) -> &EfuseOverrideSpareAna {
        &self.efuse_override_spare_ana
    }
    #[doc = "0x850 - EFUSE_OVERRIDE_SLICER_DLY_DISABLE"]
    #[inline(always)]
    pub const fn efuse_override_slicer_dly_disable(&self) -> &EfuseOverrideSlicerDlyDisable {
        &self.efuse_override_slicer_dly_disable
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
#[doc = "MDO_CTRL (rw) register accessor: MDO_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mdo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdo_ctrl`]
module"]
#[doc(alias = "MDO_CTRL")]
pub type MdoCtrl = crate::Reg<mdo_ctrl::MdoCtrlSpec>;
#[doc = "MDO_CTRL"]
pub mod mdo_ctrl;
#[doc = "PROBE_BUS_SEL0 (rw) register accessor: PROBE_BUS_SEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_bus_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probe_bus_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_bus_sel0`]
module"]
#[doc(alias = "PROBE_BUS_SEL0")]
pub type ProbeBusSel0 = crate::Reg<probe_bus_sel0::ProbeBusSel0Spec>;
#[doc = "PROBE_BUS_SEL0"]
pub mod probe_bus_sel0;
#[doc = "PROBE_BUS_SEL1 (rw) register accessor: PROBE_BUS_SEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_bus_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probe_bus_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_bus_sel1`]
module"]
#[doc(alias = "PROBE_BUS_SEL1")]
pub type ProbeBusSel1 = crate::Reg<probe_bus_sel1::ProbeBusSel1Spec>;
#[doc = "PROBE_BUS_SEL1"]
pub mod probe_bus_sel1;
#[doc = "RS232_SLEEP_CLK_DIV (rw) register accessor: RS232_SLEEP_CLK_DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_sleep_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_sleep_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_sleep_clk_div`]
module"]
#[doc(alias = "RS232_SLEEP_CLK_DIV")]
pub type Rs232SleepClkDiv = crate::Reg<rs232_sleep_clk_div::Rs232SleepClkDivSpec>;
#[doc = "RS232_SLEEP_CLK_DIV"]
pub mod rs232_sleep_clk_div;
#[doc = "RS232_SLEEP_CLK_DIV_by2 (rw) register accessor: RS232_SLEEP_CLK_DIV_by2\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_sleep_clk_div_by2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_sleep_clk_div_by2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_sleep_clk_div_by2`]
module"]
#[doc(alias = "RS232_SLEEP_CLK_DIV_by2")]
pub type Rs232SleepClkDivBy2 = crate::Reg<rs232_sleep_clk_div_by2::Rs232SleepClkDivBy2Spec>;
#[doc = "RS232_SLEEP_CLK_DIV_by2"]
pub mod rs232_sleep_clk_div_by2;
#[doc = "RS232_STATUS (rw) register accessor: RS232_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_status`]
module"]
#[doc(alias = "RS232_STATUS")]
pub type Rs232Status = crate::Reg<rs232_status::Rs232StatusSpec>;
#[doc = "RS232_STATUS"]
pub mod rs232_status;
#[doc = "EFUSE_DIEID0 (rw) register accessor: EFUSE_DIEID0\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_dieid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_dieid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_dieid0`]
module"]
#[doc(alias = "EFUSE_DIEID0")]
pub type EfuseDieid0 = crate::Reg<efuse_dieid0::EfuseDieid0Spec>;
#[doc = "EFUSE_DIEID0"]
pub mod efuse_dieid0;
#[doc = "EFUSE_DIEID1 (rw) register accessor: EFUSE_DIEID1\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_dieid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_dieid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_dieid1`]
module"]
#[doc(alias = "EFUSE_DIEID1")]
pub type EfuseDieid1 = crate::Reg<efuse_dieid1::EfuseDieid1Spec>;
#[doc = "EFUSE_DIEID1"]
pub mod efuse_dieid1;
#[doc = "EFUSE_DIEID2 (rw) register accessor: EFUSE_DIEID2\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_dieid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_dieid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_dieid2`]
module"]
#[doc(alias = "EFUSE_DIEID2")]
pub type EfuseDieid2 = crate::Reg<efuse_dieid2::EfuseDieid2Spec>;
#[doc = "EFUSE_DIEID2"]
pub mod efuse_dieid2;
#[doc = "EFUSE_DIEID3 (rw) register accessor: EFUSE_DIEID3\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_dieid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_dieid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_dieid3`]
module"]
#[doc(alias = "EFUSE_DIEID3")]
pub type EfuseDieid3 = crate::Reg<efuse_dieid3::EfuseDieid3Spec>;
#[doc = "EFUSE_DIEID3"]
pub mod efuse_dieid3;
#[doc = "EFUSE_UID0 (rw) register accessor: EFUSE_UID0\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_uid0`]
module"]
#[doc(alias = "EFUSE_UID0")]
pub type EfuseUid0 = crate::Reg<efuse_uid0::EfuseUid0Spec>;
#[doc = "EFUSE_UID0"]
pub mod efuse_uid0;
#[doc = "EFUSE_UID1 (rw) register accessor: EFUSE_UID1\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_uid1`]
module"]
#[doc(alias = "EFUSE_UID1")]
pub type EfuseUid1 = crate::Reg<efuse_uid1::EfuseUid1Spec>;
#[doc = "EFUSE_UID1"]
pub mod efuse_uid1;
#[doc = "EFUSE_UID2 (rw) register accessor: EFUSE_UID2\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_uid2`]
module"]
#[doc(alias = "EFUSE_UID2")]
pub type EfuseUid2 = crate::Reg<efuse_uid2::EfuseUid2Spec>;
#[doc = "EFUSE_UID2"]
pub mod efuse_uid2;
#[doc = "EFUSE_UID3 (rw) register accessor: EFUSE_UID3\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_uid3`]
module"]
#[doc(alias = "EFUSE_UID3")]
pub type EfuseUid3 = crate::Reg<efuse_uid3::EfuseUid3Spec>;
#[doc = "EFUSE_UID3"]
pub mod efuse_uid3;
#[doc = "PREVIOUS_NAME (rw) register accessor: PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@previous_name`]
module"]
#[doc(alias = "PREVIOUS_NAME")]
pub type PreviousName = crate::Reg<previous_name::PreviousNameSpec>;
#[doc = "PREVIOUS_NAME"]
pub mod previous_name;
#[doc = "EFUSE_FROM0_CHECKSUM (rw) register accessor: EFUSE_FROM0_CHECKSUM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_from0_checksum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_from0_checksum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_from0_checksum`]
module"]
#[doc(alias = "EFUSE_FROM0_CHECKSUM")]
pub type EfuseFrom0Checksum = crate::Reg<efuse_from0_checksum::EfuseFrom0ChecksumSpec>;
#[doc = "EFUSE_FROM0_CHECKSUM"]
pub mod efuse_from0_checksum;
#[doc = "EFUSE_ROM_SEQ_UPDATE0 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE0\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update0`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE0")]
pub type EfuseRomSeqUpdate0 = crate::Reg<efuse_rom_seq_update0::EfuseRomSeqUpdate0Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE0"]
pub mod efuse_rom_seq_update0;
#[doc = "EFUSE_ROM_SEQ_UPDATE1 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE1\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update1`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE1")]
pub type EfuseRomSeqUpdate1 = crate::Reg<efuse_rom_seq_update1::EfuseRomSeqUpdate1Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE1"]
pub mod efuse_rom_seq_update1;
#[doc = "EFUSE_ROM_SEQ_UPDATE2 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update2`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE2")]
pub type EfuseRomSeqUpdate2 = crate::Reg<efuse_rom_seq_update2::EfuseRomSeqUpdate2Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE2"]
pub mod efuse_rom_seq_update2;
#[doc = "EFUSE_ROM_SEQ_UPDATE3 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE3\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update3`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE3")]
pub type EfuseRomSeqUpdate3 = crate::Reg<efuse_rom_seq_update3::EfuseRomSeqUpdate3Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE3"]
pub mod efuse_rom_seq_update3;
#[doc = "EFUSE_ROM_SEQ_UPDATE4 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE4\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update4`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE4")]
pub type EfuseRomSeqUpdate4 = crate::Reg<efuse_rom_seq_update4::EfuseRomSeqUpdate4Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE4"]
pub mod efuse_rom_seq_update4;
#[doc = "EFUSE_ROM_SEQ_UPDATE5 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE5\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update5`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE5")]
pub type EfuseRomSeqUpdate5 = crate::Reg<efuse_rom_seq_update5::EfuseRomSeqUpdate5Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE5"]
pub mod efuse_rom_seq_update5;
#[doc = "EFUSE_ROM_SEQ_UPDATE6 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE6\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update6`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE6")]
pub type EfuseRomSeqUpdate6 = crate::Reg<efuse_rom_seq_update6::EfuseRomSeqUpdate6Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE6"]
pub mod efuse_rom_seq_update6;
#[doc = "EFUSE_ROM_SEQ_UPDATE7 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE7\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update7`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE7")]
pub type EfuseRomSeqUpdate7 = crate::Reg<efuse_rom_seq_update7::EfuseRomSeqUpdate7Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE7"]
pub mod efuse_rom_seq_update7;
#[doc = "EFUSE_ROM_SEQ_UPDATE8 (rw) register accessor: EFUSE_ROM_SEQ_UPDATE8\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_rom_seq_update8`]
module"]
#[doc(alias = "EFUSE_ROM_SEQ_UPDATE8")]
pub type EfuseRomSeqUpdate8 = crate::Reg<efuse_rom_seq_update8::EfuseRomSeqUpdate8Spec>;
#[doc = "EFUSE_ROM_SEQ_UPDATE8"]
pub mod efuse_rom_seq_update8;
#[doc = "EFUSE0_ROW_61 (rw) register accessor: EFUSE0_ROW_61\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse0_row_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse0_row_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse0_row_61`]
module"]
#[doc(alias = "EFUSE0_ROW_61")]
pub type Efuse0Row61 = crate::Reg<efuse0_row_61::Efuse0Row61Spec>;
#[doc = "EFUSE0_ROW_61"]
pub mod efuse0_row_61;
#[doc = "EFUSE0_ROW_62 (rw) register accessor: EFUSE0_ROW_62\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse0_row_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse0_row_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse0_row_62`]
module"]
#[doc(alias = "EFUSE0_ROW_62")]
pub type Efuse0Row62 = crate::Reg<efuse0_row_62::Efuse0Row62Spec>;
#[doc = "EFUSE0_ROW_62"]
pub mod efuse0_row_62;
#[doc = "EFUSE0_ROW_63 (rw) register accessor: EFUSE0_ROW_63\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse0_row_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse0_row_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse0_row_63`]
module"]
#[doc(alias = "EFUSE0_ROW_63")]
pub type Efuse0Row63 = crate::Reg<efuse0_row_63::Efuse0Row63Spec>;
#[doc = "EFUSE0_ROW_63"]
pub mod efuse0_row_63;
#[doc = "EFUSE1_ROW_5 (rw) register accessor: EFUSE1_ROW_5\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_5`]
module"]
#[doc(alias = "EFUSE1_ROW_5")]
pub type Efuse1Row5 = crate::Reg<efuse1_row_5::Efuse1Row5Spec>;
#[doc = "EFUSE1_ROW_5"]
pub mod efuse1_row_5;
#[doc = "EFUSE1_ROW_6 (rw) register accessor: EFUSE1_ROW_6\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_6`]
module"]
#[doc(alias = "EFUSE1_ROW_6")]
pub type Efuse1Row6 = crate::Reg<efuse1_row_6::Efuse1Row6Spec>;
#[doc = "EFUSE1_ROW_6"]
pub mod efuse1_row_6;
#[doc = "EFUSE1_ROW_7 (rw) register accessor: EFUSE1_ROW_7\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_7`]
module"]
#[doc(alias = "EFUSE1_ROW_7")]
pub type Efuse1Row7 = crate::Reg<efuse1_row_7::Efuse1Row7Spec>;
#[doc = "EFUSE1_ROW_7"]
pub mod efuse1_row_7;
#[doc = "EFUSE1_ROW_8 (rw) register accessor: EFUSE1_ROW_8\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_8`]
module"]
#[doc(alias = "EFUSE1_ROW_8")]
pub type Efuse1Row8 = crate::Reg<efuse1_row_8::Efuse1Row8Spec>;
#[doc = "EFUSE1_ROW_8"]
pub mod efuse1_row_8;
#[doc = "EFUSE1_ROW_9 (rw) register accessor: EFUSE1_ROW_9\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_9`]
module"]
#[doc(alias = "EFUSE1_ROW_9")]
pub type Efuse1Row9 = crate::Reg<efuse1_row_9::Efuse1Row9Spec>;
#[doc = "EFUSE1_ROW_9"]
pub mod efuse1_row_9;
#[doc = "EFUSE1_ROW_10 (rw) register accessor: EFUSE1_ROW_10\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_10`]
module"]
#[doc(alias = "EFUSE1_ROW_10")]
pub type Efuse1Row10 = crate::Reg<efuse1_row_10::Efuse1Row10Spec>;
#[doc = "EFUSE1_ROW_10"]
pub mod efuse1_row_10;
#[doc = "EFUSE1_ROW_11 (rw) register accessor: EFUSE1_ROW_11\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_11`]
module"]
#[doc(alias = "EFUSE1_ROW_11")]
pub type Efuse1Row11 = crate::Reg<efuse1_row_11::Efuse1Row11Spec>;
#[doc = "EFUSE1_ROW_11"]
pub mod efuse1_row_11;
#[doc = "EFUSE1_ROW_12 (rw) register accessor: EFUSE1_ROW_12\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_12`]
module"]
#[doc(alias = "EFUSE1_ROW_12")]
pub type Efuse1Row12 = crate::Reg<efuse1_row_12::Efuse1Row12Spec>;
#[doc = "EFUSE1_ROW_12"]
pub mod efuse1_row_12;
#[doc = "EFUSE1_ROW_13 (rw) register accessor: EFUSE1_ROW_13\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_13`]
module"]
#[doc(alias = "EFUSE1_ROW_13")]
pub type Efuse1Row13 = crate::Reg<efuse1_row_13::Efuse1Row13Spec>;
#[doc = "EFUSE1_ROW_13"]
pub mod efuse1_row_13;
#[doc = "EFUSE1_ROW_14 (rw) register accessor: EFUSE1_ROW_14\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_14`]
module"]
#[doc(alias = "EFUSE1_ROW_14")]
pub type Efuse1Row14 = crate::Reg<efuse1_row_14::Efuse1Row14Spec>;
#[doc = "EFUSE1_ROW_14"]
pub mod efuse1_row_14;
#[doc = "EFUSE1_ROW_15 (rw) register accessor: EFUSE1_ROW_15\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_15`]
module"]
#[doc(alias = "EFUSE1_ROW_15")]
pub type Efuse1Row15 = crate::Reg<efuse1_row_15::Efuse1Row15Spec>;
#[doc = "EFUSE1_ROW_15"]
pub mod efuse1_row_15;
#[doc = "EFUSE1_ROW_16 (rw) register accessor: EFUSE1_ROW_16\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_16`]
module"]
#[doc(alias = "EFUSE1_ROW_16")]
pub type Efuse1Row16 = crate::Reg<efuse1_row_16::Efuse1Row16Spec>;
#[doc = "EFUSE1_ROW_16"]
pub mod efuse1_row_16;
#[doc = "EFUSE1_ROW_17 (rw) register accessor: EFUSE1_ROW_17\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_17`]
module"]
#[doc(alias = "EFUSE1_ROW_17")]
pub type Efuse1Row17 = crate::Reg<efuse1_row_17::Efuse1Row17Spec>;
#[doc = "EFUSE1_ROW_17"]
pub mod efuse1_row_17;
#[doc = "EFUSE1_ROW_18 (rw) register accessor: EFUSE1_ROW_18\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_18`]
module"]
#[doc(alias = "EFUSE1_ROW_18")]
pub type Efuse1Row18 = crate::Reg<efuse1_row_18::Efuse1Row18Spec>;
#[doc = "EFUSE1_ROW_18"]
pub mod efuse1_row_18;
#[doc = "EFUSE1_ROW_19 (rw) register accessor: EFUSE1_ROW_19\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_19`]
module"]
#[doc(alias = "EFUSE1_ROW_19")]
pub type Efuse1Row19 = crate::Reg<efuse1_row_19::Efuse1Row19Spec>;
#[doc = "EFUSE1_ROW_19"]
pub mod efuse1_row_19;
#[doc = "EFUSE1_ROW_20 (rw) register accessor: EFUSE1_ROW_20\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_20`]
module"]
#[doc(alias = "EFUSE1_ROW_20")]
pub type Efuse1Row20 = crate::Reg<efuse1_row_20::Efuse1Row20Spec>;
#[doc = "EFUSE1_ROW_20"]
pub mod efuse1_row_20;
#[doc = "EFUSE1_ROW_21 (rw) register accessor: EFUSE1_ROW_21\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_21`]
module"]
#[doc(alias = "EFUSE1_ROW_21")]
pub type Efuse1Row21 = crate::Reg<efuse1_row_21::Efuse1Row21Spec>;
#[doc = "EFUSE1_ROW_21"]
pub mod efuse1_row_21;
#[doc = "EFUSE1_ROW_22 (rw) register accessor: EFUSE1_ROW_22\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_22`]
module"]
#[doc(alias = "EFUSE1_ROW_22")]
pub type Efuse1Row22 = crate::Reg<efuse1_row_22::Efuse1Row22Spec>;
#[doc = "EFUSE1_ROW_22"]
pub mod efuse1_row_22;
#[doc = "EFUSE1_ROW_23 (rw) register accessor: EFUSE1_ROW_23\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_23`]
module"]
#[doc(alias = "EFUSE1_ROW_23")]
pub type Efuse1Row23 = crate::Reg<efuse1_row_23::Efuse1Row23Spec>;
#[doc = "EFUSE1_ROW_23"]
pub mod efuse1_row_23;
#[doc = "EFUSE1_ROW_24 (rw) register accessor: EFUSE1_ROW_24\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_24`]
module"]
#[doc(alias = "EFUSE1_ROW_24")]
pub type Efuse1Row24 = crate::Reg<efuse1_row_24::Efuse1Row24Spec>;
#[doc = "EFUSE1_ROW_24"]
pub mod efuse1_row_24;
#[doc = "EFUSE1_ROW_25 (rw) register accessor: EFUSE1_ROW_25\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_25`]
module"]
#[doc(alias = "EFUSE1_ROW_25")]
pub type Efuse1Row25 = crate::Reg<efuse1_row_25::Efuse1Row25Spec>;
#[doc = "EFUSE1_ROW_25"]
pub mod efuse1_row_25;
#[doc = "EFUSE1_ROW_26 (rw) register accessor: EFUSE1_ROW_26\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_26`]
module"]
#[doc(alias = "EFUSE1_ROW_26")]
pub type Efuse1Row26 = crate::Reg<efuse1_row_26::Efuse1Row26Spec>;
#[doc = "EFUSE1_ROW_26"]
pub mod efuse1_row_26;
#[doc = "EFUSE1_ROW_27 (rw) register accessor: EFUSE1_ROW_27\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_27`]
module"]
#[doc(alias = "EFUSE1_ROW_27")]
pub type Efuse1Row27 = crate::Reg<efuse1_row_27::Efuse1Row27Spec>;
#[doc = "EFUSE1_ROW_27"]
pub mod efuse1_row_27;
#[doc = "EFUSE1_ROW_28 (rw) register accessor: EFUSE1_ROW_28\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_28`]
module"]
#[doc(alias = "EFUSE1_ROW_28")]
pub type Efuse1Row28 = crate::Reg<efuse1_row_28::Efuse1Row28Spec>;
#[doc = "EFUSE1_ROW_28"]
pub mod efuse1_row_28;
#[doc = "EFUSE1_ROW_29 (rw) register accessor: EFUSE1_ROW_29\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_29`]
module"]
#[doc(alias = "EFUSE1_ROW_29")]
pub type Efuse1Row29 = crate::Reg<efuse1_row_29::Efuse1Row29Spec>;
#[doc = "EFUSE1_ROW_29"]
pub mod efuse1_row_29;
#[doc = "EFUSE1_ROW_30 (rw) register accessor: EFUSE1_ROW_30\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_30`]
module"]
#[doc(alias = "EFUSE1_ROW_30")]
pub type Efuse1Row30 = crate::Reg<efuse1_row_30::Efuse1Row30Spec>;
#[doc = "EFUSE1_ROW_30"]
pub mod efuse1_row_30;
#[doc = "EFUSE1_ROW_31 (rw) register accessor: EFUSE1_ROW_31\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_31`]
module"]
#[doc(alias = "EFUSE1_ROW_31")]
pub type Efuse1Row31 = crate::Reg<efuse1_row_31::Efuse1Row31Spec>;
#[doc = "EFUSE1_ROW_31"]
pub mod efuse1_row_31;
#[doc = "EFUSE1_ROW_32 (rw) register accessor: EFUSE1_ROW_32\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_32`]
module"]
#[doc(alias = "EFUSE1_ROW_32")]
pub type Efuse1Row32 = crate::Reg<efuse1_row_32::Efuse1Row32Spec>;
#[doc = "EFUSE1_ROW_32"]
pub mod efuse1_row_32;
#[doc = "EFUSE1_ROW_33 (rw) register accessor: EFUSE1_ROW_33\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_33`]
module"]
#[doc(alias = "EFUSE1_ROW_33")]
pub type Efuse1Row33 = crate::Reg<efuse1_row_33::Efuse1Row33Spec>;
#[doc = "EFUSE1_ROW_33"]
pub mod efuse1_row_33;
#[doc = "EFUSE1_ROW_34 (rw) register accessor: EFUSE1_ROW_34\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_34`]
module"]
#[doc(alias = "EFUSE1_ROW_34")]
pub type Efuse1Row34 = crate::Reg<efuse1_row_34::Efuse1Row34Spec>;
#[doc = "EFUSE1_ROW_34"]
pub mod efuse1_row_34;
#[doc = "EFUSE1_ROW_35 (rw) register accessor: EFUSE1_ROW_35\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_35`]
module"]
#[doc(alias = "EFUSE1_ROW_35")]
pub type Efuse1Row35 = crate::Reg<efuse1_row_35::Efuse1Row35Spec>;
#[doc = "EFUSE1_ROW_35"]
pub mod efuse1_row_35;
#[doc = "EFUSE1_ROW_36 (rw) register accessor: EFUSE1_ROW_36\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_36`]
module"]
#[doc(alias = "EFUSE1_ROW_36")]
pub type Efuse1Row36 = crate::Reg<efuse1_row_36::Efuse1Row36Spec>;
#[doc = "EFUSE1_ROW_36"]
pub mod efuse1_row_36;
#[doc = "EFUSE1_ROW_37 (rw) register accessor: EFUSE1_ROW_37\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_37`]
module"]
#[doc(alias = "EFUSE1_ROW_37")]
pub type Efuse1Row37 = crate::Reg<efuse1_row_37::Efuse1Row37Spec>;
#[doc = "EFUSE1_ROW_37"]
pub mod efuse1_row_37;
#[doc = "EFUSE1_ROW_38 (rw) register accessor: EFUSE1_ROW_38\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_38`]
module"]
#[doc(alias = "EFUSE1_ROW_38")]
pub type Efuse1Row38 = crate::Reg<efuse1_row_38::Efuse1Row38Spec>;
#[doc = "EFUSE1_ROW_38"]
pub mod efuse1_row_38;
#[doc = "EFUSE1_ROW_39 (rw) register accessor: EFUSE1_ROW_39\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_39`]
module"]
#[doc(alias = "EFUSE1_ROW_39")]
pub type Efuse1Row39 = crate::Reg<efuse1_row_39::Efuse1Row39Spec>;
#[doc = "EFUSE1_ROW_39"]
pub mod efuse1_row_39;
#[doc = "EFUSE1_ROW_40 (rw) register accessor: EFUSE1_ROW_40\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_40`]
module"]
#[doc(alias = "EFUSE1_ROW_40")]
pub type Efuse1Row40 = crate::Reg<efuse1_row_40::Efuse1Row40Spec>;
#[doc = "EFUSE1_ROW_40"]
pub mod efuse1_row_40;
#[doc = "EFUSE1_ROW_41 (rw) register accessor: EFUSE1_ROW_41\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_41`]
module"]
#[doc(alias = "EFUSE1_ROW_41")]
pub type Efuse1Row41 = crate::Reg<efuse1_row_41::Efuse1Row41Spec>;
#[doc = "EFUSE1_ROW_41"]
pub mod efuse1_row_41;
#[doc = "EFUSE1_ROW_42 (rw) register accessor: EFUSE1_ROW_42\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_42`]
module"]
#[doc(alias = "EFUSE1_ROW_42")]
pub type Efuse1Row42 = crate::Reg<efuse1_row_42::Efuse1Row42Spec>;
#[doc = "EFUSE1_ROW_42"]
pub mod efuse1_row_42;
#[doc = "EFUSE1_ROW_43 (rw) register accessor: EFUSE1_ROW_43\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse1_row_43`]
module"]
#[doc(alias = "EFUSE1_ROW_43")]
pub type Efuse1Row43 = crate::Reg<efuse1_row_43::Efuse1Row43Spec>;
#[doc = "EFUSE1_ROW_43"]
pub mod efuse1_row_43;
#[doc = "EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN (rw) register accessor: EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_hsm_halt_on_rom_ecc_err_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_hsm_halt_on_rom_ecc_err_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_hsm_halt_on_rom_ecc_err_en`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN")]
pub type EfuseOverrideHsmHaltOnRomEccErrEn =
    crate::Reg<efuse_override_hsm_halt_on_rom_ecc_err_en::EfuseOverrideHsmHaltOnRomEccErrEnSpec>;
#[doc = "EFUSE_OVERRIDE_HSM_HALT_ON_ROM_ECC_ERR_EN"]
pub mod efuse_override_hsm_halt_on_rom_ecc_err_en;
#[doc = "EFUSE_OVERRIDE_MEM_MARGINCTRL (rw) register accessor: EFUSE_OVERRIDE_MEM_MARGINCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_mem_marginctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_mem_marginctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_mem_marginctrl`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_MEM_MARGINCTRL")]
pub type EfuseOverrideMemMarginctrl =
    crate::Reg<efuse_override_mem_marginctrl::EfuseOverrideMemMarginctrlSpec>;
#[doc = "EFUSE_OVERRIDE_MEM_MARGINCTRL"]
pub mod efuse_override_mem_marginctrl;
#[doc = "EFUSE_OVERRIDE_LVDS_BGAP_TRIM (rw) register accessor: EFUSE_OVERRIDE_LVDS_BGAP_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_lvds_bgap_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_lvds_bgap_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_lvds_bgap_trim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_LVDS_BGAP_TRIM")]
pub type EfuseOverrideLvdsBgapTrim =
    crate::Reg<efuse_override_lvds_bgap_trim::EfuseOverrideLvdsBgapTrimSpec>;
#[doc = "EFUSE_OVERRIDE_LVDS_BGAP_TRIM"]
pub mod efuse_override_lvds_bgap_trim;
#[doc = "EFUSE_OVERRIDE_XTAL_STABLIZATION_WAIT (rw) register accessor: EFUSE_OVERRIDE_XTAL_STABLIZATION_WAIT\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_xtal_stablization_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_xtal_stablization_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_xtal_stablization_wait`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_XTAL_STABLIZATION_WAIT")]
pub type EfuseOverrideXtalStablizationWait =
    crate::Reg<efuse_override_xtal_stablization_wait::EfuseOverrideXtalStablizationWaitSpec>;
#[doc = "EFUSE_OVERRIDE_XTAL_STABLIZATION_WAIT"]
pub mod efuse_override_xtal_stablization_wait;
#[doc = "EFUSE_OVERRIDE_SLICER_BIAS_RTRIM (rw) register accessor: EFUSE_OVERRIDE_SLICER_BIAS_RTRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_slicer_bias_rtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_slicer_bias_rtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_slicer_bias_rtrim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_SLICER_BIAS_RTRIM")]
pub type EfuseOverrideSlicerBiasRtrim =
    crate::Reg<efuse_override_slicer_bias_rtrim::EfuseOverrideSlicerBiasRtrimSpec>;
#[doc = "EFUSE_OVERRIDE_SLICER_BIAS_RTRIM"]
pub mod efuse_override_slicer_bias_rtrim;
#[doc = "EFUSE_OVERRIDE_XO_OUTPUT_DRIVE (rw) register accessor: EFUSE_OVERRIDE_XO_OUTPUT_DRIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_xo_output_drive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_xo_output_drive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_xo_output_drive`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_XO_OUTPUT_DRIVE")]
pub type EfuseOverrideXoOutputDrive =
    crate::Reg<efuse_override_xo_output_drive::EfuseOverrideXoOutputDriveSpec>;
#[doc = "EFUSE_OVERRIDE_XO_OUTPUT_DRIVE"]
pub mod efuse_override_xo_output_drive;
#[doc = "EFUSE_OVERRIDE_RCOSC_TRIM_CODE (rw) register accessor: EFUSE_OVERRIDE_RCOSC_TRIM_CODE\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_rcosc_trim_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_rcosc_trim_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_rcosc_trim_code`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_RCOSC_TRIM_CODE")]
pub type EfuseOverrideRcoscTrimCode =
    crate::Reg<efuse_override_rcosc_trim_code::EfuseOverrideRcoscTrimCodeSpec>;
#[doc = "EFUSE_OVERRIDE_RCOSC_TRIM_CODE"]
pub mod efuse_override_rcosc_trim_code;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_RTRIM (rw) register accessor: EFUSE_OVERRIDE_IP1_BG1_RTRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_ip1_bg1_rtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_ip1_bg1_rtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_ip1_bg1_rtrim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_IP1_BG1_RTRIM")]
pub type EfuseOverrideIp1Bg1Rtrim =
    crate::Reg<efuse_override_ip1_bg1_rtrim::EfuseOverrideIp1Bg1RtrimSpec>;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_RTRIM"]
pub mod efuse_override_ip1_bg1_rtrim;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_SLOPE (rw) register accessor: EFUSE_OVERRIDE_IP1_BG1_SLOPE\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_ip1_bg1_slope::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_ip1_bg1_slope::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_ip1_bg1_slope`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_IP1_BG1_SLOPE")]
pub type EfuseOverrideIp1Bg1Slope =
    crate::Reg<efuse_override_ip1_bg1_slope::EfuseOverrideIp1Bg1SlopeSpec>;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_SLOPE"]
pub mod efuse_override_ip1_bg1_slope;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_MAG (rw) register accessor: EFUSE_OVERRIDE_IP1_BG1_MAG\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_ip1_bg1_mag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_ip1_bg1_mag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_ip1_bg1_mag`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_IP1_BG1_MAG")]
pub type EfuseOverrideIp1Bg1Mag =
    crate::Reg<efuse_override_ip1_bg1_mag::EfuseOverrideIp1Bg1MagSpec>;
#[doc = "EFUSE_OVERRIDE_IP1_BG1_MAG"]
pub mod efuse_override_ip1_bg1_mag;
#[doc = "EFUSE_OVERRIDE_RS232_CLKMODE (rw) register accessor: EFUSE_OVERRIDE_RS232_CLKMODE\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_rs232_clkmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_rs232_clkmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_rs232_clkmode`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_RS232_CLKMODE")]
pub type EfuseOverrideRs232Clkmode =
    crate::Reg<efuse_override_rs232_clkmode::EfuseOverrideRs232ClkmodeSpec>;
#[doc = "EFUSE_OVERRIDE_RS232_CLKMODE"]
pub mod efuse_override_rs232_clkmode;
#[doc = "EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM (rw) register accessor: EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_vmon_vdd_ov_uv_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_vmon_vdd_ov_uv_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_vmon_vdd_ov_uv_trim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM")]
pub type EfuseOverrideVmonVddOvUvTrim =
    crate::Reg<efuse_override_vmon_vdd_ov_uv_trim::EfuseOverrideVmonVddOvUvTrimSpec>;
#[doc = "EFUSE_OVERRIDE_VMON_VDD_OV_UV_TRIM"]
pub mod efuse_override_vmon_vdd_ov_uv_trim;
#[doc = "EFUSE_OVERRIDE_VMON_VDDS_3P3_UV_TRIM (rw) register accessor: EFUSE_OVERRIDE_VMON_VDDS_3P3_UV_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_vmon_vdds_3p3_uv_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_vmon_vdds_3p3_uv_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_vmon_vdds_3p3_uv_trim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_VMON_VDDS_3P3_UV_TRIM")]
pub type EfuseOverrideVmonVdds3p3UvTrim =
    crate::Reg<efuse_override_vmon_vdds_3p3_uv_trim::EfuseOverrideVmonVdds3p3UvTrimSpec>;
#[doc = "EFUSE_OVERRIDE_VMON_VDDS_3P3_UV_TRIM"]
pub mod efuse_override_vmon_vdds_3p3_uv_trim;
#[doc = "EFUSE_OVERRIDE_VMON_VDDA_OSC_TRIM (rw) register accessor: EFUSE_OVERRIDE_VMON_VDDA_OSC_TRIM\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_vmon_vdda_osc_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_vmon_vdda_osc_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_vmon_vdda_osc_trim`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_VMON_VDDA_OSC_TRIM")]
pub type EfuseOverrideVmonVddaOscTrim =
    crate::Reg<efuse_override_vmon_vdda_osc_trim::EfuseOverrideVmonVddaOscTrimSpec>;
#[doc = "EFUSE_OVERRIDE_VMON_VDDA_OSC_TRIM"]
pub mod efuse_override_vmon_vdda_osc_trim;
#[doc = "EFUSE_OVERRIDE_VDD_VT_DET (rw) register accessor: EFUSE_OVERRIDE_VDD_VT_DET\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_vdd_vt_det::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_vdd_vt_det::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_vdd_vt_det`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_VDD_VT_DET")]
pub type EfuseOverrideVddVtDet = crate::Reg<efuse_override_vdd_vt_det::EfuseOverrideVddVtDetSpec>;
#[doc = "EFUSE_OVERRIDE_VDD_VT_DET"]
pub mod efuse_override_vdd_vt_det;
#[doc = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_VAL (rw) register accessor: EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_mask_cpu_clk_out_ctrl_lowv_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_mask_cpu_clk_out_ctrl_lowv_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_mask_cpu_clk_out_ctrl_lowv_val`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_VAL")]
pub type EfuseOverrideMaskCpuClkOutCtrlLowvVal = crate::Reg<
    efuse_override_mask_cpu_clk_out_ctrl_lowv_val::EfuseOverrideMaskCpuClkOutCtrlLowvValSpec,
>;
#[doc = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_VAL"]
pub mod efuse_override_mask_cpu_clk_out_ctrl_lowv_val;
#[doc = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_SEL (rw) register accessor: EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_mask_cpu_clk_out_ctrl_lowv_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_mask_cpu_clk_out_ctrl_lowv_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_mask_cpu_clk_out_ctrl_lowv_sel`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_SEL")]
pub type EfuseOverrideMaskCpuClkOutCtrlLowvSel = crate::Reg<
    efuse_override_mask_cpu_clk_out_ctrl_lowv_sel::EfuseOverrideMaskCpuClkOutCtrlLowvSelSpec,
>;
#[doc = "EFUSE_OVERRIDE_MASK_CPU_CLK_OUT_CTRL_LOWV_SEL"]
pub mod efuse_override_mask_cpu_clk_out_ctrl_lowv_sel;
#[doc = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC (rw) register accessor: EFUSE_OVERRIDE_EN_VOL_MON_FUNC\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_en_vol_mon_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_en_vol_mon_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_en_vol_mon_func`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC")]
pub type EfuseOverrideEnVolMonFunc =
    crate::Reg<efuse_override_en_vol_mon_func::EfuseOverrideEnVolMonFuncSpec>;
#[doc = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC"]
pub mod efuse_override_en_vol_mon_func;
#[doc = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC_ (rw) register accessor: EFUSE_OVERRIDE_EN_VOL_MON_FUNC\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_en_vol_mon_func_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_en_vol_mon_func_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_en_vol_mon_func_`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC_")]
pub type EfuseOverrideEnVolMonFunc_ =
    crate::Reg<efuse_override_en_vol_mon_func_::EfuseOverrideEnVolMonFunc_Spec>;
#[doc = "EFUSE_OVERRIDE_EN_VOL_MON_FUNC"]
pub mod efuse_override_en_vol_mon_func_;
#[doc = "EFUSE_OVERRIDE_SPARE_ANA (rw) register accessor: EFUSE_OVERRIDE_SPARE_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_spare_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_spare_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_spare_ana`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_SPARE_ANA")]
pub type EfuseOverrideSpareAna = crate::Reg<efuse_override_spare_ana::EfuseOverrideSpareAnaSpec>;
#[doc = "EFUSE_OVERRIDE_SPARE_ANA"]
pub mod efuse_override_spare_ana;
#[doc = "EFUSE_OVERRIDE_SLICER_DLY_DISABLE (rw) register accessor: EFUSE_OVERRIDE_SLICER_DLY_DISABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_slicer_dly_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_slicer_dly_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_override_slicer_dly_disable`]
module"]
#[doc(alias = "EFUSE_OVERRIDE_SLICER_DLY_DISABLE")]
pub type EfuseOverrideSlicerDlyDisable =
    crate::Reg<efuse_override_slicer_dly_disable::EfuseOverrideSlicerDlyDisableSpec>;
#[doc = "EFUSE_OVERRIDE_SLICER_DLY_DISABLE"]
pub mod efuse_override_slicer_dly_disable;
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
