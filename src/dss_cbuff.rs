#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config_reg_0: ConfigReg0,
    cfg_sphdr_address: CfgSphdrAddress,
    cfg_cmd_hsval: CfgCmdHsval,
    cfg_cmd_heval: CfgCmdHeval,
    cfg_cmd_vsval: CfgCmdVsval,
    cfg_cmd_veval: CfgCmdVeval,
    cfg_lphdr_address: CfgLphdrAddress,
    _reserved7: [u8; 0x04],
    cfg_chirps_per_frame: CfgChirpsPerFrame,
    cfg_fifo_free_threshold: CfgFifoFreeThreshold,
    cfg_lppyld_address: CfgLppyldAddress,
    cfg_delay_config: CfgDelayConfig,
    cfg_data_ll0: CfgDataLl0,
    cfg_data_ll0_lphdr_val: CfgDataLl0LphdrVal,
    cfg_data_ll0_threshold: CfgDataLl0Threshold,
    cfg_data_ll1: CfgDataLl1,
    cfg_data_ll1_lphdr_val: CfgDataLl1LphdrVal,
    cfg_data_ll1_threshold: CfgDataLl1Threshold,
    cfg_data_ll2: CfgDataLl2,
    cfg_data_ll2_lphdr_val: CfgDataLl2LphdrVal,
    cfg_data_ll2_threshold: CfgDataLl2Threshold,
    cfg_data_ll3: CfgDataLl3,
    cfg_data_ll3_lphdr_val: CfgDataLl3LphdrVal,
    cfg_data_ll3_threshold: CfgDataLl3Threshold,
    cfg_data_ll4: CfgDataLl4,
    cfg_data_ll4_lphdr_val: CfgDataLl4LphdrVal,
    cfg_data_ll4_threshold: CfgDataLl4Threshold,
    cfg_data_ll5: CfgDataLl5,
    cfg_data_ll5_lphdr_val: CfgDataLl5LphdrVal,
    cfg_data_ll5_threshold: CfgDataLl5Threshold,
    cfg_data_ll6: CfgDataLl6,
    cfg_data_ll6_lphdr_val: CfgDataLl6LphdrVal,
    cfg_data_ll6_threshold: CfgDataLl6Threshold,
    cfg_data_ll7: CfgDataLl7,
    cfg_data_ll7_lphdr_val: CfgDataLl7LphdrVal,
    cfg_data_ll7_threshold: CfgDataLl7Threshold,
    cfg_data_ll8: CfgDataLl8,
    cfg_data_ll8_lphdr_val: CfgDataLl8LphdrVal,
    cfg_data_ll8_threshold: CfgDataLl8Threshold,
    cfg_data_ll9: CfgDataLl9,
    cfg_data_ll9_lphdr_val: CfgDataLl9LphdrVal,
    cfg_data_ll9_threshold: CfgDataLl9Threshold,
    cfg_data_ll10: CfgDataLl10,
    cfg_data_ll10_lphdr_val: CfgDataLl10LphdrVal,
    cfg_data_ll10_threshold: CfgDataLl10Threshold,
    cfg_data_ll11: CfgDataLl11,
    cfg_data_ll11_lphdr_val: CfgDataLl11LphdrVal,
    cfg_data_ll11_threshold: CfgDataLl11Threshold,
    cfg_data_ll12: CfgDataLl12,
    cfg_data_ll12_lphdr_val: CfgDataLl12LphdrVal,
    cfg_data_ll12_threshold: CfgDataLl12Threshold,
    cfg_data_ll13: CfgDataLl13,
    cfg_data_ll13_lphdr_val: CfgDataLl13LphdrVal,
    cfg_data_ll13_threshold: CfgDataLl13Threshold,
    cfg_data_ll14: CfgDataLl14,
    cfg_data_ll14_lphdr_val: CfgDataLl14LphdrVal,
    cfg_data_ll14_threshold: CfgDataLl14Threshold,
    cfg_data_ll15: CfgDataLl15,
    cfg_data_ll15_lphdr_val: CfgDataLl15LphdrVal,
    cfg_data_ll15_threshold: CfgDataLl15Threshold,
    cfg_data_ll16: CfgDataLl16,
    cfg_data_ll16_lphdr_val: CfgDataLl16LphdrVal,
    cfg_data_ll16_threshold: CfgDataLl16Threshold,
    cfg_data_ll17: CfgDataLl17,
    cfg_data_ll17_lphdr_val: CfgDataLl17LphdrVal,
    cfg_data_ll17_threshold: CfgDataLl17Threshold,
    cfg_data_ll18: CfgDataLl18,
    cfg_data_ll18_lphdr_val: CfgDataLl18LphdrVal,
    cfg_data_ll18_threshold: CfgDataLl18Threshold,
    cfg_data_ll19: CfgDataLl19,
    cfg_data_ll19_lphdr_val: CfgDataLl19LphdrVal,
    cfg_data_ll19_threshold: CfgDataLl19Threshold,
    cfg_data_ll20: CfgDataLl20,
    cfg_data_ll20_lphdr_val: CfgDataLl20LphdrVal,
    cfg_data_ll20_threshold: CfgDataLl20Threshold,
    cfg_data_ll21: CfgDataLl21,
    cfg_data_ll21_lphdr_val: CfgDataLl21LphdrVal,
    cfg_data_ll21_threshold: CfgDataLl21Threshold,
    cfg_data_ll22: CfgDataLl22,
    cfg_data_ll22_lphdr_val: CfgDataLl22LphdrVal,
    cfg_data_ll22_threshold: CfgDataLl22Threshold,
    cfg_data_ll23: CfgDataLl23,
    cfg_data_ll23_lphdr_val: CfgDataLl23LphdrVal,
    cfg_data_ll23_threshold: CfgDataLl23Threshold,
    cfg_data_ll24: CfgDataLl24,
    cfg_data_ll24_lphdr_val: CfgDataLl24LphdrVal,
    cfg_data_ll24_threshold: CfgDataLl24Threshold,
    cfg_data_ll25: CfgDataLl25,
    cfg_data_ll25_lphdr_val: CfgDataLl25LphdrVal,
    cfg_data_ll25_threshold: CfgDataLl25Threshold,
    cfg_data_ll26: CfgDataLl26,
    cfg_data_ll26_lphdr_val: CfgDataLl26LphdrVal,
    cfg_data_ll26_threshold: CfgDataLl26Threshold,
    cfg_data_ll27: CfgDataLl27,
    cfg_data_ll27_lphdr_val: CfgDataLl27LphdrVal,
    cfg_data_ll27_threshold: CfgDataLl27Threshold,
    cfg_data_ll28: CfgDataLl28,
    cfg_data_ll28_lphdr_val: CfgDataLl28LphdrVal,
    cfg_data_ll28_threshold: CfgDataLl28Threshold,
    cfg_data_ll29: CfgDataLl29,
    cfg_data_ll29_lphdr_val: CfgDataLl29LphdrVal,
    cfg_data_ll29_threshold: CfgDataLl29Threshold,
    cfg_data_ll30: CfgDataLl30,
    cfg_data_ll30_lphdr_val: CfgDataLl30LphdrVal,
    cfg_data_ll30_threshold: CfgDataLl30Threshold,
    cfg_data_ll31: CfgDataLl31,
    cfg_data_ll31_lphdr_val: CfgDataLl31LphdrVal,
    cfg_data_ll31_threshold: CfgDataLl31Threshold,
    cfg_lvds_mapping_lane0_fmt_0: CfgLvdsMappingLane0Fmt0,
    cfg_lvds_mapping_lane1_fmt_0: CfgLvdsMappingLane1Fmt0,
    cfg_lvds_mapping_lane2_fmt_0: CfgLvdsMappingLane2Fmt0,
    cfg_lvds_mapping_lane3_fmt_0: CfgLvdsMappingLane3Fmt0,
    cfg_lvds_mapping_lane0_fmt_1: CfgLvdsMappingLane0Fmt1,
    cfg_lvds_mapping_lane1_fmt_1: CfgLvdsMappingLane1Fmt1,
    cfg_lvds_mapping_lane2_fmt_1: CfgLvdsMappingLane2Fmt1,
    cfg_lvds_mapping_lane3_fmt_1: CfgLvdsMappingLane3Fmt1,
    cfg_lvds_gen_0: CfgLvdsGen0,
    cfg_lvds_gen_1: CfgLvdsGen1,
    cfg_lvds_gen_2: CfgLvdsGen2,
    cfg_mask_reg0: CfgMaskReg0,
    cfg_mask_reg1: CfgMaskReg1,
    cfg_mask_reg2: CfgMaskReg2,
    cfg_mask_reg3: CfgMaskReg3,
    stat_cbuff_reg0: StatCbuffReg0,
    stat_cbuff_reg1: StatCbuffReg1,
    stat_cbuff_reg2: StatCbuffReg2,
    stat_cbuff_reg3: StatCbuffReg3,
    stat_lvds_reg0: StatLvdsReg0,
    stat_lvds_reg1: StatLvdsReg1,
    stat_lvds_reg2: StatLvdsReg2,
    stat_lvds_reg3: StatLvdsReg3,
    clr_cbuff_reg0: ClrCbuffReg0,
    clr_cbuff_reg1: ClrCbuffReg1,
    clr_lvds_reg0: ClrLvdsReg0,
    clr_lvds_reg1: ClrLvdsReg1,
    stat_cbuff_ecc_reg: StatCbuffEccReg,
    mask_cbuff_ecc_reg: MaskCbuffEccReg,
    clr_cbuff_ecc_reg: ClrCbuffEccReg,
    stat_safety: StatSafety,
    mask_safety: MaskSafety,
    clr_safety: ClrSafety,
}
impl RegisterBlock {
    #[doc = "0x00 - Basic Config register"]
    #[inline(always)]
    pub const fn config_reg_0(&self) -> &ConfigReg0 {
        &self.config_reg_0
    }
    #[doc = "0x04 - Short Packet Header Address"]
    #[inline(always)]
    pub const fn cfg_sphdr_address(&self) -> &CfgSphdrAddress {
        &self.cfg_sphdr_address
    }
    #[doc = "0x08 - HSYNC Value"]
    #[inline(always)]
    pub const fn cfg_cmd_hsval(&self) -> &CfgCmdHsval {
        &self.cfg_cmd_hsval
    }
    #[doc = "0x0c - HEND Value"]
    #[inline(always)]
    pub const fn cfg_cmd_heval(&self) -> &CfgCmdHeval {
        &self.cfg_cmd_heval
    }
    #[doc = "0x10 - VSYNC Value"]
    #[inline(always)]
    pub const fn cfg_cmd_vsval(&self) -> &CfgCmdVsval {
        &self.cfg_cmd_vsval
    }
    #[doc = "0x14 - VEND Value"]
    #[inline(always)]
    pub const fn cfg_cmd_veval(&self) -> &CfgCmdVeval {
        &self.cfg_cmd_veval
    }
    #[doc = "0x18 - Long Packet Address"]
    #[inline(always)]
    pub const fn cfg_lphdr_address(&self) -> &CfgLphdrAddress {
        &self.cfg_lphdr_address
    }
    #[doc = "0x20 - Number of Chirps per Frame"]
    #[inline(always)]
    pub const fn cfg_chirps_per_frame(&self) -> &CfgChirpsPerFrame {
        &self.cfg_chirps_per_frame
    }
    #[doc = "0x24 - CSI2 FIFO threshold for transferring data from CBUFF to CSI2"]
    #[inline(always)]
    pub const fn cfg_fifo_free_threshold(&self) -> &CfgFifoFreeThreshold {
        &self.cfg_fifo_free_threshold
    }
    #[doc = "0x28 - Long payload Address"]
    #[inline(always)]
    pub const fn cfg_lppyld_address(&self) -> &CfgLppyldAddress {
        &self.cfg_lppyld_address
    }
    #[doc = "0x2c - Delay Config Registers"]
    #[inline(always)]
    pub const fn cfg_delay_config(&self) -> &CfgDelayConfig {
        &self.cfg_delay_config
    }
    #[doc = "0x30 - Payload Description : Linked list entry 0"]
    #[inline(always)]
    pub const fn cfg_data_ll0(&self) -> &CfgDataLl0 {
        &self.cfg_data_ll0
    }
    #[doc = "0x34 - Payload Description : Linked list entry 0"]
    #[inline(always)]
    pub const fn cfg_data_ll0_lphdr_val(&self) -> &CfgDataLl0LphdrVal {
        &self.cfg_data_ll0_lphdr_val
    }
    #[doc = "0x38 - CFG_DATA_LL0_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll0_threshold(&self) -> &CfgDataLl0Threshold {
        &self.cfg_data_ll0_threshold
    }
    #[doc = "0x3c - CFG_DATA_LL1"]
    #[inline(always)]
    pub const fn cfg_data_ll1(&self) -> &CfgDataLl1 {
        &self.cfg_data_ll1
    }
    #[doc = "0x40 - CFG_DATA_LL1_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll1_lphdr_val(&self) -> &CfgDataLl1LphdrVal {
        &self.cfg_data_ll1_lphdr_val
    }
    #[doc = "0x44 - CFG_DATA_LL1_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll1_threshold(&self) -> &CfgDataLl1Threshold {
        &self.cfg_data_ll1_threshold
    }
    #[doc = "0x48 - CFG_DATA_LL2"]
    #[inline(always)]
    pub const fn cfg_data_ll2(&self) -> &CfgDataLl2 {
        &self.cfg_data_ll2
    }
    #[doc = "0x4c - CFG_DATA_LL2_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll2_lphdr_val(&self) -> &CfgDataLl2LphdrVal {
        &self.cfg_data_ll2_lphdr_val
    }
    #[doc = "0x50 - CFG_DATA_LL2_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll2_threshold(&self) -> &CfgDataLl2Threshold {
        &self.cfg_data_ll2_threshold
    }
    #[doc = "0x54 - CFG_DATA_LL3"]
    #[inline(always)]
    pub const fn cfg_data_ll3(&self) -> &CfgDataLl3 {
        &self.cfg_data_ll3
    }
    #[doc = "0x58 - CFG_DATA_LL3_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll3_lphdr_val(&self) -> &CfgDataLl3LphdrVal {
        &self.cfg_data_ll3_lphdr_val
    }
    #[doc = "0x5c - CFG_DATA_LL3_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll3_threshold(&self) -> &CfgDataLl3Threshold {
        &self.cfg_data_ll3_threshold
    }
    #[doc = "0x60 - CFG_DATA_LL4"]
    #[inline(always)]
    pub const fn cfg_data_ll4(&self) -> &CfgDataLl4 {
        &self.cfg_data_ll4
    }
    #[doc = "0x64 - CFG_DATA_LL4_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll4_lphdr_val(&self) -> &CfgDataLl4LphdrVal {
        &self.cfg_data_ll4_lphdr_val
    }
    #[doc = "0x68 - CFG_DATA_LL4_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll4_threshold(&self) -> &CfgDataLl4Threshold {
        &self.cfg_data_ll4_threshold
    }
    #[doc = "0x6c - CFG_DATA_LL5"]
    #[inline(always)]
    pub const fn cfg_data_ll5(&self) -> &CfgDataLl5 {
        &self.cfg_data_ll5
    }
    #[doc = "0x70 - CFG_DATA_LL5_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll5_lphdr_val(&self) -> &CfgDataLl5LphdrVal {
        &self.cfg_data_ll5_lphdr_val
    }
    #[doc = "0x74 - CFG_DATA_LL5_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll5_threshold(&self) -> &CfgDataLl5Threshold {
        &self.cfg_data_ll5_threshold
    }
    #[doc = "0x78 - CFG_DATA_LL6"]
    #[inline(always)]
    pub const fn cfg_data_ll6(&self) -> &CfgDataLl6 {
        &self.cfg_data_ll6
    }
    #[doc = "0x7c - CFG_DATA_LL6_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll6_lphdr_val(&self) -> &CfgDataLl6LphdrVal {
        &self.cfg_data_ll6_lphdr_val
    }
    #[doc = "0x80 - CFG_DATA_LL6_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll6_threshold(&self) -> &CfgDataLl6Threshold {
        &self.cfg_data_ll6_threshold
    }
    #[doc = "0x84 - CFG_DATA_LL7"]
    #[inline(always)]
    pub const fn cfg_data_ll7(&self) -> &CfgDataLl7 {
        &self.cfg_data_ll7
    }
    #[doc = "0x88 - CFG_DATA_LL7_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll7_lphdr_val(&self) -> &CfgDataLl7LphdrVal {
        &self.cfg_data_ll7_lphdr_val
    }
    #[doc = "0x8c - CFG_DATA_LL7_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll7_threshold(&self) -> &CfgDataLl7Threshold {
        &self.cfg_data_ll7_threshold
    }
    #[doc = "0x90 - CFG_DATA_LL8"]
    #[inline(always)]
    pub const fn cfg_data_ll8(&self) -> &CfgDataLl8 {
        &self.cfg_data_ll8
    }
    #[doc = "0x94 - CFG_DATA_LL8_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll8_lphdr_val(&self) -> &CfgDataLl8LphdrVal {
        &self.cfg_data_ll8_lphdr_val
    }
    #[doc = "0x98 - CFG_DATA_LL8_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll8_threshold(&self) -> &CfgDataLl8Threshold {
        &self.cfg_data_ll8_threshold
    }
    #[doc = "0x9c - CFG_DATA_LL9"]
    #[inline(always)]
    pub const fn cfg_data_ll9(&self) -> &CfgDataLl9 {
        &self.cfg_data_ll9
    }
    #[doc = "0xa0 - CFG_DATA_LL9_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll9_lphdr_val(&self) -> &CfgDataLl9LphdrVal {
        &self.cfg_data_ll9_lphdr_val
    }
    #[doc = "0xa4 - CFG_DATA_LL9_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll9_threshold(&self) -> &CfgDataLl9Threshold {
        &self.cfg_data_ll9_threshold
    }
    #[doc = "0xa8 - CFG_DATA_LL10"]
    #[inline(always)]
    pub const fn cfg_data_ll10(&self) -> &CfgDataLl10 {
        &self.cfg_data_ll10
    }
    #[doc = "0xac - CFG_DATA_LL10_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll10_lphdr_val(&self) -> &CfgDataLl10LphdrVal {
        &self.cfg_data_ll10_lphdr_val
    }
    #[doc = "0xb0 - CFG_DATA_LL10_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll10_threshold(&self) -> &CfgDataLl10Threshold {
        &self.cfg_data_ll10_threshold
    }
    #[doc = "0xb4 - CFG_DATA_LL11"]
    #[inline(always)]
    pub const fn cfg_data_ll11(&self) -> &CfgDataLl11 {
        &self.cfg_data_ll11
    }
    #[doc = "0xb8 - CFG_DATA_LL11_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll11_lphdr_val(&self) -> &CfgDataLl11LphdrVal {
        &self.cfg_data_ll11_lphdr_val
    }
    #[doc = "0xbc - CFG_DATA_LL11_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll11_threshold(&self) -> &CfgDataLl11Threshold {
        &self.cfg_data_ll11_threshold
    }
    #[doc = "0xc0 - CFG_DATA_LL12"]
    #[inline(always)]
    pub const fn cfg_data_ll12(&self) -> &CfgDataLl12 {
        &self.cfg_data_ll12
    }
    #[doc = "0xc4 - CFG_DATA_LL12_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll12_lphdr_val(&self) -> &CfgDataLl12LphdrVal {
        &self.cfg_data_ll12_lphdr_val
    }
    #[doc = "0xc8 - CFG_DATA_LL12_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll12_threshold(&self) -> &CfgDataLl12Threshold {
        &self.cfg_data_ll12_threshold
    }
    #[doc = "0xcc - CFG_DATA_LL13"]
    #[inline(always)]
    pub const fn cfg_data_ll13(&self) -> &CfgDataLl13 {
        &self.cfg_data_ll13
    }
    #[doc = "0xd0 - CFG_DATA_LL13_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll13_lphdr_val(&self) -> &CfgDataLl13LphdrVal {
        &self.cfg_data_ll13_lphdr_val
    }
    #[doc = "0xd4 - CFG_DATA_LL13_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll13_threshold(&self) -> &CfgDataLl13Threshold {
        &self.cfg_data_ll13_threshold
    }
    #[doc = "0xd8 - CFG_DATA_LL14"]
    #[inline(always)]
    pub const fn cfg_data_ll14(&self) -> &CfgDataLl14 {
        &self.cfg_data_ll14
    }
    #[doc = "0xdc - CFG_DATA_LL14_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll14_lphdr_val(&self) -> &CfgDataLl14LphdrVal {
        &self.cfg_data_ll14_lphdr_val
    }
    #[doc = "0xe0 - CFG_DATA_LL14_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll14_threshold(&self) -> &CfgDataLl14Threshold {
        &self.cfg_data_ll14_threshold
    }
    #[doc = "0xe4 - CFG_DATA_LL15"]
    #[inline(always)]
    pub const fn cfg_data_ll15(&self) -> &CfgDataLl15 {
        &self.cfg_data_ll15
    }
    #[doc = "0xe8 - CFG_DATA_LL15_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll15_lphdr_val(&self) -> &CfgDataLl15LphdrVal {
        &self.cfg_data_ll15_lphdr_val
    }
    #[doc = "0xec - CFG_DATA_LL15_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll15_threshold(&self) -> &CfgDataLl15Threshold {
        &self.cfg_data_ll15_threshold
    }
    #[doc = "0xf0 - CFG_DATA_LL16"]
    #[inline(always)]
    pub const fn cfg_data_ll16(&self) -> &CfgDataLl16 {
        &self.cfg_data_ll16
    }
    #[doc = "0xf4 - CFG_DATA_LL16_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll16_lphdr_val(&self) -> &CfgDataLl16LphdrVal {
        &self.cfg_data_ll16_lphdr_val
    }
    #[doc = "0xf8 - CFG_DATA_LL16_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll16_threshold(&self) -> &CfgDataLl16Threshold {
        &self.cfg_data_ll16_threshold
    }
    #[doc = "0xfc - CFG_DATA_LL17"]
    #[inline(always)]
    pub const fn cfg_data_ll17(&self) -> &CfgDataLl17 {
        &self.cfg_data_ll17
    }
    #[doc = "0x100 - CFG_DATA_LL17_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll17_lphdr_val(&self) -> &CfgDataLl17LphdrVal {
        &self.cfg_data_ll17_lphdr_val
    }
    #[doc = "0x104 - CFG_DATA_LL17_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll17_threshold(&self) -> &CfgDataLl17Threshold {
        &self.cfg_data_ll17_threshold
    }
    #[doc = "0x108 - CFG_DATA_LL18"]
    #[inline(always)]
    pub const fn cfg_data_ll18(&self) -> &CfgDataLl18 {
        &self.cfg_data_ll18
    }
    #[doc = "0x10c - CFG_DATA_LL18_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll18_lphdr_val(&self) -> &CfgDataLl18LphdrVal {
        &self.cfg_data_ll18_lphdr_val
    }
    #[doc = "0x110 - CFG_DATA_LL18_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll18_threshold(&self) -> &CfgDataLl18Threshold {
        &self.cfg_data_ll18_threshold
    }
    #[doc = "0x114 - CFG_DATA_LL19"]
    #[inline(always)]
    pub const fn cfg_data_ll19(&self) -> &CfgDataLl19 {
        &self.cfg_data_ll19
    }
    #[doc = "0x118 - CFG_DATA_LL19_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll19_lphdr_val(&self) -> &CfgDataLl19LphdrVal {
        &self.cfg_data_ll19_lphdr_val
    }
    #[doc = "0x11c - CFG_DATA_LL19_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll19_threshold(&self) -> &CfgDataLl19Threshold {
        &self.cfg_data_ll19_threshold
    }
    #[doc = "0x120 - CFG_DATA_LL20"]
    #[inline(always)]
    pub const fn cfg_data_ll20(&self) -> &CfgDataLl20 {
        &self.cfg_data_ll20
    }
    #[doc = "0x124 - CFG_DATA_LL20_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll20_lphdr_val(&self) -> &CfgDataLl20LphdrVal {
        &self.cfg_data_ll20_lphdr_val
    }
    #[doc = "0x128 - CFG_DATA_LL20_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll20_threshold(&self) -> &CfgDataLl20Threshold {
        &self.cfg_data_ll20_threshold
    }
    #[doc = "0x12c - CFG_DATA_LL21"]
    #[inline(always)]
    pub const fn cfg_data_ll21(&self) -> &CfgDataLl21 {
        &self.cfg_data_ll21
    }
    #[doc = "0x130 - CFG_DATA_LL21_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll21_lphdr_val(&self) -> &CfgDataLl21LphdrVal {
        &self.cfg_data_ll21_lphdr_val
    }
    #[doc = "0x134 - CFG_DATA_LL21_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll21_threshold(&self) -> &CfgDataLl21Threshold {
        &self.cfg_data_ll21_threshold
    }
    #[doc = "0x138 - CFG_DATA_LL22"]
    #[inline(always)]
    pub const fn cfg_data_ll22(&self) -> &CfgDataLl22 {
        &self.cfg_data_ll22
    }
    #[doc = "0x13c - CFG_DATA_LL22_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll22_lphdr_val(&self) -> &CfgDataLl22LphdrVal {
        &self.cfg_data_ll22_lphdr_val
    }
    #[doc = "0x140 - CFG_DATA_LL22_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll22_threshold(&self) -> &CfgDataLl22Threshold {
        &self.cfg_data_ll22_threshold
    }
    #[doc = "0x144 - CFG_DATA_LL23"]
    #[inline(always)]
    pub const fn cfg_data_ll23(&self) -> &CfgDataLl23 {
        &self.cfg_data_ll23
    }
    #[doc = "0x148 - CFG_DATA_LL23_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll23_lphdr_val(&self) -> &CfgDataLl23LphdrVal {
        &self.cfg_data_ll23_lphdr_val
    }
    #[doc = "0x14c - CFG_DATA_LL23_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll23_threshold(&self) -> &CfgDataLl23Threshold {
        &self.cfg_data_ll23_threshold
    }
    #[doc = "0x150 - CFG_DATA_LL24"]
    #[inline(always)]
    pub const fn cfg_data_ll24(&self) -> &CfgDataLl24 {
        &self.cfg_data_ll24
    }
    #[doc = "0x154 - CFG_DATA_LL24_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll24_lphdr_val(&self) -> &CfgDataLl24LphdrVal {
        &self.cfg_data_ll24_lphdr_val
    }
    #[doc = "0x158 - CFG_DATA_LL24_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll24_threshold(&self) -> &CfgDataLl24Threshold {
        &self.cfg_data_ll24_threshold
    }
    #[doc = "0x15c - CFG_DATA_LL25"]
    #[inline(always)]
    pub const fn cfg_data_ll25(&self) -> &CfgDataLl25 {
        &self.cfg_data_ll25
    }
    #[doc = "0x160 - CFG_DATA_LL25_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll25_lphdr_val(&self) -> &CfgDataLl25LphdrVal {
        &self.cfg_data_ll25_lphdr_val
    }
    #[doc = "0x164 - CFG_DATA_LL25_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll25_threshold(&self) -> &CfgDataLl25Threshold {
        &self.cfg_data_ll25_threshold
    }
    #[doc = "0x168 - CFG_DATA_LL26"]
    #[inline(always)]
    pub const fn cfg_data_ll26(&self) -> &CfgDataLl26 {
        &self.cfg_data_ll26
    }
    #[doc = "0x16c - CFG_DATA_LL26_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll26_lphdr_val(&self) -> &CfgDataLl26LphdrVal {
        &self.cfg_data_ll26_lphdr_val
    }
    #[doc = "0x170 - CFG_DATA_LL26_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll26_threshold(&self) -> &CfgDataLl26Threshold {
        &self.cfg_data_ll26_threshold
    }
    #[doc = "0x174 - CFG_DATA_LL27"]
    #[inline(always)]
    pub const fn cfg_data_ll27(&self) -> &CfgDataLl27 {
        &self.cfg_data_ll27
    }
    #[doc = "0x178 - CFG_DATA_LL27_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll27_lphdr_val(&self) -> &CfgDataLl27LphdrVal {
        &self.cfg_data_ll27_lphdr_val
    }
    #[doc = "0x17c - CFG_DATA_LL27_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll27_threshold(&self) -> &CfgDataLl27Threshold {
        &self.cfg_data_ll27_threshold
    }
    #[doc = "0x180 - CFG_DATA_LL28"]
    #[inline(always)]
    pub const fn cfg_data_ll28(&self) -> &CfgDataLl28 {
        &self.cfg_data_ll28
    }
    #[doc = "0x184 - CFG_DATA_LL28_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll28_lphdr_val(&self) -> &CfgDataLl28LphdrVal {
        &self.cfg_data_ll28_lphdr_val
    }
    #[doc = "0x188 - CFG_DATA_LL28_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll28_threshold(&self) -> &CfgDataLl28Threshold {
        &self.cfg_data_ll28_threshold
    }
    #[doc = "0x18c - CFG_DATA_LL29"]
    #[inline(always)]
    pub const fn cfg_data_ll29(&self) -> &CfgDataLl29 {
        &self.cfg_data_ll29
    }
    #[doc = "0x190 - CFG_DATA_LL29_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll29_lphdr_val(&self) -> &CfgDataLl29LphdrVal {
        &self.cfg_data_ll29_lphdr_val
    }
    #[doc = "0x194 - CFG_DATA_LL29_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll29_threshold(&self) -> &CfgDataLl29Threshold {
        &self.cfg_data_ll29_threshold
    }
    #[doc = "0x198 - CFG_DATA_LL30"]
    #[inline(always)]
    pub const fn cfg_data_ll30(&self) -> &CfgDataLl30 {
        &self.cfg_data_ll30
    }
    #[doc = "0x19c - CFG_DATA_LL30_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll30_lphdr_val(&self) -> &CfgDataLl30LphdrVal {
        &self.cfg_data_ll30_lphdr_val
    }
    #[doc = "0x1a0 - CFG_DATA_LL30_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll30_threshold(&self) -> &CfgDataLl30Threshold {
        &self.cfg_data_ll30_threshold
    }
    #[doc = "0x1a4 - CFG_DATA_LL31"]
    #[inline(always)]
    pub const fn cfg_data_ll31(&self) -> &CfgDataLl31 {
        &self.cfg_data_ll31
    }
    #[doc = "0x1a8 - CFG_DATA_LL31_LPHDR_VAL"]
    #[inline(always)]
    pub const fn cfg_data_ll31_lphdr_val(&self) -> &CfgDataLl31LphdrVal {
        &self.cfg_data_ll31_lphdr_val
    }
    #[doc = "0x1ac - CFG_DATA_LL31_THRESHOLD"]
    #[inline(always)]
    pub const fn cfg_data_ll31_threshold(&self) -> &CfgDataLl31Threshold {
        &self.cfg_data_ll31_threshold
    }
    #[doc = "0x1b0 - CFG_LVDS_MAPPING_LANE0_FMT_0"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane0_fmt_0(&self) -> &CfgLvdsMappingLane0Fmt0 {
        &self.cfg_lvds_mapping_lane0_fmt_0
    }
    #[doc = "0x1b4 - CFG_LVDS_MAPPING_LANE1_FMT_0"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane1_fmt_0(&self) -> &CfgLvdsMappingLane1Fmt0 {
        &self.cfg_lvds_mapping_lane1_fmt_0
    }
    #[doc = "0x1b8 - CFG_LVDS_MAPPING_LANE2_FMT_0"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane2_fmt_0(&self) -> &CfgLvdsMappingLane2Fmt0 {
        &self.cfg_lvds_mapping_lane2_fmt_0
    }
    #[doc = "0x1bc - CFG_LVDS_MAPPING_LANE3_FMT_0"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane3_fmt_0(&self) -> &CfgLvdsMappingLane3Fmt0 {
        &self.cfg_lvds_mapping_lane3_fmt_0
    }
    #[doc = "0x1c0 - CFG_LVDS_MAPPING_LANE0_FMT_1"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane0_fmt_1(&self) -> &CfgLvdsMappingLane0Fmt1 {
        &self.cfg_lvds_mapping_lane0_fmt_1
    }
    #[doc = "0x1c4 - CFG_LVDS_MAPPING_LANE1_FMT_1"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane1_fmt_1(&self) -> &CfgLvdsMappingLane1Fmt1 {
        &self.cfg_lvds_mapping_lane1_fmt_1
    }
    #[doc = "0x1c8 - CFG_LVDS_MAPPING_LANE2_FMT_1"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane2_fmt_1(&self) -> &CfgLvdsMappingLane2Fmt1 {
        &self.cfg_lvds_mapping_lane2_fmt_1
    }
    #[doc = "0x1cc - CFG_LVDS_MAPPING_LANE3_FMT_1"]
    #[inline(always)]
    pub const fn cfg_lvds_mapping_lane3_fmt_1(&self) -> &CfgLvdsMappingLane3Fmt1 {
        &self.cfg_lvds_mapping_lane3_fmt_1
    }
    #[doc = "0x1d0 - CFG_LVDS_GEN_0"]
    #[inline(always)]
    pub const fn cfg_lvds_gen_0(&self) -> &CfgLvdsGen0 {
        &self.cfg_lvds_gen_0
    }
    #[doc = "0x1d4 - CFG_LVDS_GEN_1"]
    #[inline(always)]
    pub const fn cfg_lvds_gen_1(&self) -> &CfgLvdsGen1 {
        &self.cfg_lvds_gen_1
    }
    #[doc = "0x1d8 - CFG_LVDS_GEN_2"]
    #[inline(always)]
    pub const fn cfg_lvds_gen_2(&self) -> &CfgLvdsGen2 {
        &self.cfg_lvds_gen_2
    }
    #[doc = "0x1dc - CFG_MASK_REG0"]
    #[inline(always)]
    pub const fn cfg_mask_reg0(&self) -> &CfgMaskReg0 {
        &self.cfg_mask_reg0
    }
    #[doc = "0x1e0 - CFG_MASK_REG1"]
    #[inline(always)]
    pub const fn cfg_mask_reg1(&self) -> &CfgMaskReg1 {
        &self.cfg_mask_reg1
    }
    #[doc = "0x1e4 - CFG_MASK_REG2"]
    #[inline(always)]
    pub const fn cfg_mask_reg2(&self) -> &CfgMaskReg2 {
        &self.cfg_mask_reg2
    }
    #[doc = "0x1e8 - CFG_MASK_REG3"]
    #[inline(always)]
    pub const fn cfg_mask_reg3(&self) -> &CfgMaskReg3 {
        &self.cfg_mask_reg3
    }
    #[doc = "0x1ec - STAT_CBUFF_REG0"]
    #[inline(always)]
    pub const fn stat_cbuff_reg0(&self) -> &StatCbuffReg0 {
        &self.stat_cbuff_reg0
    }
    #[doc = "0x1f0 - STAT_CBUFF_REG1"]
    #[inline(always)]
    pub const fn stat_cbuff_reg1(&self) -> &StatCbuffReg1 {
        &self.stat_cbuff_reg1
    }
    #[doc = "0x1f4 - STAT_CBUFF_REG2"]
    #[inline(always)]
    pub const fn stat_cbuff_reg2(&self) -> &StatCbuffReg2 {
        &self.stat_cbuff_reg2
    }
    #[doc = "0x1f8 - STAT_CBUFF_REG3"]
    #[inline(always)]
    pub const fn stat_cbuff_reg3(&self) -> &StatCbuffReg3 {
        &self.stat_cbuff_reg3
    }
    #[doc = "0x1fc - STAT_LVDS_REG0"]
    #[inline(always)]
    pub const fn stat_lvds_reg0(&self) -> &StatLvdsReg0 {
        &self.stat_lvds_reg0
    }
    #[doc = "0x200 - STAT_LVDS_REG1"]
    #[inline(always)]
    pub const fn stat_lvds_reg1(&self) -> &StatLvdsReg1 {
        &self.stat_lvds_reg1
    }
    #[doc = "0x204 - STAT_LVDS_REG2"]
    #[inline(always)]
    pub const fn stat_lvds_reg2(&self) -> &StatLvdsReg2 {
        &self.stat_lvds_reg2
    }
    #[doc = "0x208 - STAT_LVDS_REG3"]
    #[inline(always)]
    pub const fn stat_lvds_reg3(&self) -> &StatLvdsReg3 {
        &self.stat_lvds_reg3
    }
    #[doc = "0x20c - CLR_CBUFF_REG0"]
    #[inline(always)]
    pub const fn clr_cbuff_reg0(&self) -> &ClrCbuffReg0 {
        &self.clr_cbuff_reg0
    }
    #[doc = "0x210 - CLR_CBUFF_REG1"]
    #[inline(always)]
    pub const fn clr_cbuff_reg1(&self) -> &ClrCbuffReg1 {
        &self.clr_cbuff_reg1
    }
    #[doc = "0x214 - CLR_LVDS_REG0"]
    #[inline(always)]
    pub const fn clr_lvds_reg0(&self) -> &ClrLvdsReg0 {
        &self.clr_lvds_reg0
    }
    #[doc = "0x218 - CLR_LVDS_REG1"]
    #[inline(always)]
    pub const fn clr_lvds_reg1(&self) -> &ClrLvdsReg1 {
        &self.clr_lvds_reg1
    }
    #[doc = "0x21c - STAT_CBUFF_ECC_REG"]
    #[inline(always)]
    pub const fn stat_cbuff_ecc_reg(&self) -> &StatCbuffEccReg {
        &self.stat_cbuff_ecc_reg
    }
    #[doc = "0x220 - MASK_CBUFF_ECC_REG"]
    #[inline(always)]
    pub const fn mask_cbuff_ecc_reg(&self) -> &MaskCbuffEccReg {
        &self.mask_cbuff_ecc_reg
    }
    #[doc = "0x224 - CLR_CBUFF_ECC_REG"]
    #[inline(always)]
    pub const fn clr_cbuff_ecc_reg(&self) -> &ClrCbuffEccReg {
        &self.clr_cbuff_ecc_reg
    }
    #[doc = "0x228 - STAT_SAFETY"]
    #[inline(always)]
    pub const fn stat_safety(&self) -> &StatSafety {
        &self.stat_safety
    }
    #[doc = "0x22c - MASK_SAFETY"]
    #[inline(always)]
    pub const fn mask_safety(&self) -> &MaskSafety {
        &self.mask_safety
    }
    #[doc = "0x230 - CLR_SAFETY"]
    #[inline(always)]
    pub const fn clr_safety(&self) -> &ClrSafety {
        &self.clr_safety
    }
}
#[doc = "CONFIG_REG_0 (rw) register accessor: Basic Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`config_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_reg_0`]
module"]
#[doc(alias = "CONFIG_REG_0")]
pub type ConfigReg0 = crate::Reg<config_reg_0::ConfigReg0Spec>;
#[doc = "Basic Config register"]
pub mod config_reg_0;
#[doc = "CFG_SPHDR_ADDRESS (rw) register accessor: Short Packet Header Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_sphdr_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_sphdr_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sphdr_address`]
module"]
#[doc(alias = "CFG_SPHDR_ADDRESS")]
pub type CfgSphdrAddress = crate::Reg<cfg_sphdr_address::CfgSphdrAddressSpec>;
#[doc = "Short Packet Header Address"]
pub mod cfg_sphdr_address;
#[doc = "CFG_CMD_HSVAL (rw) register accessor: HSYNC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_hsval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_hsval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cmd_hsval`]
module"]
#[doc(alias = "CFG_CMD_HSVAL")]
pub type CfgCmdHsval = crate::Reg<cfg_cmd_hsval::CfgCmdHsvalSpec>;
#[doc = "HSYNC Value"]
pub mod cfg_cmd_hsval;
#[doc = "CFG_CMD_HEVAL (rw) register accessor: HEND Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_heval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_heval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cmd_heval`]
module"]
#[doc(alias = "CFG_CMD_HEVAL")]
pub type CfgCmdHeval = crate::Reg<cfg_cmd_heval::CfgCmdHevalSpec>;
#[doc = "HEND Value"]
pub mod cfg_cmd_heval;
#[doc = "CFG_CMD_VSVAL (rw) register accessor: VSYNC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_vsval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_vsval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cmd_vsval`]
module"]
#[doc(alias = "CFG_CMD_VSVAL")]
pub type CfgCmdVsval = crate::Reg<cfg_cmd_vsval::CfgCmdVsvalSpec>;
#[doc = "VSYNC Value"]
pub mod cfg_cmd_vsval;
#[doc = "CFG_CMD_VEVAL (rw) register accessor: VEND Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_veval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_veval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cmd_veval`]
module"]
#[doc(alias = "CFG_CMD_VEVAL")]
pub type CfgCmdVeval = crate::Reg<cfg_cmd_veval::CfgCmdVevalSpec>;
#[doc = "VEND Value"]
pub mod cfg_cmd_veval;
#[doc = "CFG_LPHDR_ADDRESS (rw) register accessor: Long Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lphdr_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lphdr_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lphdr_address`]
module"]
#[doc(alias = "CFG_LPHDR_ADDRESS")]
pub type CfgLphdrAddress = crate::Reg<cfg_lphdr_address::CfgLphdrAddressSpec>;
#[doc = "Long Packet Address"]
pub mod cfg_lphdr_address;
#[doc = "CFG_CHIRPS_PER_FRAME (rw) register accessor: Number of Chirps per Frame\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_chirps_per_frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_chirps_per_frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_chirps_per_frame`]
module"]
#[doc(alias = "CFG_CHIRPS_PER_FRAME")]
pub type CfgChirpsPerFrame = crate::Reg<cfg_chirps_per_frame::CfgChirpsPerFrameSpec>;
#[doc = "Number of Chirps per Frame"]
pub mod cfg_chirps_per_frame;
#[doc = "CFG_FIFO_FREE_THRESHOLD (rw) register accessor: CSI2 FIFO threshold for transferring data from CBUFF to CSI2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_fifo_free_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_fifo_free_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_fifo_free_threshold`]
module"]
#[doc(alias = "CFG_FIFO_FREE_THRESHOLD")]
pub type CfgFifoFreeThreshold = crate::Reg<cfg_fifo_free_threshold::CfgFifoFreeThresholdSpec>;
#[doc = "CSI2 FIFO threshold for transferring data from CBUFF to CSI2"]
pub mod cfg_fifo_free_threshold;
#[doc = "CFG_LPPYLD_ADDRESS (rw) register accessor: Long payload Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lppyld_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lppyld_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lppyld_address`]
module"]
#[doc(alias = "CFG_LPPYLD_ADDRESS")]
pub type CfgLppyldAddress = crate::Reg<cfg_lppyld_address::CfgLppyldAddressSpec>;
#[doc = "Long payload Address"]
pub mod cfg_lppyld_address;
#[doc = "CFG_DELAY_CONFIG (rw) register accessor: Delay Config Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_delay_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_delay_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_delay_config`]
module"]
#[doc(alias = "CFG_DELAY_CONFIG")]
pub type CfgDelayConfig = crate::Reg<cfg_delay_config::CfgDelayConfigSpec>;
#[doc = "Delay Config Registers"]
pub mod cfg_delay_config;
#[doc = "CFG_DATA_LL0 (rw) register accessor: Payload Description : Linked list entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll0`]
module"]
#[doc(alias = "CFG_DATA_LL0")]
pub type CfgDataLl0 = crate::Reg<cfg_data_ll0::CfgDataLl0Spec>;
#[doc = "Payload Description : Linked list entry 0"]
pub mod cfg_data_ll0;
#[doc = "CFG_DATA_LL0_LPHDR_VAL (rw) register accessor: Payload Description : Linked list entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll0_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll0_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll0_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL0_LPHDR_VAL")]
pub type CfgDataLl0LphdrVal = crate::Reg<cfg_data_ll0_lphdr_val::CfgDataLl0LphdrValSpec>;
#[doc = "Payload Description : Linked list entry 0"]
pub mod cfg_data_ll0_lphdr_val;
#[doc = "CFG_DATA_LL0_THRESHOLD (rw) register accessor: CFG_DATA_LL0_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll0_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll0_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll0_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL0_THRESHOLD")]
pub type CfgDataLl0Threshold = crate::Reg<cfg_data_ll0_threshold::CfgDataLl0ThresholdSpec>;
#[doc = "CFG_DATA_LL0_THRESHOLD"]
pub mod cfg_data_ll0_threshold;
#[doc = "CFG_DATA_LL1 (rw) register accessor: CFG_DATA_LL1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll1`]
module"]
#[doc(alias = "CFG_DATA_LL1")]
pub type CfgDataLl1 = crate::Reg<cfg_data_ll1::CfgDataLl1Spec>;
#[doc = "CFG_DATA_LL1"]
pub mod cfg_data_ll1;
#[doc = "CFG_DATA_LL1_LPHDR_VAL (rw) register accessor: CFG_DATA_LL1_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll1_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll1_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll1_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL1_LPHDR_VAL")]
pub type CfgDataLl1LphdrVal = crate::Reg<cfg_data_ll1_lphdr_val::CfgDataLl1LphdrValSpec>;
#[doc = "CFG_DATA_LL1_LPHDR_VAL"]
pub mod cfg_data_ll1_lphdr_val;
#[doc = "CFG_DATA_LL1_THRESHOLD (rw) register accessor: CFG_DATA_LL1_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll1_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll1_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll1_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL1_THRESHOLD")]
pub type CfgDataLl1Threshold = crate::Reg<cfg_data_ll1_threshold::CfgDataLl1ThresholdSpec>;
#[doc = "CFG_DATA_LL1_THRESHOLD"]
pub mod cfg_data_ll1_threshold;
#[doc = "CFG_DATA_LL2 (rw) register accessor: CFG_DATA_LL2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll2`]
module"]
#[doc(alias = "CFG_DATA_LL2")]
pub type CfgDataLl2 = crate::Reg<cfg_data_ll2::CfgDataLl2Spec>;
#[doc = "CFG_DATA_LL2"]
pub mod cfg_data_ll2;
#[doc = "CFG_DATA_LL2_LPHDR_VAL (rw) register accessor: CFG_DATA_LL2_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll2_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll2_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll2_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL2_LPHDR_VAL")]
pub type CfgDataLl2LphdrVal = crate::Reg<cfg_data_ll2_lphdr_val::CfgDataLl2LphdrValSpec>;
#[doc = "CFG_DATA_LL2_LPHDR_VAL"]
pub mod cfg_data_ll2_lphdr_val;
#[doc = "CFG_DATA_LL2_THRESHOLD (rw) register accessor: CFG_DATA_LL2_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll2_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll2_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll2_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL2_THRESHOLD")]
pub type CfgDataLl2Threshold = crate::Reg<cfg_data_ll2_threshold::CfgDataLl2ThresholdSpec>;
#[doc = "CFG_DATA_LL2_THRESHOLD"]
pub mod cfg_data_ll2_threshold;
#[doc = "CFG_DATA_LL3 (rw) register accessor: CFG_DATA_LL3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll3`]
module"]
#[doc(alias = "CFG_DATA_LL3")]
pub type CfgDataLl3 = crate::Reg<cfg_data_ll3::CfgDataLl3Spec>;
#[doc = "CFG_DATA_LL3"]
pub mod cfg_data_ll3;
#[doc = "CFG_DATA_LL3_LPHDR_VAL (rw) register accessor: CFG_DATA_LL3_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll3_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll3_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll3_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL3_LPHDR_VAL")]
pub type CfgDataLl3LphdrVal = crate::Reg<cfg_data_ll3_lphdr_val::CfgDataLl3LphdrValSpec>;
#[doc = "CFG_DATA_LL3_LPHDR_VAL"]
pub mod cfg_data_ll3_lphdr_val;
#[doc = "CFG_DATA_LL3_THRESHOLD (rw) register accessor: CFG_DATA_LL3_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll3_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll3_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll3_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL3_THRESHOLD")]
pub type CfgDataLl3Threshold = crate::Reg<cfg_data_ll3_threshold::CfgDataLl3ThresholdSpec>;
#[doc = "CFG_DATA_LL3_THRESHOLD"]
pub mod cfg_data_ll3_threshold;
#[doc = "CFG_DATA_LL4 (rw) register accessor: CFG_DATA_LL4\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll4`]
module"]
#[doc(alias = "CFG_DATA_LL4")]
pub type CfgDataLl4 = crate::Reg<cfg_data_ll4::CfgDataLl4Spec>;
#[doc = "CFG_DATA_LL4"]
pub mod cfg_data_ll4;
#[doc = "CFG_DATA_LL4_LPHDR_VAL (rw) register accessor: CFG_DATA_LL4_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll4_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll4_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll4_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL4_LPHDR_VAL")]
pub type CfgDataLl4LphdrVal = crate::Reg<cfg_data_ll4_lphdr_val::CfgDataLl4LphdrValSpec>;
#[doc = "CFG_DATA_LL4_LPHDR_VAL"]
pub mod cfg_data_ll4_lphdr_val;
#[doc = "CFG_DATA_LL4_THRESHOLD (rw) register accessor: CFG_DATA_LL4_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll4_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll4_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll4_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL4_THRESHOLD")]
pub type CfgDataLl4Threshold = crate::Reg<cfg_data_ll4_threshold::CfgDataLl4ThresholdSpec>;
#[doc = "CFG_DATA_LL4_THRESHOLD"]
pub mod cfg_data_ll4_threshold;
#[doc = "CFG_DATA_LL5 (rw) register accessor: CFG_DATA_LL5\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll5`]
module"]
#[doc(alias = "CFG_DATA_LL5")]
pub type CfgDataLl5 = crate::Reg<cfg_data_ll5::CfgDataLl5Spec>;
#[doc = "CFG_DATA_LL5"]
pub mod cfg_data_ll5;
#[doc = "CFG_DATA_LL5_LPHDR_VAL (rw) register accessor: CFG_DATA_LL5_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll5_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll5_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll5_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL5_LPHDR_VAL")]
pub type CfgDataLl5LphdrVal = crate::Reg<cfg_data_ll5_lphdr_val::CfgDataLl5LphdrValSpec>;
#[doc = "CFG_DATA_LL5_LPHDR_VAL"]
pub mod cfg_data_ll5_lphdr_val;
#[doc = "CFG_DATA_LL5_THRESHOLD (rw) register accessor: CFG_DATA_LL5_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll5_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll5_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll5_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL5_THRESHOLD")]
pub type CfgDataLl5Threshold = crate::Reg<cfg_data_ll5_threshold::CfgDataLl5ThresholdSpec>;
#[doc = "CFG_DATA_LL5_THRESHOLD"]
pub mod cfg_data_ll5_threshold;
#[doc = "CFG_DATA_LL6 (rw) register accessor: CFG_DATA_LL6\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll6`]
module"]
#[doc(alias = "CFG_DATA_LL6")]
pub type CfgDataLl6 = crate::Reg<cfg_data_ll6::CfgDataLl6Spec>;
#[doc = "CFG_DATA_LL6"]
pub mod cfg_data_ll6;
#[doc = "CFG_DATA_LL6_LPHDR_VAL (rw) register accessor: CFG_DATA_LL6_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll6_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll6_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll6_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL6_LPHDR_VAL")]
pub type CfgDataLl6LphdrVal = crate::Reg<cfg_data_ll6_lphdr_val::CfgDataLl6LphdrValSpec>;
#[doc = "CFG_DATA_LL6_LPHDR_VAL"]
pub mod cfg_data_ll6_lphdr_val;
#[doc = "CFG_DATA_LL6_THRESHOLD (rw) register accessor: CFG_DATA_LL6_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll6_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll6_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll6_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL6_THRESHOLD")]
pub type CfgDataLl6Threshold = crate::Reg<cfg_data_ll6_threshold::CfgDataLl6ThresholdSpec>;
#[doc = "CFG_DATA_LL6_THRESHOLD"]
pub mod cfg_data_ll6_threshold;
#[doc = "CFG_DATA_LL7 (rw) register accessor: CFG_DATA_LL7\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll7`]
module"]
#[doc(alias = "CFG_DATA_LL7")]
pub type CfgDataLl7 = crate::Reg<cfg_data_ll7::CfgDataLl7Spec>;
#[doc = "CFG_DATA_LL7"]
pub mod cfg_data_ll7;
#[doc = "CFG_DATA_LL7_LPHDR_VAL (rw) register accessor: CFG_DATA_LL7_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll7_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll7_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll7_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL7_LPHDR_VAL")]
pub type CfgDataLl7LphdrVal = crate::Reg<cfg_data_ll7_lphdr_val::CfgDataLl7LphdrValSpec>;
#[doc = "CFG_DATA_LL7_LPHDR_VAL"]
pub mod cfg_data_ll7_lphdr_val;
#[doc = "CFG_DATA_LL7_THRESHOLD (rw) register accessor: CFG_DATA_LL7_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll7_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll7_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll7_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL7_THRESHOLD")]
pub type CfgDataLl7Threshold = crate::Reg<cfg_data_ll7_threshold::CfgDataLl7ThresholdSpec>;
#[doc = "CFG_DATA_LL7_THRESHOLD"]
pub mod cfg_data_ll7_threshold;
#[doc = "CFG_DATA_LL8 (rw) register accessor: CFG_DATA_LL8\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll8`]
module"]
#[doc(alias = "CFG_DATA_LL8")]
pub type CfgDataLl8 = crate::Reg<cfg_data_ll8::CfgDataLl8Spec>;
#[doc = "CFG_DATA_LL8"]
pub mod cfg_data_ll8;
#[doc = "CFG_DATA_LL8_LPHDR_VAL (rw) register accessor: CFG_DATA_LL8_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll8_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll8_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll8_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL8_LPHDR_VAL")]
pub type CfgDataLl8LphdrVal = crate::Reg<cfg_data_ll8_lphdr_val::CfgDataLl8LphdrValSpec>;
#[doc = "CFG_DATA_LL8_LPHDR_VAL"]
pub mod cfg_data_ll8_lphdr_val;
#[doc = "CFG_DATA_LL8_THRESHOLD (rw) register accessor: CFG_DATA_LL8_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll8_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll8_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll8_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL8_THRESHOLD")]
pub type CfgDataLl8Threshold = crate::Reg<cfg_data_ll8_threshold::CfgDataLl8ThresholdSpec>;
#[doc = "CFG_DATA_LL8_THRESHOLD"]
pub mod cfg_data_ll8_threshold;
#[doc = "CFG_DATA_LL9 (rw) register accessor: CFG_DATA_LL9\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll9`]
module"]
#[doc(alias = "CFG_DATA_LL9")]
pub type CfgDataLl9 = crate::Reg<cfg_data_ll9::CfgDataLl9Spec>;
#[doc = "CFG_DATA_LL9"]
pub mod cfg_data_ll9;
#[doc = "CFG_DATA_LL9_LPHDR_VAL (rw) register accessor: CFG_DATA_LL9_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll9_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll9_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll9_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL9_LPHDR_VAL")]
pub type CfgDataLl9LphdrVal = crate::Reg<cfg_data_ll9_lphdr_val::CfgDataLl9LphdrValSpec>;
#[doc = "CFG_DATA_LL9_LPHDR_VAL"]
pub mod cfg_data_ll9_lphdr_val;
#[doc = "CFG_DATA_LL9_THRESHOLD (rw) register accessor: CFG_DATA_LL9_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll9_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll9_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll9_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL9_THRESHOLD")]
pub type CfgDataLl9Threshold = crate::Reg<cfg_data_ll9_threshold::CfgDataLl9ThresholdSpec>;
#[doc = "CFG_DATA_LL9_THRESHOLD"]
pub mod cfg_data_ll9_threshold;
#[doc = "CFG_DATA_LL10 (rw) register accessor: CFG_DATA_LL10\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll10`]
module"]
#[doc(alias = "CFG_DATA_LL10")]
pub type CfgDataLl10 = crate::Reg<cfg_data_ll10::CfgDataLl10Spec>;
#[doc = "CFG_DATA_LL10"]
pub mod cfg_data_ll10;
#[doc = "CFG_DATA_LL10_LPHDR_VAL (rw) register accessor: CFG_DATA_LL10_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll10_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll10_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll10_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL10_LPHDR_VAL")]
pub type CfgDataLl10LphdrVal = crate::Reg<cfg_data_ll10_lphdr_val::CfgDataLl10LphdrValSpec>;
#[doc = "CFG_DATA_LL10_LPHDR_VAL"]
pub mod cfg_data_ll10_lphdr_val;
#[doc = "CFG_DATA_LL10_THRESHOLD (rw) register accessor: CFG_DATA_LL10_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll10_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll10_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll10_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL10_THRESHOLD")]
pub type CfgDataLl10Threshold = crate::Reg<cfg_data_ll10_threshold::CfgDataLl10ThresholdSpec>;
#[doc = "CFG_DATA_LL10_THRESHOLD"]
pub mod cfg_data_ll10_threshold;
#[doc = "CFG_DATA_LL11 (rw) register accessor: CFG_DATA_LL11\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll11`]
module"]
#[doc(alias = "CFG_DATA_LL11")]
pub type CfgDataLl11 = crate::Reg<cfg_data_ll11::CfgDataLl11Spec>;
#[doc = "CFG_DATA_LL11"]
pub mod cfg_data_ll11;
#[doc = "CFG_DATA_LL11_LPHDR_VAL (rw) register accessor: CFG_DATA_LL11_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll11_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll11_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll11_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL11_LPHDR_VAL")]
pub type CfgDataLl11LphdrVal = crate::Reg<cfg_data_ll11_lphdr_val::CfgDataLl11LphdrValSpec>;
#[doc = "CFG_DATA_LL11_LPHDR_VAL"]
pub mod cfg_data_ll11_lphdr_val;
#[doc = "CFG_DATA_LL11_THRESHOLD (rw) register accessor: CFG_DATA_LL11_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll11_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll11_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll11_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL11_THRESHOLD")]
pub type CfgDataLl11Threshold = crate::Reg<cfg_data_ll11_threshold::CfgDataLl11ThresholdSpec>;
#[doc = "CFG_DATA_LL11_THRESHOLD"]
pub mod cfg_data_ll11_threshold;
#[doc = "CFG_DATA_LL12 (rw) register accessor: CFG_DATA_LL12\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll12`]
module"]
#[doc(alias = "CFG_DATA_LL12")]
pub type CfgDataLl12 = crate::Reg<cfg_data_ll12::CfgDataLl12Spec>;
#[doc = "CFG_DATA_LL12"]
pub mod cfg_data_ll12;
#[doc = "CFG_DATA_LL12_LPHDR_VAL (rw) register accessor: CFG_DATA_LL12_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll12_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll12_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll12_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL12_LPHDR_VAL")]
pub type CfgDataLl12LphdrVal = crate::Reg<cfg_data_ll12_lphdr_val::CfgDataLl12LphdrValSpec>;
#[doc = "CFG_DATA_LL12_LPHDR_VAL"]
pub mod cfg_data_ll12_lphdr_val;
#[doc = "CFG_DATA_LL12_THRESHOLD (rw) register accessor: CFG_DATA_LL12_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll12_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll12_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll12_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL12_THRESHOLD")]
pub type CfgDataLl12Threshold = crate::Reg<cfg_data_ll12_threshold::CfgDataLl12ThresholdSpec>;
#[doc = "CFG_DATA_LL12_THRESHOLD"]
pub mod cfg_data_ll12_threshold;
#[doc = "CFG_DATA_LL13 (rw) register accessor: CFG_DATA_LL13\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll13`]
module"]
#[doc(alias = "CFG_DATA_LL13")]
pub type CfgDataLl13 = crate::Reg<cfg_data_ll13::CfgDataLl13Spec>;
#[doc = "CFG_DATA_LL13"]
pub mod cfg_data_ll13;
#[doc = "CFG_DATA_LL13_LPHDR_VAL (rw) register accessor: CFG_DATA_LL13_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll13_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll13_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll13_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL13_LPHDR_VAL")]
pub type CfgDataLl13LphdrVal = crate::Reg<cfg_data_ll13_lphdr_val::CfgDataLl13LphdrValSpec>;
#[doc = "CFG_DATA_LL13_LPHDR_VAL"]
pub mod cfg_data_ll13_lphdr_val;
#[doc = "CFG_DATA_LL13_THRESHOLD (rw) register accessor: CFG_DATA_LL13_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll13_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll13_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll13_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL13_THRESHOLD")]
pub type CfgDataLl13Threshold = crate::Reg<cfg_data_ll13_threshold::CfgDataLl13ThresholdSpec>;
#[doc = "CFG_DATA_LL13_THRESHOLD"]
pub mod cfg_data_ll13_threshold;
#[doc = "CFG_DATA_LL14 (rw) register accessor: CFG_DATA_LL14\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll14`]
module"]
#[doc(alias = "CFG_DATA_LL14")]
pub type CfgDataLl14 = crate::Reg<cfg_data_ll14::CfgDataLl14Spec>;
#[doc = "CFG_DATA_LL14"]
pub mod cfg_data_ll14;
#[doc = "CFG_DATA_LL14_LPHDR_VAL (rw) register accessor: CFG_DATA_LL14_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll14_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll14_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll14_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL14_LPHDR_VAL")]
pub type CfgDataLl14LphdrVal = crate::Reg<cfg_data_ll14_lphdr_val::CfgDataLl14LphdrValSpec>;
#[doc = "CFG_DATA_LL14_LPHDR_VAL"]
pub mod cfg_data_ll14_lphdr_val;
#[doc = "CFG_DATA_LL14_THRESHOLD (rw) register accessor: CFG_DATA_LL14_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll14_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll14_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll14_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL14_THRESHOLD")]
pub type CfgDataLl14Threshold = crate::Reg<cfg_data_ll14_threshold::CfgDataLl14ThresholdSpec>;
#[doc = "CFG_DATA_LL14_THRESHOLD"]
pub mod cfg_data_ll14_threshold;
#[doc = "CFG_DATA_LL15 (rw) register accessor: CFG_DATA_LL15\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll15`]
module"]
#[doc(alias = "CFG_DATA_LL15")]
pub type CfgDataLl15 = crate::Reg<cfg_data_ll15::CfgDataLl15Spec>;
#[doc = "CFG_DATA_LL15"]
pub mod cfg_data_ll15;
#[doc = "CFG_DATA_LL15_LPHDR_VAL (rw) register accessor: CFG_DATA_LL15_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll15_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll15_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll15_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL15_LPHDR_VAL")]
pub type CfgDataLl15LphdrVal = crate::Reg<cfg_data_ll15_lphdr_val::CfgDataLl15LphdrValSpec>;
#[doc = "CFG_DATA_LL15_LPHDR_VAL"]
pub mod cfg_data_ll15_lphdr_val;
#[doc = "CFG_DATA_LL15_THRESHOLD (rw) register accessor: CFG_DATA_LL15_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll15_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll15_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll15_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL15_THRESHOLD")]
pub type CfgDataLl15Threshold = crate::Reg<cfg_data_ll15_threshold::CfgDataLl15ThresholdSpec>;
#[doc = "CFG_DATA_LL15_THRESHOLD"]
pub mod cfg_data_ll15_threshold;
#[doc = "CFG_DATA_LL16 (rw) register accessor: CFG_DATA_LL16\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll16`]
module"]
#[doc(alias = "CFG_DATA_LL16")]
pub type CfgDataLl16 = crate::Reg<cfg_data_ll16::CfgDataLl16Spec>;
#[doc = "CFG_DATA_LL16"]
pub mod cfg_data_ll16;
#[doc = "CFG_DATA_LL16_LPHDR_VAL (rw) register accessor: CFG_DATA_LL16_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll16_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll16_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll16_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL16_LPHDR_VAL")]
pub type CfgDataLl16LphdrVal = crate::Reg<cfg_data_ll16_lphdr_val::CfgDataLl16LphdrValSpec>;
#[doc = "CFG_DATA_LL16_LPHDR_VAL"]
pub mod cfg_data_ll16_lphdr_val;
#[doc = "CFG_DATA_LL16_THRESHOLD (rw) register accessor: CFG_DATA_LL16_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll16_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll16_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll16_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL16_THRESHOLD")]
pub type CfgDataLl16Threshold = crate::Reg<cfg_data_ll16_threshold::CfgDataLl16ThresholdSpec>;
#[doc = "CFG_DATA_LL16_THRESHOLD"]
pub mod cfg_data_ll16_threshold;
#[doc = "CFG_DATA_LL17 (rw) register accessor: CFG_DATA_LL17\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll17`]
module"]
#[doc(alias = "CFG_DATA_LL17")]
pub type CfgDataLl17 = crate::Reg<cfg_data_ll17::CfgDataLl17Spec>;
#[doc = "CFG_DATA_LL17"]
pub mod cfg_data_ll17;
#[doc = "CFG_DATA_LL17_LPHDR_VAL (rw) register accessor: CFG_DATA_LL17_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll17_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll17_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll17_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL17_LPHDR_VAL")]
pub type CfgDataLl17LphdrVal = crate::Reg<cfg_data_ll17_lphdr_val::CfgDataLl17LphdrValSpec>;
#[doc = "CFG_DATA_LL17_LPHDR_VAL"]
pub mod cfg_data_ll17_lphdr_val;
#[doc = "CFG_DATA_LL17_THRESHOLD (rw) register accessor: CFG_DATA_LL17_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll17_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll17_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll17_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL17_THRESHOLD")]
pub type CfgDataLl17Threshold = crate::Reg<cfg_data_ll17_threshold::CfgDataLl17ThresholdSpec>;
#[doc = "CFG_DATA_LL17_THRESHOLD"]
pub mod cfg_data_ll17_threshold;
#[doc = "CFG_DATA_LL18 (rw) register accessor: CFG_DATA_LL18\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll18`]
module"]
#[doc(alias = "CFG_DATA_LL18")]
pub type CfgDataLl18 = crate::Reg<cfg_data_ll18::CfgDataLl18Spec>;
#[doc = "CFG_DATA_LL18"]
pub mod cfg_data_ll18;
#[doc = "CFG_DATA_LL18_LPHDR_VAL (rw) register accessor: CFG_DATA_LL18_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll18_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll18_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll18_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL18_LPHDR_VAL")]
pub type CfgDataLl18LphdrVal = crate::Reg<cfg_data_ll18_lphdr_val::CfgDataLl18LphdrValSpec>;
#[doc = "CFG_DATA_LL18_LPHDR_VAL"]
pub mod cfg_data_ll18_lphdr_val;
#[doc = "CFG_DATA_LL18_THRESHOLD (rw) register accessor: CFG_DATA_LL18_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll18_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll18_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll18_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL18_THRESHOLD")]
pub type CfgDataLl18Threshold = crate::Reg<cfg_data_ll18_threshold::CfgDataLl18ThresholdSpec>;
#[doc = "CFG_DATA_LL18_THRESHOLD"]
pub mod cfg_data_ll18_threshold;
#[doc = "CFG_DATA_LL19 (rw) register accessor: CFG_DATA_LL19\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll19`]
module"]
#[doc(alias = "CFG_DATA_LL19")]
pub type CfgDataLl19 = crate::Reg<cfg_data_ll19::CfgDataLl19Spec>;
#[doc = "CFG_DATA_LL19"]
pub mod cfg_data_ll19;
#[doc = "CFG_DATA_LL19_LPHDR_VAL (rw) register accessor: CFG_DATA_LL19_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll19_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll19_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll19_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL19_LPHDR_VAL")]
pub type CfgDataLl19LphdrVal = crate::Reg<cfg_data_ll19_lphdr_val::CfgDataLl19LphdrValSpec>;
#[doc = "CFG_DATA_LL19_LPHDR_VAL"]
pub mod cfg_data_ll19_lphdr_val;
#[doc = "CFG_DATA_LL19_THRESHOLD (rw) register accessor: CFG_DATA_LL19_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll19_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll19_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll19_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL19_THRESHOLD")]
pub type CfgDataLl19Threshold = crate::Reg<cfg_data_ll19_threshold::CfgDataLl19ThresholdSpec>;
#[doc = "CFG_DATA_LL19_THRESHOLD"]
pub mod cfg_data_ll19_threshold;
#[doc = "CFG_DATA_LL20 (rw) register accessor: CFG_DATA_LL20\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll20`]
module"]
#[doc(alias = "CFG_DATA_LL20")]
pub type CfgDataLl20 = crate::Reg<cfg_data_ll20::CfgDataLl20Spec>;
#[doc = "CFG_DATA_LL20"]
pub mod cfg_data_ll20;
#[doc = "CFG_DATA_LL20_LPHDR_VAL (rw) register accessor: CFG_DATA_LL20_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll20_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll20_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll20_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL20_LPHDR_VAL")]
pub type CfgDataLl20LphdrVal = crate::Reg<cfg_data_ll20_lphdr_val::CfgDataLl20LphdrValSpec>;
#[doc = "CFG_DATA_LL20_LPHDR_VAL"]
pub mod cfg_data_ll20_lphdr_val;
#[doc = "CFG_DATA_LL20_THRESHOLD (rw) register accessor: CFG_DATA_LL20_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll20_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll20_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll20_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL20_THRESHOLD")]
pub type CfgDataLl20Threshold = crate::Reg<cfg_data_ll20_threshold::CfgDataLl20ThresholdSpec>;
#[doc = "CFG_DATA_LL20_THRESHOLD"]
pub mod cfg_data_ll20_threshold;
#[doc = "CFG_DATA_LL21 (rw) register accessor: CFG_DATA_LL21\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll21`]
module"]
#[doc(alias = "CFG_DATA_LL21")]
pub type CfgDataLl21 = crate::Reg<cfg_data_ll21::CfgDataLl21Spec>;
#[doc = "CFG_DATA_LL21"]
pub mod cfg_data_ll21;
#[doc = "CFG_DATA_LL21_LPHDR_VAL (rw) register accessor: CFG_DATA_LL21_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll21_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll21_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll21_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL21_LPHDR_VAL")]
pub type CfgDataLl21LphdrVal = crate::Reg<cfg_data_ll21_lphdr_val::CfgDataLl21LphdrValSpec>;
#[doc = "CFG_DATA_LL21_LPHDR_VAL"]
pub mod cfg_data_ll21_lphdr_val;
#[doc = "CFG_DATA_LL21_THRESHOLD (rw) register accessor: CFG_DATA_LL21_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll21_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll21_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll21_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL21_THRESHOLD")]
pub type CfgDataLl21Threshold = crate::Reg<cfg_data_ll21_threshold::CfgDataLl21ThresholdSpec>;
#[doc = "CFG_DATA_LL21_THRESHOLD"]
pub mod cfg_data_ll21_threshold;
#[doc = "CFG_DATA_LL22 (rw) register accessor: CFG_DATA_LL22\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll22`]
module"]
#[doc(alias = "CFG_DATA_LL22")]
pub type CfgDataLl22 = crate::Reg<cfg_data_ll22::CfgDataLl22Spec>;
#[doc = "CFG_DATA_LL22"]
pub mod cfg_data_ll22;
#[doc = "CFG_DATA_LL22_LPHDR_VAL (rw) register accessor: CFG_DATA_LL22_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll22_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll22_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll22_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL22_LPHDR_VAL")]
pub type CfgDataLl22LphdrVal = crate::Reg<cfg_data_ll22_lphdr_val::CfgDataLl22LphdrValSpec>;
#[doc = "CFG_DATA_LL22_LPHDR_VAL"]
pub mod cfg_data_ll22_lphdr_val;
#[doc = "CFG_DATA_LL22_THRESHOLD (rw) register accessor: CFG_DATA_LL22_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll22_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll22_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll22_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL22_THRESHOLD")]
pub type CfgDataLl22Threshold = crate::Reg<cfg_data_ll22_threshold::CfgDataLl22ThresholdSpec>;
#[doc = "CFG_DATA_LL22_THRESHOLD"]
pub mod cfg_data_ll22_threshold;
#[doc = "CFG_DATA_LL23 (rw) register accessor: CFG_DATA_LL23\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll23`]
module"]
#[doc(alias = "CFG_DATA_LL23")]
pub type CfgDataLl23 = crate::Reg<cfg_data_ll23::CfgDataLl23Spec>;
#[doc = "CFG_DATA_LL23"]
pub mod cfg_data_ll23;
#[doc = "CFG_DATA_LL23_LPHDR_VAL (rw) register accessor: CFG_DATA_LL23_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll23_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll23_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll23_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL23_LPHDR_VAL")]
pub type CfgDataLl23LphdrVal = crate::Reg<cfg_data_ll23_lphdr_val::CfgDataLl23LphdrValSpec>;
#[doc = "CFG_DATA_LL23_LPHDR_VAL"]
pub mod cfg_data_ll23_lphdr_val;
#[doc = "CFG_DATA_LL23_THRESHOLD (rw) register accessor: CFG_DATA_LL23_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll23_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll23_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll23_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL23_THRESHOLD")]
pub type CfgDataLl23Threshold = crate::Reg<cfg_data_ll23_threshold::CfgDataLl23ThresholdSpec>;
#[doc = "CFG_DATA_LL23_THRESHOLD"]
pub mod cfg_data_ll23_threshold;
#[doc = "CFG_DATA_LL24 (rw) register accessor: CFG_DATA_LL24\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll24`]
module"]
#[doc(alias = "CFG_DATA_LL24")]
pub type CfgDataLl24 = crate::Reg<cfg_data_ll24::CfgDataLl24Spec>;
#[doc = "CFG_DATA_LL24"]
pub mod cfg_data_ll24;
#[doc = "CFG_DATA_LL24_LPHDR_VAL (rw) register accessor: CFG_DATA_LL24_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll24_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll24_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll24_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL24_LPHDR_VAL")]
pub type CfgDataLl24LphdrVal = crate::Reg<cfg_data_ll24_lphdr_val::CfgDataLl24LphdrValSpec>;
#[doc = "CFG_DATA_LL24_LPHDR_VAL"]
pub mod cfg_data_ll24_lphdr_val;
#[doc = "CFG_DATA_LL24_THRESHOLD (rw) register accessor: CFG_DATA_LL24_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll24_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll24_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll24_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL24_THRESHOLD")]
pub type CfgDataLl24Threshold = crate::Reg<cfg_data_ll24_threshold::CfgDataLl24ThresholdSpec>;
#[doc = "CFG_DATA_LL24_THRESHOLD"]
pub mod cfg_data_ll24_threshold;
#[doc = "CFG_DATA_LL25 (rw) register accessor: CFG_DATA_LL25\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll25`]
module"]
#[doc(alias = "CFG_DATA_LL25")]
pub type CfgDataLl25 = crate::Reg<cfg_data_ll25::CfgDataLl25Spec>;
#[doc = "CFG_DATA_LL25"]
pub mod cfg_data_ll25;
#[doc = "CFG_DATA_LL25_LPHDR_VAL (rw) register accessor: CFG_DATA_LL25_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll25_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll25_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll25_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL25_LPHDR_VAL")]
pub type CfgDataLl25LphdrVal = crate::Reg<cfg_data_ll25_lphdr_val::CfgDataLl25LphdrValSpec>;
#[doc = "CFG_DATA_LL25_LPHDR_VAL"]
pub mod cfg_data_ll25_lphdr_val;
#[doc = "CFG_DATA_LL25_THRESHOLD (rw) register accessor: CFG_DATA_LL25_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll25_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll25_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll25_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL25_THRESHOLD")]
pub type CfgDataLl25Threshold = crate::Reg<cfg_data_ll25_threshold::CfgDataLl25ThresholdSpec>;
#[doc = "CFG_DATA_LL25_THRESHOLD"]
pub mod cfg_data_ll25_threshold;
#[doc = "CFG_DATA_LL26 (rw) register accessor: CFG_DATA_LL26\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll26`]
module"]
#[doc(alias = "CFG_DATA_LL26")]
pub type CfgDataLl26 = crate::Reg<cfg_data_ll26::CfgDataLl26Spec>;
#[doc = "CFG_DATA_LL26"]
pub mod cfg_data_ll26;
#[doc = "CFG_DATA_LL26_LPHDR_VAL (rw) register accessor: CFG_DATA_LL26_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll26_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll26_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll26_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL26_LPHDR_VAL")]
pub type CfgDataLl26LphdrVal = crate::Reg<cfg_data_ll26_lphdr_val::CfgDataLl26LphdrValSpec>;
#[doc = "CFG_DATA_LL26_LPHDR_VAL"]
pub mod cfg_data_ll26_lphdr_val;
#[doc = "CFG_DATA_LL26_THRESHOLD (rw) register accessor: CFG_DATA_LL26_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll26_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll26_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll26_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL26_THRESHOLD")]
pub type CfgDataLl26Threshold = crate::Reg<cfg_data_ll26_threshold::CfgDataLl26ThresholdSpec>;
#[doc = "CFG_DATA_LL26_THRESHOLD"]
pub mod cfg_data_ll26_threshold;
#[doc = "CFG_DATA_LL27 (rw) register accessor: CFG_DATA_LL27\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll27`]
module"]
#[doc(alias = "CFG_DATA_LL27")]
pub type CfgDataLl27 = crate::Reg<cfg_data_ll27::CfgDataLl27Spec>;
#[doc = "CFG_DATA_LL27"]
pub mod cfg_data_ll27;
#[doc = "CFG_DATA_LL27_LPHDR_VAL (rw) register accessor: CFG_DATA_LL27_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll27_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll27_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll27_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL27_LPHDR_VAL")]
pub type CfgDataLl27LphdrVal = crate::Reg<cfg_data_ll27_lphdr_val::CfgDataLl27LphdrValSpec>;
#[doc = "CFG_DATA_LL27_LPHDR_VAL"]
pub mod cfg_data_ll27_lphdr_val;
#[doc = "CFG_DATA_LL27_THRESHOLD (rw) register accessor: CFG_DATA_LL27_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll27_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll27_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll27_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL27_THRESHOLD")]
pub type CfgDataLl27Threshold = crate::Reg<cfg_data_ll27_threshold::CfgDataLl27ThresholdSpec>;
#[doc = "CFG_DATA_LL27_THRESHOLD"]
pub mod cfg_data_ll27_threshold;
#[doc = "CFG_DATA_LL28 (rw) register accessor: CFG_DATA_LL28\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll28`]
module"]
#[doc(alias = "CFG_DATA_LL28")]
pub type CfgDataLl28 = crate::Reg<cfg_data_ll28::CfgDataLl28Spec>;
#[doc = "CFG_DATA_LL28"]
pub mod cfg_data_ll28;
#[doc = "CFG_DATA_LL28_LPHDR_VAL (rw) register accessor: CFG_DATA_LL28_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll28_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll28_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll28_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL28_LPHDR_VAL")]
pub type CfgDataLl28LphdrVal = crate::Reg<cfg_data_ll28_lphdr_val::CfgDataLl28LphdrValSpec>;
#[doc = "CFG_DATA_LL28_LPHDR_VAL"]
pub mod cfg_data_ll28_lphdr_val;
#[doc = "CFG_DATA_LL28_THRESHOLD (rw) register accessor: CFG_DATA_LL28_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll28_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll28_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll28_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL28_THRESHOLD")]
pub type CfgDataLl28Threshold = crate::Reg<cfg_data_ll28_threshold::CfgDataLl28ThresholdSpec>;
#[doc = "CFG_DATA_LL28_THRESHOLD"]
pub mod cfg_data_ll28_threshold;
#[doc = "CFG_DATA_LL29 (rw) register accessor: CFG_DATA_LL29\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll29`]
module"]
#[doc(alias = "CFG_DATA_LL29")]
pub type CfgDataLl29 = crate::Reg<cfg_data_ll29::CfgDataLl29Spec>;
#[doc = "CFG_DATA_LL29"]
pub mod cfg_data_ll29;
#[doc = "CFG_DATA_LL29_LPHDR_VAL (rw) register accessor: CFG_DATA_LL29_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll29_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll29_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll29_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL29_LPHDR_VAL")]
pub type CfgDataLl29LphdrVal = crate::Reg<cfg_data_ll29_lphdr_val::CfgDataLl29LphdrValSpec>;
#[doc = "CFG_DATA_LL29_LPHDR_VAL"]
pub mod cfg_data_ll29_lphdr_val;
#[doc = "CFG_DATA_LL29_THRESHOLD (rw) register accessor: CFG_DATA_LL29_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll29_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll29_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll29_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL29_THRESHOLD")]
pub type CfgDataLl29Threshold = crate::Reg<cfg_data_ll29_threshold::CfgDataLl29ThresholdSpec>;
#[doc = "CFG_DATA_LL29_THRESHOLD"]
pub mod cfg_data_ll29_threshold;
#[doc = "CFG_DATA_LL30 (rw) register accessor: CFG_DATA_LL30\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll30`]
module"]
#[doc(alias = "CFG_DATA_LL30")]
pub type CfgDataLl30 = crate::Reg<cfg_data_ll30::CfgDataLl30Spec>;
#[doc = "CFG_DATA_LL30"]
pub mod cfg_data_ll30;
#[doc = "CFG_DATA_LL30_LPHDR_VAL (rw) register accessor: CFG_DATA_LL30_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll30_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll30_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll30_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL30_LPHDR_VAL")]
pub type CfgDataLl30LphdrVal = crate::Reg<cfg_data_ll30_lphdr_val::CfgDataLl30LphdrValSpec>;
#[doc = "CFG_DATA_LL30_LPHDR_VAL"]
pub mod cfg_data_ll30_lphdr_val;
#[doc = "CFG_DATA_LL30_THRESHOLD (rw) register accessor: CFG_DATA_LL30_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll30_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll30_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll30_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL30_THRESHOLD")]
pub type CfgDataLl30Threshold = crate::Reg<cfg_data_ll30_threshold::CfgDataLl30ThresholdSpec>;
#[doc = "CFG_DATA_LL30_THRESHOLD"]
pub mod cfg_data_ll30_threshold;
#[doc = "CFG_DATA_LL31 (rw) register accessor: CFG_DATA_LL31\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll31`]
module"]
#[doc(alias = "CFG_DATA_LL31")]
pub type CfgDataLl31 = crate::Reg<cfg_data_ll31::CfgDataLl31Spec>;
#[doc = "CFG_DATA_LL31"]
pub mod cfg_data_ll31;
#[doc = "CFG_DATA_LL31_LPHDR_VAL (rw) register accessor: CFG_DATA_LL31_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll31_lphdr_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll31_lphdr_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll31_lphdr_val`]
module"]
#[doc(alias = "CFG_DATA_LL31_LPHDR_VAL")]
pub type CfgDataLl31LphdrVal = crate::Reg<cfg_data_ll31_lphdr_val::CfgDataLl31LphdrValSpec>;
#[doc = "CFG_DATA_LL31_LPHDR_VAL"]
pub mod cfg_data_ll31_lphdr_val;
#[doc = "CFG_DATA_LL31_THRESHOLD (rw) register accessor: CFG_DATA_LL31_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll31_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll31_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_ll31_threshold`]
module"]
#[doc(alias = "CFG_DATA_LL31_THRESHOLD")]
pub type CfgDataLl31Threshold = crate::Reg<cfg_data_ll31_threshold::CfgDataLl31ThresholdSpec>;
#[doc = "CFG_DATA_LL31_THRESHOLD"]
pub mod cfg_data_ll31_threshold;
#[doc = "CFG_LVDS_MAPPING_LANE0_FMT_0 (rw) register accessor: CFG_LVDS_MAPPING_LANE0_FMT_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane0_fmt_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane0_fmt_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane0_fmt_0`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE0_FMT_0")]
pub type CfgLvdsMappingLane0Fmt0 =
    crate::Reg<cfg_lvds_mapping_lane0_fmt_0::CfgLvdsMappingLane0Fmt0Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE0_FMT_0"]
pub mod cfg_lvds_mapping_lane0_fmt_0;
#[doc = "CFG_LVDS_MAPPING_LANE1_FMT_0 (rw) register accessor: CFG_LVDS_MAPPING_LANE1_FMT_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane1_fmt_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane1_fmt_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane1_fmt_0`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE1_FMT_0")]
pub type CfgLvdsMappingLane1Fmt0 =
    crate::Reg<cfg_lvds_mapping_lane1_fmt_0::CfgLvdsMappingLane1Fmt0Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE1_FMT_0"]
pub mod cfg_lvds_mapping_lane1_fmt_0;
#[doc = "CFG_LVDS_MAPPING_LANE2_FMT_0 (rw) register accessor: CFG_LVDS_MAPPING_LANE2_FMT_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane2_fmt_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane2_fmt_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane2_fmt_0`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE2_FMT_0")]
pub type CfgLvdsMappingLane2Fmt0 =
    crate::Reg<cfg_lvds_mapping_lane2_fmt_0::CfgLvdsMappingLane2Fmt0Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE2_FMT_0"]
pub mod cfg_lvds_mapping_lane2_fmt_0;
#[doc = "CFG_LVDS_MAPPING_LANE3_FMT_0 (rw) register accessor: CFG_LVDS_MAPPING_LANE3_FMT_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane3_fmt_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane3_fmt_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane3_fmt_0`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE3_FMT_0")]
pub type CfgLvdsMappingLane3Fmt0 =
    crate::Reg<cfg_lvds_mapping_lane3_fmt_0::CfgLvdsMappingLane3Fmt0Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE3_FMT_0"]
pub mod cfg_lvds_mapping_lane3_fmt_0;
#[doc = "CFG_LVDS_MAPPING_LANE0_FMT_1 (rw) register accessor: CFG_LVDS_MAPPING_LANE0_FMT_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane0_fmt_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane0_fmt_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane0_fmt_1`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE0_FMT_1")]
pub type CfgLvdsMappingLane0Fmt1 =
    crate::Reg<cfg_lvds_mapping_lane0_fmt_1::CfgLvdsMappingLane0Fmt1Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE0_FMT_1"]
pub mod cfg_lvds_mapping_lane0_fmt_1;
#[doc = "CFG_LVDS_MAPPING_LANE1_FMT_1 (rw) register accessor: CFG_LVDS_MAPPING_LANE1_FMT_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane1_fmt_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane1_fmt_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane1_fmt_1`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE1_FMT_1")]
pub type CfgLvdsMappingLane1Fmt1 =
    crate::Reg<cfg_lvds_mapping_lane1_fmt_1::CfgLvdsMappingLane1Fmt1Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE1_FMT_1"]
pub mod cfg_lvds_mapping_lane1_fmt_1;
#[doc = "CFG_LVDS_MAPPING_LANE2_FMT_1 (rw) register accessor: CFG_LVDS_MAPPING_LANE2_FMT_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane2_fmt_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane2_fmt_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane2_fmt_1`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE2_FMT_1")]
pub type CfgLvdsMappingLane2Fmt1 =
    crate::Reg<cfg_lvds_mapping_lane2_fmt_1::CfgLvdsMappingLane2Fmt1Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE2_FMT_1"]
pub mod cfg_lvds_mapping_lane2_fmt_1;
#[doc = "CFG_LVDS_MAPPING_LANE3_FMT_1 (rw) register accessor: CFG_LVDS_MAPPING_LANE3_FMT_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_mapping_lane3_fmt_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_mapping_lane3_fmt_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_mapping_lane3_fmt_1`]
module"]
#[doc(alias = "CFG_LVDS_MAPPING_LANE3_FMT_1")]
pub type CfgLvdsMappingLane3Fmt1 =
    crate::Reg<cfg_lvds_mapping_lane3_fmt_1::CfgLvdsMappingLane3Fmt1Spec>;
#[doc = "CFG_LVDS_MAPPING_LANE3_FMT_1"]
pub mod cfg_lvds_mapping_lane3_fmt_1;
#[doc = "CFG_LVDS_GEN_0 (rw) register accessor: CFG_LVDS_GEN_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_gen_0`]
module"]
#[doc(alias = "CFG_LVDS_GEN_0")]
pub type CfgLvdsGen0 = crate::Reg<cfg_lvds_gen_0::CfgLvdsGen0Spec>;
#[doc = "CFG_LVDS_GEN_0"]
pub mod cfg_lvds_gen_0;
#[doc = "CFG_LVDS_GEN_1 (rw) register accessor: CFG_LVDS_GEN_1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_gen_1`]
module"]
#[doc(alias = "CFG_LVDS_GEN_1")]
pub type CfgLvdsGen1 = crate::Reg<cfg_lvds_gen_1::CfgLvdsGen1Spec>;
#[doc = "CFG_LVDS_GEN_1"]
pub mod cfg_lvds_gen_1;
#[doc = "CFG_LVDS_GEN_2 (rw) register accessor: CFG_LVDS_GEN_2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lvds_gen_2`]
module"]
#[doc(alias = "CFG_LVDS_GEN_2")]
pub type CfgLvdsGen2 = crate::Reg<cfg_lvds_gen_2::CfgLvdsGen2Spec>;
#[doc = "CFG_LVDS_GEN_2"]
pub mod cfg_lvds_gen_2;
#[doc = "CFG_MASK_REG0 (rw) register accessor: CFG_MASK_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_mask_reg0`]
module"]
#[doc(alias = "CFG_MASK_REG0")]
pub type CfgMaskReg0 = crate::Reg<cfg_mask_reg0::CfgMaskReg0Spec>;
#[doc = "CFG_MASK_REG0"]
pub mod cfg_mask_reg0;
#[doc = "CFG_MASK_REG1 (rw) register accessor: CFG_MASK_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_mask_reg1`]
module"]
#[doc(alias = "CFG_MASK_REG1")]
pub type CfgMaskReg1 = crate::Reg<cfg_mask_reg1::CfgMaskReg1Spec>;
#[doc = "CFG_MASK_REG1"]
pub mod cfg_mask_reg1;
#[doc = "CFG_MASK_REG2 (rw) register accessor: CFG_MASK_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_mask_reg2`]
module"]
#[doc(alias = "CFG_MASK_REG2")]
pub type CfgMaskReg2 = crate::Reg<cfg_mask_reg2::CfgMaskReg2Spec>;
#[doc = "CFG_MASK_REG2"]
pub mod cfg_mask_reg2;
#[doc = "CFG_MASK_REG3 (rw) register accessor: CFG_MASK_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_mask_reg3`]
module"]
#[doc(alias = "CFG_MASK_REG3")]
pub type CfgMaskReg3 = crate::Reg<cfg_mask_reg3::CfgMaskReg3Spec>;
#[doc = "CFG_MASK_REG3"]
pub mod cfg_mask_reg3;
#[doc = "STAT_CBUFF_REG0 (rw) register accessor: STAT_CBUFF_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cbuff_reg0`]
module"]
#[doc(alias = "STAT_CBUFF_REG0")]
pub type StatCbuffReg0 = crate::Reg<stat_cbuff_reg0::StatCbuffReg0Spec>;
#[doc = "STAT_CBUFF_REG0"]
pub mod stat_cbuff_reg0;
#[doc = "STAT_CBUFF_REG1 (rw) register accessor: STAT_CBUFF_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cbuff_reg1`]
module"]
#[doc(alias = "STAT_CBUFF_REG1")]
pub type StatCbuffReg1 = crate::Reg<stat_cbuff_reg1::StatCbuffReg1Spec>;
#[doc = "STAT_CBUFF_REG1"]
pub mod stat_cbuff_reg1;
#[doc = "STAT_CBUFF_REG2 (rw) register accessor: STAT_CBUFF_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cbuff_reg2`]
module"]
#[doc(alias = "STAT_CBUFF_REG2")]
pub type StatCbuffReg2 = crate::Reg<stat_cbuff_reg2::StatCbuffReg2Spec>;
#[doc = "STAT_CBUFF_REG2"]
pub mod stat_cbuff_reg2;
#[doc = "STAT_CBUFF_REG3 (rw) register accessor: STAT_CBUFF_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cbuff_reg3`]
module"]
#[doc(alias = "STAT_CBUFF_REG3")]
pub type StatCbuffReg3 = crate::Reg<stat_cbuff_reg3::StatCbuffReg3Spec>;
#[doc = "STAT_CBUFF_REG3"]
pub mod stat_cbuff_reg3;
#[doc = "STAT_LVDS_REG0 (rw) register accessor: STAT_LVDS_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_lvds_reg0`]
module"]
#[doc(alias = "STAT_LVDS_REG0")]
pub type StatLvdsReg0 = crate::Reg<stat_lvds_reg0::StatLvdsReg0Spec>;
#[doc = "STAT_LVDS_REG0"]
pub mod stat_lvds_reg0;
#[doc = "STAT_LVDS_REG1 (rw) register accessor: STAT_LVDS_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_lvds_reg1`]
module"]
#[doc(alias = "STAT_LVDS_REG1")]
pub type StatLvdsReg1 = crate::Reg<stat_lvds_reg1::StatLvdsReg1Spec>;
#[doc = "STAT_LVDS_REG1"]
pub mod stat_lvds_reg1;
#[doc = "STAT_LVDS_REG2 (rw) register accessor: STAT_LVDS_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_lvds_reg2`]
module"]
#[doc(alias = "STAT_LVDS_REG2")]
pub type StatLvdsReg2 = crate::Reg<stat_lvds_reg2::StatLvdsReg2Spec>;
#[doc = "STAT_LVDS_REG2"]
pub mod stat_lvds_reg2;
#[doc = "STAT_LVDS_REG3 (rw) register accessor: STAT_LVDS_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_lvds_reg3`]
module"]
#[doc(alias = "STAT_LVDS_REG3")]
pub type StatLvdsReg3 = crate::Reg<stat_lvds_reg3::StatLvdsReg3Spec>;
#[doc = "STAT_LVDS_REG3"]
pub mod stat_lvds_reg3;
#[doc = "CLR_CBUFF_REG0 (rw) register accessor: CLR_CBUFF_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_cbuff_reg0`]
module"]
#[doc(alias = "CLR_CBUFF_REG0")]
pub type ClrCbuffReg0 = crate::Reg<clr_cbuff_reg0::ClrCbuffReg0Spec>;
#[doc = "CLR_CBUFF_REG0"]
pub mod clr_cbuff_reg0;
#[doc = "CLR_CBUFF_REG1 (rw) register accessor: CLR_CBUFF_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_cbuff_reg1`]
module"]
#[doc(alias = "CLR_CBUFF_REG1")]
pub type ClrCbuffReg1 = crate::Reg<clr_cbuff_reg1::ClrCbuffReg1Spec>;
#[doc = "CLR_CBUFF_REG1"]
pub mod clr_cbuff_reg1;
#[doc = "CLR_LVDS_REG0 (rw) register accessor: CLR_LVDS_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_lvds_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_lvds_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_lvds_reg0`]
module"]
#[doc(alias = "CLR_LVDS_REG0")]
pub type ClrLvdsReg0 = crate::Reg<clr_lvds_reg0::ClrLvdsReg0Spec>;
#[doc = "CLR_LVDS_REG0"]
pub mod clr_lvds_reg0;
#[doc = "CLR_LVDS_REG1 (rw) register accessor: CLR_LVDS_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_lvds_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_lvds_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_lvds_reg1`]
module"]
#[doc(alias = "CLR_LVDS_REG1")]
pub type ClrLvdsReg1 = crate::Reg<clr_lvds_reg1::ClrLvdsReg1Spec>;
#[doc = "CLR_LVDS_REG1"]
pub mod clr_lvds_reg1;
#[doc = "STAT_CBUFF_ECC_REG (rw) register accessor: STAT_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_ecc_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_ecc_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cbuff_ecc_reg`]
module"]
#[doc(alias = "STAT_CBUFF_ECC_REG")]
pub type StatCbuffEccReg = crate::Reg<stat_cbuff_ecc_reg::StatCbuffEccRegSpec>;
#[doc = "STAT_CBUFF_ECC_REG"]
pub mod stat_cbuff_ecc_reg;
#[doc = "MASK_CBUFF_ECC_REG (rw) register accessor: MASK_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_cbuff_ecc_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_cbuff_ecc_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_cbuff_ecc_reg`]
module"]
#[doc(alias = "MASK_CBUFF_ECC_REG")]
pub type MaskCbuffEccReg = crate::Reg<mask_cbuff_ecc_reg::MaskCbuffEccRegSpec>;
#[doc = "MASK_CBUFF_ECC_REG"]
pub mod mask_cbuff_ecc_reg;
#[doc = "CLR_CBUFF_ECC_REG (rw) register accessor: CLR_CBUFF_ECC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_ecc_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_ecc_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_cbuff_ecc_reg`]
module"]
#[doc(alias = "CLR_CBUFF_ECC_REG")]
pub type ClrCbuffEccReg = crate::Reg<clr_cbuff_ecc_reg::ClrCbuffEccRegSpec>;
#[doc = "CLR_CBUFF_ECC_REG"]
pub mod clr_cbuff_ecc_reg;
#[doc = "STAT_SAFETY (rw) register accessor: STAT_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_safety::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_safety::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_safety`]
module"]
#[doc(alias = "STAT_SAFETY")]
pub type StatSafety = crate::Reg<stat_safety::StatSafetySpec>;
#[doc = "STAT_SAFETY"]
pub mod stat_safety;
#[doc = "MASK_SAFETY (rw) register accessor: MASK_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_safety::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_safety::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_safety`]
module"]
#[doc(alias = "MASK_SAFETY")]
pub type MaskSafety = crate::Reg<mask_safety::MaskSafetySpec>;
#[doc = "MASK_SAFETY"]
pub mod mask_safety;
#[doc = "CLR_SAFETY (rw) register accessor: CLR_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_safety::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_safety::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_safety`]
module"]
#[doc(alias = "CLR_SAFETY")]
pub type ClrSafety = crate::Reg<clr_safety::ClrSafetySpec>;
#[doc = "CLR_SAFETY"]
pub mod clr_safety;
