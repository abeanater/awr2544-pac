#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpsw_nuss_idver_reg: CpswNussIdverReg,
    ss_synce_count_reg: SsSynceCountReg,
    ss_synce_mux_reg: SsSynceMuxReg,
    ss_control_reg: SsControlReg,
    _reserved4: [u8; 0x08],
    ss_int_control_reg: SsIntControlReg,
    ss_status_reg: SsStatusReg,
    subsystem_config_reg: SubsystemConfigReg,
    _reserved7: [u8; 0x0c],
    rgmii1_status_reg: Rgmii1StatusReg,
    _reserved8: [u8; 0x0ecc],
    mdio_mdio_version_reg: MdioMdioVersionReg,
    mdio_control_reg: MdioControlReg,
    mdio_alive_reg: MdioAliveReg,
    mdio_link_reg: MdioLinkReg,
    mdio_link_int_raw_reg: MdioLinkIntRawReg,
    mdio_link_int_masked_reg: MdioLinkIntMaskedReg,
    mdio_link_int_mask_set_reg: MdioLinkIntMaskSetReg,
    mdio_link_int_mask_clear_reg: MdioLinkIntMaskClearReg,
    mdio_user_int_raw_reg: MdioUserIntRawReg,
    mdio_user_int_masked_reg: MdioUserIntMaskedReg,
    mdio_user_int_mask_set_reg: MdioUserIntMaskSetReg,
    mdio_user_int_mask_clear_reg: MdioUserIntMaskClearReg,
    mdio_manual_if_reg: MdioManualIfReg,
    mdio_poll_reg: MdioPollReg,
    mdio_poll_en_reg: MdioPollEnReg,
    mdio_claus45_reg: MdioClaus45Reg,
    mdio_user_addr0_reg: MdioUserAddr0Reg,
    mdio_user_addr1_reg: MdioUserAddr1Reg,
    _reserved26: [u8; 0x38],
    user_group0_user_access_reg: UserGroup0UserAccessReg,
    user_group0_user_phy_sel_reg: UserGroup0UserPhySelReg,
    user_group1_user_access_reg: UserGroup1UserAccessReg,
    user_group1_user_phy_sel_reg: UserGroup1UserPhySelReg,
    _reserved30: [u8; 0x0870],
    regs_int_ss_c0_th_thresh_pulse_en_reg: RegsIntSsC0ThThreshPulseEnReg,
    regs_int_ss_c0_th_pulse_en_reg: RegsIntSsC0ThPulseEnReg,
    regs_int_ss_c0_fh_pulse_en_reg: RegsIntSsC0FhPulseEnReg,
    regs_int_ss_c0_misc_en_reg: RegsIntSsC0MiscEnReg,
    regs_int_ss_c0_th_thresh_pulse_status_reg: RegsIntSsC0ThThreshPulseStatusReg,
    regs_int_ss_c0_th_pulse_status_reg: RegsIntSsC0ThPulseStatusReg,
    regs_int_ss_c0_fh_pulse_status_reg: RegsIntSsC0FhPulseStatusReg,
    regs_int_ss_c0_misc_status_reg: RegsIntSsC0MiscStatusReg,
    regs_int_ss_c0_th_imax_reg: RegsIntSsC0ThImaxReg,
    regs_int_ss_c0_fh_imax_reg: RegsIntSsC0FhImaxReg,
    _reserved40: [u8; 0x0001_e7d8],
    cpsw_nc_cpsw_id_ver_reg: CpswNcCpswIdVerReg,
    cpsw_nc_control_reg: CpswNcControlReg,
    _reserved42: [u8; 0x08],
    cpsw_nc_em_control_reg: CpswNcEmControlReg,
    cpsw_nc_stat_port_en_reg: CpswNcStatPortEnReg,
    cpsw_nc_ptype_reg: CpswNcPtypeReg,
    cpsw_nc_soft_idle_reg: CpswNcSoftIdleReg,
    cpsw_nc_thru_rate_reg: CpswNcThruRateReg,
    cpsw_nc_gap_thresh_reg: CpswNcGapThreshReg,
    _reserved48: [u8; 0x04],
    cpsw_nc_eee_prescale_reg: CpswNcEeePrescaleReg,
    cpsw_nc_tx_g_oflow_thresh_set_reg: CpswNcTxGOflowThreshSetReg,
    cpsw_nc_tx_g_oflow_thresh_clr_reg: CpswNcTxGOflowThreshClrReg,
    cpsw_nc_tx_g_buf_thresh_set_l_reg: CpswNcTxGBufThreshSetLReg,
    cpsw_nc_tx_g_buf_thresh_set_h_reg: CpswNcTxGBufThreshSetHReg,
    cpsw_nc_tx_g_buf_thresh_clr_l_reg: CpswNcTxGBufThreshClrLReg,
    cpsw_nc_tx_g_buf_thresh_clr_h_reg: CpswNcTxGBufThreshClrHReg,
    _reserved55: [u8; 0x08],
    cpsw_nc_vlan_ltype_reg: CpswNcVlanLtypeReg,
    cpsw_nc_est_ts_domain_reg: CpswNcEstTsDomainReg,
    _reserved57: [u8; 0xa8],
    cpsw_nc_tx_pri0_maxlen_reg: CpswNcTxPri0MaxlenReg,
    cpsw_nc_tx_pri1_maxlen_reg: CpswNcTxPri1MaxlenReg,
    cpsw_nc_tx_pri2_maxlen_reg: CpswNcTxPri2MaxlenReg,
    cpsw_nc_tx_pri3_maxlen_reg: CpswNcTxPri3MaxlenReg,
    cpsw_nc_tx_pri4_maxlen_reg: CpswNcTxPri4MaxlenReg,
    cpsw_nc_tx_pri5_maxlen_reg: CpswNcTxPri5MaxlenReg,
    cpsw_nc_tx_pri6_maxlen_reg: CpswNcTxPri6MaxlenReg,
    cpsw_nc_tx_pri7_maxlen_reg: CpswNcTxPri7MaxlenReg,
    _reserved65: [u8; 0x0ee4],
    cpsw_nc_cppi_p0_control_reg: CpswNcCppiP0ControlReg,
    cpsw_nc_cppi_p0_flow_id_offset_reg: CpswNcCppiP0FlowIdOffsetReg,
    _reserved67: [u8; 0x04],
    cpsw_nc_cppi_p0_blk_cnt_reg: CpswNcCppiP0BlkCntReg,
    cpsw_nc_cppi_p0_port_vlan_reg: CpswNcCppiP0PortVlanReg,
    cpsw_nc_cppi_p0_tx_pri_map_reg: CpswNcCppiP0TxPriMapReg,
    cpsw_nc_cppi_p0_pri_ctl_reg: CpswNcCppiP0PriCtlReg,
    cpsw_nc_cppi_p0_rx_pri_map_reg: CpswNcCppiP0RxPriMapReg,
    cpsw_nc_cppi_p0_rx_maxlen_reg: CpswNcCppiP0RxMaxlenReg,
    cpsw_nc_cppi_p0_tx_blks_pri_reg: CpswNcCppiP0TxBlksPriReg,
    _reserved74: [u8; 0x04],
    cpsw_nc_cppi_p0_idle2lpi_reg: CpswNcCppiP0Idle2lpiReg,
    cpsw_nc_cppi_p0_lpi2wake_reg: CpswNcCppiP0Lpi2wakeReg,
    cpsw_nc_cppi_p0_eee_status_reg: CpswNcCppiP0EeeStatusReg,
    cpsw_nc_cppi_p0_rx_pkts_pri_reg: CpswNcCppiP0RxPktsPriReg,
    _reserved78: [u8; 0x0c],
    cpsw_nc_cppi_p0_rx_gap_reg: CpswNcCppiP0RxGapReg,
    cpsw_nc_cppi_p0_fifo_status_reg: CpswNcCppiP0FifoStatusReg,
    _reserved80: [u8; 0x2c],
    cpsw_nc_cppi_p0_max_blks_reg: CpswNcCppiP0MaxBlksReg,
    _reserved81: [u8; 0x9c],
    cpsw_nc_cppi_p0_rx_dscp_map_reg_0: CpswNcCppiP0RxDscpMapReg0,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_1: CpswNcCppiP0RxDscpMapReg1,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_2: CpswNcCppiP0RxDscpMapReg2,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_3: CpswNcCppiP0RxDscpMapReg3,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_4: CpswNcCppiP0RxDscpMapReg4,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_5: CpswNcCppiP0RxDscpMapReg5,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_6: CpswNcCppiP0RxDscpMapReg6,
    cpsw_nc_cppi_p0_rx_dscp_map_reg_7: CpswNcCppiP0RxDscpMapReg7,
    cpsw_nc_cppi_p0_pri_cir_reg_0: CpswNcCppiP0PriCirReg0,
    cpsw_nc_cppi_p0_pri_cir_reg_1: CpswNcCppiP0PriCirReg1,
    cpsw_nc_cppi_p0_pri_cir_reg_2: CpswNcCppiP0PriCirReg2,
    cpsw_nc_cppi_p0_pri_cir_reg_3: CpswNcCppiP0PriCirReg3,
    cpsw_nc_cppi_p0_pri_cir_reg_4: CpswNcCppiP0PriCirReg4,
    cpsw_nc_cppi_p0_pri_cir_reg_5: CpswNcCppiP0PriCirReg5,
    cpsw_nc_cppi_p0_pri_cir_reg_6: CpswNcCppiP0PriCirReg6,
    cpsw_nc_cppi_p0_pri_cir_reg_7: CpswNcCppiP0PriCirReg7,
    cpsw_nc_cppi_p0_pri_eir_reg_0: CpswNcCppiP0PriEirReg0,
    cpsw_nc_cppi_p0_pri_eir_reg_1: CpswNcCppiP0PriEirReg1,
    cpsw_nc_cppi_p0_pri_eir_reg_2: CpswNcCppiP0PriEirReg2,
    cpsw_nc_cppi_p0_pri_eir_reg_3: CpswNcCppiP0PriEirReg3,
    cpsw_nc_cppi_p0_pri_eir_reg_4: CpswNcCppiP0PriEirReg4,
    cpsw_nc_cppi_p0_pri_eir_reg_5: CpswNcCppiP0PriEirReg5,
    cpsw_nc_cppi_p0_pri_eir_reg_6: CpswNcCppiP0PriEirReg6,
    cpsw_nc_cppi_p0_pri_eir_reg_7: CpswNcCppiP0PriEirReg7,
    cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg: CpswNcCppiP0TxDThreshSetLReg,
    cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg: CpswNcCppiP0TxDThreshSetHReg,
    cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg: CpswNcCppiP0TxDThreshClrLReg,
    cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg: CpswNcCppiP0TxDThreshClrHReg,
    cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg: CpswNcCppiP0TxGBufThreshSetLReg,
    cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg: CpswNcCppiP0TxGBufThreshSetHReg,
    cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg: CpswNcCppiP0TxGBufThreshClrLReg,
    cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg: CpswNcCppiP0TxGBufThreshClrHReg,
    _reserved113: [u8; 0x0160],
    cpsw_nc_cppi_p0_src_id_a_reg: CpswNcCppiP0SrcIdAReg,
    cpsw_nc_cppi_p0_src_id_b_reg: CpswNcCppiP0SrcIdBReg,
    _reserved115: [u8; 0x18],
    cpsw_nc_cppi_p0_host_blks_pri_reg: CpswNcCppiP0HostBlksPriReg,
    _reserved116: [u8; 0x0cdc],
    cpsw_nc_eth_mac_0_pn_reserved_reg: CpswNcEthMac0PnReservedReg,
    cpsw_nc_eth_mac_0_pn_control_reg: CpswNcEthMac0PnControlReg,
    cpsw_nc_eth_mac_0_pn_max_blks_reg: CpswNcEthMac0PnMaxBlksReg,
    _reserved119: [u8; 0x04],
    cpsw_nc_eth_mac_0_pn_blk_cnt_reg: CpswNcEthMac0PnBlkCntReg,
    cpsw_nc_eth_mac_0_pn_port_vlan_reg: CpswNcEthMac0PnPortVlanReg,
    cpsw_nc_eth_mac_0_pn_tx_pri_map_reg: CpswNcEthMac0PnTxPriMapReg,
    cpsw_nc_eth_mac_0_pn_pri_ctl_reg: CpswNcEthMac0PnPriCtlReg,
    cpsw_nc_eth_mac_0_pn_rx_pri_map_reg: CpswNcEthMac0PnRxPriMapReg,
    cpsw_nc_eth_mac_0_pn_rx_maxlen_reg: CpswNcEthMac0PnRxMaxlenReg,
    cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg: CpswNcEthMac0PnTxBlksPriReg,
    cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg: CpswNcEthMac0PnRxFlowThreshReg,
    cpsw_nc_eth_mac_0_pn_idle2lpi_reg: CpswNcEthMac0PnIdle2lpiReg,
    cpsw_nc_eth_mac_0_pn_lpi2wake_reg: CpswNcEthMac0PnLpi2wakeReg,
    cpsw_nc_eth_mac_0_pn_eee_status_reg: CpswNcEthMac0PnEeeStatusReg,
    _reserved130: [u8; 0x14],
    cpsw_nc_eth_mac_0_pn_fifo_status_reg: CpswNcEthMac0PnFifoStatusReg,
    _reserved131: [u8; 0x0c],
    cpsw_nc_eth_mac_0_pn_est_control_reg: CpswNcEthMac0PnEstControlReg,
    _reserved132: [u8; 0xbc],
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0: CpswNcEthMac0PnRxDscpMapReg0,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1: CpswNcEthMac0PnRxDscpMapReg1,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2: CpswNcEthMac0PnRxDscpMapReg2,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3: CpswNcEthMac0PnRxDscpMapReg3,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4: CpswNcEthMac0PnRxDscpMapReg4,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5: CpswNcEthMac0PnRxDscpMapReg5,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6: CpswNcEthMac0PnRxDscpMapReg6,
    cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7: CpswNcEthMac0PnRxDscpMapReg7,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_0: CpswNcEthMac0PnPriCirReg0,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_1: CpswNcEthMac0PnPriCirReg1,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_2: CpswNcEthMac0PnPriCirReg2,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_3: CpswNcEthMac0PnPriCirReg3,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_4: CpswNcEthMac0PnPriCirReg4,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_5: CpswNcEthMac0PnPriCirReg5,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_6: CpswNcEthMac0PnPriCirReg6,
    cpsw_nc_eth_mac_0_pn_pri_cir_reg_7: CpswNcEthMac0PnPriCirReg7,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_0: CpswNcEthMac0PnPriEirReg0,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_1: CpswNcEthMac0PnPriEirReg1,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_2: CpswNcEthMac0PnPriEirReg2,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_3: CpswNcEthMac0PnPriEirReg3,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_4: CpswNcEthMac0PnPriEirReg4,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_5: CpswNcEthMac0PnPriEirReg5,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_6: CpswNcEthMac0PnPriEirReg6,
    cpsw_nc_eth_mac_0_pn_pri_eir_reg_7: CpswNcEthMac0PnPriEirReg7,
    cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg: CpswNcEthMac0PnTxDThreshSetLReg,
    cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg: CpswNcEthMac0PnTxDThreshSetHReg,
    cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg: CpswNcEthMac0PnTxDThreshClrLReg,
    cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg: CpswNcEthMac0PnTxDThreshClrHReg,
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg: CpswNcEthMac0PnTxGBufThreshSetLReg,
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg: CpswNcEthMac0PnTxGBufThreshSetHReg,
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg: CpswNcEthMac0PnTxGBufThreshClrLReg,
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg: CpswNcEthMac0PnTxGBufThreshClrHReg,
    _reserved164: [u8; 0x0160],
    cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg: CpswNcEthMac0PnTxDOflowAddvalLReg,
    cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg: CpswNcEthMac0PnTxDOflowAddvalHReg,
    cpsw_nc_eth_mac_0_pn_sa_l_reg: CpswNcEthMac0PnSaLReg,
    cpsw_nc_eth_mac_0_pn_sa_h_reg: CpswNcEthMac0PnSaHReg,
    cpsw_nc_eth_mac_0_pn_ts_ctl_reg: CpswNcEthMac0PnTsCtlReg,
    cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg: CpswNcEthMac0PnTsSeqLtypeReg,
    cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg: CpswNcEthMac0PnTsVlanLtypeReg,
    cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg: CpswNcEthMac0PnTsCtlLtype2Reg,
    cpsw_nc_eth_mac_0_pn_ts_ctl2_reg: CpswNcEthMac0PnTsCtl2Reg,
    _reserved173: [u8; 0x0c],
    cpsw_nc_eth_mac_0_pn_mac_control_reg: CpswNcEthMac0PnMacControlReg,
    cpsw_nc_eth_mac_0_pn_mac_status_reg: CpswNcEthMac0PnMacStatusReg,
    cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg: CpswNcEthMac0PnMacSoftResetReg,
    cpsw_nc_eth_mac_0_pn_mac_bofftest_reg: CpswNcEthMac0PnMacBofftestReg,
    cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg: CpswNcEthMac0PnMacRxPausetimerReg,
    _reserved178: [u8; 0x0c],
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0: CpswNcEthMac0PnMacRxnPausetimerReg0,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1: CpswNcEthMac0PnMacRxnPausetimerReg1,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2: CpswNcEthMac0PnMacRxnPausetimerReg2,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3: CpswNcEthMac0PnMacRxnPausetimerReg3,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4: CpswNcEthMac0PnMacRxnPausetimerReg4,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5: CpswNcEthMac0PnMacRxnPausetimerReg5,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6: CpswNcEthMac0PnMacRxnPausetimerReg6,
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7: CpswNcEthMac0PnMacRxnPausetimerReg7,
    cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg: CpswNcEthMac0PnMacTxPausetimerReg,
    _reserved187: [u8; 0x0c],
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0: CpswNcEthMac0PnMacTxnPausetimerReg0,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1: CpswNcEthMac0PnMacTxnPausetimerReg1,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2: CpswNcEthMac0PnMacTxnPausetimerReg2,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3: CpswNcEthMac0PnMacTxnPausetimerReg3,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4: CpswNcEthMac0PnMacTxnPausetimerReg4,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5: CpswNcEthMac0PnMacTxnPausetimerReg5,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6: CpswNcEthMac0PnMacTxnPausetimerReg6,
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7: CpswNcEthMac0PnMacTxnPausetimerReg7,
    cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg: CpswNcEthMac0PnMacEmcontrolReg,
    cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg: CpswNcEthMac0PnMacTxGapReg,
    cpsw_nc_eth_mac_0_pn_mac_port_config: CpswNcEthMac0PnMacPortConfig,
    cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg: CpswNcEthMac0PnIntervlanOpxPointerReg,
    cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg: CpswNcEthMac0PnIntervlanOpxAReg,
    cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg: CpswNcEthMac0PnIntervlanOpxBReg,
    cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg: CpswNcEthMac0PnIntervlanOpxCReg,
    cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg: CpswNcEthMac0PnIntervlanOpxDReg,
    _reserved203: [u8; 0xfc40],
    cpsw_nc_est_fetch_loc_0: CpswNcEstFetchLoc0,
    cpsw_nc_est_fetch_loc_1: CpswNcEstFetchLoc1,
    cpsw_nc_est_fetch_loc_2: CpswNcEstFetchLoc2,
    cpsw_nc_est_fetch_loc_3: CpswNcEstFetchLoc3,
    cpsw_nc_est_fetch_loc_4: CpswNcEstFetchLoc4,
    cpsw_nc_est_fetch_loc_5: CpswNcEstFetchLoc5,
    cpsw_nc_est_fetch_loc_6: CpswNcEstFetchLoc6,
    cpsw_nc_est_fetch_loc_7: CpswNcEstFetchLoc7,
    cpsw_nc_est_fetch_loc_8: CpswNcEstFetchLoc8,
    cpsw_nc_est_fetch_loc_9: CpswNcEstFetchLoc9,
    cpsw_nc_est_fetch_loc_10: CpswNcEstFetchLoc10,
    cpsw_nc_est_fetch_loc_11: CpswNcEstFetchLoc11,
    cpsw_nc_est_fetch_loc_12: CpswNcEstFetchLoc12,
    cpsw_nc_est_fetch_loc_13: CpswNcEstFetchLoc13,
    cpsw_nc_est_fetch_loc_14: CpswNcEstFetchLoc14,
    cpsw_nc_est_fetch_loc_15: CpswNcEstFetchLoc15,
    cpsw_nc_est_fetch_loc_16: CpswNcEstFetchLoc16,
    cpsw_nc_est_fetch_loc_17: CpswNcEstFetchLoc17,
    cpsw_nc_est_fetch_loc_18: CpswNcEstFetchLoc18,
    cpsw_nc_est_fetch_loc_19: CpswNcEstFetchLoc19,
    cpsw_nc_est_fetch_loc_20: CpswNcEstFetchLoc20,
    cpsw_nc_est_fetch_loc_21: CpswNcEstFetchLoc21,
    cpsw_nc_est_fetch_loc_22: CpswNcEstFetchLoc22,
    cpsw_nc_est_fetch_loc_23: CpswNcEstFetchLoc23,
    cpsw_nc_est_fetch_loc_24: CpswNcEstFetchLoc24,
    cpsw_nc_est_fetch_loc_25: CpswNcEstFetchLoc25,
    cpsw_nc_est_fetch_loc_26: CpswNcEstFetchLoc26,
    cpsw_nc_est_fetch_loc_27: CpswNcEstFetchLoc27,
    cpsw_nc_est_fetch_loc_28: CpswNcEstFetchLoc28,
    cpsw_nc_est_fetch_loc_29: CpswNcEstFetchLoc29,
    cpsw_nc_est_fetch_loc_30: CpswNcEstFetchLoc30,
    cpsw_nc_est_fetch_loc_31: CpswNcEstFetchLoc31,
    cpsw_nc_est_fetch_loc_32: CpswNcEstFetchLoc32,
    cpsw_nc_est_fetch_loc_33: CpswNcEstFetchLoc33,
    cpsw_nc_est_fetch_loc_34: CpswNcEstFetchLoc34,
    cpsw_nc_est_fetch_loc_35: CpswNcEstFetchLoc35,
    cpsw_nc_est_fetch_loc_36: CpswNcEstFetchLoc36,
    cpsw_nc_est_fetch_loc_37: CpswNcEstFetchLoc37,
    cpsw_nc_est_fetch_loc_38: CpswNcEstFetchLoc38,
    cpsw_nc_est_fetch_loc_39: CpswNcEstFetchLoc39,
    cpsw_nc_est_fetch_loc_40: CpswNcEstFetchLoc40,
    cpsw_nc_est_fetch_loc_41: CpswNcEstFetchLoc41,
    cpsw_nc_est_fetch_loc_42: CpswNcEstFetchLoc42,
    cpsw_nc_est_fetch_loc_43: CpswNcEstFetchLoc43,
    cpsw_nc_est_fetch_loc_44: CpswNcEstFetchLoc44,
    cpsw_nc_est_fetch_loc_45: CpswNcEstFetchLoc45,
    cpsw_nc_est_fetch_loc_46: CpswNcEstFetchLoc46,
    cpsw_nc_est_fetch_loc_47: CpswNcEstFetchLoc47,
    cpsw_nc_est_fetch_loc_48: CpswNcEstFetchLoc48,
    cpsw_nc_est_fetch_loc_49: CpswNcEstFetchLoc49,
    cpsw_nc_est_fetch_loc_50: CpswNcEstFetchLoc50,
    cpsw_nc_est_fetch_loc_51: CpswNcEstFetchLoc51,
    cpsw_nc_est_fetch_loc_52: CpswNcEstFetchLoc52,
    cpsw_nc_est_fetch_loc_53: CpswNcEstFetchLoc53,
    cpsw_nc_est_fetch_loc_54: CpswNcEstFetchLoc54,
    cpsw_nc_est_fetch_loc_55: CpswNcEstFetchLoc55,
    cpsw_nc_est_fetch_loc_56: CpswNcEstFetchLoc56,
    cpsw_nc_est_fetch_loc_57: CpswNcEstFetchLoc57,
    cpsw_nc_est_fetch_loc_58: CpswNcEstFetchLoc58,
    cpsw_nc_est_fetch_loc_59: CpswNcEstFetchLoc59,
    cpsw_nc_est_fetch_loc_60: CpswNcEstFetchLoc60,
    cpsw_nc_est_fetch_loc_61: CpswNcEstFetchLoc61,
    cpsw_nc_est_fetch_loc_62: CpswNcEstFetchLoc62,
    cpsw_nc_est_fetch_loc_63: CpswNcEstFetchLoc63,
    cpsw_nc_est_fetch_loc_64: CpswNcEstFetchLoc64,
    cpsw_nc_est_fetch_loc_65: CpswNcEstFetchLoc65,
    cpsw_nc_est_fetch_loc_66: CpswNcEstFetchLoc66,
    cpsw_nc_est_fetch_loc_67: CpswNcEstFetchLoc67,
    cpsw_nc_est_fetch_loc_68: CpswNcEstFetchLoc68,
    cpsw_nc_est_fetch_loc_69: CpswNcEstFetchLoc69,
    cpsw_nc_est_fetch_loc_70: CpswNcEstFetchLoc70,
    cpsw_nc_est_fetch_loc_71: CpswNcEstFetchLoc71,
    cpsw_nc_est_fetch_loc_72: CpswNcEstFetchLoc72,
    cpsw_nc_est_fetch_loc_73: CpswNcEstFetchLoc73,
    cpsw_nc_est_fetch_loc_74: CpswNcEstFetchLoc74,
    cpsw_nc_est_fetch_loc_75: CpswNcEstFetchLoc75,
    cpsw_nc_est_fetch_loc_76: CpswNcEstFetchLoc76,
    cpsw_nc_est_fetch_loc_77: CpswNcEstFetchLoc77,
    cpsw_nc_est_fetch_loc_78: CpswNcEstFetchLoc78,
    cpsw_nc_est_fetch_loc_79: CpswNcEstFetchLoc79,
    cpsw_nc_est_fetch_loc_80: CpswNcEstFetchLoc80,
    cpsw_nc_est_fetch_loc_81: CpswNcEstFetchLoc81,
    cpsw_nc_est_fetch_loc_82: CpswNcEstFetchLoc82,
    cpsw_nc_est_fetch_loc_83: CpswNcEstFetchLoc83,
    cpsw_nc_est_fetch_loc_84: CpswNcEstFetchLoc84,
    cpsw_nc_est_fetch_loc_85: CpswNcEstFetchLoc85,
    cpsw_nc_est_fetch_loc_86: CpswNcEstFetchLoc86,
    cpsw_nc_est_fetch_loc_87: CpswNcEstFetchLoc87,
    cpsw_nc_est_fetch_loc_88: CpswNcEstFetchLoc88,
    cpsw_nc_est_fetch_loc_89: CpswNcEstFetchLoc89,
    cpsw_nc_est_fetch_loc_90: CpswNcEstFetchLoc90,
    cpsw_nc_est_fetch_loc_91: CpswNcEstFetchLoc91,
    cpsw_nc_est_fetch_loc_92: CpswNcEstFetchLoc92,
    cpsw_nc_est_fetch_loc_93: CpswNcEstFetchLoc93,
    cpsw_nc_est_fetch_loc_94: CpswNcEstFetchLoc94,
    cpsw_nc_est_fetch_loc_95: CpswNcEstFetchLoc95,
    cpsw_nc_est_fetch_loc_96: CpswNcEstFetchLoc96,
    cpsw_nc_est_fetch_loc_97: CpswNcEstFetchLoc97,
    cpsw_nc_est_fetch_loc_98: CpswNcEstFetchLoc98,
    cpsw_nc_est_fetch_loc_99: CpswNcEstFetchLoc99,
    cpsw_nc_est_fetch_loc_100: CpswNcEstFetchLoc100,
    cpsw_nc_est_fetch_loc_101: CpswNcEstFetchLoc101,
    cpsw_nc_est_fetch_loc_102: CpswNcEstFetchLoc102,
    cpsw_nc_est_fetch_loc_103: CpswNcEstFetchLoc103,
    cpsw_nc_est_fetch_loc_104: CpswNcEstFetchLoc104,
    cpsw_nc_est_fetch_loc_105: CpswNcEstFetchLoc105,
    cpsw_nc_est_fetch_loc_106: CpswNcEstFetchLoc106,
    cpsw_nc_est_fetch_loc_107: CpswNcEstFetchLoc107,
    cpsw_nc_est_fetch_loc_108: CpswNcEstFetchLoc108,
    cpsw_nc_est_fetch_loc_109: CpswNcEstFetchLoc109,
    cpsw_nc_est_fetch_loc_110: CpswNcEstFetchLoc110,
    cpsw_nc_est_fetch_loc_111: CpswNcEstFetchLoc111,
    cpsw_nc_est_fetch_loc_112: CpswNcEstFetchLoc112,
    cpsw_nc_est_fetch_loc_113: CpswNcEstFetchLoc113,
    cpsw_nc_est_fetch_loc_114: CpswNcEstFetchLoc114,
    cpsw_nc_est_fetch_loc_115: CpswNcEstFetchLoc115,
    cpsw_nc_est_fetch_loc_116: CpswNcEstFetchLoc116,
    cpsw_nc_est_fetch_loc_117: CpswNcEstFetchLoc117,
    cpsw_nc_est_fetch_loc_118: CpswNcEstFetchLoc118,
    cpsw_nc_est_fetch_loc_119: CpswNcEstFetchLoc119,
    cpsw_nc_est_fetch_loc_120: CpswNcEstFetchLoc120,
    cpsw_nc_est_fetch_loc_121: CpswNcEstFetchLoc121,
    cpsw_nc_est_fetch_loc_122: CpswNcEstFetchLoc122,
    cpsw_nc_est_fetch_loc_123: CpswNcEstFetchLoc123,
    cpsw_nc_est_fetch_loc_124: CpswNcEstFetchLoc124,
    cpsw_nc_est_fetch_loc_125: CpswNcEstFetchLoc125,
    cpsw_nc_est_fetch_loc_126: CpswNcEstFetchLoc126,
    cpsw_nc_est_fetch_loc_127: CpswNcEstFetchLoc127,
    _reserved331: [u8; 0x1e00],
    cpsw_cpdma_regs_cpdma_fh_idver_reg: CpswCpdmaRegsCpdmaFhIdverReg,
    cpsw_cpdma_regs_cpdma_fh_control_reg: CpswCpdmaRegsCpdmaFhControlReg,
    cpsw_cpdma_regs_cpdma_fh_teardown_reg: CpswCpdmaRegsCpdmaFhTeardownReg,
    cpsw_cpdma_regs_cpdma_fh_eoq_int: CpswCpdmaRegsCpdmaFhEoqInt,
    cpsw_cpdma_regs_cpdma_th_idver_reg: CpswCpdmaRegsCpdmaThIdverReg,
    cpsw_cpdma_regs_cpdma_th_control_reg: CpswCpdmaRegsCpdmaThControlReg,
    cpsw_cpdma_regs_cpdma_th_teardown_reg: CpswCpdmaRegsCpdmaThTeardownReg,
    cpsw_cpdma_regs_cpdma_soft_reset_reg: CpswCpdmaRegsCpdmaSoftResetReg,
    cpsw_cpdma_regs_cpdma_control_reg: CpswCpdmaRegsCpdmaControlReg,
    cpsw_cpdma_regs_cpdma_status_reg: CpswCpdmaRegsCpdmaStatusReg,
    cpsw_cpdma_regs_cpdma_th_buffer_offset_reg: CpswCpdmaRegsCpdmaThBufferOffsetReg,
    cpsw_cpdma_regs_cpdma_emulation_control_reg: CpswCpdmaRegsCpdmaEmulationControlReg,
    _reserved343: [u8; 0x50],
    cpsw_cpdma_int_cpdma_fh_intstat_raw_reg: CpswCpdmaIntCpdmaFhIntstatRawReg,
    cpsw_cpdma_int_cpdma_fh_intstat_masked_reg: CpswCpdmaIntCpdmaFhIntstatMaskedReg,
    cpsw_cpdma_int_cpdma_fh_intmask_set_reg: CpswCpdmaIntCpdmaFhIntmaskSetReg,
    cpsw_cpdma_int_cpdma_fh_intmask_clear_reg: CpswCpdmaIntCpdmaFhIntmaskClearReg,
    cpsw_cpdma_int_cpdma_in_vector_reg: CpswCpdmaIntCpdmaInVectorReg,
    cpsw_cpdma_int_cpdma_eoi_vector_reg: CpswCpdmaIntCpdmaEoiVectorReg,
    _reserved349: [u8; 0x08],
    cpsw_cpdma_int_cpdma_th_intstat_raw_reg: CpswCpdmaIntCpdmaThIntstatRawReg,
    cpsw_cpdma_int_cpdma_th_intstat_masked_reg: CpswCpdmaIntCpdmaThIntstatMaskedReg,
    cpsw_cpdma_int_cpdma_th_intmask_set_reg: CpswCpdmaIntCpdmaThIntmaskSetReg,
    cpsw_cpdma_int_cpdma_th_intmask_clear_reg: CpswCpdmaIntCpdmaThIntmaskClearReg,
    cpsw_cpdma_int_cpdma_intstat_raw_reg: CpswCpdmaIntCpdmaIntstatRawReg,
    cpsw_cpdma_int_cpdma_intstat_masked_reg: CpswCpdmaIntCpdmaIntstatMaskedReg,
    cpsw_cpdma_int_cpdma_intmask_set_reg: CpswCpdmaIntCpdmaIntmaskSetReg,
    cpsw_cpdma_int_cpdma_intmask_clear_reg: CpswCpdmaIntCpdmaIntmaskClearReg,
    cpsw_cpdma_int_cpdma_th0_pendthresh_reg: CpswCpdmaIntCpdmaTh0PendthreshReg,
    cpsw_cpdma_int_cpdma_th1_pendthresh_reg: CpswCpdmaIntCpdmaTh1PendthreshReg,
    cpsw_cpdma_int_cpdma_th2_pendthresh_reg: CpswCpdmaIntCpdmaTh2PendthreshReg,
    cpsw_cpdma_int_cpdma_th3_pendthresh_reg: CpswCpdmaIntCpdmaTh3PendthreshReg,
    cpsw_cpdma_int_cpdma_th4_pendthresh_reg: CpswCpdmaIntCpdmaTh4PendthreshReg,
    cpsw_cpdma_int_cpdma_th5_pendthresh_reg: CpswCpdmaIntCpdmaTh5PendthreshReg,
    cpsw_cpdma_int_cpdma_th6_pendthresh_reg: CpswCpdmaIntCpdmaTh6PendthreshReg,
    cpsw_cpdma_int_cpdma_th7_pendthresh_reg: CpswCpdmaIntCpdmaTh7PendthreshReg,
    cpsw_cpdma_int_cpdma_th0_freebuffer_reg: CpswCpdmaIntCpdmaTh0FreebufferReg,
    cpsw_cpdma_int_cpdma_th1_freebuffer_reg: CpswCpdmaIntCpdmaTh1FreebufferReg,
    cpsw_cpdma_int_cpdma_th2_freebuffer_reg: CpswCpdmaIntCpdmaTh2FreebufferReg,
    cpsw_cpdma_int_cpdma_th3_freebuffer_reg: CpswCpdmaIntCpdmaTh3FreebufferReg,
    cpsw_cpdma_int_cpdma_th4_freebuffer_reg: CpswCpdmaIntCpdmaTh4FreebufferReg,
    cpsw_cpdma_int_cpdma_th5_freebuffer_reg: CpswCpdmaIntCpdmaTh5FreebufferReg,
    cpsw_cpdma_int_cpdma_th6_freebuffer_reg: CpswCpdmaIntCpdmaTh6FreebufferReg,
    cpsw_cpdma_int_cpdma_th7_freebuffer_reg: CpswCpdmaIntCpdmaTh7FreebufferReg,
    _reserved373: [u8; 0x0100],
    cpsw_cpdma_sram_cpdma_fh0_hdp_reg: CpswCpdmaSramCpdmaFh0HdpReg,
    cpsw_cpdma_sram_cpdma_fh1_hdp_reg: CpswCpdmaSramCpdmaFh1HdpReg,
    cpsw_cpdma_sram_cpdma_fh2_hdp_reg: CpswCpdmaSramCpdmaFh2HdpReg,
    cpsw_cpdma_sram_cpdma_fh3_hdp_reg: CpswCpdmaSramCpdmaFh3HdpReg,
    cpsw_cpdma_sram_cpdma_fh4_hdp_reg: CpswCpdmaSramCpdmaFh4HdpReg,
    cpsw_cpdma_sram_cpdma_fh5_hdp_reg: CpswCpdmaSramCpdmaFh5HdpReg,
    cpsw_cpdma_sram_cpdma_fh6_hdp_reg: CpswCpdmaSramCpdmaFh6HdpReg,
    cpsw_cpdma_sram_cpdma_fh7_hdp_reg: CpswCpdmaSramCpdmaFh7HdpReg,
    cpsw_cpdma_sram_cpdma_th0_hdp_reg: CpswCpdmaSramCpdmaTh0HdpReg,
    cpsw_cpdma_sram_cpdma_th1_hdp_reg: CpswCpdmaSramCpdmaTh1HdpReg,
    cpsw_cpdma_sram_cpdma_th2_hdp_reg: CpswCpdmaSramCpdmaTh2HdpReg,
    cpsw_cpdma_sram_cpdma_th3_hdp_reg: CpswCpdmaSramCpdmaTh3HdpReg,
    cpsw_cpdma_sram_cpdma_th4_hdp_reg: CpswCpdmaSramCpdmaTh4HdpReg,
    cpsw_cpdma_sram_cpdma_th5_hdp_reg: CpswCpdmaSramCpdmaTh5HdpReg,
    cpsw_cpdma_sram_cpdma_th6_hdp_reg: CpswCpdmaSramCpdmaTh6HdpReg,
    cpsw_cpdma_sram_cpdma_th7_hdp_reg: CpswCpdmaSramCpdmaTh7HdpReg,
    cpsw_cpdma_sram_cpdma_fh0_cp_reg: CpswCpdmaSramCpdmaFh0CpReg,
    cpsw_cpdma_sram_cpdma_fh1_cp_reg: CpswCpdmaSramCpdmaFh1CpReg,
    cpsw_cpdma_sram_cpdma_fh2_cp_reg: CpswCpdmaSramCpdmaFh2CpReg,
    cpsw_cpdma_sram_cpdma_fh3_cp_reg: CpswCpdmaSramCpdmaFh3CpReg,
    cpsw_cpdma_sram_cpdma_fh4_cp_reg: CpswCpdmaSramCpdmaFh4CpReg,
    cpsw_cpdma_sram_cpdma_fh5_cp_reg: CpswCpdmaSramCpdmaFh5CpReg,
    cpsw_cpdma_sram_cpdma_fh6_cp_reg: CpswCpdmaSramCpdmaFh6CpReg,
    cpsw_cpdma_sram_cpdma_fh7_cp_reg: CpswCpdmaSramCpdmaFh7CpReg,
    cpsw_cpdma_sram_cpdma_th0_cp_reg: CpswCpdmaSramCpdmaTh0CpReg,
    cpsw_cpdma_sram_cpdma_th1_cp_reg: CpswCpdmaSramCpdmaTh1CpReg,
    cpsw_cpdma_sram_cpdma_th2_cp_reg: CpswCpdmaSramCpdmaTh2CpReg,
    cpsw_cpdma_sram_cpdma_th3_cp_reg: CpswCpdmaSramCpdmaTh3CpReg,
    cpsw_cpdma_sram_cpdma_th4_cp_reg: CpswCpdmaSramCpdmaTh4CpReg,
    cpsw_cpdma_sram_cpdma_th5_cp_reg: CpswCpdmaSramCpdmaTh5CpReg,
    cpsw_cpdma_sram_cpdma_th6_cp_reg: CpswCpdmaSramCpdmaTh6CpReg,
    cpsw_cpdma_sram_cpdma_th7_cp_reg: CpswCpdmaSramCpdmaTh7CpReg,
    _reserved405: [u8; 0x80],
    cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg: CpswCpdmaSramTestCpdmaFh0HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg: CpswCpdmaSramTestCpdmaFh1HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg: CpswCpdmaSramTestCpdmaFh2HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg: CpswCpdmaSramTestCpdmaFh3HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg: CpswCpdmaSramTestCpdmaFh4HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg: CpswCpdmaSramTestCpdmaFh5HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg: CpswCpdmaSramTestCpdmaFh6HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg: CpswCpdmaSramTestCpdmaFh7HdpReg,
    cpsw_cpdma_sram_test_cpdma_th0_hdp_reg: CpswCpdmaSramTestCpdmaTh0HdpReg,
    cpsw_cpdma_sram_test_cpdma_th1_hdp_reg: CpswCpdmaSramTestCpdmaTh1HdpReg,
    cpsw_cpdma_sram_test_cpdma_th2_hdp_reg: CpswCpdmaSramTestCpdmaTh2HdpReg,
    cpsw_cpdma_sram_test_cpdma_th3_hdp_reg: CpswCpdmaSramTestCpdmaTh3HdpReg,
    cpsw_cpdma_sram_test_cpdma_th4_hdp_reg: CpswCpdmaSramTestCpdmaTh4HdpReg,
    cpsw_cpdma_sram_test_cpdma_th5_hdp_reg: CpswCpdmaSramTestCpdmaTh5HdpReg,
    cpsw_cpdma_sram_test_cpdma_th6_hdp_reg: CpswCpdmaSramTestCpdmaTh6HdpReg,
    cpsw_cpdma_sram_test_cpdma_th7_hdp_reg: CpswCpdmaSramTestCpdmaTh7HdpReg,
    cpsw_cpdma_sram_test_cpdma_fh0_cp_reg: CpswCpdmaSramTestCpdmaFh0CpReg,
    cpsw_cpdma_sram_test_cpdma_fh1_cp_reg: CpswCpdmaSramTestCpdmaFh1CpReg,
    cpsw_cpdma_sram_test_cpdma_fh2_cp_reg: CpswCpdmaSramTestCpdmaFh2CpReg,
    cpsw_cpdma_sram_test_cpdma_fh3_cp_reg: CpswCpdmaSramTestCpdmaFh3CpReg,
    cpsw_cpdma_sram_test_cpdma_fh4_cp_reg: CpswCpdmaSramTestCpdmaFh4CpReg,
    cpsw_cpdma_sram_test_cpdma_fh5_cp_reg: CpswCpdmaSramTestCpdmaFh5CpReg,
    cpsw_cpdma_sram_test_cpdma_fh6_cp_reg: CpswCpdmaSramTestCpdmaFh6CpReg,
    cpsw_cpdma_sram_test_cpdma_fh7_cp_reg: CpswCpdmaSramTestCpdmaFh7CpReg,
    cpsw_cpdma_sram_test_cpdma_th0_cp_reg: CpswCpdmaSramTestCpdmaTh0CpReg,
    cpsw_cpdma_sram_test_cpdma_th1_cp_reg: CpswCpdmaSramTestCpdmaTh1CpReg,
    cpsw_cpdma_sram_test_cpdma_th2_cp_reg: CpswCpdmaSramTestCpdmaTh2CpReg,
    cpsw_cpdma_sram_test_cpdma_th3_cp_reg: CpswCpdmaSramTestCpdmaTh3CpReg,
    cpsw_cpdma_sram_test_cpdma_th4_cp_reg: CpswCpdmaSramTestCpdmaTh4CpReg,
    cpsw_cpdma_sram_test_cpdma_th5_cp_reg: CpswCpdmaSramTestCpdmaTh5CpReg,
    cpsw_cpdma_sram_test_cpdma_th6_cp_reg: CpswCpdmaSramTestCpdmaTh6CpReg,
    cpsw_cpdma_sram_test_cpdma_th7_cp_reg: CpswCpdmaSramTestCpdmaTh7CpReg,
    _reserved437: [u8; 0x5c80],
    cpsw_nc_stat_0_rxgoodframes: CpswNcStat0Rxgoodframes,
    cpsw_nc_stat_0_rxbroadcastframes: CpswNcStat0Rxbroadcastframes,
    cpsw_nc_stat_0_rxmulticastframes: CpswNcStat0Rxmulticastframes,
    _reserved440: [u8; 0x04],
    cpsw_nc_stat_0_rxcrcerrors: CpswNcStat0Rxcrcerrors,
    _reserved441: [u8; 0x04],
    cpsw_nc_stat_0_rxoversizedframes: CpswNcStat0Rxoversizedframes,
    _reserved442: [u8; 0x04],
    cpsw_nc_stat_0_rxundersizedframes: CpswNcStat0Rxundersizedframes,
    cpsw_nc_stat_0_rxfragments: CpswNcStat0Rxfragments,
    cpsw_nc_stat_0_ale_drop: CpswNcStat0AleDrop,
    cpsw_nc_stat_0_ale_overrun_drop: CpswNcStat0AleOverrunDrop,
    cpsw_nc_stat_0_rxoctets: CpswNcStat0Rxoctets,
    cpsw_nc_stat_0_txgoodframes: CpswNcStat0Txgoodframes,
    cpsw_nc_stat_0_txbroadcastframes: CpswNcStat0Txbroadcastframes,
    cpsw_nc_stat_0_txmulticastframes: CpswNcStat0Txmulticastframes,
    _reserved450: [u8; 0x0c],
    cpsw_nc_stat_0_txsinglecollframes: CpswNcStat0Txsinglecollframes,
    cpsw_nc_stat_0_txmultcollframes: CpswNcStat0Txmultcollframes,
    _reserved452: [u8; 0x10],
    cpsw_nc_stat_0_txoctets: CpswNcStat0Txoctets,
    cpsw_nc_stat_0_octetframes64: CpswNcStat0Octetframes64,
    cpsw_nc_stat_0_octetframes65t127: CpswNcStat0Octetframes65t127,
    cpsw_nc_stat_0_octetframes128t255: CpswNcStat0Octetframes128t255,
    cpsw_nc_stat_0_octetframes256t511: CpswNcStat0Octetframes256t511,
    cpsw_nc_stat_0_octetframes512t1023: CpswNcStat0Octetframes512t1023,
    cpsw_nc_stat_0_octetframes1024tup: CpswNcStat0Octetframes1024tup,
    cpsw_nc_stat_0_netoctets: CpswNcStat0Netoctets,
    cpsw_nc_stat_0_rx_bottom_of_fifo_drop: CpswNcStat0RxBottomOfFifoDrop,
    cpsw_nc_stat_0_portmask_drop: CpswNcStat0PortmaskDrop,
    cpsw_nc_stat_0_rx_top_of_fifo_drop: CpswNcStat0RxTopOfFifoDrop,
    cpsw_nc_stat_0_ale_rate_limit_drop: CpswNcStat0AleRateLimitDrop,
    cpsw_nc_stat_0_ale_vid_ingress_drop: CpswNcStat0AleVidIngressDrop,
    cpsw_nc_stat_0_ale_da_eq_sa_drop: CpswNcStat0AleDaEqSaDrop,
    cpsw_nc_stat_0_ale_block_drop: CpswNcStat0AleBlockDrop,
    cpsw_nc_stat_0_ale_secure_drop: CpswNcStat0AleSecureDrop,
    cpsw_nc_stat_0_ale_auth_drop: CpswNcStat0AleAuthDrop,
    cpsw_nc_stat_0_ale_unkn_uni: CpswNcStat0AleUnknUni,
    cpsw_nc_stat_0_ale_unkn_uni_bcnt: CpswNcStat0AleUnknUniBcnt,
    cpsw_nc_stat_0_ale_unkn_mlt: CpswNcStat0AleUnknMlt,
    cpsw_nc_stat_0_ale_unkn_mlt_bcnt: CpswNcStat0AleUnknMltBcnt,
    cpsw_nc_stat_0_ale_unkn_brd: CpswNcStat0AleUnknBrd,
    cpsw_nc_stat_0_ale_unkn_brd_bcnt: CpswNcStat0AleUnknBrdBcnt,
    cpsw_nc_stat_0_ale_pol_match: CpswNcStat0AlePolMatch,
    cpsw_nc_stat_0_ale_pol_match_red: CpswNcStat0AlePolMatchRed,
    cpsw_nc_stat_0_ale_pol_match_yellow: CpswNcStat0AlePolMatchYellow,
    cpsw_nc_stat_0_ale_mult_sa_drop: CpswNcStat0AleMultSaDrop,
    cpsw_nc_stat_0_ale_dual_vlan_drop: CpswNcStat0AleDualVlanDrop,
    cpsw_nc_stat_0_ale_len_error_drop: CpswNcStat0AleLenErrorDrop,
    cpsw_nc_stat_0_ale_ip_next_hdr_drop: CpswNcStat0AleIpNextHdrDrop,
    cpsw_nc_stat_0_ale_ipv4_frag_drop: CpswNcStat0AleIpv4FragDrop,
    _reserved483: [u8; 0x9c],
    cpsw_nc_stat_0_tx_memory_protect_error: CpswNcStat0TxMemoryProtectError,
    _reserved484: [u8; 0x80],
    cpsw_nc_stat_1_rxgoodframes: CpswNcStat1Rxgoodframes,
    cpsw_nc_stat_1_rxbroadcastframes: CpswNcStat1Rxbroadcastframes,
    cpsw_nc_stat_1_rxmulticastframes: CpswNcStat1Rxmulticastframes,
    cpsw_nc_stat_1_rxpauseframes: CpswNcStat1Rxpauseframes,
    cpsw_nc_stat_1_rxcrcerrors: CpswNcStat1Rxcrcerrors,
    cpsw_nc_stat_1_rxaligncodeerrors: CpswNcStat1Rxaligncodeerrors,
    cpsw_nc_stat_1_rxoversizedframes: CpswNcStat1Rxoversizedframes,
    cpsw_nc_stat_1_rxjabberframes: CpswNcStat1Rxjabberframes,
    cpsw_nc_stat_1_rxundersizedframes: CpswNcStat1Rxundersizedframes,
    cpsw_nc_stat_1_rxfragments: CpswNcStat1Rxfragments,
    cpsw_nc_stat_1_ale_drop: CpswNcStat1AleDrop,
    cpsw_nc_stat_1_ale_overrun_drop: CpswNcStat1AleOverrunDrop,
    cpsw_nc_stat_1_rxoctets: CpswNcStat1Rxoctets,
    cpsw_nc_stat_1_txgoodframes: CpswNcStat1Txgoodframes,
    cpsw_nc_stat_1_txbroadcastframes: CpswNcStat1Txbroadcastframes,
    cpsw_nc_stat_1_txmulticastframes: CpswNcStat1Txmulticastframes,
    cpsw_nc_stat_1_txpauseframes: CpswNcStat1Txpauseframes,
    cpsw_nc_stat_1_txdeferredframes: CpswNcStat1Txdeferredframes,
    cpsw_nc_stat_1_txcollisionframes: CpswNcStat1Txcollisionframes,
    cpsw_nc_stat_1_txsinglecollframes: CpswNcStat1Txsinglecollframes,
    cpsw_nc_stat_1_txmultcollframes: CpswNcStat1Txmultcollframes,
    cpsw_nc_stat_1_txexcessivecollisions: CpswNcStat1Txexcessivecollisions,
    cpsw_nc_stat_1_txlatecollisions: CpswNcStat1Txlatecollisions,
    cpsw_nc_stat_1_rxipgerror: CpswNcStat1Rxipgerror,
    cpsw_nc_stat_1_txcarriersenseerrors: CpswNcStat1Txcarriersenseerrors,
    cpsw_nc_stat_1_txoctets: CpswNcStat1Txoctets,
    cpsw_nc_stat_1_octetframes64: CpswNcStat1Octetframes64,
    cpsw_nc_stat_1_octetframes65t127: CpswNcStat1Octetframes65t127,
    cpsw_nc_stat_1_octetframes128t255: CpswNcStat1Octetframes128t255,
    cpsw_nc_stat_1_octetframes256t511: CpswNcStat1Octetframes256t511,
    cpsw_nc_stat_1_octetframes512t1023: CpswNcStat1Octetframes512t1023,
    cpsw_nc_stat_1_octetframes1024tup: CpswNcStat1Octetframes1024tup,
    cpsw_nc_stat_1_netoctets: CpswNcStat1Netoctets,
    cpsw_nc_stat_1_rx_bottom_of_fifo_drop: CpswNcStat1RxBottomOfFifoDrop,
    cpsw_nc_stat_1_portmask_drop: CpswNcStat1PortmaskDrop,
    cpsw_nc_stat_1_rx_top_of_fifo_drop: CpswNcStat1RxTopOfFifoDrop,
    cpsw_nc_stat_1_ale_rate_limit_drop: CpswNcStat1AleRateLimitDrop,
    cpsw_nc_stat_1_ale_vid_ingress_drop: CpswNcStat1AleVidIngressDrop,
    cpsw_nc_stat_1_ale_da_eq_sa_drop: CpswNcStat1AleDaEqSaDrop,
    cpsw_nc_stat_1_ale_block_drop: CpswNcStat1AleBlockDrop,
    cpsw_nc_stat_1_ale_secure_drop: CpswNcStat1AleSecureDrop,
    cpsw_nc_stat_1_ale_auth_drop: CpswNcStat1AleAuthDrop,
    cpsw_nc_stat_1_ale_unkn_uni: CpswNcStat1AleUnknUni,
    cpsw_nc_stat_1_ale_unkn_uni_bcnt: CpswNcStat1AleUnknUniBcnt,
    cpsw_nc_stat_1_ale_unkn_mlt: CpswNcStat1AleUnknMlt,
    cpsw_nc_stat_1_ale_unkn_mlt_bcnt: CpswNcStat1AleUnknMltBcnt,
    cpsw_nc_stat_1_ale_unkn_brd: CpswNcStat1AleUnknBrd,
    cpsw_nc_stat_1_ale_unkn_brd_bcnt: CpswNcStat1AleUnknBrdBcnt,
    cpsw_nc_stat_1_ale_pol_match: CpswNcStat1AlePolMatch,
    cpsw_nc_stat_1_ale_pol_match_red: CpswNcStat1AlePolMatchRed,
    cpsw_nc_stat_1_ale_pol_match_yellow: CpswNcStat1AlePolMatchYellow,
    cpsw_nc_stat_1_ale_mult_sa_drop: CpswNcStat1AleMultSaDrop,
    cpsw_nc_stat_1_ale_dual_vlan_drop: CpswNcStat1AleDualVlanDrop,
    cpsw_nc_stat_1_ale_len_error_drop: CpswNcStat1AleLenErrorDrop,
    cpsw_nc_stat_1_ale_ip_next_hdr_drop: CpswNcStat1AleIpNextHdrDrop,
    cpsw_nc_stat_1_ale_ipv4_frag_drop: CpswNcStat1AleIpv4FragDrop,
    _reserved540: [u8; 0x9c],
    cpsw_nc_stat_1_tx_memory_protect_error: CpswNcStat1TxMemoryProtectError,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_0: CpswNcStat1EnetPnTxPriReg0,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_1: CpswNcStat1EnetPnTxPriReg1,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_2: CpswNcStat1EnetPnTxPriReg2,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_3: CpswNcStat1EnetPnTxPriReg3,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_4: CpswNcStat1EnetPnTxPriReg4,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_5: CpswNcStat1EnetPnTxPriReg5,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_6: CpswNcStat1EnetPnTxPriReg6,
    cpsw_nc_stat_1_enet_pn_tx_pri_reg_7: CpswNcStat1EnetPnTxPriReg7,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0: CpswNcStat1EnetPnTxPriBcntReg0,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1: CpswNcStat1EnetPnTxPriBcntReg1,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2: CpswNcStat1EnetPnTxPriBcntReg2,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3: CpswNcStat1EnetPnTxPriBcntReg3,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4: CpswNcStat1EnetPnTxPriBcntReg4,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5: CpswNcStat1EnetPnTxPriBcntReg5,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6: CpswNcStat1EnetPnTxPriBcntReg6,
    cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7: CpswNcStat1EnetPnTxPriBcntReg7,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0: CpswNcStat1EnetPnTxPriDropReg0,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1: CpswNcStat1EnetPnTxPriDropReg1,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2: CpswNcStat1EnetPnTxPriDropReg2,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3: CpswNcStat1EnetPnTxPriDropReg3,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4: CpswNcStat1EnetPnTxPriDropReg4,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5: CpswNcStat1EnetPnTxPriDropReg5,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6: CpswNcStat1EnetPnTxPriDropReg6,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7: CpswNcStat1EnetPnTxPriDropReg7,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0: CpswNcStat1EnetPnTxPriDropBcntReg0,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1: CpswNcStat1EnetPnTxPriDropBcntReg1,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2: CpswNcStat1EnetPnTxPriDropBcntReg2,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3: CpswNcStat1EnetPnTxPriDropBcntReg3,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4: CpswNcStat1EnetPnTxPriDropBcntReg4,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5: CpswNcStat1EnetPnTxPriDropBcntReg5,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6: CpswNcStat1EnetPnTxPriDropBcntReg6,
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7: CpswNcStat1EnetPnTxPriDropBcntReg7,
    _reserved573: [u8; 0x2c00],
    idver_reg: IdverReg,
    cpts_control_reg: CptsControlReg,
    cpts_rftclk_sel_reg: CptsRftclkSelReg,
    cpts_ts_push_reg: CptsTsPushReg,
    ts_load_val_reg: TsLoadValReg,
    cpts_ts_load_en_reg: CptsTsLoadEnReg,
    ts_comp_val_reg: TsCompValReg,
    cpts_ts_comp_len_reg: CptsTsCompLenReg,
    cpts_intstat_raw_reg: CptsIntstatRawReg,
    cpts_intstat_masked_reg: CptsIntstatMaskedReg,
    cpts_int_enable_reg: CptsIntEnableReg,
    cpts_ts_comp_nudge_reg: CptsTsCompNudgeReg,
    cpts_event_pop_reg: CptsEventPopReg,
    cpts_event_0_reg: CptsEvent0Reg,
    cpts_event_1_reg: CptsEvent1Reg,
    cpts_event_2_reg: CptsEvent2Reg,
    cpts_event_3_reg: CptsEvent3Reg,
    cpts_ts_load_high_val_reg: CptsTsLoadHighValReg,
    cpts_ts_comp_high_val_reg: CptsTsCompHighValReg,
    cpts_ts_add_val_reg: CptsTsAddValReg,
    cpts_ts_ppm_low_val_reg: CptsTsPpmLowValReg,
    cpts_ts_ppm_high_val_reg: CptsTsPpmHighValReg,
    cpts_ts_nudge_val_reg: CptsTsNudgeValReg,
    _reserved596: [u8; 0x74],
    cpts_ts_config: CptsTsConfig,
    _reserved597: [u8; 0x0c],
    ts_genf0_comp_low_reg: TsGenf0CompLowReg,
    ts_genf0_comp_high_reg: TsGenf0CompHighReg,
    ts_genf0_control_reg: TsGenf0ControlReg,
    ts_genf0_length_reg: TsGenf0LengthReg,
    ts_genf0_ppm_low_reg: TsGenf0PpmLowReg,
    ts_genf0_ppm_high_reg: TsGenf0PpmHighReg,
    ts_genf0_nudge_reg: TsGenf0NudgeReg,
    _reserved604: [u8; 0x04],
    ts_genf1_comp_low_reg: TsGenf1CompLowReg,
    ts_genf1_comp_high_reg: TsGenf1CompHighReg,
    ts_genf1_control_reg: TsGenf1ControlReg,
    ts_genf1_length_reg: TsGenf1LengthReg,
    ts_genf1_ppm_low_reg: TsGenf1PpmLowReg,
    ts_genf1_ppm_high_reg: TsGenf1PpmHighReg,
    ts_genf1_nudge_reg: TsGenf1NudgeReg,
    _reserved611: [u8; 0x04],
    ts_genf2_comp_low_reg: TsGenf2CompLowReg,
    ts_genf2_comp_high_reg: TsGenf2CompHighReg,
    ts_genf2_control_reg: TsGenf2ControlReg,
    ts_genf2_length_reg: TsGenf2LengthReg,
    ts_genf2_ppm_low_reg: TsGenf2PpmLowReg,
    ts_genf2_ppm_high_reg: TsGenf2PpmHighReg,
    ts_genf2_nudge_reg: TsGenf2NudgeReg,
    _reserved618: [u8; 0xc4],
    ts_estf_comp_low_reg: TsEstfCompLowReg,
    ts_estf_comp_high_reg: TsEstfCompHighReg,
    ts_estf_control_reg: TsEstfControlReg,
    ts_estf_length_reg: TsEstfLengthReg,
    ts_estf_ppm_low_reg: TsEstfPpmLowReg,
    ts_estf_ppm_high_reg: TsEstfPpmHighReg,
    ts_estf_nudge_reg: TsEstfNudgeReg,
    _reserved625: [u8; 0x0de4],
    ale_mod_ver: AleModVer,
    ale_ale_status: AleAleStatus,
    ale_ale_control: AleAleControl,
    ale_ale_ctrl2: AleAleCtrl2,
    ale_ale_prescale: AleAlePrescale,
    ale_ale_aging_ctrl: AleAleAgingCtrl,
    _reserved631: [u8; 0x04],
    ale_ale_nxt_hdr: AleAleNxtHdr,
    ale_ale_tblctl: AleAleTblctl,
    _reserved633: [u8; 0x10],
    ale_ale_tblw2: AleAleTblw2,
    ale_ale_tblw1: AleAleTblw1,
    ale_ale_tblw0: AleAleTblw0,
    ale_i0_ale_portctl0_0: AleI0AlePortctl0_0,
    ale_i0_ale_portctl0_1: AleI0AlePortctl0_1,
    _reserved638: [u8; 0x48],
    ale_ale_uvlan_member: AleAleUvlanMember,
    ale_ale_uvlan_urcast: AleAleUvlanUrcast,
    ale_ale_uvlan_rmcast: AleAleUvlanRmcast,
    ale_ale_uvlan_untag: AleAleUvlanUntag,
    _reserved642: [u8; 0x14],
    ale_ale_fast_lut: AleAleFastLut,
    ale_ale_stat_diag: AleAleStatDiag,
    ale_ale_oam_lb_ctrl: AleAleOamLbCtrl,
    ale_ale_msk_mux0: AleAleMskMux0,
    ale_i1_ale_msk_mux1_0: AleI1AleMskMux1_0,
    ale_i1_ale_msk_mux1_1: AleI1AleMskMux1_1,
    ale_i1_ale_msk_mux1_2: AleI1AleMskMux1_2,
    _reserved649: [u8; 0x2c],
    ale_egressop: AleEgressop,
    ale_policecfg0: AlePolicecfg0,
    ale_policecfg1: AlePolicecfg1,
    ale_policecfg2: AlePolicecfg2,
    ale_policecfg3: AlePolicecfg3,
    ale_policecfg4: AlePolicecfg4,
    _reserved655: [u8; 0x04],
    ale_policecfg6: AlePolicecfg6,
    ale_policecfg7: AlePolicecfg7,
    ale_policetblctl: AlePolicetblctl,
    ale_policecontrol: AlePolicecontrol,
    ale_policetestctl: AlePolicetestctl,
    ale_policehstat: AlePolicehstat,
    _reserved661: [u8; 0x04],
    ale_threadmapdef: AleThreadmapdef,
    ale_threadmapctl: AleThreadmapctl,
    ale_threadmapval: AleThreadmapval,
    _reserved664: [u8; 0x0ec0],
    rev: Rev,
    _reserved665: [u8; 0x04],
    vector: Vector,
    stat: Stat,
    ecc_reserved_svbus_0: EccReservedSvbus0,
    ecc_reserved_svbus_1: EccReservedSvbus1,
    ecc_reserved_svbus_2: EccReservedSvbus2,
    ecc_reserved_svbus_3: EccReservedSvbus3,
    ecc_reserved_svbus_4: EccReservedSvbus4,
    ecc_reserved_svbus_5: EccReservedSvbus5,
    ecc_reserved_svbus_6: EccReservedSvbus6,
    ecc_reserved_svbus_7: EccReservedSvbus7,
    _reserved675: [u8; 0x0c],
    ecc_sec_eoi_reg: EccSecEoiReg,
    ecc_sec_status_reg0: EccSecStatusReg0,
    _reserved677: [u8; 0x3c],
    ecc_sec_enable_set_reg0: EccSecEnableSetReg0,
    _reserved678: [u8; 0x3c],
    ecc_sec_enable_clr_reg0: EccSecEnableClrReg0,
    _reserved679: [u8; 0x78],
    ecc_ded_eoi_reg: EccDedEoiReg,
    ecc_ded_status_reg0: EccDedStatusReg0,
    _reserved681: [u8; 0x3c],
    ecc_ded_enable_set_reg0: EccDedEnableSetReg0,
    _reserved682: [u8; 0x3c],
    ecc_ded_enable_clr_reg0: EccDedEnableClrReg0,
    _reserved683: [u8; 0x3c],
    aggr_enable_set: AggrEnableSet,
    aggr_enable_clr: AggrEnableClr,
    aggr_status_set: AggrStatusSet,
    aggr_status_clr: AggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - ID Version Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_idver_reg(&self) -> &CpswNussIdverReg {
        &self.cpsw_nuss_idver_reg
    }
    #[doc = "0x04 - SS SYNCE Count Register"]
    #[inline(always)]
    pub const fn ss_synce_count_reg(&self) -> &SsSynceCountReg {
        &self.ss_synce_count_reg
    }
    #[doc = "0x08 - SS Synce Mux Register"]
    #[inline(always)]
    pub const fn ss_synce_mux_reg(&self) -> &SsSynceMuxReg {
        &self.ss_synce_mux_reg
    }
    #[doc = "0x0c - SS Control Register"]
    #[inline(always)]
    pub const fn ss_control_reg(&self) -> &SsControlReg {
        &self.ss_control_reg
    }
    #[doc = "0x18 - SS Interrupt Control Register"]
    #[inline(always)]
    pub const fn ss_int_control_reg(&self) -> &SsIntControlReg {
        &self.ss_int_control_reg
    }
    #[doc = "0x1c - SS Status Register"]
    #[inline(always)]
    pub const fn ss_status_reg(&self) -> &SsStatusReg {
        &self.ss_status_reg
    }
    #[doc = "0x20 - Subsystem Configuration Register"]
    #[inline(always)]
    pub const fn subsystem_config_reg(&self) -> &SubsystemConfigReg {
        &self.subsystem_config_reg
    }
    #[doc = "0x30 - RGMII1 Status Register"]
    #[inline(always)]
    pub const fn rgmii1_status_reg(&self) -> &Rgmii1StatusReg {
        &self.rgmii1_status_reg
    }
    #[doc = "0xf00 - MDIO Version Register"]
    #[inline(always)]
    pub const fn mdio_mdio_version_reg(&self) -> &MdioMdioVersionReg {
        &self.mdio_mdio_version_reg
    }
    #[doc = "0xf04 - MDIO Control Register"]
    #[inline(always)]
    pub const fn mdio_control_reg(&self) -> &MdioControlReg {
        &self.mdio_control_reg
    }
    #[doc = "0xf08 - MDIO Alive Register"]
    #[inline(always)]
    pub const fn mdio_alive_reg(&self) -> &MdioAliveReg {
        &self.mdio_alive_reg
    }
    #[doc = "0xf0c - MDIO Link Register"]
    #[inline(always)]
    pub const fn mdio_link_reg(&self) -> &MdioLinkReg {
        &self.mdio_link_reg
    }
    #[doc = "0xf10 - MDIO Link Interrupt Raw Register"]
    #[inline(always)]
    pub const fn mdio_link_int_raw_reg(&self) -> &MdioLinkIntRawReg {
        &self.mdio_link_int_raw_reg
    }
    #[doc = "0xf14 - MDIO Link Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mdio_link_int_masked_reg(&self) -> &MdioLinkIntMaskedReg {
        &self.mdio_link_int_masked_reg
    }
    #[doc = "0xf18 - MDIO Link Interrupt Mask Set Register"]
    #[inline(always)]
    pub const fn mdio_link_int_mask_set_reg(&self) -> &MdioLinkIntMaskSetReg {
        &self.mdio_link_int_mask_set_reg
    }
    #[doc = "0xf1c - MDIO Link Interrupt Mask Clear Register"]
    #[inline(always)]
    pub const fn mdio_link_int_mask_clear_reg(&self) -> &MdioLinkIntMaskClearReg {
        &self.mdio_link_int_mask_clear_reg
    }
    #[doc = "0xf20 - MDIO User Interrupt Raw Register"]
    #[inline(always)]
    pub const fn mdio_user_int_raw_reg(&self) -> &MdioUserIntRawReg {
        &self.mdio_user_int_raw_reg
    }
    #[doc = "0xf24 - MDIO User Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mdio_user_int_masked_reg(&self) -> &MdioUserIntMaskedReg {
        &self.mdio_user_int_masked_reg
    }
    #[doc = "0xf28 - MDIO User Interrupt Mask Set Register"]
    #[inline(always)]
    pub const fn mdio_user_int_mask_set_reg(&self) -> &MdioUserIntMaskSetReg {
        &self.mdio_user_int_mask_set_reg
    }
    #[doc = "0xf2c - MDIO User Interrupt Mask Clear Register"]
    #[inline(always)]
    pub const fn mdio_user_int_mask_clear_reg(&self) -> &MdioUserIntMaskClearReg {
        &self.mdio_user_int_mask_clear_reg
    }
    #[doc = "0xf30 - MDIO Manual Interface Register"]
    #[inline(always)]
    pub const fn mdio_manual_if_reg(&self) -> &MdioManualIfReg {
        &self.mdio_manual_if_reg
    }
    #[doc = "0xf34 - MDIO Poll Register"]
    #[inline(always)]
    pub const fn mdio_poll_reg(&self) -> &MdioPollReg {
        &self.mdio_poll_reg
    }
    #[doc = "0xf38 - MDIO Poll Enable Register"]
    #[inline(always)]
    pub const fn mdio_poll_en_reg(&self) -> &MdioPollEnReg {
        &self.mdio_poll_en_reg
    }
    #[doc = "0xf3c - MDIO Clause45 Register"]
    #[inline(always)]
    pub const fn mdio_claus45_reg(&self) -> &MdioClaus45Reg {
        &self.mdio_claus45_reg
    }
    #[doc = "0xf40 - MDIO Address 0 Register"]
    #[inline(always)]
    pub const fn mdio_user_addr0_reg(&self) -> &MdioUserAddr0Reg {
        &self.mdio_user_addr0_reg
    }
    #[doc = "0xf44 - MDIO Address 1 Register"]
    #[inline(always)]
    pub const fn mdio_user_addr1_reg(&self) -> &MdioUserAddr1Reg {
        &self.mdio_user_addr1_reg
    }
    #[doc = "0xf80 - MDIO User Access Register"]
    #[inline(always)]
    pub const fn user_group0_user_access_reg(&self) -> &UserGroup0UserAccessReg {
        &self.user_group0_user_access_reg
    }
    #[doc = "0xf84 - MDIO User PHY Select Register"]
    #[inline(always)]
    pub const fn user_group0_user_phy_sel_reg(&self) -> &UserGroup0UserPhySelReg {
        &self.user_group0_user_phy_sel_reg
    }
    #[doc = "0xf88 - MDIO User Access Register"]
    #[inline(always)]
    pub const fn user_group1_user_access_reg(&self) -> &UserGroup1UserAccessReg {
        &self.user_group1_user_access_reg
    }
    #[doc = "0xf8c - MDIO User PHY Select Register"]
    #[inline(always)]
    pub const fn user_group1_user_phy_sel_reg(&self) -> &UserGroup1UserPhySelReg {
        &self.user_group1_user_phy_sel_reg
    }
    #[doc = "0x1800 - Core 0 THost Threshold Pulse Interrupt Enable Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_th_thresh_pulse_en_reg(&self) -> &RegsIntSsC0ThThreshPulseEnReg {
        &self.regs_int_ss_c0_th_thresh_pulse_en_reg
    }
    #[doc = "0x1804 - Core 0 THost Pulse Interrupt Enable Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_th_pulse_en_reg(&self) -> &RegsIntSsC0ThPulseEnReg {
        &self.regs_int_ss_c0_th_pulse_en_reg
    }
    #[doc = "0x1808 - Core 0 FHost Pulse Interrupt Enable Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_fh_pulse_en_reg(&self) -> &RegsIntSsC0FhPulseEnReg {
        &self.regs_int_ss_c0_fh_pulse_en_reg
    }
    #[doc = "0x180c - Core 0 Misc Interrupt Enable Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_misc_en_reg(&self) -> &RegsIntSsC0MiscEnReg {
        &self.regs_int_ss_c0_misc_en_reg
    }
    #[doc = "0x1810 - THost Threshold Pulse Interrupt Status Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_th_thresh_pulse_status_reg(
        &self,
    ) -> &RegsIntSsC0ThThreshPulseStatusReg {
        &self.regs_int_ss_c0_th_thresh_pulse_status_reg
    }
    #[doc = "0x1814 - THost Pulse Interrupt Status Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_th_pulse_status_reg(&self) -> &RegsIntSsC0ThPulseStatusReg {
        &self.regs_int_ss_c0_th_pulse_status_reg
    }
    #[doc = "0x1818 - FHost Pulse Interrupt Status Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_fh_pulse_status_reg(&self) -> &RegsIntSsC0FhPulseStatusReg {
        &self.regs_int_ss_c0_fh_pulse_status_reg
    }
    #[doc = "0x181c - Misc Interrupt Status Register - Set bits in this register indicate that an enabled interrupt is asserted"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_misc_status_reg(&self) -> &RegsIntSsC0MiscStatusReg {
        &self.regs_int_ss_c0_misc_status_reg
    }
    #[doc = "0x1820 - Core 0 THost Interrupt Max Register Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_th_imax_reg(&self) -> &RegsIntSsC0ThImaxReg {
        &self.regs_int_ss_c0_th_imax_reg
    }
    #[doc = "0x1824 - Core 0 FHost Interrupt Max Register Register"]
    #[inline(always)]
    pub const fn regs_int_ss_c0_fh_imax_reg(&self) -> &RegsIntSsC0FhImaxReg {
        &self.regs_int_ss_c0_fh_imax_reg
    }
    #[doc = "0x20000 - CPSW ID Version"]
    #[inline(always)]
    pub const fn cpsw_nc_cpsw_id_ver_reg(&self) -> &CpswNcCpswIdVerReg {
        &self.cpsw_nc_cpsw_id_ver_reg
    }
    #[doc = "0x20004 - CPSW Switch Control"]
    #[inline(always)]
    pub const fn cpsw_nc_control_reg(&self) -> &CpswNcControlReg {
        &self.cpsw_nc_control_reg
    }
    #[doc = "0x20010 - CPSW Emulation Control"]
    #[inline(always)]
    pub const fn cpsw_nc_em_control_reg(&self) -> &CpswNcEmControlReg {
        &self.cpsw_nc_em_control_reg
    }
    #[doc = "0x20014 - CPSW Statistics Port Enable"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_port_en_reg(&self) -> &CpswNcStatPortEnReg {
        &self.cpsw_nc_stat_port_en_reg
    }
    #[doc = "0x20018 - CPSW Transmit Priority Type"]
    #[inline(always)]
    pub const fn cpsw_nc_ptype_reg(&self) -> &CpswNcPtypeReg {
        &self.cpsw_nc_ptype_reg
    }
    #[doc = "0x2001c - CPSW Software Idle"]
    #[inline(always)]
    pub const fn cpsw_nc_soft_idle_reg(&self) -> &CpswNcSoftIdleReg {
        &self.cpsw_nc_soft_idle_reg
    }
    #[doc = "0x20020 - CPSW Thru Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_thru_rate_reg(&self) -> &CpswNcThruRateReg {
        &self.cpsw_nc_thru_rate_reg
    }
    #[doc = "0x20024 - CPSW Transmit FIFO Short Gap Threshold"]
    #[inline(always)]
    pub const fn cpsw_nc_gap_thresh_reg(&self) -> &CpswNcGapThreshReg {
        &self.cpsw_nc_gap_thresh_reg
    }
    #[doc = "0x2002c - CPSW Energy Efficient Ethernet Prescale Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eee_prescale_reg(&self) -> &CpswNcEeePrescaleReg {
        &self.cpsw_nc_eee_prescale_reg
    }
    #[doc = "0x20030 - CPSW PFC Tx Global Out Flow Threshold Set"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_oflow_thresh_set_reg(&self) -> &CpswNcTxGOflowThreshSetReg {
        &self.cpsw_nc_tx_g_oflow_thresh_set_reg
    }
    #[doc = "0x20034 - CPSW PFC Tx Global Out Flow Threshold Clear"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_oflow_thresh_clr_reg(&self) -> &CpswNcTxGOflowThreshClrReg {
        &self.cpsw_nc_tx_g_oflow_thresh_clr_reg
    }
    #[doc = "0x20038 - CPSW PFC Global Tx Buffer Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_buf_thresh_set_l_reg(&self) -> &CpswNcTxGBufThreshSetLReg {
        &self.cpsw_nc_tx_g_buf_thresh_set_l_reg
    }
    #[doc = "0x2003c - CPSW PFC Global Tx Buffer Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_buf_thresh_set_h_reg(&self) -> &CpswNcTxGBufThreshSetHReg {
        &self.cpsw_nc_tx_g_buf_thresh_set_h_reg
    }
    #[doc = "0x20040 - CPSW PFC Global Tx Buffer Threshold Clear Low"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_buf_thresh_clr_l_reg(&self) -> &CpswNcTxGBufThreshClrLReg {
        &self.cpsw_nc_tx_g_buf_thresh_clr_l_reg
    }
    #[doc = "0x20044 - CPSW PFC Global Tx Buffer Threshold Clear High"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_g_buf_thresh_clr_h_reg(&self) -> &CpswNcTxGBufThreshClrHReg {
        &self.cpsw_nc_tx_g_buf_thresh_clr_h_reg
    }
    #[doc = "0x20050 - VLAN Length/type"]
    #[inline(always)]
    pub const fn cpsw_nc_vlan_ltype_reg(&self) -> &CpswNcVlanLtypeReg {
        &self.cpsw_nc_vlan_ltype_reg
    }
    #[doc = "0x20054 - Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    pub const fn cpsw_nc_est_ts_domain_reg(&self) -> &CpswNcEstTsDomainReg {
        &self.cpsw_nc_est_ts_domain_reg
    }
    #[doc = "0x20100 - Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri0_maxlen_reg(&self) -> &CpswNcTxPri0MaxlenReg {
        &self.cpsw_nc_tx_pri0_maxlen_reg
    }
    #[doc = "0x20104 - Transmit Priority 1 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri1_maxlen_reg(&self) -> &CpswNcTxPri1MaxlenReg {
        &self.cpsw_nc_tx_pri1_maxlen_reg
    }
    #[doc = "0x20108 - Transmit Priority 2 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri2_maxlen_reg(&self) -> &CpswNcTxPri2MaxlenReg {
        &self.cpsw_nc_tx_pri2_maxlen_reg
    }
    #[doc = "0x2010c - Transmit Priority 3 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri3_maxlen_reg(&self) -> &CpswNcTxPri3MaxlenReg {
        &self.cpsw_nc_tx_pri3_maxlen_reg
    }
    #[doc = "0x20110 - Transmit Priority 4 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri4_maxlen_reg(&self) -> &CpswNcTxPri4MaxlenReg {
        &self.cpsw_nc_tx_pri4_maxlen_reg
    }
    #[doc = "0x20114 - Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri5_maxlen_reg(&self) -> &CpswNcTxPri5MaxlenReg {
        &self.cpsw_nc_tx_pri5_maxlen_reg
    }
    #[doc = "0x20118 - Transmit Priority 6 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri6_maxlen_reg(&self) -> &CpswNcTxPri6MaxlenReg {
        &self.cpsw_nc_tx_pri6_maxlen_reg
    }
    #[doc = "0x2011c - Transmit Priority 7 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nc_tx_pri7_maxlen_reg(&self) -> &CpswNcTxPri7MaxlenReg {
        &self.cpsw_nc_tx_pri7_maxlen_reg
    }
    #[doc = "0x21004 - CPPI Port 0 Control"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_control_reg(&self) -> &CpswNcCppiP0ControlReg {
        &self.cpsw_nc_cppi_p0_control_reg
    }
    #[doc = "0x21008 - CPPI Port 0 Flow ID Offset"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_flow_id_offset_reg(&self) -> &CpswNcCppiP0FlowIdOffsetReg {
        &self.cpsw_nc_cppi_p0_flow_id_offset_reg
    }
    #[doc = "0x21010 - CPPI Port 0 FIFO Block Usage Count"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_blk_cnt_reg(&self) -> &CpswNcCppiP0BlkCntReg {
        &self.cpsw_nc_cppi_p0_blk_cnt_reg
    }
    #[doc = "0x21014 - CPPI Port 0 VLAN"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_port_vlan_reg(&self) -> &CpswNcCppiP0PortVlanReg {
        &self.cpsw_nc_cppi_p0_port_vlan_reg
    }
    #[doc = "0x21018 - CPPI Port 0 Tx Header Pri to Switch Pri Mapping"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_pri_map_reg(&self) -> &CpswNcCppiP0TxPriMapReg {
        &self.cpsw_nc_cppi_p0_tx_pri_map_reg
    }
    #[doc = "0x2101c - CPPI Port 0 Priority Control"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_ctl_reg(&self) -> &CpswNcCppiP0PriCtlReg {
        &self.cpsw_nc_cppi_p0_pri_ctl_reg
    }
    #[doc = "0x21020 - CPPI Port 0 RX Pkt Pri to Header Pri Map"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_pri_map_reg(&self) -> &CpswNcCppiP0RxPriMapReg {
        &self.cpsw_nc_cppi_p0_rx_pri_map_reg
    }
    #[doc = "0x21024 - CPPI Port 0 Receive Frame Max Length"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_maxlen_reg(&self) -> &CpswNcCppiP0RxMaxlenReg {
        &self.cpsw_nc_cppi_p0_rx_maxlen_reg
    }
    #[doc = "0x21028 - CPPI Port 0 Transmit Block Sub Per Priority"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_blks_pri_reg(&self) -> &CpswNcCppiP0TxBlksPriReg {
        &self.cpsw_nc_cppi_p0_tx_blks_pri_reg
    }
    #[doc = "0x21030 - Port 0 EEE Idle to LPI counter"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_idle2lpi_reg(&self) -> &CpswNcCppiP0Idle2lpiReg {
        &self.cpsw_nc_cppi_p0_idle2lpi_reg
    }
    #[doc = "0x21034 - Port 0 EEE LPI to wake counter"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_lpi2wake_reg(&self) -> &CpswNcCppiP0Lpi2wakeReg {
        &self.cpsw_nc_cppi_p0_lpi2wake_reg
    }
    #[doc = "0x21038 - Port 0 EEE status"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_eee_status_reg(&self) -> &CpswNcCppiP0EeeStatusReg {
        &self.cpsw_nc_cppi_p0_eee_status_reg
    }
    #[doc = "0x2103c - CPPI Port Receive Packets per priority"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_pkts_pri_reg(&self) -> &CpswNcCppiP0RxPktsPriReg {
        &self.cpsw_nc_cppi_p0_rx_pkts_pri_reg
    }
    #[doc = "0x2104c - Port 0 Receive Gap Register"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_gap_reg(&self) -> &CpswNcCppiP0RxGapReg {
        &self.cpsw_nc_cppi_p0_rx_gap_reg
    }
    #[doc = "0x21050 - Port 0 FIFO Status"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_fifo_status_reg(&self) -> &CpswNcCppiP0FifoStatusReg {
        &self.cpsw_nc_cppi_p0_fifo_status_reg
    }
    #[doc = "0x21080 - Port 0 FIFO Max Blocks"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_max_blks_reg(&self) -> &CpswNcCppiP0MaxBlksReg {
        &self.cpsw_nc_cppi_p0_max_blks_reg
    }
    #[doc = "0x21120 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_0(&self) -> &CpswNcCppiP0RxDscpMapReg0 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_0
    }
    #[doc = "0x21124 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_1(&self) -> &CpswNcCppiP0RxDscpMapReg1 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_1
    }
    #[doc = "0x21128 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_2(&self) -> &CpswNcCppiP0RxDscpMapReg2 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_2
    }
    #[doc = "0x2112c - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_3(&self) -> &CpswNcCppiP0RxDscpMapReg3 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_3
    }
    #[doc = "0x21130 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_4(&self) -> &CpswNcCppiP0RxDscpMapReg4 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_4
    }
    #[doc = "0x21134 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_5(&self) -> &CpswNcCppiP0RxDscpMapReg5 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_5
    }
    #[doc = "0x21138 - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_6(&self) -> &CpswNcCppiP0RxDscpMapReg6 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_6
    }
    #[doc = "0x2113c - CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_rx_dscp_map_reg_7(&self) -> &CpswNcCppiP0RxDscpMapReg7 {
        &self.cpsw_nc_cppi_p0_rx_dscp_map_reg_7
    }
    #[doc = "0x21140 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_0(&self) -> &CpswNcCppiP0PriCirReg0 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_0
    }
    #[doc = "0x21144 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_1(&self) -> &CpswNcCppiP0PriCirReg1 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_1
    }
    #[doc = "0x21148 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_2(&self) -> &CpswNcCppiP0PriCirReg2 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_2
    }
    #[doc = "0x2114c - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_3(&self) -> &CpswNcCppiP0PriCirReg3 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_3
    }
    #[doc = "0x21150 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_4(&self) -> &CpswNcCppiP0PriCirReg4 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_4
    }
    #[doc = "0x21154 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_5(&self) -> &CpswNcCppiP0PriCirReg5 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_5
    }
    #[doc = "0x21158 - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_6(&self) -> &CpswNcCppiP0PriCirReg6 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_6
    }
    #[doc = "0x2115c - CPPI Port 0 Rx Priority P Committed Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_cir_reg_7(&self) -> &CpswNcCppiP0PriCirReg7 {
        &self.cpsw_nc_cppi_p0_pri_cir_reg_7
    }
    #[doc = "0x21160 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_0(&self) -> &CpswNcCppiP0PriEirReg0 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_0
    }
    #[doc = "0x21164 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_1(&self) -> &CpswNcCppiP0PriEirReg1 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_1
    }
    #[doc = "0x21168 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_2(&self) -> &CpswNcCppiP0PriEirReg2 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_2
    }
    #[doc = "0x2116c - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_3(&self) -> &CpswNcCppiP0PriEirReg3 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_3
    }
    #[doc = "0x21170 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_4(&self) -> &CpswNcCppiP0PriEirReg4 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_4
    }
    #[doc = "0x21174 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_5(&self) -> &CpswNcCppiP0PriEirReg5 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_5
    }
    #[doc = "0x21178 - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_6(&self) -> &CpswNcCppiP0PriEirReg6 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_6
    }
    #[doc = "0x2117c - CPPI Port 0 Rx Priority P Excess Information Rate"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_pri_eir_reg_7(&self) -> &CpswNcCppiP0PriEirReg7 {
        &self.cpsw_nc_cppi_p0_pri_eir_reg_7
    }
    #[doc = "0x21180 - CPPI Port 0 Tx PFC Destination Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg(&self) -> &CpswNcCppiP0TxDThreshSetLReg {
        &self.cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg
    }
    #[doc = "0x21184 - CPPI Port 0 Tx PFC Destination Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg(&self) -> &CpswNcCppiP0TxDThreshSetHReg {
        &self.cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg
    }
    #[doc = "0x21188 - CPPI Port 0 Tx PFC Destination Threshold Clr Low"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg(&self) -> &CpswNcCppiP0TxDThreshClrLReg {
        &self.cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg
    }
    #[doc = "0x2118c - CPPI Port 0 Tx PFC Destination Threshold Clr High"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg(&self) -> &CpswNcCppiP0TxDThreshClrHReg {
        &self.cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg
    }
    #[doc = "0x21190 - CPPI Port 0 Tx PFC Global Buffer Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg(
        &self,
    ) -> &CpswNcCppiP0TxGBufThreshSetLReg {
        &self.cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg
    }
    #[doc = "0x21194 - CPPI Port 0 Tx PFC Global Buffer Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg(
        &self,
    ) -> &CpswNcCppiP0TxGBufThreshSetHReg {
        &self.cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg
    }
    #[doc = "0x21198 - CPPI Port 0 Tx PFC Global Buffer Threshold Clr Low"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg(
        &self,
    ) -> &CpswNcCppiP0TxGBufThreshClrLReg {
        &self.cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg
    }
    #[doc = "0x2119c - CPPI Port 0 Tx PFC Global Buffer Threshold Clr High"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg(
        &self,
    ) -> &CpswNcCppiP0TxGBufThreshClrHReg {
        &self.cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg
    }
    #[doc = "0x21300 - CPPI Port 0 CPPI Source ID A"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_src_id_a_reg(&self) -> &CpswNcCppiP0SrcIdAReg {
        &self.cpsw_nc_cppi_p0_src_id_a_reg
    }
    #[doc = "0x21304 - CPPI Port 0 CPPI Source ID B"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_src_id_b_reg(&self) -> &CpswNcCppiP0SrcIdBReg {
        &self.cpsw_nc_cppi_p0_src_id_b_reg
    }
    #[doc = "0x21320 - CPPI Port 0 Host Blocks Priority"]
    #[inline(always)]
    pub const fn cpsw_nc_cppi_p0_host_blks_pri_reg(&self) -> &CpswNcCppiP0HostBlksPriReg {
        &self.cpsw_nc_cppi_p0_host_blks_pri_reg
    }
    #[doc = "0x22000 - Reserved"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_reserved_reg(&self) -> &CpswNcEthMac0PnReservedReg {
        &self.cpsw_nc_eth_mac_0_pn_reserved_reg
    }
    #[doc = "0x22004 - Enet Port N Control"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_control_reg(&self) -> &CpswNcEthMac0PnControlReg {
        &self.cpsw_nc_eth_mac_0_pn_control_reg
    }
    #[doc = "0x22008 - Enet Port N FIFO Max Blocks"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_max_blks_reg(&self) -> &CpswNcEthMac0PnMaxBlksReg {
        &self.cpsw_nc_eth_mac_0_pn_max_blks_reg
    }
    #[doc = "0x22010 - Enet Port N FIFO Block Usage Count"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_blk_cnt_reg(&self) -> &CpswNcEthMac0PnBlkCntReg {
        &self.cpsw_nc_eth_mac_0_pn_blk_cnt_reg
    }
    #[doc = "0x22014 - Enet Port N VLAN"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_port_vlan_reg(&self) -> &CpswNcEthMac0PnPortVlanReg {
        &self.cpsw_nc_eth_mac_0_pn_port_vlan_reg
    }
    #[doc = "0x22018 - Enet Port N Tx Header Pri to Switch Pri Mapping"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_pri_map_reg(&self) -> &CpswNcEthMac0PnTxPriMapReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_pri_map_reg
    }
    #[doc = "0x2201c - Enet Port N Priority Control"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_ctl_reg(&self) -> &CpswNcEthMac0PnPriCtlReg {
        &self.cpsw_nc_eth_mac_0_pn_pri_ctl_reg
    }
    #[doc = "0x22020 - Enet Port N RX Pkt Pri to Header Pri Map"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_pri_map_reg(&self) -> &CpswNcEthMac0PnRxPriMapReg {
        &self.cpsw_nc_eth_mac_0_pn_rx_pri_map_reg
    }
    #[doc = "0x22024 - Enet Port N Receive Frame Max Length"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_maxlen_reg(&self) -> &CpswNcEthMac0PnRxMaxlenReg {
        &self.cpsw_nc_eth_mac_0_pn_rx_maxlen_reg
    }
    #[doc = "0x22028 - Enet Port N Transmit Block Sub Per Priority"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg(&self) -> &CpswNcEthMac0PnTxBlksPriReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg
    }
    #[doc = "0x2202c - Enet MAC Receive Flow Threshold in Receive Buffer Words"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg(&self) -> &CpswNcEthMac0PnRxFlowThreshReg {
        &self.cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg
    }
    #[doc = "0x22030 - Enet Port N EEE Idle to LPI counter"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_idle2lpi_reg(&self) -> &CpswNcEthMac0PnIdle2lpiReg {
        &self.cpsw_nc_eth_mac_0_pn_idle2lpi_reg
    }
    #[doc = "0x22034 - Enet Port N EEE LPI to wake counter"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_lpi2wake_reg(&self) -> &CpswNcEthMac0PnLpi2wakeReg {
        &self.cpsw_nc_eth_mac_0_pn_lpi2wake_reg
    }
    #[doc = "0x22038 - Enet Port N EEE status"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_eee_status_reg(&self) -> &CpswNcEthMac0PnEeeStatusReg {
        &self.cpsw_nc_eth_mac_0_pn_eee_status_reg
    }
    #[doc = "0x22050 - Enet Port N FIFO STATUS"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_fifo_status_reg(&self) -> &CpswNcEthMac0PnFifoStatusReg {
        &self.cpsw_nc_eth_mac_0_pn_fifo_status_reg
    }
    #[doc = "0x22060 - Enet Port N EST CONTROL"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_est_control_reg(&self) -> &CpswNcEthMac0PnEstControlReg {
        &self.cpsw_nc_eth_mac_0_pn_est_control_reg
    }
    #[doc = "0x22120 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0(&self) -> &CpswNcEthMac0PnRxDscpMapReg0 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0
    }
    #[doc = "0x22124 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1(&self) -> &CpswNcEthMac0PnRxDscpMapReg1 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1
    }
    #[doc = "0x22128 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2(&self) -> &CpswNcEthMac0PnRxDscpMapReg2 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2
    }
    #[doc = "0x2212c - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3(&self) -> &CpswNcEthMac0PnRxDscpMapReg3 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3
    }
    #[doc = "0x22130 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4(&self) -> &CpswNcEthMac0PnRxDscpMapReg4 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4
    }
    #[doc = "0x22134 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5(&self) -> &CpswNcEthMac0PnRxDscpMapReg5 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5
    }
    #[doc = "0x22138 - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6(&self) -> &CpswNcEthMac0PnRxDscpMapReg6 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6
    }
    #[doc = "0x2213c - Enet Port N Receive IPV4/IPV6 DSCP Map M"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7(&self) -> &CpswNcEthMac0PnRxDscpMapReg7 {
        &self.cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7
    }
    #[doc = "0x22140 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_0(&self) -> &CpswNcEthMac0PnPriCirReg0 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_0
    }
    #[doc = "0x22144 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_1(&self) -> &CpswNcEthMac0PnPriCirReg1 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_1
    }
    #[doc = "0x22148 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_2(&self) -> &CpswNcEthMac0PnPriCirReg2 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_2
    }
    #[doc = "0x2214c - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_3(&self) -> &CpswNcEthMac0PnPriCirReg3 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_3
    }
    #[doc = "0x22150 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_4(&self) -> &CpswNcEthMac0PnPriCirReg4 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_4
    }
    #[doc = "0x22154 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_5(&self) -> &CpswNcEthMac0PnPriCirReg5 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_5
    }
    #[doc = "0x22158 - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_6(&self) -> &CpswNcEthMac0PnPriCirReg6 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_6
    }
    #[doc = "0x2215c - Enet Port N Rx Priority P Committed Information Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_cir_reg_7(&self) -> &CpswNcEthMac0PnPriCirReg7 {
        &self.cpsw_nc_eth_mac_0_pn_pri_cir_reg_7
    }
    #[doc = "0x22160 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_0(&self) -> &CpswNcEthMac0PnPriEirReg0 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_0
    }
    #[doc = "0x22164 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_1(&self) -> &CpswNcEthMac0PnPriEirReg1 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_1
    }
    #[doc = "0x22168 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_2(&self) -> &CpswNcEthMac0PnPriEirReg2 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_2
    }
    #[doc = "0x2216c - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_3(&self) -> &CpswNcEthMac0PnPriEirReg3 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_3
    }
    #[doc = "0x22170 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_4(&self) -> &CpswNcEthMac0PnPriEirReg4 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_4
    }
    #[doc = "0x22174 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_5(&self) -> &CpswNcEthMac0PnPriEirReg5 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_5
    }
    #[doc = "0x22178 - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_6(&self) -> &CpswNcEthMac0PnPriEirReg6 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_6
    }
    #[doc = "0x2217c - Enet Port N Rx Priority P Excess Informatoin Rate Value"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_pri_eir_reg_7(&self) -> &CpswNcEthMac0PnPriEirReg7 {
        &self.cpsw_nc_eth_mac_0_pn_pri_eir_reg_7
    }
    #[doc = "0x22180 - Enet Port N Tx PFC Destination Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDThreshSetLReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg
    }
    #[doc = "0x22184 - Enet Port N Tx PFC Destination Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDThreshSetHReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg
    }
    #[doc = "0x22188 - Enet Port N Tx PFC Destination Threshold Clr Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDThreshClrLReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg
    }
    #[doc = "0x2218c - Enet Port N Tx PFC Destination Threshold Clr High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDThreshClrHReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg
    }
    #[doc = "0x22190 - Enet Port N Tx PFC Global Buffer Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxGBufThreshSetLReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg
    }
    #[doc = "0x22194 - Enet Port N Tx PFC Global Buffer Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxGBufThreshSetHReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg
    }
    #[doc = "0x22198 - Enet Port N Tx PFC Global Buffer Threshold Clr Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxGBufThreshClrLReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg
    }
    #[doc = "0x2219c - Enet Port N Tx PFC Global Buffer Threshold Clr High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxGBufThreshClrHReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg
    }
    #[doc = "0x22300 - Enet Port N Tx Destination Out Flow Add Values Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDOflowAddvalLReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg
    }
    #[doc = "0x22304 - Enet Port N Tx Destination Out Flow Add Values High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg(
        &self,
    ) -> &CpswNcEthMac0PnTxDOflowAddvalHReg {
        &self.cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg
    }
    #[doc = "0x22308 - Enet Port N Tx Pause Frame Source Address Low"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_sa_l_reg(&self) -> &CpswNcEthMac0PnSaLReg {
        &self.cpsw_nc_eth_mac_0_pn_sa_l_reg
    }
    #[doc = "0x2230c - Enet Port N Tx Pause Frame Source Address High"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_sa_h_reg(&self) -> &CpswNcEthMac0PnSaHReg {
        &self.cpsw_nc_eth_mac_0_pn_sa_h_reg
    }
    #[doc = "0x22310 - Enet Port N Time Sync Control"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_ts_ctl_reg(&self) -> &CpswNcEthMac0PnTsCtlReg {
        &self.cpsw_nc_eth_mac_0_pn_ts_ctl_reg
    }
    #[doc = "0x22314 - Enet Port N Time Sync LTYPE (and SEQ_ID_OFFSET)"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg(&self) -> &CpswNcEthMac0PnTsSeqLtypeReg {
        &self.cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg
    }
    #[doc = "0x22318 - Enet Port N Time Sync VLAN2 and VLAN2"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg(&self) -> &CpswNcEthMac0PnTsVlanLtypeReg {
        &self.cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg
    }
    #[doc = "0x2231c - Enet Port N Time Sync Control and LTYPE 2"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg(&self) -> &CpswNcEthMac0PnTsCtlLtype2Reg {
        &self.cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg
    }
    #[doc = "0x22320 - Enet Port N Time Sync Control 2"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_ts_ctl2_reg(&self) -> &CpswNcEthMac0PnTsCtl2Reg {
        &self.cpsw_nc_eth_mac_0_pn_ts_ctl2_reg
    }
    #[doc = "0x22330 - Enet Port N Mac Control"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_control_reg(&self) -> &CpswNcEthMac0PnMacControlReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_control_reg
    }
    #[doc = "0x22334 - Enet Port N Mac Status"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_status_reg(&self) -> &CpswNcEthMac0PnMacStatusReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_status_reg
    }
    #[doc = "0x22338 - Enet Port N Mac Soft Reset"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg(&self) -> &CpswNcEthMac0PnMacSoftResetReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg
    }
    #[doc = "0x2233c - Enet Port N Mac Backoff Test"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_bofftest_reg(&self) -> &CpswNcEthMac0PnMacBofftestReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_bofftest_reg
    }
    #[doc = "0x22340 - Enet Port N 802.3 Receive Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg(
        &self,
    ) -> &CpswNcEthMac0PnMacRxPausetimerReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg
    }
    #[doc = "0x22350 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg0 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0
    }
    #[doc = "0x22354 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg1 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1
    }
    #[doc = "0x22358 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg2 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2
    }
    #[doc = "0x2235c - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg3 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3
    }
    #[doc = "0x22360 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg4 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4
    }
    #[doc = "0x22364 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg5 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5
    }
    #[doc = "0x22368 - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg6 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6
    }
    #[doc = "0x2236c - Enet Port N PFC Priority P Rx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7(
        &self,
    ) -> &CpswNcEthMac0PnMacRxnPausetimerReg7 {
        &self.cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7
    }
    #[doc = "0x22370 - Enet Port N 802.3 Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg(
        &self,
    ) -> &CpswNcEthMac0PnMacTxPausetimerReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg
    }
    #[doc = "0x22380 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg0 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0
    }
    #[doc = "0x22384 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg1 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1
    }
    #[doc = "0x22388 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg2 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2
    }
    #[doc = "0x2238c - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg3 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3
    }
    #[doc = "0x22390 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg4 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4
    }
    #[doc = "0x22394 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg5 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5
    }
    #[doc = "0x22398 - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg6 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6
    }
    #[doc = "0x2239c - Enet Port N PFC Priority P Tx Pause Timer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7(
        &self,
    ) -> &CpswNcEthMac0PnMacTxnPausetimerReg7 {
        &self.cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7
    }
    #[doc = "0x223a0 - Enet Port N Emulation Control"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg(&self) -> &CpswNcEthMac0PnMacEmcontrolReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg
    }
    #[doc = "0x223a4 - Enet Port N Tx Inter Packet Gap"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg(&self) -> &CpswNcEthMac0PnMacTxGapReg {
        &self.cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg
    }
    #[doc = "0x223a8 - Enet Port N Port Configuration"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_mac_port_config(&self) -> &CpswNcEthMac0PnMacPortConfig {
        &self.cpsw_nc_eth_mac_0_pn_mac_port_config
    }
    #[doc = "0x223ac - Enet Port N Tx Egress InterVLAN Operation Pointer"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg(
        &self,
    ) -> &CpswNcEthMac0PnIntervlanOpxPointerReg {
        &self.cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg
    }
    #[doc = "0x223b0 - Enet Port N Tx Egress InterVLAN A"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg(
        &self,
    ) -> &CpswNcEthMac0PnIntervlanOpxAReg {
        &self.cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg
    }
    #[doc = "0x223b4 - Enet Port N Tx Egress InterVLAN B"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg(
        &self,
    ) -> &CpswNcEthMac0PnIntervlanOpxBReg {
        &self.cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg
    }
    #[doc = "0x223b8 - Enet Port N Tx Egress InterVLAN C"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg(
        &self,
    ) -> &CpswNcEthMac0PnIntervlanOpxCReg {
        &self.cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg
    }
    #[doc = "0x223bc - Enet Port N Tx Egress InterVLAN D"]
    #[inline(always)]
    pub const fn cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg(
        &self,
    ) -> &CpswNcEthMac0PnIntervlanOpxDReg {
        &self.cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg
    }
    #[doc = "0x32000 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_0(&self) -> &CpswNcEstFetchLoc0 {
        &self.cpsw_nc_est_fetch_loc_0
    }
    #[doc = "0x32004 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_1(&self) -> &CpswNcEstFetchLoc1 {
        &self.cpsw_nc_est_fetch_loc_1
    }
    #[doc = "0x32008 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_2(&self) -> &CpswNcEstFetchLoc2 {
        &self.cpsw_nc_est_fetch_loc_2
    }
    #[doc = "0x3200c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_3(&self) -> &CpswNcEstFetchLoc3 {
        &self.cpsw_nc_est_fetch_loc_3
    }
    #[doc = "0x32010 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_4(&self) -> &CpswNcEstFetchLoc4 {
        &self.cpsw_nc_est_fetch_loc_4
    }
    #[doc = "0x32014 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_5(&self) -> &CpswNcEstFetchLoc5 {
        &self.cpsw_nc_est_fetch_loc_5
    }
    #[doc = "0x32018 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_6(&self) -> &CpswNcEstFetchLoc6 {
        &self.cpsw_nc_est_fetch_loc_6
    }
    #[doc = "0x3201c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_7(&self) -> &CpswNcEstFetchLoc7 {
        &self.cpsw_nc_est_fetch_loc_7
    }
    #[doc = "0x32020 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_8(&self) -> &CpswNcEstFetchLoc8 {
        &self.cpsw_nc_est_fetch_loc_8
    }
    #[doc = "0x32024 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_9(&self) -> &CpswNcEstFetchLoc9 {
        &self.cpsw_nc_est_fetch_loc_9
    }
    #[doc = "0x32028 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_10(&self) -> &CpswNcEstFetchLoc10 {
        &self.cpsw_nc_est_fetch_loc_10
    }
    #[doc = "0x3202c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_11(&self) -> &CpswNcEstFetchLoc11 {
        &self.cpsw_nc_est_fetch_loc_11
    }
    #[doc = "0x32030 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_12(&self) -> &CpswNcEstFetchLoc12 {
        &self.cpsw_nc_est_fetch_loc_12
    }
    #[doc = "0x32034 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_13(&self) -> &CpswNcEstFetchLoc13 {
        &self.cpsw_nc_est_fetch_loc_13
    }
    #[doc = "0x32038 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_14(&self) -> &CpswNcEstFetchLoc14 {
        &self.cpsw_nc_est_fetch_loc_14
    }
    #[doc = "0x3203c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_15(&self) -> &CpswNcEstFetchLoc15 {
        &self.cpsw_nc_est_fetch_loc_15
    }
    #[doc = "0x32040 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_16(&self) -> &CpswNcEstFetchLoc16 {
        &self.cpsw_nc_est_fetch_loc_16
    }
    #[doc = "0x32044 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_17(&self) -> &CpswNcEstFetchLoc17 {
        &self.cpsw_nc_est_fetch_loc_17
    }
    #[doc = "0x32048 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_18(&self) -> &CpswNcEstFetchLoc18 {
        &self.cpsw_nc_est_fetch_loc_18
    }
    #[doc = "0x3204c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_19(&self) -> &CpswNcEstFetchLoc19 {
        &self.cpsw_nc_est_fetch_loc_19
    }
    #[doc = "0x32050 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_20(&self) -> &CpswNcEstFetchLoc20 {
        &self.cpsw_nc_est_fetch_loc_20
    }
    #[doc = "0x32054 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_21(&self) -> &CpswNcEstFetchLoc21 {
        &self.cpsw_nc_est_fetch_loc_21
    }
    #[doc = "0x32058 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_22(&self) -> &CpswNcEstFetchLoc22 {
        &self.cpsw_nc_est_fetch_loc_22
    }
    #[doc = "0x3205c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_23(&self) -> &CpswNcEstFetchLoc23 {
        &self.cpsw_nc_est_fetch_loc_23
    }
    #[doc = "0x32060 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_24(&self) -> &CpswNcEstFetchLoc24 {
        &self.cpsw_nc_est_fetch_loc_24
    }
    #[doc = "0x32064 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_25(&self) -> &CpswNcEstFetchLoc25 {
        &self.cpsw_nc_est_fetch_loc_25
    }
    #[doc = "0x32068 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_26(&self) -> &CpswNcEstFetchLoc26 {
        &self.cpsw_nc_est_fetch_loc_26
    }
    #[doc = "0x3206c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_27(&self) -> &CpswNcEstFetchLoc27 {
        &self.cpsw_nc_est_fetch_loc_27
    }
    #[doc = "0x32070 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_28(&self) -> &CpswNcEstFetchLoc28 {
        &self.cpsw_nc_est_fetch_loc_28
    }
    #[doc = "0x32074 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_29(&self) -> &CpswNcEstFetchLoc29 {
        &self.cpsw_nc_est_fetch_loc_29
    }
    #[doc = "0x32078 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_30(&self) -> &CpswNcEstFetchLoc30 {
        &self.cpsw_nc_est_fetch_loc_30
    }
    #[doc = "0x3207c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_31(&self) -> &CpswNcEstFetchLoc31 {
        &self.cpsw_nc_est_fetch_loc_31
    }
    #[doc = "0x32080 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_32(&self) -> &CpswNcEstFetchLoc32 {
        &self.cpsw_nc_est_fetch_loc_32
    }
    #[doc = "0x32084 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_33(&self) -> &CpswNcEstFetchLoc33 {
        &self.cpsw_nc_est_fetch_loc_33
    }
    #[doc = "0x32088 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_34(&self) -> &CpswNcEstFetchLoc34 {
        &self.cpsw_nc_est_fetch_loc_34
    }
    #[doc = "0x3208c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_35(&self) -> &CpswNcEstFetchLoc35 {
        &self.cpsw_nc_est_fetch_loc_35
    }
    #[doc = "0x32090 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_36(&self) -> &CpswNcEstFetchLoc36 {
        &self.cpsw_nc_est_fetch_loc_36
    }
    #[doc = "0x32094 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_37(&self) -> &CpswNcEstFetchLoc37 {
        &self.cpsw_nc_est_fetch_loc_37
    }
    #[doc = "0x32098 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_38(&self) -> &CpswNcEstFetchLoc38 {
        &self.cpsw_nc_est_fetch_loc_38
    }
    #[doc = "0x3209c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_39(&self) -> &CpswNcEstFetchLoc39 {
        &self.cpsw_nc_est_fetch_loc_39
    }
    #[doc = "0x320a0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_40(&self) -> &CpswNcEstFetchLoc40 {
        &self.cpsw_nc_est_fetch_loc_40
    }
    #[doc = "0x320a4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_41(&self) -> &CpswNcEstFetchLoc41 {
        &self.cpsw_nc_est_fetch_loc_41
    }
    #[doc = "0x320a8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_42(&self) -> &CpswNcEstFetchLoc42 {
        &self.cpsw_nc_est_fetch_loc_42
    }
    #[doc = "0x320ac - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_43(&self) -> &CpswNcEstFetchLoc43 {
        &self.cpsw_nc_est_fetch_loc_43
    }
    #[doc = "0x320b0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_44(&self) -> &CpswNcEstFetchLoc44 {
        &self.cpsw_nc_est_fetch_loc_44
    }
    #[doc = "0x320b4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_45(&self) -> &CpswNcEstFetchLoc45 {
        &self.cpsw_nc_est_fetch_loc_45
    }
    #[doc = "0x320b8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_46(&self) -> &CpswNcEstFetchLoc46 {
        &self.cpsw_nc_est_fetch_loc_46
    }
    #[doc = "0x320bc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_47(&self) -> &CpswNcEstFetchLoc47 {
        &self.cpsw_nc_est_fetch_loc_47
    }
    #[doc = "0x320c0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_48(&self) -> &CpswNcEstFetchLoc48 {
        &self.cpsw_nc_est_fetch_loc_48
    }
    #[doc = "0x320c4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_49(&self) -> &CpswNcEstFetchLoc49 {
        &self.cpsw_nc_est_fetch_loc_49
    }
    #[doc = "0x320c8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_50(&self) -> &CpswNcEstFetchLoc50 {
        &self.cpsw_nc_est_fetch_loc_50
    }
    #[doc = "0x320cc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_51(&self) -> &CpswNcEstFetchLoc51 {
        &self.cpsw_nc_est_fetch_loc_51
    }
    #[doc = "0x320d0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_52(&self) -> &CpswNcEstFetchLoc52 {
        &self.cpsw_nc_est_fetch_loc_52
    }
    #[doc = "0x320d4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_53(&self) -> &CpswNcEstFetchLoc53 {
        &self.cpsw_nc_est_fetch_loc_53
    }
    #[doc = "0x320d8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_54(&self) -> &CpswNcEstFetchLoc54 {
        &self.cpsw_nc_est_fetch_loc_54
    }
    #[doc = "0x320dc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_55(&self) -> &CpswNcEstFetchLoc55 {
        &self.cpsw_nc_est_fetch_loc_55
    }
    #[doc = "0x320e0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_56(&self) -> &CpswNcEstFetchLoc56 {
        &self.cpsw_nc_est_fetch_loc_56
    }
    #[doc = "0x320e4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_57(&self) -> &CpswNcEstFetchLoc57 {
        &self.cpsw_nc_est_fetch_loc_57
    }
    #[doc = "0x320e8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_58(&self) -> &CpswNcEstFetchLoc58 {
        &self.cpsw_nc_est_fetch_loc_58
    }
    #[doc = "0x320ec - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_59(&self) -> &CpswNcEstFetchLoc59 {
        &self.cpsw_nc_est_fetch_loc_59
    }
    #[doc = "0x320f0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_60(&self) -> &CpswNcEstFetchLoc60 {
        &self.cpsw_nc_est_fetch_loc_60
    }
    #[doc = "0x320f4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_61(&self) -> &CpswNcEstFetchLoc61 {
        &self.cpsw_nc_est_fetch_loc_61
    }
    #[doc = "0x320f8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_62(&self) -> &CpswNcEstFetchLoc62 {
        &self.cpsw_nc_est_fetch_loc_62
    }
    #[doc = "0x320fc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_63(&self) -> &CpswNcEstFetchLoc63 {
        &self.cpsw_nc_est_fetch_loc_63
    }
    #[doc = "0x32100 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_64(&self) -> &CpswNcEstFetchLoc64 {
        &self.cpsw_nc_est_fetch_loc_64
    }
    #[doc = "0x32104 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_65(&self) -> &CpswNcEstFetchLoc65 {
        &self.cpsw_nc_est_fetch_loc_65
    }
    #[doc = "0x32108 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_66(&self) -> &CpswNcEstFetchLoc66 {
        &self.cpsw_nc_est_fetch_loc_66
    }
    #[doc = "0x3210c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_67(&self) -> &CpswNcEstFetchLoc67 {
        &self.cpsw_nc_est_fetch_loc_67
    }
    #[doc = "0x32110 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_68(&self) -> &CpswNcEstFetchLoc68 {
        &self.cpsw_nc_est_fetch_loc_68
    }
    #[doc = "0x32114 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_69(&self) -> &CpswNcEstFetchLoc69 {
        &self.cpsw_nc_est_fetch_loc_69
    }
    #[doc = "0x32118 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_70(&self) -> &CpswNcEstFetchLoc70 {
        &self.cpsw_nc_est_fetch_loc_70
    }
    #[doc = "0x3211c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_71(&self) -> &CpswNcEstFetchLoc71 {
        &self.cpsw_nc_est_fetch_loc_71
    }
    #[doc = "0x32120 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_72(&self) -> &CpswNcEstFetchLoc72 {
        &self.cpsw_nc_est_fetch_loc_72
    }
    #[doc = "0x32124 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_73(&self) -> &CpswNcEstFetchLoc73 {
        &self.cpsw_nc_est_fetch_loc_73
    }
    #[doc = "0x32128 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_74(&self) -> &CpswNcEstFetchLoc74 {
        &self.cpsw_nc_est_fetch_loc_74
    }
    #[doc = "0x3212c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_75(&self) -> &CpswNcEstFetchLoc75 {
        &self.cpsw_nc_est_fetch_loc_75
    }
    #[doc = "0x32130 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_76(&self) -> &CpswNcEstFetchLoc76 {
        &self.cpsw_nc_est_fetch_loc_76
    }
    #[doc = "0x32134 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_77(&self) -> &CpswNcEstFetchLoc77 {
        &self.cpsw_nc_est_fetch_loc_77
    }
    #[doc = "0x32138 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_78(&self) -> &CpswNcEstFetchLoc78 {
        &self.cpsw_nc_est_fetch_loc_78
    }
    #[doc = "0x3213c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_79(&self) -> &CpswNcEstFetchLoc79 {
        &self.cpsw_nc_est_fetch_loc_79
    }
    #[doc = "0x32140 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_80(&self) -> &CpswNcEstFetchLoc80 {
        &self.cpsw_nc_est_fetch_loc_80
    }
    #[doc = "0x32144 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_81(&self) -> &CpswNcEstFetchLoc81 {
        &self.cpsw_nc_est_fetch_loc_81
    }
    #[doc = "0x32148 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_82(&self) -> &CpswNcEstFetchLoc82 {
        &self.cpsw_nc_est_fetch_loc_82
    }
    #[doc = "0x3214c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_83(&self) -> &CpswNcEstFetchLoc83 {
        &self.cpsw_nc_est_fetch_loc_83
    }
    #[doc = "0x32150 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_84(&self) -> &CpswNcEstFetchLoc84 {
        &self.cpsw_nc_est_fetch_loc_84
    }
    #[doc = "0x32154 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_85(&self) -> &CpswNcEstFetchLoc85 {
        &self.cpsw_nc_est_fetch_loc_85
    }
    #[doc = "0x32158 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_86(&self) -> &CpswNcEstFetchLoc86 {
        &self.cpsw_nc_est_fetch_loc_86
    }
    #[doc = "0x3215c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_87(&self) -> &CpswNcEstFetchLoc87 {
        &self.cpsw_nc_est_fetch_loc_87
    }
    #[doc = "0x32160 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_88(&self) -> &CpswNcEstFetchLoc88 {
        &self.cpsw_nc_est_fetch_loc_88
    }
    #[doc = "0x32164 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_89(&self) -> &CpswNcEstFetchLoc89 {
        &self.cpsw_nc_est_fetch_loc_89
    }
    #[doc = "0x32168 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_90(&self) -> &CpswNcEstFetchLoc90 {
        &self.cpsw_nc_est_fetch_loc_90
    }
    #[doc = "0x3216c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_91(&self) -> &CpswNcEstFetchLoc91 {
        &self.cpsw_nc_est_fetch_loc_91
    }
    #[doc = "0x32170 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_92(&self) -> &CpswNcEstFetchLoc92 {
        &self.cpsw_nc_est_fetch_loc_92
    }
    #[doc = "0x32174 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_93(&self) -> &CpswNcEstFetchLoc93 {
        &self.cpsw_nc_est_fetch_loc_93
    }
    #[doc = "0x32178 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_94(&self) -> &CpswNcEstFetchLoc94 {
        &self.cpsw_nc_est_fetch_loc_94
    }
    #[doc = "0x3217c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_95(&self) -> &CpswNcEstFetchLoc95 {
        &self.cpsw_nc_est_fetch_loc_95
    }
    #[doc = "0x32180 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_96(&self) -> &CpswNcEstFetchLoc96 {
        &self.cpsw_nc_est_fetch_loc_96
    }
    #[doc = "0x32184 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_97(&self) -> &CpswNcEstFetchLoc97 {
        &self.cpsw_nc_est_fetch_loc_97
    }
    #[doc = "0x32188 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_98(&self) -> &CpswNcEstFetchLoc98 {
        &self.cpsw_nc_est_fetch_loc_98
    }
    #[doc = "0x3218c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_99(&self) -> &CpswNcEstFetchLoc99 {
        &self.cpsw_nc_est_fetch_loc_99
    }
    #[doc = "0x32190 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_100(&self) -> &CpswNcEstFetchLoc100 {
        &self.cpsw_nc_est_fetch_loc_100
    }
    #[doc = "0x32194 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_101(&self) -> &CpswNcEstFetchLoc101 {
        &self.cpsw_nc_est_fetch_loc_101
    }
    #[doc = "0x32198 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_102(&self) -> &CpswNcEstFetchLoc102 {
        &self.cpsw_nc_est_fetch_loc_102
    }
    #[doc = "0x3219c - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_103(&self) -> &CpswNcEstFetchLoc103 {
        &self.cpsw_nc_est_fetch_loc_103
    }
    #[doc = "0x321a0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_104(&self) -> &CpswNcEstFetchLoc104 {
        &self.cpsw_nc_est_fetch_loc_104
    }
    #[doc = "0x321a4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_105(&self) -> &CpswNcEstFetchLoc105 {
        &self.cpsw_nc_est_fetch_loc_105
    }
    #[doc = "0x321a8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_106(&self) -> &CpswNcEstFetchLoc106 {
        &self.cpsw_nc_est_fetch_loc_106
    }
    #[doc = "0x321ac - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_107(&self) -> &CpswNcEstFetchLoc107 {
        &self.cpsw_nc_est_fetch_loc_107
    }
    #[doc = "0x321b0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_108(&self) -> &CpswNcEstFetchLoc108 {
        &self.cpsw_nc_est_fetch_loc_108
    }
    #[doc = "0x321b4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_109(&self) -> &CpswNcEstFetchLoc109 {
        &self.cpsw_nc_est_fetch_loc_109
    }
    #[doc = "0x321b8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_110(&self) -> &CpswNcEstFetchLoc110 {
        &self.cpsw_nc_est_fetch_loc_110
    }
    #[doc = "0x321bc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_111(&self) -> &CpswNcEstFetchLoc111 {
        &self.cpsw_nc_est_fetch_loc_111
    }
    #[doc = "0x321c0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_112(&self) -> &CpswNcEstFetchLoc112 {
        &self.cpsw_nc_est_fetch_loc_112
    }
    #[doc = "0x321c4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_113(&self) -> &CpswNcEstFetchLoc113 {
        &self.cpsw_nc_est_fetch_loc_113
    }
    #[doc = "0x321c8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_114(&self) -> &CpswNcEstFetchLoc114 {
        &self.cpsw_nc_est_fetch_loc_114
    }
    #[doc = "0x321cc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_115(&self) -> &CpswNcEstFetchLoc115 {
        &self.cpsw_nc_est_fetch_loc_115
    }
    #[doc = "0x321d0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_116(&self) -> &CpswNcEstFetchLoc116 {
        &self.cpsw_nc_est_fetch_loc_116
    }
    #[doc = "0x321d4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_117(&self) -> &CpswNcEstFetchLoc117 {
        &self.cpsw_nc_est_fetch_loc_117
    }
    #[doc = "0x321d8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_118(&self) -> &CpswNcEstFetchLoc118 {
        &self.cpsw_nc_est_fetch_loc_118
    }
    #[doc = "0x321dc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_119(&self) -> &CpswNcEstFetchLoc119 {
        &self.cpsw_nc_est_fetch_loc_119
    }
    #[doc = "0x321e0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_120(&self) -> &CpswNcEstFetchLoc120 {
        &self.cpsw_nc_est_fetch_loc_120
    }
    #[doc = "0x321e4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_121(&self) -> &CpswNcEstFetchLoc121 {
        &self.cpsw_nc_est_fetch_loc_121
    }
    #[doc = "0x321e8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_122(&self) -> &CpswNcEstFetchLoc122 {
        &self.cpsw_nc_est_fetch_loc_122
    }
    #[doc = "0x321ec - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_123(&self) -> &CpswNcEstFetchLoc123 {
        &self.cpsw_nc_est_fetch_loc_123
    }
    #[doc = "0x321f0 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_124(&self) -> &CpswNcEstFetchLoc124 {
        &self.cpsw_nc_est_fetch_loc_124
    }
    #[doc = "0x321f4 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_125(&self) -> &CpswNcEstFetchLoc125 {
        &self.cpsw_nc_est_fetch_loc_125
    }
    #[doc = "0x321f8 - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_126(&self) -> &CpswNcEstFetchLoc126 {
        &self.cpsw_nc_est_fetch_loc_126
    }
    #[doc = "0x321fc - The Revision Register contains the ID and revision information."]
    #[inline(always)]
    pub const fn cpsw_nc_est_fetch_loc_127(&self) -> &CpswNcEstFetchLoc127 {
        &self.cpsw_nc_est_fetch_loc_127
    }
    #[doc = "0x34000 - CPDMA FHost IDVER"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_fh_idver_reg(&self) -> &CpswCpdmaRegsCpdmaFhIdverReg {
        &self.cpsw_cpdma_regs_cpdma_fh_idver_reg
    }
    #[doc = "0x34004 - CPDMA FHost Control Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_fh_control_reg(&self) -> &CpswCpdmaRegsCpdmaFhControlReg {
        &self.cpsw_cpdma_regs_cpdma_fh_control_reg
    }
    #[doc = "0x34008 - CPDMA FHost Teardown Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_fh_teardown_reg(&self) -> &CpswCpdmaRegsCpdmaFhTeardownReg {
        &self.cpsw_cpdma_regs_cpdma_fh_teardown_reg
    }
    #[doc = "0x3400c - CPDMA FHost Interrupt on EOQ only Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_fh_eoq_int(&self) -> &CpswCpdmaRegsCpdmaFhEoqInt {
        &self.cpsw_cpdma_regs_cpdma_fh_eoq_int
    }
    #[doc = "0x34010 - CPDMA THost IDVER"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_th_idver_reg(&self) -> &CpswCpdmaRegsCpdmaThIdverReg {
        &self.cpsw_cpdma_regs_cpdma_th_idver_reg
    }
    #[doc = "0x34014 - CPDMA THost Control Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_th_control_reg(&self) -> &CpswCpdmaRegsCpdmaThControlReg {
        &self.cpsw_cpdma_regs_cpdma_th_control_reg
    }
    #[doc = "0x34018 - CPDMA THost Teardown Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_th_teardown_reg(&self) -> &CpswCpdmaRegsCpdmaThTeardownReg {
        &self.cpsw_cpdma_regs_cpdma_th_teardown_reg
    }
    #[doc = "0x3401c - CPDMA Soft Reset Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_soft_reset_reg(&self) -> &CpswCpdmaRegsCpdmaSoftResetReg {
        &self.cpsw_cpdma_regs_cpdma_soft_reset_reg
    }
    #[doc = "0x34020 - CPDMA Control Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_control_reg(&self) -> &CpswCpdmaRegsCpdmaControlReg {
        &self.cpsw_cpdma_regs_cpdma_control_reg
    }
    #[doc = "0x34024 - CPDMA Status Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_status_reg(&self) -> &CpswCpdmaRegsCpdmaStatusReg {
        &self.cpsw_cpdma_regs_cpdma_status_reg
    }
    #[doc = "0x34028 - CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_th_buffer_offset_reg(
        &self,
    ) -> &CpswCpdmaRegsCpdmaThBufferOffsetReg {
        &self.cpsw_cpdma_regs_cpdma_th_buffer_offset_reg
    }
    #[doc = "0x3402c - CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_regs_cpdma_emulation_control_reg(
        &self,
    ) -> &CpswCpdmaRegsCpdmaEmulationControlReg {
        &self.cpsw_cpdma_regs_cpdma_emulation_control_reg
    }
    #[doc = "0x34080 - CPDMA FHost Interrupt Status RAW"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_fh_intstat_raw_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaFhIntstatRawReg {
        &self.cpsw_cpdma_int_cpdma_fh_intstat_raw_reg
    }
    #[doc = "0x34084 - CPDMA FHost Interrupt Status MASKED"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_fh_intstat_masked_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaFhIntstatMaskedReg {
        &self.cpsw_cpdma_int_cpdma_fh_intstat_masked_reg
    }
    #[doc = "0x34088 - CPDMA FHost Interrupt Masked SET"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_fh_intmask_set_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaFhIntmaskSetReg {
        &self.cpsw_cpdma_int_cpdma_fh_intmask_set_reg
    }
    #[doc = "0x3408c - CPDMA FHost Interrupt Masked CLR"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_fh_intmask_clear_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaFhIntmaskClearReg {
        &self.cpsw_cpdma_int_cpdma_fh_intmask_clear_reg
    }
    #[doc = "0x34090 - CPDMA DMA IN Vector"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_in_vector_reg(&self) -> &CpswCpdmaIntCpdmaInVectorReg {
        &self.cpsw_cpdma_int_cpdma_in_vector_reg
    }
    #[doc = "0x34094 - CPDMA DMA EOI Vector"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_eoi_vector_reg(&self) -> &CpswCpdmaIntCpdmaEoiVectorReg {
        &self.cpsw_cpdma_int_cpdma_eoi_vector_reg
    }
    #[doc = "0x340a0 - CPDMA Receive Interrupt Status RAW"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th_intstat_raw_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaThIntstatRawReg {
        &self.cpsw_cpdma_int_cpdma_th_intstat_raw_reg
    }
    #[doc = "0x340a4 - CPDMA Receive Interrupt Status MASKED"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th_intstat_masked_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaThIntstatMaskedReg {
        &self.cpsw_cpdma_int_cpdma_th_intstat_masked_reg
    }
    #[doc = "0x340a8 - CPDMA THost Interrupt Masked SET"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th_intmask_set_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaThIntmaskSetReg {
        &self.cpsw_cpdma_int_cpdma_th_intmask_set_reg
    }
    #[doc = "0x340ac - CPDMA THost Interrupt Masked CLR"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th_intmask_clear_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaThIntmaskClearReg {
        &self.cpsw_cpdma_int_cpdma_th_intmask_clear_reg
    }
    #[doc = "0x340b0 - CPDMA DMA Interrupt Status RAW"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_intstat_raw_reg(&self) -> &CpswCpdmaIntCpdmaIntstatRawReg {
        &self.cpsw_cpdma_int_cpdma_intstat_raw_reg
    }
    #[doc = "0x340b4 - CPDMA DMA Interrupt Status MASKED"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_intstat_masked_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaIntstatMaskedReg {
        &self.cpsw_cpdma_int_cpdma_intstat_masked_reg
    }
    #[doc = "0x340b8 - CPDMA DMA Interrupt Status SET"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_intmask_set_reg(&self) -> &CpswCpdmaIntCpdmaIntmaskSetReg {
        &self.cpsw_cpdma_int_cpdma_intmask_set_reg
    }
    #[doc = "0x340bc - CPDMA DMA Interrupt Status CLR"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_intmask_clear_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaIntmaskClearReg {
        &self.cpsw_cpdma_int_cpdma_intmask_clear_reg
    }
    #[doc = "0x340c0 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th0_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh0PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th0_pendthresh_reg
    }
    #[doc = "0x340c4 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th1_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh1PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th1_pendthresh_reg
    }
    #[doc = "0x340c8 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th2_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh2PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th2_pendthresh_reg
    }
    #[doc = "0x340cc - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th3_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh3PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th3_pendthresh_reg
    }
    #[doc = "0x340d0 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th4_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh4PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th4_pendthresh_reg
    }
    #[doc = "0x340d4 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th5_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh5PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th5_pendthresh_reg
    }
    #[doc = "0x340d8 - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th6_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh6PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th6_pendthresh_reg
    }
    #[doc = "0x340dc - CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th7_pendthresh_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh7PendthreshReg {
        &self.cpsw_cpdma_int_cpdma_th7_pendthresh_reg
    }
    #[doc = "0x340e0 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th0_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh0FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th0_freebuffer_reg
    }
    #[doc = "0x340e4 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th1_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh1FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th1_freebuffer_reg
    }
    #[doc = "0x340e8 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th2_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh2FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th2_freebuffer_reg
    }
    #[doc = "0x340ec - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th3_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh3FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th3_freebuffer_reg
    }
    #[doc = "0x340f0 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th4_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh4FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th4_freebuffer_reg
    }
    #[doc = "0x340f4 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th5_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh5FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th5_freebuffer_reg
    }
    #[doc = "0x340f8 - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th6_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh6FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th6_freebuffer_reg
    }
    #[doc = "0x340fc - CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub const fn cpsw_cpdma_int_cpdma_th7_freebuffer_reg(
        &self,
    ) -> &CpswCpdmaIntCpdmaTh7FreebufferReg {
        &self.cpsw_cpdma_int_cpdma_th7_freebuffer_reg
    }
    #[doc = "0x34200 - CPDMA FHost Channel 0 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh0_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh0HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh0_hdp_reg
    }
    #[doc = "0x34204 - CPDMA FHost Channel 1 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh1_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh1HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh1_hdp_reg
    }
    #[doc = "0x34208 - CPDMA FHost Channel 2 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh2_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh2HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh2_hdp_reg
    }
    #[doc = "0x3420c - CPDMA FHost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh3_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh3HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh3_hdp_reg
    }
    #[doc = "0x34210 - CPDMA FHost Channel 4 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh4_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh4HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh4_hdp_reg
    }
    #[doc = "0x34214 - CPDMA FHost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh5_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh5HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh5_hdp_reg
    }
    #[doc = "0x34218 - CPDMA FHost Channel 6 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh6_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh6HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh6_hdp_reg
    }
    #[doc = "0x3421c - CPDMA FHost Channel 7 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh7_hdp_reg(&self) -> &CpswCpdmaSramCpdmaFh7HdpReg {
        &self.cpsw_cpdma_sram_cpdma_fh7_hdp_reg
    }
    #[doc = "0x34220 - CPDMA THost Channel 0 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th0_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh0HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th0_hdp_reg
    }
    #[doc = "0x34224 - CPDMA THost Channel 1 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th1_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh1HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th1_hdp_reg
    }
    #[doc = "0x34228 - CPDMA THost Channel 2 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th2_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh2HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th2_hdp_reg
    }
    #[doc = "0x3422c - CPDMA THost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th3_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh3HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th3_hdp_reg
    }
    #[doc = "0x34230 - CPDMA THost Channel 4 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th4_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh4HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th4_hdp_reg
    }
    #[doc = "0x34234 - CPDMA THost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th5_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh5HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th5_hdp_reg
    }
    #[doc = "0x34238 - CPDMA THost Channel 6 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th6_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh6HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th6_hdp_reg
    }
    #[doc = "0x3423c - CPDMA THost Channel 7 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th7_hdp_reg(&self) -> &CpswCpdmaSramCpdmaTh7HdpReg {
        &self.cpsw_cpdma_sram_cpdma_th7_hdp_reg
    }
    #[doc = "0x34240 - CPDMA FHost Channel 0 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh0_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh0CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh0_cp_reg
    }
    #[doc = "0x34244 - CPDMA FHost Channel 1 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh1_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh1CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh1_cp_reg
    }
    #[doc = "0x34248 - CPDMA FHost Channel 2 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh2_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh2CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh2_cp_reg
    }
    #[doc = "0x3424c - CPDMA FHost Channel 3 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh3_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh3CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh3_cp_reg
    }
    #[doc = "0x34250 - CPDMA FHost Channel 4 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh4_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh4CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh4_cp_reg
    }
    #[doc = "0x34254 - CPDMA FHost Channel 5 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh5_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh5CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh5_cp_reg
    }
    #[doc = "0x34258 - CPDMA FHost Channel 6 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh6_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh6CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh6_cp_reg
    }
    #[doc = "0x3425c - CPDMA FHost Channel 7 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_fh7_cp_reg(&self) -> &CpswCpdmaSramCpdmaFh7CpReg {
        &self.cpsw_cpdma_sram_cpdma_fh7_cp_reg
    }
    #[doc = "0x34260 - CPDMA THost Channel 0 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th0_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh0CpReg {
        &self.cpsw_cpdma_sram_cpdma_th0_cp_reg
    }
    #[doc = "0x34264 - CPDMA THost Channel 1 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th1_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh1CpReg {
        &self.cpsw_cpdma_sram_cpdma_th1_cp_reg
    }
    #[doc = "0x34268 - CPDMA THost Channel 2 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th2_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh2CpReg {
        &self.cpsw_cpdma_sram_cpdma_th2_cp_reg
    }
    #[doc = "0x3426c - CPDMA THost Channel 3 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th3_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh3CpReg {
        &self.cpsw_cpdma_sram_cpdma_th3_cp_reg
    }
    #[doc = "0x34270 - CPDMA THost Channel 4 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th4_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh4CpReg {
        &self.cpsw_cpdma_sram_cpdma_th4_cp_reg
    }
    #[doc = "0x34274 - CPDMA THost Channel 5 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th5_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh5CpReg {
        &self.cpsw_cpdma_sram_cpdma_th5_cp_reg
    }
    #[doc = "0x34278 - CPDMA THost Channel 6 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th6_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh6CpReg {
        &self.cpsw_cpdma_sram_cpdma_th6_cp_reg
    }
    #[doc = "0x3427c - CPDMA THost Channel 7 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_cpdma_th7_cp_reg(&self) -> &CpswCpdmaSramCpdmaTh7CpReg {
        &self.cpsw_cpdma_sram_cpdma_th7_cp_reg
    }
    #[doc = "0x34300 - Test CPDMA FHost Channel 0 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh0HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg
    }
    #[doc = "0x34304 - Test CPDMA FHost Channel 1 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh1HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg
    }
    #[doc = "0x34308 - Test CPDMA FHost Channel 2 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh2HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg
    }
    #[doc = "0x3430c - Test CPDMA FHost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh3HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg
    }
    #[doc = "0x34310 - Test CPDMA FHost Channel 4 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh4HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg
    }
    #[doc = "0x34314 - Test CPDMA FHost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh5HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg
    }
    #[doc = "0x34318 - Test CPDMA FHost Channel 6 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh6HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg
    }
    #[doc = "0x3431c - Test CPDMA FHost Channel 7 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh7HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg
    }
    #[doc = "0x34320 - Test CPDMA THost Channel 0 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th0_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh0HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th0_hdp_reg
    }
    #[doc = "0x34324 - Test CPDMA THost Channel 1 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th1_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh1HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th1_hdp_reg
    }
    #[doc = "0x34328 - Test CPDMA THost Channel 2 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th2_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh2HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th2_hdp_reg
    }
    #[doc = "0x3432c - Test CPDMA THost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th3_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh3HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th3_hdp_reg
    }
    #[doc = "0x34330 - Test CPDMA THost Channel 4 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th4_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh4HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th4_hdp_reg
    }
    #[doc = "0x34334 - Test CPDMA THost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th5_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh5HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th5_hdp_reg
    }
    #[doc = "0x34338 - Test CPDMA THost Channel 6 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th6_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh6HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th6_hdp_reg
    }
    #[doc = "0x3433c - Test CPDMA THost Channel 7 Head Descriptor Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th7_hdp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh7HdpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th7_hdp_reg
    }
    #[doc = "0x34340 - Test CPDMA FHost Channel 0 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh0_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh0CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh0_cp_reg
    }
    #[doc = "0x34344 - Test CPDMA FHost Channel 1 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh1_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh1CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh1_cp_reg
    }
    #[doc = "0x34348 - Test CPDMA FHost Channel 2 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh2_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh2CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh2_cp_reg
    }
    #[doc = "0x3434c - Test CPDMA FHost Channel 3 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh3_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh3CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh3_cp_reg
    }
    #[doc = "0x34350 - Test CPDMA FHost Channel 4 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh4_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh4CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh4_cp_reg
    }
    #[doc = "0x34354 - Test CPDMA FHost Channel 5 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh5_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh5CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh5_cp_reg
    }
    #[doc = "0x34358 - Test CPDMA FHost Channel 6 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh6_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh6CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh6_cp_reg
    }
    #[doc = "0x3435c - Test CPDMA FHost Channel 7 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_fh7_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaFh7CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_fh7_cp_reg
    }
    #[doc = "0x34360 - Test CPDMA THost Channel 0 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th0_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh0CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th0_cp_reg
    }
    #[doc = "0x34364 - Test CPDMA THost Channel 1 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th1_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh1CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th1_cp_reg
    }
    #[doc = "0x34368 - Test CPDMA THost Channel 2 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th2_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh2CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th2_cp_reg
    }
    #[doc = "0x3436c - Test CPDMA THost Channel 3 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th3_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh3CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th3_cp_reg
    }
    #[doc = "0x34370 - Test CPDMA THost Channel 4 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th4_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh4CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th4_cp_reg
    }
    #[doc = "0x34374 - Test CPDMA THost Channel 5 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th5_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh5CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th5_cp_reg
    }
    #[doc = "0x34378 - Test CPDMA THost Channel 6 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th6_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh6CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th6_cp_reg
    }
    #[doc = "0x3437c - Test CPDMA THost Channel 7 Completion Pointer"]
    #[inline(always)]
    pub const fn cpsw_cpdma_sram_test_cpdma_th7_cp_reg(&self) -> &CpswCpdmaSramTestCpdmaTh7CpReg {
        &self.cpsw_cpdma_sram_test_cpdma_th7_cp_reg
    }
    #[doc = "0x3a000 - Total number of good frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxgoodframes(&self) -> &CpswNcStat0Rxgoodframes {
        &self.cpsw_nc_stat_0_rxgoodframes
    }
    #[doc = "0x3a004 - Total number of good broadcast frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxbroadcastframes(&self) -> &CpswNcStat0Rxbroadcastframes {
        &self.cpsw_nc_stat_0_rxbroadcastframes
    }
    #[doc = "0x3a008 - Total number of good multicast frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxmulticastframes(&self) -> &CpswNcStat0Rxmulticastframes {
        &self.cpsw_nc_stat_0_rxmulticastframes
    }
    #[doc = "0x3a010 - Total number of CRC errors frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxcrcerrors(&self) -> &CpswNcStat0Rxcrcerrors {
        &self.cpsw_nc_stat_0_rxcrcerrors
    }
    #[doc = "0x3a018 - Total number of oversized frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxoversizedframes(&self) -> &CpswNcStat0Rxoversizedframes {
        &self.cpsw_nc_stat_0_rxoversizedframes
    }
    #[doc = "0x3a020 - Total number of undersized frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxundersizedframes(&self) -> &CpswNcStat0Rxundersizedframes {
        &self.cpsw_nc_stat_0_rxundersizedframes
    }
    #[doc = "0x3a024 - Total number of fragmented frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxfragments(&self) -> &CpswNcStat0Rxfragments {
        &self.cpsw_nc_stat_0_rxfragments
    }
    #[doc = "0x3a028 - Total number of frames dropped by the ALE"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_drop(&self) -> &CpswNcStat0AleDrop {
        &self.cpsw_nc_stat_0_ale_drop
    }
    #[doc = "0x3a02c - Total number of overrun frames dropped by the ALE"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_overrun_drop(&self) -> &CpswNcStat0AleOverrunDrop {
        &self.cpsw_nc_stat_0_ale_overrun_drop
    }
    #[doc = "0x3a030 - Total number of received bytes in good frames"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rxoctets(&self) -> &CpswNcStat0Rxoctets {
        &self.cpsw_nc_stat_0_rxoctets
    }
    #[doc = "0x3a034 - Total number of good frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txgoodframes(&self) -> &CpswNcStat0Txgoodframes {
        &self.cpsw_nc_stat_0_txgoodframes
    }
    #[doc = "0x3a038 - Total number of good broadcast frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txbroadcastframes(&self) -> &CpswNcStat0Txbroadcastframes {
        &self.cpsw_nc_stat_0_txbroadcastframes
    }
    #[doc = "0x3a03c - Total number of good multicast frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txmulticastframes(&self) -> &CpswNcStat0Txmulticastframes {
        &self.cpsw_nc_stat_0_txmulticastframes
    }
    #[doc = "0x3a04c - Total number of transmitted frames experiencing a single collision"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txsinglecollframes(&self) -> &CpswNcStat0Txsinglecollframes {
        &self.cpsw_nc_stat_0_txsinglecollframes
    }
    #[doc = "0x3a050 - Total number of transmitted frames experiencing multiple collisions"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txmultcollframes(&self) -> &CpswNcStat0Txmultcollframes {
        &self.cpsw_nc_stat_0_txmultcollframes
    }
    #[doc = "0x3a064 - Total number of bytes in all good frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_txoctets(&self) -> &CpswNcStat0Txoctets {
        &self.cpsw_nc_stat_0_txoctets
    }
    #[doc = "0x3a068 - Total number of 64-byte frames received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes64(&self) -> &CpswNcStat0Octetframes64 {
        &self.cpsw_nc_stat_0_octetframes64
    }
    #[doc = "0x3a06c - Total number of frames of size 65 to 127 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes65t127(&self) -> &CpswNcStat0Octetframes65t127 {
        &self.cpsw_nc_stat_0_octetframes65t127
    }
    #[doc = "0x3a070 - Total number of frames of size 128 to 255 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes128t255(&self) -> &CpswNcStat0Octetframes128t255 {
        &self.cpsw_nc_stat_0_octetframes128t255
    }
    #[doc = "0x3a074 - Total number of frames of size 256 to 511 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes256t511(&self) -> &CpswNcStat0Octetframes256t511 {
        &self.cpsw_nc_stat_0_octetframes256t511
    }
    #[doc = "0x3a078 - Total number of frames of size 512 to 1023 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes512t1023(&self) -> &CpswNcStat0Octetframes512t1023 {
        &self.cpsw_nc_stat_0_octetframes512t1023
    }
    #[doc = "0x3a07c - Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_octetframes1024tup(&self) -> &CpswNcStat0Octetframes1024tup {
        &self.cpsw_nc_stat_0_octetframes1024tup
    }
    #[doc = "0x3a080 - Total number of bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_netoctets(&self) -> &CpswNcStat0Netoctets {
        &self.cpsw_nc_stat_0_netoctets
    }
    #[doc = "0x3a084 - Receive Bottom of FIFO Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rx_bottom_of_fifo_drop(&self) -> &CpswNcStat0RxBottomOfFifoDrop {
        &self.cpsw_nc_stat_0_rx_bottom_of_fifo_drop
    }
    #[doc = "0x3a088 - Total number of dropped frames received due to portmask"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_portmask_drop(&self) -> &CpswNcStat0PortmaskDrop {
        &self.cpsw_nc_stat_0_portmask_drop
    }
    #[doc = "0x3a08c - Receive Top of FIFO Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_rx_top_of_fifo_drop(&self) -> &CpswNcStat0RxTopOfFifoDrop {
        &self.cpsw_nc_stat_0_rx_top_of_fifo_drop
    }
    #[doc = "0x3a090 - Total number of dropped frames due to ALE Rate Limiting"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_rate_limit_drop(&self) -> &CpswNcStat0AleRateLimitDrop {
        &self.cpsw_nc_stat_0_ale_rate_limit_drop
    }
    #[doc = "0x3a094 - Total number of dropped frames due to ALE VID Ingress"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_vid_ingress_drop(&self) -> &CpswNcStat0AleVidIngressDrop {
        &self.cpsw_nc_stat_0_ale_vid_ingress_drop
    }
    #[doc = "0x3a098 - Total number of dropped frames due to DA=SA"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_da_eq_sa_drop(&self) -> &CpswNcStat0AleDaEqSaDrop {
        &self.cpsw_nc_stat_0_ale_da_eq_sa_drop
    }
    #[doc = "0x3a09c - Total number of dropped frames due to ALE Block Mode"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_block_drop(&self) -> &CpswNcStat0AleBlockDrop {
        &self.cpsw_nc_stat_0_ale_block_drop
    }
    #[doc = "0x3a0a0 - Total number of dropped frames due to ALE Secure Mode"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_secure_drop(&self) -> &CpswNcStat0AleSecureDrop {
        &self.cpsw_nc_stat_0_ale_secure_drop
    }
    #[doc = "0x3a0a4 - Total number of dropped frames due to ALE Authentication"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_auth_drop(&self) -> &CpswNcStat0AleAuthDrop {
        &self.cpsw_nc_stat_0_ale_auth_drop
    }
    #[doc = "0x3a0a8 - ALE Receive Unknown Unicast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_uni(&self) -> &CpswNcStat0AleUnknUni {
        &self.cpsw_nc_stat_0_ale_unkn_uni
    }
    #[doc = "0x3a0ac - ALE Receive Unknown Unicast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_uni_bcnt(&self) -> &CpswNcStat0AleUnknUniBcnt {
        &self.cpsw_nc_stat_0_ale_unkn_uni_bcnt
    }
    #[doc = "0x3a0b0 - ALE Receive Unknown Multicast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_mlt(&self) -> &CpswNcStat0AleUnknMlt {
        &self.cpsw_nc_stat_0_ale_unkn_mlt
    }
    #[doc = "0x3a0b4 - ALE Receive Unknown Multicast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_mlt_bcnt(&self) -> &CpswNcStat0AleUnknMltBcnt {
        &self.cpsw_nc_stat_0_ale_unkn_mlt_bcnt
    }
    #[doc = "0x3a0b8 - ALE Receive Unknown Broadcast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_brd(&self) -> &CpswNcStat0AleUnknBrd {
        &self.cpsw_nc_stat_0_ale_unkn_brd
    }
    #[doc = "0x3a0bc - ALE Receive Unknown Broadcast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_unkn_brd_bcnt(&self) -> &CpswNcStat0AleUnknBrdBcnt {
        &self.cpsw_nc_stat_0_ale_unkn_brd_bcnt
    }
    #[doc = "0x3a0c0 - ALE Policer Matched"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_pol_match(&self) -> &CpswNcStat0AlePolMatch {
        &self.cpsw_nc_stat_0_ale_pol_match
    }
    #[doc = "0x3a0c4 - ALE Policer Matched and Condition Red"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_pol_match_red(&self) -> &CpswNcStat0AlePolMatchRed {
        &self.cpsw_nc_stat_0_ale_pol_match_red
    }
    #[doc = "0x3a0c8 - ALE Policer Matched and Condition Yellow"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_pol_match_yellow(&self) -> &CpswNcStat0AlePolMatchYellow {
        &self.cpsw_nc_stat_0_ale_pol_match_yellow
    }
    #[doc = "0x3a0cc - ALE Multicast Source Address Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_mult_sa_drop(&self) -> &CpswNcStat0AleMultSaDrop {
        &self.cpsw_nc_stat_0_ale_mult_sa_drop
    }
    #[doc = "0x3a0d0 - ALE Dual VLAN Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_dual_vlan_drop(&self) -> &CpswNcStat0AleDualVlanDrop {
        &self.cpsw_nc_stat_0_ale_dual_vlan_drop
    }
    #[doc = "0x3a0d4 - ALE Length Error Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_len_error_drop(&self) -> &CpswNcStat0AleLenErrorDrop {
        &self.cpsw_nc_stat_0_ale_len_error_drop
    }
    #[doc = "0x3a0d8 - ALE IP Next Header Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_ip_next_hdr_drop(&self) -> &CpswNcStat0AleIpNextHdrDrop {
        &self.cpsw_nc_stat_0_ale_ip_next_hdr_drop
    }
    #[doc = "0x3a0dc - ALE IPV4 Frag Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_ale_ipv4_frag_drop(&self) -> &CpswNcStat0AleIpv4FragDrop {
        &self.cpsw_nc_stat_0_ale_ipv4_frag_drop
    }
    #[doc = "0x3a17c - Transmit Memory Protect CRC Error"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_0_tx_memory_protect_error(&self) -> &CpswNcStat0TxMemoryProtectError {
        &self.cpsw_nc_stat_0_tx_memory_protect_error
    }
    #[doc = "0x3a200 - Total number of good frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxgoodframes(&self) -> &CpswNcStat1Rxgoodframes {
        &self.cpsw_nc_stat_1_rxgoodframes
    }
    #[doc = "0x3a204 - Total number of good broadcast frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxbroadcastframes(&self) -> &CpswNcStat1Rxbroadcastframes {
        &self.cpsw_nc_stat_1_rxbroadcastframes
    }
    #[doc = "0x3a208 - Total number of good multicast frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxmulticastframes(&self) -> &CpswNcStat1Rxmulticastframes {
        &self.cpsw_nc_stat_1_rxmulticastframes
    }
    #[doc = "0x3a20c - Total number of pause frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxpauseframes(&self) -> &CpswNcStat1Rxpauseframes {
        &self.cpsw_nc_stat_1_rxpauseframes
    }
    #[doc = "0x3a210 - Total number of CRC errors frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxcrcerrors(&self) -> &CpswNcStat1Rxcrcerrors {
        &self.cpsw_nc_stat_1_rxcrcerrors
    }
    #[doc = "0x3a214 - Total number of alignment/code errors received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxaligncodeerrors(&self) -> &CpswNcStat1Rxaligncodeerrors {
        &self.cpsw_nc_stat_1_rxaligncodeerrors
    }
    #[doc = "0x3a218 - Total number of oversized frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxoversizedframes(&self) -> &CpswNcStat1Rxoversizedframes {
        &self.cpsw_nc_stat_1_rxoversizedframes
    }
    #[doc = "0x3a21c - Total number of jabber frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxjabberframes(&self) -> &CpswNcStat1Rxjabberframes {
        &self.cpsw_nc_stat_1_rxjabberframes
    }
    #[doc = "0x3a220 - Total number of undersized frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxundersizedframes(&self) -> &CpswNcStat1Rxundersizedframes {
        &self.cpsw_nc_stat_1_rxundersizedframes
    }
    #[doc = "0x3a224 - Total number of fragmented frames received"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxfragments(&self) -> &CpswNcStat1Rxfragments {
        &self.cpsw_nc_stat_1_rxfragments
    }
    #[doc = "0x3a228 - Total number of frames dropped by the ALE"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_drop(&self) -> &CpswNcStat1AleDrop {
        &self.cpsw_nc_stat_1_ale_drop
    }
    #[doc = "0x3a22c - Total number of overrun frames dropped by the ALE"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_overrun_drop(&self) -> &CpswNcStat1AleOverrunDrop {
        &self.cpsw_nc_stat_1_ale_overrun_drop
    }
    #[doc = "0x3a230 - Total number of received bytes in good frames"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxoctets(&self) -> &CpswNcStat1Rxoctets {
        &self.cpsw_nc_stat_1_rxoctets
    }
    #[doc = "0x3a234 - Total number of good frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txgoodframes(&self) -> &CpswNcStat1Txgoodframes {
        &self.cpsw_nc_stat_1_txgoodframes
    }
    #[doc = "0x3a238 - Total number of good broadcast frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txbroadcastframes(&self) -> &CpswNcStat1Txbroadcastframes {
        &self.cpsw_nc_stat_1_txbroadcastframes
    }
    #[doc = "0x3a23c - Total number of good multicast frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txmulticastframes(&self) -> &CpswNcStat1Txmulticastframes {
        &self.cpsw_nc_stat_1_txmulticastframes
    }
    #[doc = "0x3a240 - Total number of pause frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txpauseframes(&self) -> &CpswNcStat1Txpauseframes {
        &self.cpsw_nc_stat_1_txpauseframes
    }
    #[doc = "0x3a244 - Total number of deferred frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txdeferredframes(&self) -> &CpswNcStat1Txdeferredframes {
        &self.cpsw_nc_stat_1_txdeferredframes
    }
    #[doc = "0x3a248 - Total number of transmitted frames experiencing a collision"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txcollisionframes(&self) -> &CpswNcStat1Txcollisionframes {
        &self.cpsw_nc_stat_1_txcollisionframes
    }
    #[doc = "0x3a24c - Total number of transmitted frames experiencing a single collision"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txsinglecollframes(&self) -> &CpswNcStat1Txsinglecollframes {
        &self.cpsw_nc_stat_1_txsinglecollframes
    }
    #[doc = "0x3a250 - Total number of transmitted frames experiencing multiple collisions"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txmultcollframes(&self) -> &CpswNcStat1Txmultcollframes {
        &self.cpsw_nc_stat_1_txmultcollframes
    }
    #[doc = "0x3a254 - Total number of transmitted frames abandoned due to excessive collisions"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txexcessivecollisions(&self) -> &CpswNcStat1Txexcessivecollisions {
        &self.cpsw_nc_stat_1_txexcessivecollisions
    }
    #[doc = "0x3a258 - Total number of transmitted frames abandoned due to a late collision"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txlatecollisions(&self) -> &CpswNcStat1Txlatecollisions {
        &self.cpsw_nc_stat_1_txlatecollisions
    }
    #[doc = "0x3a25c - Total number of receive inter-packet gap errors (10G only)"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rxipgerror(&self) -> &CpswNcStat1Rxipgerror {
        &self.cpsw_nc_stat_1_rxipgerror
    }
    #[doc = "0x3a260 - Total number of transmitted frames that experienced a carrier loss"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txcarriersenseerrors(&self) -> &CpswNcStat1Txcarriersenseerrors {
        &self.cpsw_nc_stat_1_txcarriersenseerrors
    }
    #[doc = "0x3a264 - Total number of bytes in all good frames transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_txoctets(&self) -> &CpswNcStat1Txoctets {
        &self.cpsw_nc_stat_1_txoctets
    }
    #[doc = "0x3a268 - Total number of 64-byte frames received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes64(&self) -> &CpswNcStat1Octetframes64 {
        &self.cpsw_nc_stat_1_octetframes64
    }
    #[doc = "0x3a26c - Total number of frames of size 65 to 127 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes65t127(&self) -> &CpswNcStat1Octetframes65t127 {
        &self.cpsw_nc_stat_1_octetframes65t127
    }
    #[doc = "0x3a270 - Total number of frames of size 128 to 255 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes128t255(&self) -> &CpswNcStat1Octetframes128t255 {
        &self.cpsw_nc_stat_1_octetframes128t255
    }
    #[doc = "0x3a274 - Total number of frames of size 256 to 511 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes256t511(&self) -> &CpswNcStat1Octetframes256t511 {
        &self.cpsw_nc_stat_1_octetframes256t511
    }
    #[doc = "0x3a278 - Total number of frames of size 512 to 1023 bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes512t1023(&self) -> &CpswNcStat1Octetframes512t1023 {
        &self.cpsw_nc_stat_1_octetframes512t1023
    }
    #[doc = "0x3a27c - Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_octetframes1024tup(&self) -> &CpswNcStat1Octetframes1024tup {
        &self.cpsw_nc_stat_1_octetframes1024tup
    }
    #[doc = "0x3a280 - Total number of bytes received and transmitted"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_netoctets(&self) -> &CpswNcStat1Netoctets {
        &self.cpsw_nc_stat_1_netoctets
    }
    #[doc = "0x3a284 - Receive Bottom of FIFO Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rx_bottom_of_fifo_drop(&self) -> &CpswNcStat1RxBottomOfFifoDrop {
        &self.cpsw_nc_stat_1_rx_bottom_of_fifo_drop
    }
    #[doc = "0x3a288 - Total number of dropped frames received due to portmask"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_portmask_drop(&self) -> &CpswNcStat1PortmaskDrop {
        &self.cpsw_nc_stat_1_portmask_drop
    }
    #[doc = "0x3a28c - Receive Top of FIFO Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_rx_top_of_fifo_drop(&self) -> &CpswNcStat1RxTopOfFifoDrop {
        &self.cpsw_nc_stat_1_rx_top_of_fifo_drop
    }
    #[doc = "0x3a290 - Total number of dropped frames due to ALE Rate Limiting"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_rate_limit_drop(&self) -> &CpswNcStat1AleRateLimitDrop {
        &self.cpsw_nc_stat_1_ale_rate_limit_drop
    }
    #[doc = "0x3a294 - Total number of dropped frames due to ALE VID Ingress"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_vid_ingress_drop(&self) -> &CpswNcStat1AleVidIngressDrop {
        &self.cpsw_nc_stat_1_ale_vid_ingress_drop
    }
    #[doc = "0x3a298 - Total number of dropped frames due to DA=SA"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_da_eq_sa_drop(&self) -> &CpswNcStat1AleDaEqSaDrop {
        &self.cpsw_nc_stat_1_ale_da_eq_sa_drop
    }
    #[doc = "0x3a29c - Total number of dropped frames due to ALE Block Mode"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_block_drop(&self) -> &CpswNcStat1AleBlockDrop {
        &self.cpsw_nc_stat_1_ale_block_drop
    }
    #[doc = "0x3a2a0 - Total number of dropped frames due to ALE Secure Mode"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_secure_drop(&self) -> &CpswNcStat1AleSecureDrop {
        &self.cpsw_nc_stat_1_ale_secure_drop
    }
    #[doc = "0x3a2a4 - Total number of dropped frames due to ALE Authentication"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_auth_drop(&self) -> &CpswNcStat1AleAuthDrop {
        &self.cpsw_nc_stat_1_ale_auth_drop
    }
    #[doc = "0x3a2a8 - ALE Receive Unknown Unicast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_uni(&self) -> &CpswNcStat1AleUnknUni {
        &self.cpsw_nc_stat_1_ale_unkn_uni
    }
    #[doc = "0x3a2ac - ALE Receive Unknown Unicast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_uni_bcnt(&self) -> &CpswNcStat1AleUnknUniBcnt {
        &self.cpsw_nc_stat_1_ale_unkn_uni_bcnt
    }
    #[doc = "0x3a2b0 - ALE Receive Unknown Multicast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_mlt(&self) -> &CpswNcStat1AleUnknMlt {
        &self.cpsw_nc_stat_1_ale_unkn_mlt
    }
    #[doc = "0x3a2b4 - ALE Receive Unknown Multicast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_mlt_bcnt(&self) -> &CpswNcStat1AleUnknMltBcnt {
        &self.cpsw_nc_stat_1_ale_unkn_mlt_bcnt
    }
    #[doc = "0x3a2b8 - ALE Receive Unknown Broadcast"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_brd(&self) -> &CpswNcStat1AleUnknBrd {
        &self.cpsw_nc_stat_1_ale_unkn_brd
    }
    #[doc = "0x3a2bc - ALE Receive Unknown Broadcast Bytecount"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_unkn_brd_bcnt(&self) -> &CpswNcStat1AleUnknBrdBcnt {
        &self.cpsw_nc_stat_1_ale_unkn_brd_bcnt
    }
    #[doc = "0x3a2c0 - ALE Policer Matched"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_pol_match(&self) -> &CpswNcStat1AlePolMatch {
        &self.cpsw_nc_stat_1_ale_pol_match
    }
    #[doc = "0x3a2c4 - ALE Policer Matched and Condition Red"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_pol_match_red(&self) -> &CpswNcStat1AlePolMatchRed {
        &self.cpsw_nc_stat_1_ale_pol_match_red
    }
    #[doc = "0x3a2c8 - ALE Policer Matched and Condition Yellow"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_pol_match_yellow(&self) -> &CpswNcStat1AlePolMatchYellow {
        &self.cpsw_nc_stat_1_ale_pol_match_yellow
    }
    #[doc = "0x3a2cc - ALE Multicast Source Address Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_mult_sa_drop(&self) -> &CpswNcStat1AleMultSaDrop {
        &self.cpsw_nc_stat_1_ale_mult_sa_drop
    }
    #[doc = "0x3a2d0 - ALE Dual VLAN Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_dual_vlan_drop(&self) -> &CpswNcStat1AleDualVlanDrop {
        &self.cpsw_nc_stat_1_ale_dual_vlan_drop
    }
    #[doc = "0x3a2d4 - ALE Length Error Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_len_error_drop(&self) -> &CpswNcStat1AleLenErrorDrop {
        &self.cpsw_nc_stat_1_ale_len_error_drop
    }
    #[doc = "0x3a2d8 - ALE IP Next Header Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_ip_next_hdr_drop(&self) -> &CpswNcStat1AleIpNextHdrDrop {
        &self.cpsw_nc_stat_1_ale_ip_next_hdr_drop
    }
    #[doc = "0x3a2dc - ALE IPV4 Frag Drop"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_ale_ipv4_frag_drop(&self) -> &CpswNcStat1AleIpv4FragDrop {
        &self.cpsw_nc_stat_1_ale_ipv4_frag_drop
    }
    #[doc = "0x3a37c - Transmit Memory Protect CRC Error"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_tx_memory_protect_error(&self) -> &CpswNcStat1TxMemoryProtectError {
        &self.cpsw_nc_stat_1_tx_memory_protect_error
    }
    #[doc = "0x3a380 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_0(&self) -> &CpswNcStat1EnetPnTxPriReg0 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_0
    }
    #[doc = "0x3a384 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_1(&self) -> &CpswNcStat1EnetPnTxPriReg1 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_1
    }
    #[doc = "0x3a388 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_2(&self) -> &CpswNcStat1EnetPnTxPriReg2 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_2
    }
    #[doc = "0x3a38c - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_3(&self) -> &CpswNcStat1EnetPnTxPriReg3 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_3
    }
    #[doc = "0x3a390 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_4(&self) -> &CpswNcStat1EnetPnTxPriReg4 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_4
    }
    #[doc = "0x3a394 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_5(&self) -> &CpswNcStat1EnetPnTxPriReg5 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_5
    }
    #[doc = "0x3a398 - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_6(&self) -> &CpswNcStat1EnetPnTxPriReg6 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_6
    }
    #[doc = "0x3a39c - ENET Port n PRIORITY N Packet Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_reg_7(&self) -> &CpswNcStat1EnetPnTxPriReg7 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_reg_7
    }
    #[doc = "0x3a3a0 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg0 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0
    }
    #[doc = "0x3a3a4 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg1 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1
    }
    #[doc = "0x3a3a8 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg2 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2
    }
    #[doc = "0x3a3ac - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg3 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3
    }
    #[doc = "0x3a3b0 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg4 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4
    }
    #[doc = "0x3a3b4 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg5 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5
    }
    #[doc = "0x3a3b8 - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg6 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6
    }
    #[doc = "0x3a3bc - ENET Port n PRIORITY N Packet Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriBcntReg7 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7
    }
    #[doc = "0x3a3c0 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg0 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0
    }
    #[doc = "0x3a3c4 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg1 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1
    }
    #[doc = "0x3a3c8 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg2 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2
    }
    #[doc = "0x3a3cc - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg3 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3
    }
    #[doc = "0x3a3d0 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg4 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4
    }
    #[doc = "0x3a3d4 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg5 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5
    }
    #[doc = "0x3a3d8 - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg6 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6
    }
    #[doc = "0x3a3dc - ENET Port n PRIORITY N Packet Drop Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropReg7 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7
    }
    #[doc = "0x3a3e0 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg0 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0
    }
    #[doc = "0x3a3e4 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg1 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1
    }
    #[doc = "0x3a3e8 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg2 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2
    }
    #[doc = "0x3a3ec - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg3 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3
    }
    #[doc = "0x3a3f0 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg4 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4
    }
    #[doc = "0x3a3f4 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg5 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5
    }
    #[doc = "0x3a3f8 - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg6 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6
    }
    #[doc = "0x3a3fc - ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub const fn cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7(
        &self,
    ) -> &CpswNcStat1EnetPnTxPriDropBcntReg7 {
        &self.cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7
    }
    #[doc = "0x3d000 - Identification and Version Register"]
    #[inline(always)]
    pub const fn idver_reg(&self) -> &IdverReg {
        &self.idver_reg
    }
    #[doc = "0x3d004 - Time Sync Control Register"]
    #[inline(always)]
    pub const fn cpts_control_reg(&self) -> &CptsControlReg {
        &self.cpts_control_reg
    }
    #[doc = "0x3d008 - RFTCLK Select Register"]
    #[inline(always)]
    pub const fn cpts_rftclk_sel_reg(&self) -> &CptsRftclkSelReg {
        &self.cpts_rftclk_sel_reg
    }
    #[doc = "0x3d00c - Time Stamp Event Push Register"]
    #[inline(always)]
    pub const fn cpts_ts_push_reg(&self) -> &CptsTsPushReg {
        &self.cpts_ts_push_reg
    }
    #[doc = "0x3d010 - Time Stamp Load Low Value Register"]
    #[inline(always)]
    pub const fn ts_load_val_reg(&self) -> &TsLoadValReg {
        &self.ts_load_val_reg
    }
    #[doc = "0x3d014 - Time Stamp Load Enable Register"]
    #[inline(always)]
    pub const fn cpts_ts_load_en_reg(&self) -> &CptsTsLoadEnReg {
        &self.cpts_ts_load_en_reg
    }
    #[doc = "0x3d018 - Time Stamp Comparison Low Value Register"]
    #[inline(always)]
    pub const fn ts_comp_val_reg(&self) -> &TsCompValReg {
        &self.ts_comp_val_reg
    }
    #[doc = "0x3d01c - Time Stamp Comparison Length Register"]
    #[inline(always)]
    pub const fn cpts_ts_comp_len_reg(&self) -> &CptsTsCompLenReg {
        &self.cpts_ts_comp_len_reg
    }
    #[doc = "0x3d020 - Interrupt Status Register Raw"]
    #[inline(always)]
    pub const fn cpts_intstat_raw_reg(&self) -> &CptsIntstatRawReg {
        &self.cpts_intstat_raw_reg
    }
    #[doc = "0x3d024 - Interrupt Status Register Masked"]
    #[inline(always)]
    pub const fn cpts_intstat_masked_reg(&self) -> &CptsIntstatMaskedReg {
        &self.cpts_intstat_masked_reg
    }
    #[doc = "0x3d028 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cpts_int_enable_reg(&self) -> &CptsIntEnableReg {
        &self.cpts_int_enable_reg
    }
    #[doc = "0x3d02c - Time Stamp Comparison Nudge Register"]
    #[inline(always)]
    pub const fn cpts_ts_comp_nudge_reg(&self) -> &CptsTsCompNudgeReg {
        &self.cpts_ts_comp_nudge_reg
    }
    #[doc = "0x3d030 - Event Pop Register"]
    #[inline(always)]
    pub const fn cpts_event_pop_reg(&self) -> &CptsEventPopReg {
        &self.cpts_event_pop_reg
    }
    #[doc = "0x3d034 - Event 0 Register"]
    #[inline(always)]
    pub const fn cpts_event_0_reg(&self) -> &CptsEvent0Reg {
        &self.cpts_event_0_reg
    }
    #[doc = "0x3d038 - Event 1 Register"]
    #[inline(always)]
    pub const fn cpts_event_1_reg(&self) -> &CptsEvent1Reg {
        &self.cpts_event_1_reg
    }
    #[doc = "0x3d03c - Event 2 Register"]
    #[inline(always)]
    pub const fn cpts_event_2_reg(&self) -> &CptsEvent2Reg {
        &self.cpts_event_2_reg
    }
    #[doc = "0x3d040 - Event 3 Register"]
    #[inline(always)]
    pub const fn cpts_event_3_reg(&self) -> &CptsEvent3Reg {
        &self.cpts_event_3_reg
    }
    #[doc = "0x3d044 - Time Stamp Load High Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_load_high_val_reg(&self) -> &CptsTsLoadHighValReg {
        &self.cpts_ts_load_high_val_reg
    }
    #[doc = "0x3d048 - Time Stamp Comparison High Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_comp_high_val_reg(&self) -> &CptsTsCompHighValReg {
        &self.cpts_ts_comp_high_val_reg
    }
    #[doc = "0x3d04c - TS Add Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_add_val_reg(&self) -> &CptsTsAddValReg {
        &self.cpts_ts_add_val_reg
    }
    #[doc = "0x3d050 - Time Stamp PPM Low Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_ppm_low_val_reg(&self) -> &CptsTsPpmLowValReg {
        &self.cpts_ts_ppm_low_val_reg
    }
    #[doc = "0x3d054 - Time Stamp PPM High Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_ppm_high_val_reg(&self) -> &CptsTsPpmHighValReg {
        &self.cpts_ts_ppm_high_val_reg
    }
    #[doc = "0x3d058 - Time Stamp Nudge Value Register"]
    #[inline(always)]
    pub const fn cpts_ts_nudge_val_reg(&self) -> &CptsTsNudgeValReg {
        &self.cpts_ts_nudge_val_reg
    }
    #[doc = "0x3d0d0 - Time Stamp Configuration Read"]
    #[inline(always)]
    pub const fn cpts_ts_config(&self) -> &CptsTsConfig {
        &self.cpts_ts_config
    }
    #[doc = "0x3d0e0 - Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn ts_genf0_comp_low_reg(&self) -> &TsGenf0CompLowReg {
        &self.ts_genf0_comp_low_reg
    }
    #[doc = "0x3d0e4 - Time Stamp Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn ts_genf0_comp_high_reg(&self) -> &TsGenf0CompHighReg {
        &self.ts_genf0_comp_high_reg
    }
    #[doc = "0x3d0e8 - Time Stamp Generate Function Control"]
    #[inline(always)]
    pub const fn ts_genf0_control_reg(&self) -> &TsGenf0ControlReg {
        &self.ts_genf0_control_reg
    }
    #[doc = "0x3d0ec - Time Stamp Generate Function Length Value"]
    #[inline(always)]
    pub const fn ts_genf0_length_reg(&self) -> &TsGenf0LengthReg {
        &self.ts_genf0_length_reg
    }
    #[doc = "0x3d0f0 - Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn ts_genf0_ppm_low_reg(&self) -> &TsGenf0PpmLowReg {
        &self.ts_genf0_ppm_low_reg
    }
    #[doc = "0x3d0f4 - Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn ts_genf0_ppm_high_reg(&self) -> &TsGenf0PpmHighReg {
        &self.ts_genf0_ppm_high_reg
    }
    #[doc = "0x3d0f8 - Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn ts_genf0_nudge_reg(&self) -> &TsGenf0NudgeReg {
        &self.ts_genf0_nudge_reg
    }
    #[doc = "0x3d100 - Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn ts_genf1_comp_low_reg(&self) -> &TsGenf1CompLowReg {
        &self.ts_genf1_comp_low_reg
    }
    #[doc = "0x3d104 - Time Stamp Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn ts_genf1_comp_high_reg(&self) -> &TsGenf1CompHighReg {
        &self.ts_genf1_comp_high_reg
    }
    #[doc = "0x3d108 - Time Stamp Generate Function Control"]
    #[inline(always)]
    pub const fn ts_genf1_control_reg(&self) -> &TsGenf1ControlReg {
        &self.ts_genf1_control_reg
    }
    #[doc = "0x3d10c - Time Stamp Generate Function Length Value"]
    #[inline(always)]
    pub const fn ts_genf1_length_reg(&self) -> &TsGenf1LengthReg {
        &self.ts_genf1_length_reg
    }
    #[doc = "0x3d110 - Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn ts_genf1_ppm_low_reg(&self) -> &TsGenf1PpmLowReg {
        &self.ts_genf1_ppm_low_reg
    }
    #[doc = "0x3d114 - Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn ts_genf1_ppm_high_reg(&self) -> &TsGenf1PpmHighReg {
        &self.ts_genf1_ppm_high_reg
    }
    #[doc = "0x3d118 - Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn ts_genf1_nudge_reg(&self) -> &TsGenf1NudgeReg {
        &self.ts_genf1_nudge_reg
    }
    #[doc = "0x3d120 - Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn ts_genf2_comp_low_reg(&self) -> &TsGenf2CompLowReg {
        &self.ts_genf2_comp_low_reg
    }
    #[doc = "0x3d124 - Time Stamp Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn ts_genf2_comp_high_reg(&self) -> &TsGenf2CompHighReg {
        &self.ts_genf2_comp_high_reg
    }
    #[doc = "0x3d128 - Time Stamp Generate Function Control"]
    #[inline(always)]
    pub const fn ts_genf2_control_reg(&self) -> &TsGenf2ControlReg {
        &self.ts_genf2_control_reg
    }
    #[doc = "0x3d12c - Time Stamp Generate Function Length Value"]
    #[inline(always)]
    pub const fn ts_genf2_length_reg(&self) -> &TsGenf2LengthReg {
        &self.ts_genf2_length_reg
    }
    #[doc = "0x3d130 - Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn ts_genf2_ppm_low_reg(&self) -> &TsGenf2PpmLowReg {
        &self.ts_genf2_ppm_low_reg
    }
    #[doc = "0x3d134 - Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn ts_genf2_ppm_high_reg(&self) -> &TsGenf2PpmHighReg {
        &self.ts_genf2_ppm_high_reg
    }
    #[doc = "0x3d138 - Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn ts_genf2_nudge_reg(&self) -> &TsGenf2NudgeReg {
        &self.ts_genf2_nudge_reg
    }
    #[doc = "0x3d200 - Time Stamp ESTF Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn ts_estf_comp_low_reg(&self) -> &TsEstfCompLowReg {
        &self.ts_estf_comp_low_reg
    }
    #[doc = "0x3d204 - Time Stamp ESTF Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn ts_estf_comp_high_reg(&self) -> &TsEstfCompHighReg {
        &self.ts_estf_comp_high_reg
    }
    #[doc = "0x3d208 - Time Stamp ESTF Generate Function Control"]
    #[inline(always)]
    pub const fn ts_estf_control_reg(&self) -> &TsEstfControlReg {
        &self.ts_estf_control_reg
    }
    #[doc = "0x3d20c - Time Stamp ESTF Generate Function Length Value"]
    #[inline(always)]
    pub const fn ts_estf_length_reg(&self) -> &TsEstfLengthReg {
        &self.ts_estf_length_reg
    }
    #[doc = "0x3d210 - Time Stamp ESTF Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn ts_estf_ppm_low_reg(&self) -> &TsEstfPpmLowReg {
        &self.ts_estf_ppm_low_reg
    }
    #[doc = "0x3d214 - Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn ts_estf_ppm_high_reg(&self) -> &TsEstfPpmHighReg {
        &self.ts_estf_ppm_high_reg
    }
    #[doc = "0x3d218 - Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn ts_estf_nudge_reg(&self) -> &TsEstfNudgeReg {
        &self.ts_estf_nudge_reg
    }
    #[doc = "0x3e000 - The Module and Version Register identifies the module identifier and revision of the ALE_2g32 module."]
    #[inline(always)]
    pub const fn ale_mod_ver(&self) -> &AleModVer {
        &self.ale_mod_ver
    }
    #[doc = "0x3e004 - The ALE status provides information on the ALE configuration and state. The ~iramdepth is used to determine how IPv6 entries are stored in the table. IPv6 entries are stored in two entries where IPv6 Entry hi is designated by the odd slice index and lo is designated by the even slice index. The slice index is above the ram depth like {SlixeIndex,RamIndex}. So for a 64 deep RAM index of 0x005, the Hi portion of the IPv6 entry is located at 0x005|0x040 and the Lo portion is located at 0x005&amp;amp;(~0x040)."]
    #[inline(always)]
    pub const fn ale_ale_status(&self) -> &AleAleStatus {
        &self.ale_ale_status
    }
    #[doc = "0x3e008 - The ALE Control Register is used to set the ALE modes used for all ports."]
    #[inline(always)]
    pub const fn ale_ale_control(&self) -> &AleAleControl {
        &self.ale_ale_control
    }
    #[doc = "0x3e00c - The ALE Control 2 Register is used to set the extended features used for all ports."]
    #[inline(always)]
    pub const fn ale_ale_ctrl2(&self) -> &AleAleCtrl2 {
        &self.ale_ale_ctrl2
    }
    #[doc = "0x3e010 - The ALE Prescale Register is used to set the Broadcast and Multicast rate limiting prescaler value."]
    #[inline(always)]
    pub const fn ale_ale_prescale(&self) -> &AleAlePrescale {
        &self.ale_ale_prescale
    }
    #[doc = "0x3e014 - The ALE Aging Control sets the aging interval which will cause periodic aging to occur. This value specifies the minimum time between aging starts."]
    #[inline(always)]
    pub const fn ale_ale_aging_ctrl(&self) -> &AleAleAgingCtrl {
        &self.ale_ale_aging_ctrl
    }
    #[doc = "0x3e01c - The ALE Next Header is used to limit the IPv6 Next header or IPv4 Protocol values found in the IP header. It is enabled via the ~iLmtNxtHdr bit in the VLAN entry. All four ~iip_nxt_hdr0-3 are compared when enabled, so if only one is required, set them all to the one value to be tested."]
    #[inline(always)]
    pub const fn ale_ale_nxt_hdr(&self) -> &AleAleNxtHdr {
        &self.ale_ale_nxt_hdr
    }
    #[doc = "0x3e020 - The ALE table control register is used to read or write that ALE table entries. After writing to this register any read or write to any ALE register will be stalled until the read or write operation completes."]
    #[inline(always)]
    pub const fn ale_ale_tblctl(&self) -> &AleAleTblctl {
        &self.ale_ale_tblctl
    }
    #[doc = "0x3e034 - The ALE Table Word 2 is the most significant word of an ALE table entry."]
    #[inline(always)]
    pub const fn ale_ale_tblw2(&self) -> &AleAleTblw2 {
        &self.ale_ale_tblw2
    }
    #[doc = "0x3e038 - The ALE Table Word 1 is the middle word of an ALE table entry."]
    #[inline(always)]
    pub const fn ale_ale_tblw1(&self) -> &AleAleTblw1 {
        &self.ale_ale_tblw1
    }
    #[doc = "0x3e03c - The ALE Table Word 0 is the least significant word of an ALE table entry."]
    #[inline(always)]
    pub const fn ale_ale_tblw0(&self) -> &AleAleTblw0 {
        &self.ale_ale_tblw0
    }
    #[doc = "0x3e040 - The ALE Port Control Register sets the port specific modes of operation."]
    #[inline(always)]
    pub const fn ale_i0_ale_portctl0_0(&self) -> &AleI0AlePortctl0_0 {
        &self.ale_i0_ale_portctl0_0
    }
    #[doc = "0x3e044 - The ALE Port Control Register sets the port specific modes of operation."]
    #[inline(always)]
    pub const fn ale_i0_ale_portctl0_1(&self) -> &AleI0AlePortctl0_1 {
        &self.ale_i0_ale_portctl0_1
    }
    #[doc = "0x3e090 - The ALE Unknown VLAN Member Mask Register is used to specify the member list for unknown VLAN ID."]
    #[inline(always)]
    pub const fn ale_ale_uvlan_member(&self) -> &AleAleUvlanMember {
        &self.ale_ale_uvlan_member
    }
    #[doc = "0x3e094 - The ALE Unknown VLAN Unregistered Multicast Flood Mask Register is used to specify which egress ports unregistered multicast addresses egress for the unregistered VLAN ID."]
    #[inline(always)]
    pub const fn ale_ale_uvlan_urcast(&self) -> &AleAleUvlanUrcast {
        &self.ale_ale_uvlan_urcast
    }
    #[doc = "0x3e098 - The ALE Unknown VLAN Registered Multicast Flood Mask Register is used to specify which egress ports registered multicast addresses egress for the unregistered VLAN ID."]
    #[inline(always)]
    pub const fn ale_ale_uvlan_rmcast(&self) -> &AleAleUvlanRmcast {
        &self.ale_ale_uvlan_rmcast
    }
    #[doc = "0x3e09c - The ALE Unknown VLAN force Untagged Egress Mask Register is used to specify which egress ports the VLAN ID will be removed."]
    #[inline(always)]
    pub const fn ale_ale_uvlan_untag(&self) -> &AleAleUvlanUntag {
        &self.ale_ale_uvlan_untag
    }
    #[doc = "0x3e0b4 - The Fast LUT registers allows the ports to be placed in Fast LUT mode."]
    #[inline(always)]
    pub const fn ale_ale_fast_lut(&self) -> &AleAleFastLut {
        &self.ale_ale_fast_lut
    }
    #[doc = "0x3e0b8 - The ALE Statistic Output Diagnostic Register allows the output statistics to diagnose the SW counters. This register is for diagnostice only."]
    #[inline(always)]
    pub const fn ale_ale_stat_diag(&self) -> &AleAleStatDiag {
        &self.ale_ale_stat_diag
    }
    #[doc = "0x3e0bc - The ALE OAM Control allows ports to be put into OAM Loopback, only non-supervisor packet are looped back to the source port."]
    #[inline(always)]
    pub const fn ale_ale_oam_lb_ctrl(&self) -> &AleAleOamLbCtrl {
        &self.ale_ale_oam_lb_ctrl
    }
    #[doc = "0x3e0c0 - VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
    #[inline(always)]
    pub const fn ale_ale_msk_mux0(&self) -> &AleAleMskMux0 {
        &self.ale_ale_msk_mux0
    }
    #[doc = "0x3e0c4 - VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
    #[inline(always)]
    pub const fn ale_i1_ale_msk_mux1_0(&self) -> &AleI1AleMskMux1_0 {
        &self.ale_i1_ale_msk_mux1_0
    }
    #[doc = "0x3e0c8 - VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
    #[inline(always)]
    pub const fn ale_i1_ale_msk_mux1_1(&self) -> &AleI1AleMskMux1_1 {
        &self.ale_i1_ale_msk_mux1_1
    }
    #[doc = "0x3e0cc - VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
    #[inline(always)]
    pub const fn ale_i1_ale_msk_mux1_2(&self) -> &AleI1AleMskMux1_2 {
        &self.ale_i1_ale_msk_mux1_2
    }
    #[doc = "0x3e0fc - The Egress Operation register allows enabled classifiers with any match like IPSA or IPDA match to use the CPSW Egress Packet Operations Inter VLAN Routing sub functions. If the packet was destined for the host or is destined to any port without any errors, but matches a clasifier that has a programmed egress opcode, it will be forwarded to the destination ports where the destination ports will use the thier egress opcode entry to modify the packet. InterVLAN Routing and mirroring need to be understood, they are orthogonal functions. Care must be taken not to violate VLAN rules as this can redirect packets based on classifier matches."]
    #[inline(always)]
    pub const fn ale_egressop(&self) -> &AleEgressop {
        &self.ale_egressop
    }
    #[doc = "0x3e100 - The Policing Config 0 holds the port, frame priority and ONU address index as well as match enables for port, frame priority and ONU address matching."]
    #[inline(always)]
    pub const fn ale_policecfg0(&self) -> &AlePolicecfg0 {
        &self.ale_policecfg0
    }
    #[doc = "0x3e104 - The Policing Config 1 holds the match enable/match index for the L2 Destination and L2 source addresses"]
    #[inline(always)]
    pub const fn ale_policecfg1(&self) -> &AlePolicecfg1 {
        &self.ale_policecfg1
    }
    #[doc = "0x3e108 - The Policing Config 2 holds the match enable/match index for the Outer VLAN and Inner VLAN addresses"]
    #[inline(always)]
    pub const fn ale_policecfg2(&self) -> &AlePolicecfg2 {
        &self.ale_policecfg2
    }
    #[doc = "0x3e10c - The Policing Config 3 holds the match enable/match index for the Ether Type and IP Source address"]
    #[inline(always)]
    pub const fn ale_policecfg3(&self) -> &AlePolicecfg3 {
        &self.ale_policecfg3
    }
    #[doc = "0x3e110 - The Policing Config 4 holds the match enable/match index for the IP Destination address"]
    #[inline(always)]
    pub const fn ale_policecfg4(&self) -> &AlePolicecfg4 {
        &self.ale_policecfg4
    }
    #[doc = "0x3e118 - The PIR counter is a 37 bit internal counter where ~ipir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED at LUT time. If the counter is negative the packet will be marked RED, else it can be YELLOW or GREEN based on the CIR counter. If only this counter is used (aka cir_idle_inc_val==0) Packet are marked RED or GREEN based on PIR counter only."]
    #[inline(always)]
    pub const fn ale_policecfg6(&self) -> &AlePolicecfg6 {
        &self.ale_policecfg6
    }
    #[doc = "0x3e11c - The CIR counter is a 37 bit internal counter where ~icir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED or YELLOW at LUT time. If the counter is positive the packet will be marked GREEN, else it can be YELLOW or RED based on the PIR counter. If only this counter is used (aka pir_idle_inc_val==0) Packet are marked YELLOW or GREEN based on CIR counter only."]
    #[inline(always)]
    pub const fn ale_policecfg7(&self) -> &AlePolicecfg7 {
        &self.ale_policecfg7
    }
    #[doc = "0x3e120 - The Policing Table Control is used to read or write the selected policing/classifier entry. The selected policing/classifier entry is only read or written after this register is written based on the value of the ~iwrite_enable bit."]
    #[inline(always)]
    pub const fn ale_policetblctl(&self) -> &AlePolicetblctl {
        &self.ale_policetblctl
    }
    #[doc = "0x3e124 - The Control Enables color marking as well as internal ALE packet dropping rules."]
    #[inline(always)]
    pub const fn ale_policecontrol(&self) -> &AlePolicecontrol {
        &self.ale_policecontrol
    }
    #[doc = "0x3e128 - The Policing Test Control enables the ability to determine which policing entry has been hit and whether they reported a red or yellow rate condition."]
    #[inline(always)]
    pub const fn ale_policetestctl(&self) -> &AlePolicetestctl {
        &self.ale_policetestctl
    }
    #[doc = "0x3e12c - The policing hit status is a read only register that reads the hit bits of the selected policing/classifier."]
    #[inline(always)]
    pub const fn ale_policehstat(&self) -> &AlePolicehstat {
        &self.ale_policehstat
    }
    #[doc = "0x3e134 - The THREAD Mapping Default Value register is used to set the default thread ID when no classifier is matched,"]
    #[inline(always)]
    pub const fn ale_threadmapdef(&self) -> &AleThreadmapdef {
        &self.ale_threadmapdef
    }
    #[doc = "0x3e138 - The THREAD Mapping Control register allows the highest matched classifier to return a particular thread ID for traffic sent to the host. This allows particular classifier matched traffic to be placed an a particular hosts queue."]
    #[inline(always)]
    pub const fn ale_threadmapctl(&self) -> &AleThreadmapctl {
        &self.ale_threadmapctl
    }
    #[doc = "0x3e13c - The THREAD Mapping Value register is used to set the thread ID for a particular classifier entry."]
    #[inline(always)]
    pub const fn ale_threadmapval(&self) -> &AleThreadmapval {
        &self.ale_threadmapval
    }
    #[doc = "0x3f000 - Revision parameters"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x3f008 - ECC Vector Register"]
    #[inline(always)]
    pub const fn vector(&self) -> &Vector {
        &self.vector
    }
    #[doc = "0x3f00c - Misc Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x3f010 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_0(&self) -> &EccReservedSvbus0 {
        &self.ecc_reserved_svbus_0
    }
    #[doc = "0x3f014 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_1(&self) -> &EccReservedSvbus1 {
        &self.ecc_reserved_svbus_1
    }
    #[doc = "0x3f018 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_2(&self) -> &EccReservedSvbus2 {
        &self.ecc_reserved_svbus_2
    }
    #[doc = "0x3f01c - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_3(&self) -> &EccReservedSvbus3 {
        &self.ecc_reserved_svbus_3
    }
    #[doc = "0x3f020 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_4(&self) -> &EccReservedSvbus4 {
        &self.ecc_reserved_svbus_4
    }
    #[doc = "0x3f024 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_5(&self) -> &EccReservedSvbus5 {
        &self.ecc_reserved_svbus_5
    }
    #[doc = "0x3f028 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_6(&self) -> &EccReservedSvbus6 {
        &self.ecc_reserved_svbus_6
    }
    #[doc = "0x3f02c - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_reserved_svbus_7(&self) -> &EccReservedSvbus7 {
        &self.ecc_reserved_svbus_7
    }
    #[doc = "0x3f03c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_sec_eoi_reg(&self) -> &EccSecEoiReg {
        &self.ecc_sec_eoi_reg
    }
    #[doc = "0x3f040 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_sec_status_reg0(&self) -> &EccSecStatusReg0 {
        &self.ecc_sec_status_reg0
    }
    #[doc = "0x3f080 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_sec_enable_set_reg0(&self) -> &EccSecEnableSetReg0 {
        &self.ecc_sec_enable_set_reg0
    }
    #[doc = "0x3f0c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_sec_enable_clr_reg0(&self) -> &EccSecEnableClrReg0 {
        &self.ecc_sec_enable_clr_reg0
    }
    #[doc = "0x3f13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_ded_eoi_reg(&self) -> &EccDedEoiReg {
        &self.ecc_ded_eoi_reg
    }
    #[doc = "0x3f140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_ded_status_reg0(&self) -> &EccDedStatusReg0 {
        &self.ecc_ded_status_reg0
    }
    #[doc = "0x3f180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_ded_enable_set_reg0(&self) -> &EccDedEnableSetReg0 {
        &self.ecc_ded_enable_set_reg0
    }
    #[doc = "0x3f1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_ded_enable_clr_reg0(&self) -> &EccDedEnableClrReg0 {
        &self.ecc_ded_enable_clr_reg0
    }
    #[doc = "0x3f200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn aggr_enable_set(&self) -> &AggrEnableSet {
        &self.aggr_enable_set
    }
    #[doc = "0x3f204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn aggr_enable_clr(&self) -> &AggrEnableClr {
        &self.aggr_enable_clr
    }
    #[doc = "0x3f208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn aggr_status_set(&self) -> &AggrStatusSet {
        &self.aggr_status_set
    }
    #[doc = "0x3f20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn aggr_status_clr(&self) -> &AggrStatusClr {
        &self.aggr_status_clr
    }
}
#[doc = "CPSW_NUSS_IDVER_REG (rw) register accessor: ID Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nuss_idver_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nuss_idver_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_idver_reg`]
module"]
#[doc(alias = "CPSW_NUSS_IDVER_REG")]
pub type CpswNussIdverReg = crate::Reg<cpsw_nuss_idver_reg::CpswNussIdverRegSpec>;
#[doc = "ID Version Register"]
pub mod cpsw_nuss_idver_reg;
#[doc = "SS_SYNCE_COUNT_REG (rw) register accessor: SS SYNCE Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_synce_count_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_synce_count_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_synce_count_reg`]
module"]
#[doc(alias = "SS_SYNCE_COUNT_REG")]
pub type SsSynceCountReg = crate::Reg<ss_synce_count_reg::SsSynceCountRegSpec>;
#[doc = "SS SYNCE Count Register"]
pub mod ss_synce_count_reg;
#[doc = "SS_SYNCE_MUX_REG (rw) register accessor: SS Synce Mux Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_synce_mux_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_synce_mux_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_synce_mux_reg`]
module"]
#[doc(alias = "SS_SYNCE_MUX_REG")]
pub type SsSynceMuxReg = crate::Reg<ss_synce_mux_reg::SsSynceMuxRegSpec>;
#[doc = "SS Synce Mux Register"]
pub mod ss_synce_mux_reg;
#[doc = "SS_CONTROL_REG (rw) register accessor: SS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_control_reg`]
module"]
#[doc(alias = "SS_CONTROL_REG")]
pub type SsControlReg = crate::Reg<ss_control_reg::SsControlRegSpec>;
#[doc = "SS Control Register"]
pub mod ss_control_reg;
#[doc = "SS_INT_CONTROL_REG (rw) register accessor: SS Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_int_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_int_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_int_control_reg`]
module"]
#[doc(alias = "SS_INT_CONTROL_REG")]
pub type SsIntControlReg = crate::Reg<ss_int_control_reg::SsIntControlRegSpec>;
#[doc = "SS Interrupt Control Register"]
pub mod ss_int_control_reg;
#[doc = "SS_STATUS_REG (rw) register accessor: SS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_status_reg`]
module"]
#[doc(alias = "SS_STATUS_REG")]
pub type SsStatusReg = crate::Reg<ss_status_reg::SsStatusRegSpec>;
#[doc = "SS Status Register"]
pub mod ss_status_reg;
#[doc = "SUBSYSTEM_CONFIG_REG (rw) register accessor: Subsystem Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`subsystem_config_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsystem_config_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsystem_config_reg`]
module"]
#[doc(alias = "SUBSYSTEM_CONFIG_REG")]
pub type SubsystemConfigReg = crate::Reg<subsystem_config_reg::SubsystemConfigRegSpec>;
#[doc = "Subsystem Configuration Register"]
pub mod subsystem_config_reg;
#[doc = "RGMII1_STATUS_REG (rw) register accessor: RGMII1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii1_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgmii1_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii1_status_reg`]
module"]
#[doc(alias = "RGMII1_STATUS_REG")]
pub type Rgmii1StatusReg = crate::Reg<rgmii1_status_reg::Rgmii1StatusRegSpec>;
#[doc = "RGMII1 Status Register"]
pub mod rgmii1_status_reg;
#[doc = "MDIO_MDIO_VERSION_REG (rw) register accessor: MDIO Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_mdio_version_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_mdio_version_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_mdio_version_reg`]
module"]
#[doc(alias = "MDIO_MDIO_VERSION_REG")]
pub type MdioMdioVersionReg = crate::Reg<mdio_mdio_version_reg::MdioMdioVersionRegSpec>;
#[doc = "MDIO Version Register"]
pub mod mdio_mdio_version_reg;
#[doc = "MDIO_CONTROL_REG (rw) register accessor: MDIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_control_reg`]
module"]
#[doc(alias = "MDIO_CONTROL_REG")]
pub type MdioControlReg = crate::Reg<mdio_control_reg::MdioControlRegSpec>;
#[doc = "MDIO Control Register"]
pub mod mdio_control_reg;
#[doc = "MDIO_ALIVE_REG (rw) register accessor: MDIO Alive Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_alive_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_alive_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_alive_reg`]
module"]
#[doc(alias = "MDIO_ALIVE_REG")]
pub type MdioAliveReg = crate::Reg<mdio_alive_reg::MdioAliveRegSpec>;
#[doc = "MDIO Alive Register"]
pub mod mdio_alive_reg;
#[doc = "MDIO_LINK_REG (rw) register accessor: MDIO Link Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_link_reg`]
module"]
#[doc(alias = "MDIO_LINK_REG")]
pub type MdioLinkReg = crate::Reg<mdio_link_reg::MdioLinkRegSpec>;
#[doc = "MDIO Link Register"]
pub mod mdio_link_reg;
#[doc = "MDIO_LINK_INT_RAW_REG (rw) register accessor: MDIO Link Interrupt Raw Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_link_int_raw_reg`]
module"]
#[doc(alias = "MDIO_LINK_INT_RAW_REG")]
pub type MdioLinkIntRawReg = crate::Reg<mdio_link_int_raw_reg::MdioLinkIntRawRegSpec>;
#[doc = "MDIO Link Interrupt Raw Register"]
pub mod mdio_link_int_raw_reg;
#[doc = "MDIO_LINK_INT_MASKED_REG (rw) register accessor: MDIO Link Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_link_int_masked_reg`]
module"]
#[doc(alias = "MDIO_LINK_INT_MASKED_REG")]
pub type MdioLinkIntMaskedReg = crate::Reg<mdio_link_int_masked_reg::MdioLinkIntMaskedRegSpec>;
#[doc = "MDIO Link Interrupt Masked Register"]
pub mod mdio_link_int_masked_reg;
#[doc = "MDIO_LINK_INT_MASK_SET_REG (rw) register accessor: MDIO Link Interrupt Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_mask_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_mask_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_link_int_mask_set_reg`]
module"]
#[doc(alias = "MDIO_LINK_INT_MASK_SET_REG")]
pub type MdioLinkIntMaskSetReg = crate::Reg<mdio_link_int_mask_set_reg::MdioLinkIntMaskSetRegSpec>;
#[doc = "MDIO Link Interrupt Mask Set Register"]
pub mod mdio_link_int_mask_set_reg;
#[doc = "MDIO_LINK_INT_MASK_CLEAR_REG (rw) register accessor: MDIO Link Interrupt Mask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_mask_clear_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_mask_clear_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_link_int_mask_clear_reg`]
module"]
#[doc(alias = "MDIO_LINK_INT_MASK_CLEAR_REG")]
pub type MdioLinkIntMaskClearReg =
    crate::Reg<mdio_link_int_mask_clear_reg::MdioLinkIntMaskClearRegSpec>;
#[doc = "MDIO Link Interrupt Mask Clear Register"]
pub mod mdio_link_int_mask_clear_reg;
#[doc = "MDIO_USER_INT_RAW_REG (rw) register accessor: MDIO User Interrupt Raw Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_int_raw_reg`]
module"]
#[doc(alias = "MDIO_USER_INT_RAW_REG")]
pub type MdioUserIntRawReg = crate::Reg<mdio_user_int_raw_reg::MdioUserIntRawRegSpec>;
#[doc = "MDIO User Interrupt Raw Register"]
pub mod mdio_user_int_raw_reg;
#[doc = "MDIO_USER_INT_MASKED_REG (rw) register accessor: MDIO User Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_int_masked_reg`]
module"]
#[doc(alias = "MDIO_USER_INT_MASKED_REG")]
pub type MdioUserIntMaskedReg = crate::Reg<mdio_user_int_masked_reg::MdioUserIntMaskedRegSpec>;
#[doc = "MDIO User Interrupt Masked Register"]
pub mod mdio_user_int_masked_reg;
#[doc = "MDIO_USER_INT_MASK_SET_REG (rw) register accessor: MDIO User Interrupt Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_mask_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_mask_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_int_mask_set_reg`]
module"]
#[doc(alias = "MDIO_USER_INT_MASK_SET_REG")]
pub type MdioUserIntMaskSetReg = crate::Reg<mdio_user_int_mask_set_reg::MdioUserIntMaskSetRegSpec>;
#[doc = "MDIO User Interrupt Mask Set Register"]
pub mod mdio_user_int_mask_set_reg;
#[doc = "MDIO_USER_INT_MASK_CLEAR_REG (rw) register accessor: MDIO User Interrupt Mask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_mask_clear_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_mask_clear_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_int_mask_clear_reg`]
module"]
#[doc(alias = "MDIO_USER_INT_MASK_CLEAR_REG")]
pub type MdioUserIntMaskClearReg =
    crate::Reg<mdio_user_int_mask_clear_reg::MdioUserIntMaskClearRegSpec>;
#[doc = "MDIO User Interrupt Mask Clear Register"]
pub mod mdio_user_int_mask_clear_reg;
#[doc = "MDIO_MANUAL_IF_REG (rw) register accessor: MDIO Manual Interface Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_manual_if_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_manual_if_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_manual_if_reg`]
module"]
#[doc(alias = "MDIO_MANUAL_IF_REG")]
pub type MdioManualIfReg = crate::Reg<mdio_manual_if_reg::MdioManualIfRegSpec>;
#[doc = "MDIO Manual Interface Register"]
pub mod mdio_manual_if_reg;
#[doc = "MDIO_POLL_REG (rw) register accessor: MDIO Poll Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_poll_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_poll_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_poll_reg`]
module"]
#[doc(alias = "MDIO_POLL_REG")]
pub type MdioPollReg = crate::Reg<mdio_poll_reg::MdioPollRegSpec>;
#[doc = "MDIO Poll Register"]
pub mod mdio_poll_reg;
#[doc = "MDIO_POLL_EN_REG (rw) register accessor: MDIO Poll Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_poll_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_poll_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_poll_en_reg`]
module"]
#[doc(alias = "MDIO_POLL_EN_REG")]
pub type MdioPollEnReg = crate::Reg<mdio_poll_en_reg::MdioPollEnRegSpec>;
#[doc = "MDIO Poll Enable Register"]
pub mod mdio_poll_en_reg;
#[doc = "MDIO_CLAUS45_REG (rw) register accessor: MDIO Clause45 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_claus45_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_claus45_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_claus45_reg`]
module"]
#[doc(alias = "MDIO_CLAUS45_REG")]
pub type MdioClaus45Reg = crate::Reg<mdio_claus45_reg::MdioClaus45RegSpec>;
#[doc = "MDIO Clause45 Register"]
pub mod mdio_claus45_reg;
#[doc = "MDIO_USER_ADDR0_REG (rw) register accessor: MDIO Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_addr0_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_addr0_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_addr0_reg`]
module"]
#[doc(alias = "MDIO_USER_ADDR0_REG")]
pub type MdioUserAddr0Reg = crate::Reg<mdio_user_addr0_reg::MdioUserAddr0RegSpec>;
#[doc = "MDIO Address 0 Register"]
pub mod mdio_user_addr0_reg;
#[doc = "MDIO_USER_ADDR1_REG (rw) register accessor: MDIO Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_addr1_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_addr1_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_user_addr1_reg`]
module"]
#[doc(alias = "MDIO_USER_ADDR1_REG")]
pub type MdioUserAddr1Reg = crate::Reg<mdio_user_addr1_reg::MdioUserAddr1RegSpec>;
#[doc = "MDIO Address 1 Register"]
pub mod mdio_user_addr1_reg;
#[doc = "USER_GROUP0_USER_ACCESS_REG (rw) register accessor: MDIO User Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group0_user_access_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group0_user_access_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_group0_user_access_reg`]
module"]
#[doc(alias = "USER_GROUP0_USER_ACCESS_REG")]
pub type UserGroup0UserAccessReg =
    crate::Reg<user_group0_user_access_reg::UserGroup0UserAccessRegSpec>;
#[doc = "MDIO User Access Register"]
pub mod user_group0_user_access_reg;
#[doc = "USER_GROUP0_USER_PHY_SEL_REG (rw) register accessor: MDIO User PHY Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group0_user_phy_sel_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group0_user_phy_sel_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_group0_user_phy_sel_reg`]
module"]
#[doc(alias = "USER_GROUP0_USER_PHY_SEL_REG")]
pub type UserGroup0UserPhySelReg =
    crate::Reg<user_group0_user_phy_sel_reg::UserGroup0UserPhySelRegSpec>;
#[doc = "MDIO User PHY Select Register"]
pub mod user_group0_user_phy_sel_reg;
#[doc = "USER_GROUP1_USER_ACCESS_REG (rw) register accessor: MDIO User Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group1_user_access_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group1_user_access_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_group1_user_access_reg`]
module"]
#[doc(alias = "USER_GROUP1_USER_ACCESS_REG")]
pub type UserGroup1UserAccessReg =
    crate::Reg<user_group1_user_access_reg::UserGroup1UserAccessRegSpec>;
#[doc = "MDIO User Access Register"]
pub mod user_group1_user_access_reg;
#[doc = "USER_GROUP1_USER_PHY_SEL_REG (rw) register accessor: MDIO User PHY Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group1_user_phy_sel_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group1_user_phy_sel_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_group1_user_phy_sel_reg`]
module"]
#[doc(alias = "USER_GROUP1_USER_PHY_SEL_REG")]
pub type UserGroup1UserPhySelReg =
    crate::Reg<user_group1_user_phy_sel_reg::UserGroup1UserPhySelRegSpec>;
#[doc = "MDIO User PHY Select Register"]
pub mod user_group1_user_phy_sel_reg;
#[doc = "REGS_INT_SS_C0_TH_THRESH_PULSE_EN_REG (rw) register accessor: Core 0 THost Threshold Pulse Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_thresh_pulse_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_thresh_pulse_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_th_thresh_pulse_en_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_TH_THRESH_PULSE_EN_REG")]
pub type RegsIntSsC0ThThreshPulseEnReg =
    crate::Reg<regs_int_ss_c0_th_thresh_pulse_en_reg::RegsIntSsC0ThThreshPulseEnRegSpec>;
#[doc = "Core 0 THost Threshold Pulse Interrupt Enable Register"]
pub mod regs_int_ss_c0_th_thresh_pulse_en_reg;
#[doc = "REGS_INT_SS_C0_TH_PULSE_EN_REG (rw) register accessor: Core 0 THost Pulse Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_pulse_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_pulse_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_th_pulse_en_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_TH_PULSE_EN_REG")]
pub type RegsIntSsC0ThPulseEnReg =
    crate::Reg<regs_int_ss_c0_th_pulse_en_reg::RegsIntSsC0ThPulseEnRegSpec>;
#[doc = "Core 0 THost Pulse Interrupt Enable Register"]
pub mod regs_int_ss_c0_th_pulse_en_reg;
#[doc = "REGS_INT_SS_C0_FH_PULSE_EN_REG (rw) register accessor: Core 0 FHost Pulse Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_fh_pulse_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_fh_pulse_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_fh_pulse_en_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_FH_PULSE_EN_REG")]
pub type RegsIntSsC0FhPulseEnReg =
    crate::Reg<regs_int_ss_c0_fh_pulse_en_reg::RegsIntSsC0FhPulseEnRegSpec>;
#[doc = "Core 0 FHost Pulse Interrupt Enable Register"]
pub mod regs_int_ss_c0_fh_pulse_en_reg;
#[doc = "REGS_INT_SS_C0_MISC_EN_REG (rw) register accessor: Core 0 Misc Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_misc_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_misc_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_misc_en_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_MISC_EN_REG")]
pub type RegsIntSsC0MiscEnReg = crate::Reg<regs_int_ss_c0_misc_en_reg::RegsIntSsC0MiscEnRegSpec>;
#[doc = "Core 0 Misc Interrupt Enable Register"]
pub mod regs_int_ss_c0_misc_en_reg;
#[doc = "REGS_INT_SS_C0_TH_THRESH_PULSE_STATUS_REG (rw) register accessor: THost Threshold Pulse Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_thresh_pulse_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_thresh_pulse_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_th_thresh_pulse_status_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_TH_THRESH_PULSE_STATUS_REG")]
pub type RegsIntSsC0ThThreshPulseStatusReg =
    crate::Reg<regs_int_ss_c0_th_thresh_pulse_status_reg::RegsIntSsC0ThThreshPulseStatusRegSpec>;
#[doc = "THost Threshold Pulse Interrupt Status Register"]
pub mod regs_int_ss_c0_th_thresh_pulse_status_reg;
#[doc = "REGS_INT_SS_C0_TH_PULSE_STATUS_REG (rw) register accessor: THost Pulse Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_pulse_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_pulse_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_th_pulse_status_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_TH_PULSE_STATUS_REG")]
pub type RegsIntSsC0ThPulseStatusReg =
    crate::Reg<regs_int_ss_c0_th_pulse_status_reg::RegsIntSsC0ThPulseStatusRegSpec>;
#[doc = "THost Pulse Interrupt Status Register"]
pub mod regs_int_ss_c0_th_pulse_status_reg;
#[doc = "REGS_INT_SS_C0_FH_PULSE_STATUS_REG (rw) register accessor: FHost Pulse Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_fh_pulse_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_fh_pulse_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_fh_pulse_status_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_FH_PULSE_STATUS_REG")]
pub type RegsIntSsC0FhPulseStatusReg =
    crate::Reg<regs_int_ss_c0_fh_pulse_status_reg::RegsIntSsC0FhPulseStatusRegSpec>;
#[doc = "FHost Pulse Interrupt Status Register"]
pub mod regs_int_ss_c0_fh_pulse_status_reg;
#[doc = "REGS_INT_SS_C0_MISC_STATUS_REG (rw) register accessor: Misc Interrupt Status Register - Set bits in this register indicate that an enabled interrupt is asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_misc_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_misc_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_misc_status_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_MISC_STATUS_REG")]
pub type RegsIntSsC0MiscStatusReg =
    crate::Reg<regs_int_ss_c0_misc_status_reg::RegsIntSsC0MiscStatusRegSpec>;
#[doc = "Misc Interrupt Status Register - Set bits in this register indicate that an enabled interrupt is asserted"]
pub mod regs_int_ss_c0_misc_status_reg;
#[doc = "REGS_INT_SS_C0_TH_IMAX_REG (rw) register accessor: Core 0 THost Interrupt Max Register Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_imax_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_imax_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_th_imax_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_TH_IMAX_REG")]
pub type RegsIntSsC0ThImaxReg = crate::Reg<regs_int_ss_c0_th_imax_reg::RegsIntSsC0ThImaxRegSpec>;
#[doc = "Core 0 THost Interrupt Max Register Register"]
pub mod regs_int_ss_c0_th_imax_reg;
#[doc = "REGS_INT_SS_C0_FH_IMAX_REG (rw) register accessor: Core 0 FHost Interrupt Max Register Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_fh_imax_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_fh_imax_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_int_ss_c0_fh_imax_reg`]
module"]
#[doc(alias = "REGS_INT_SS_C0_FH_IMAX_REG")]
pub type RegsIntSsC0FhImaxReg = crate::Reg<regs_int_ss_c0_fh_imax_reg::RegsIntSsC0FhImaxRegSpec>;
#[doc = "Core 0 FHost Interrupt Max Register Register"]
pub mod regs_int_ss_c0_fh_imax_reg;
#[doc = "CPSW_NC_CPSW_ID_VER_REG (rw) register accessor: CPSW ID Version\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cpsw_id_ver_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cpsw_id_ver_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cpsw_id_ver_reg`]
module"]
#[doc(alias = "CPSW_NC_CPSW_ID_VER_REG")]
pub type CpswNcCpswIdVerReg = crate::Reg<cpsw_nc_cpsw_id_ver_reg::CpswNcCpswIdVerRegSpec>;
#[doc = "CPSW ID Version"]
pub mod cpsw_nc_cpsw_id_ver_reg;
#[doc = "CPSW_NC_CONTROL_REG (rw) register accessor: CPSW Switch Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_control_reg`]
module"]
#[doc(alias = "CPSW_NC_CONTROL_REG")]
pub type CpswNcControlReg = crate::Reg<cpsw_nc_control_reg::CpswNcControlRegSpec>;
#[doc = "CPSW Switch Control"]
pub mod cpsw_nc_control_reg;
#[doc = "CPSW_NC_EM_CONTROL_REG (rw) register accessor: CPSW Emulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_em_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_em_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_em_control_reg`]
module"]
#[doc(alias = "CPSW_NC_EM_CONTROL_REG")]
pub type CpswNcEmControlReg = crate::Reg<cpsw_nc_em_control_reg::CpswNcEmControlRegSpec>;
#[doc = "CPSW Emulation Control"]
pub mod cpsw_nc_em_control_reg;
#[doc = "CPSW_NC_STAT_PORT_EN_REG (rw) register accessor: CPSW Statistics Port Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_port_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_port_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_port_en_reg`]
module"]
#[doc(alias = "CPSW_NC_STAT_PORT_EN_REG")]
pub type CpswNcStatPortEnReg = crate::Reg<cpsw_nc_stat_port_en_reg::CpswNcStatPortEnRegSpec>;
#[doc = "CPSW Statistics Port Enable"]
pub mod cpsw_nc_stat_port_en_reg;
#[doc = "CPSW_NC_PTYPE_REG (rw) register accessor: CPSW Transmit Priority Type\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_ptype_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_ptype_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_ptype_reg`]
module"]
#[doc(alias = "CPSW_NC_PTYPE_REG")]
pub type CpswNcPtypeReg = crate::Reg<cpsw_nc_ptype_reg::CpswNcPtypeRegSpec>;
#[doc = "CPSW Transmit Priority Type"]
pub mod cpsw_nc_ptype_reg;
#[doc = "CPSW_NC_SOFT_IDLE_REG (rw) register accessor: CPSW Software Idle\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_soft_idle_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_soft_idle_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_soft_idle_reg`]
module"]
#[doc(alias = "CPSW_NC_SOFT_IDLE_REG")]
pub type CpswNcSoftIdleReg = crate::Reg<cpsw_nc_soft_idle_reg::CpswNcSoftIdleRegSpec>;
#[doc = "CPSW Software Idle"]
pub mod cpsw_nc_soft_idle_reg;
#[doc = "CPSW_NC_THRU_RATE_REG (rw) register accessor: CPSW Thru Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_thru_rate_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_thru_rate_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_thru_rate_reg`]
module"]
#[doc(alias = "CPSW_NC_THRU_RATE_REG")]
pub type CpswNcThruRateReg = crate::Reg<cpsw_nc_thru_rate_reg::CpswNcThruRateRegSpec>;
#[doc = "CPSW Thru Rate"]
pub mod cpsw_nc_thru_rate_reg;
#[doc = "CPSW_NC_GAP_THRESH_REG (rw) register accessor: CPSW Transmit FIFO Short Gap Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_gap_thresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_gap_thresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_gap_thresh_reg`]
module"]
#[doc(alias = "CPSW_NC_GAP_THRESH_REG")]
pub type CpswNcGapThreshReg = crate::Reg<cpsw_nc_gap_thresh_reg::CpswNcGapThreshRegSpec>;
#[doc = "CPSW Transmit FIFO Short Gap Threshold"]
pub mod cpsw_nc_gap_thresh_reg;
#[doc = "CPSW_NC_EEE_PRESCALE_REG (rw) register accessor: CPSW Energy Efficient Ethernet Prescale Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eee_prescale_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eee_prescale_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eee_prescale_reg`]
module"]
#[doc(alias = "CPSW_NC_EEE_PRESCALE_REG")]
pub type CpswNcEeePrescaleReg = crate::Reg<cpsw_nc_eee_prescale_reg::CpswNcEeePrescaleRegSpec>;
#[doc = "CPSW Energy Efficient Ethernet Prescale Value"]
pub mod cpsw_nc_eee_prescale_reg;
#[doc = "CPSW_NC_TX_G_OFLOW_THRESH_SET_REG (rw) register accessor: CPSW PFC Tx Global Out Flow Threshold Set\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_oflow_thresh_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_oflow_thresh_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_oflow_thresh_set_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_OFLOW_THRESH_SET_REG")]
pub type CpswNcTxGOflowThreshSetReg =
    crate::Reg<cpsw_nc_tx_g_oflow_thresh_set_reg::CpswNcTxGOflowThreshSetRegSpec>;
#[doc = "CPSW PFC Tx Global Out Flow Threshold Set"]
pub mod cpsw_nc_tx_g_oflow_thresh_set_reg;
#[doc = "CPSW_NC_TX_G_OFLOW_THRESH_CLR_REG (rw) register accessor: CPSW PFC Tx Global Out Flow Threshold Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_oflow_thresh_clr_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_oflow_thresh_clr_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_oflow_thresh_clr_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_OFLOW_THRESH_CLR_REG")]
pub type CpswNcTxGOflowThreshClrReg =
    crate::Reg<cpsw_nc_tx_g_oflow_thresh_clr_reg::CpswNcTxGOflowThreshClrRegSpec>;
#[doc = "CPSW PFC Tx Global Out Flow Threshold Clear"]
pub mod cpsw_nc_tx_g_oflow_thresh_clr_reg;
#[doc = "CPSW_NC_TX_G_BUF_THRESH_SET_L_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_buf_thresh_set_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_buf_thresh_set_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_buf_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_BUF_THRESH_SET_L_REG")]
pub type CpswNcTxGBufThreshSetLReg =
    crate::Reg<cpsw_nc_tx_g_buf_thresh_set_l_reg::CpswNcTxGBufThreshSetLRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Set Low"]
pub mod cpsw_nc_tx_g_buf_thresh_set_l_reg;
#[doc = "CPSW_NC_TX_G_BUF_THRESH_SET_H_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Set High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_buf_thresh_set_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_buf_thresh_set_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_buf_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_BUF_THRESH_SET_H_REG")]
pub type CpswNcTxGBufThreshSetHReg =
    crate::Reg<cpsw_nc_tx_g_buf_thresh_set_h_reg::CpswNcTxGBufThreshSetHRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Set High"]
pub mod cpsw_nc_tx_g_buf_thresh_set_h_reg;
#[doc = "CPSW_NC_TX_G_BUF_THRESH_CLR_L_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Clear Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_buf_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_BUF_THRESH_CLR_L_REG")]
pub type CpswNcTxGBufThreshClrLReg =
    crate::Reg<cpsw_nc_tx_g_buf_thresh_clr_l_reg::CpswNcTxGBufThreshClrLRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear Low"]
pub mod cpsw_nc_tx_g_buf_thresh_clr_l_reg;
#[doc = "CPSW_NC_TX_G_BUF_THRESH_CLR_H_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Clear High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_buf_thresh_clr_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_buf_thresh_clr_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_g_buf_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_G_BUF_THRESH_CLR_H_REG")]
pub type CpswNcTxGBufThreshClrHReg =
    crate::Reg<cpsw_nc_tx_g_buf_thresh_clr_h_reg::CpswNcTxGBufThreshClrHRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear High"]
pub mod cpsw_nc_tx_g_buf_thresh_clr_h_reg;
#[doc = "CPSW_NC_VLAN_LTYPE_REG (rw) register accessor: VLAN Length/type\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_vlan_ltype_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_vlan_ltype_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_vlan_ltype_reg`]
module"]
#[doc(alias = "CPSW_NC_VLAN_LTYPE_REG")]
pub type CpswNcVlanLtypeReg = crate::Reg<cpsw_nc_vlan_ltype_reg::CpswNcVlanLtypeRegSpec>;
#[doc = "VLAN Length/type"]
pub mod cpsw_nc_vlan_ltype_reg;
#[doc = "CPSW_NC_EST_TS_DOMAIN_REG (rw) register accessor: Enhanced Scheduled Traffic Host Event Domain\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_ts_domain_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_ts_domain_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_ts_domain_reg`]
module"]
#[doc(alias = "CPSW_NC_EST_TS_DOMAIN_REG")]
pub type CpswNcEstTsDomainReg = crate::Reg<cpsw_nc_est_ts_domain_reg::CpswNcEstTsDomainRegSpec>;
#[doc = "Enhanced Scheduled Traffic Host Event Domain"]
pub mod cpsw_nc_est_ts_domain_reg;
#[doc = "CPSW_NC_TX_PRI0_MAXLEN_REG (rw) register accessor: Transmit Priority 0 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri0_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri0_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri0_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI0_MAXLEN_REG")]
pub type CpswNcTxPri0MaxlenReg = crate::Reg<cpsw_nc_tx_pri0_maxlen_reg::CpswNcTxPri0MaxlenRegSpec>;
#[doc = "Transmit Priority 0 Maximum Length"]
pub mod cpsw_nc_tx_pri0_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI1_MAXLEN_REG (rw) register accessor: Transmit Priority 1 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri1_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri1_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri1_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI1_MAXLEN_REG")]
pub type CpswNcTxPri1MaxlenReg = crate::Reg<cpsw_nc_tx_pri1_maxlen_reg::CpswNcTxPri1MaxlenRegSpec>;
#[doc = "Transmit Priority 1 Maximum Length"]
pub mod cpsw_nc_tx_pri1_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI2_MAXLEN_REG (rw) register accessor: Transmit Priority 2 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri2_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri2_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri2_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI2_MAXLEN_REG")]
pub type CpswNcTxPri2MaxlenReg = crate::Reg<cpsw_nc_tx_pri2_maxlen_reg::CpswNcTxPri2MaxlenRegSpec>;
#[doc = "Transmit Priority 2 Maximum Length"]
pub mod cpsw_nc_tx_pri2_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI3_MAXLEN_REG (rw) register accessor: Transmit Priority 3 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri3_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri3_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri3_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI3_MAXLEN_REG")]
pub type CpswNcTxPri3MaxlenReg = crate::Reg<cpsw_nc_tx_pri3_maxlen_reg::CpswNcTxPri3MaxlenRegSpec>;
#[doc = "Transmit Priority 3 Maximum Length"]
pub mod cpsw_nc_tx_pri3_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI4_MAXLEN_REG (rw) register accessor: Transmit Priority 4 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri4_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri4_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri4_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI4_MAXLEN_REG")]
pub type CpswNcTxPri4MaxlenReg = crate::Reg<cpsw_nc_tx_pri4_maxlen_reg::CpswNcTxPri4MaxlenRegSpec>;
#[doc = "Transmit Priority 4 Maximum Length"]
pub mod cpsw_nc_tx_pri4_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI5_MAXLEN_REG (rw) register accessor: Transmit Priority 5 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri5_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri5_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri5_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI5_MAXLEN_REG")]
pub type CpswNcTxPri5MaxlenReg = crate::Reg<cpsw_nc_tx_pri5_maxlen_reg::CpswNcTxPri5MaxlenRegSpec>;
#[doc = "Transmit Priority 5 Maximum Length"]
pub mod cpsw_nc_tx_pri5_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI6_MAXLEN_REG (rw) register accessor: Transmit Priority 6 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri6_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri6_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri6_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI6_MAXLEN_REG")]
pub type CpswNcTxPri6MaxlenReg = crate::Reg<cpsw_nc_tx_pri6_maxlen_reg::CpswNcTxPri6MaxlenRegSpec>;
#[doc = "Transmit Priority 6 Maximum Length"]
pub mod cpsw_nc_tx_pri6_maxlen_reg;
#[doc = "CPSW_NC_TX_PRI7_MAXLEN_REG (rw) register accessor: Transmit Priority 7 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri7_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri7_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_tx_pri7_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_TX_PRI7_MAXLEN_REG")]
pub type CpswNcTxPri7MaxlenReg = crate::Reg<cpsw_nc_tx_pri7_maxlen_reg::CpswNcTxPri7MaxlenRegSpec>;
#[doc = "Transmit Priority 7 Maximum Length"]
pub mod cpsw_nc_tx_pri7_maxlen_reg;
#[doc = "CPSW_NC_CPPI_P0_CONTROL_REG (rw) register accessor: CPPI Port 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_control_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_CONTROL_REG")]
pub type CpswNcCppiP0ControlReg =
    crate::Reg<cpsw_nc_cppi_p0_control_reg::CpswNcCppiP0ControlRegSpec>;
#[doc = "CPPI Port 0 Control"]
pub mod cpsw_nc_cppi_p0_control_reg;
#[doc = "CPSW_NC_CPPI_P0_FLOW_ID_OFFSET_REG (rw) register accessor: CPPI Port 0 Flow ID Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_flow_id_offset_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_flow_id_offset_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_flow_id_offset_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_FLOW_ID_OFFSET_REG")]
pub type CpswNcCppiP0FlowIdOffsetReg =
    crate::Reg<cpsw_nc_cppi_p0_flow_id_offset_reg::CpswNcCppiP0FlowIdOffsetRegSpec>;
#[doc = "CPPI Port 0 Flow ID Offset"]
pub mod cpsw_nc_cppi_p0_flow_id_offset_reg;
#[doc = "CPSW_NC_CPPI_P0_BLK_CNT_REG (rw) register accessor: CPPI Port 0 FIFO Block Usage Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_blk_cnt_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_blk_cnt_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_blk_cnt_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_BLK_CNT_REG")]
pub type CpswNcCppiP0BlkCntReg = crate::Reg<cpsw_nc_cppi_p0_blk_cnt_reg::CpswNcCppiP0BlkCntRegSpec>;
#[doc = "CPPI Port 0 FIFO Block Usage Count"]
pub mod cpsw_nc_cppi_p0_blk_cnt_reg;
#[doc = "CPSW_NC_CPPI_P0_PORT_VLAN_REG (rw) register accessor: CPPI Port 0 VLAN\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_port_vlan_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_port_vlan_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_port_vlan_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PORT_VLAN_REG")]
pub type CpswNcCppiP0PortVlanReg =
    crate::Reg<cpsw_nc_cppi_p0_port_vlan_reg::CpswNcCppiP0PortVlanRegSpec>;
#[doc = "CPPI Port 0 VLAN"]
pub mod cpsw_nc_cppi_p0_port_vlan_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_PRI_MAP_REG (rw) register accessor: CPPI Port 0 Tx Header Pri to Switch Pri Mapping\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_pri_map_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_pri_map_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_pri_map_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_PRI_MAP_REG")]
pub type CpswNcCppiP0TxPriMapReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_pri_map_reg::CpswNcCppiP0TxPriMapRegSpec>;
#[doc = "CPPI Port 0 Tx Header Pri to Switch Pri Mapping"]
pub mod cpsw_nc_cppi_p0_tx_pri_map_reg;
#[doc = "CPSW_NC_CPPI_P0_PRI_CTL_REG (rw) register accessor: CPPI Port 0 Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_ctl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_ctl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_ctl_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CTL_REG")]
pub type CpswNcCppiP0PriCtlReg = crate::Reg<cpsw_nc_cppi_p0_pri_ctl_reg::CpswNcCppiP0PriCtlRegSpec>;
#[doc = "CPPI Port 0 Priority Control"]
pub mod cpsw_nc_cppi_p0_pri_ctl_reg;
#[doc = "CPSW_NC_CPPI_P0_RX_PRI_MAP_REG (rw) register accessor: CPPI Port 0 RX Pkt Pri to Header Pri Map\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_pri_map_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_pri_map_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_pri_map_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_PRI_MAP_REG")]
pub type CpswNcCppiP0RxPriMapReg =
    crate::Reg<cpsw_nc_cppi_p0_rx_pri_map_reg::CpswNcCppiP0RxPriMapRegSpec>;
#[doc = "CPPI Port 0 RX Pkt Pri to Header Pri Map"]
pub mod cpsw_nc_cppi_p0_rx_pri_map_reg;
#[doc = "CPSW_NC_CPPI_P0_RX_MAXLEN_REG (rw) register accessor: CPPI Port 0 Receive Frame Max Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_MAXLEN_REG")]
pub type CpswNcCppiP0RxMaxlenReg =
    crate::Reg<cpsw_nc_cppi_p0_rx_maxlen_reg::CpswNcCppiP0RxMaxlenRegSpec>;
#[doc = "CPPI Port 0 Receive Frame Max Length"]
pub mod cpsw_nc_cppi_p0_rx_maxlen_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_BLKS_PRI_REG (rw) register accessor: CPPI Port 0 Transmit Block Sub Per Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_blks_pri_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_blks_pri_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_blks_pri_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_BLKS_PRI_REG")]
pub type CpswNcCppiP0TxBlksPriReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_blks_pri_reg::CpswNcCppiP0TxBlksPriRegSpec>;
#[doc = "CPPI Port 0 Transmit Block Sub Per Priority"]
pub mod cpsw_nc_cppi_p0_tx_blks_pri_reg;
#[doc = "CPSW_NC_CPPI_P0_IDLE2LPI_REG (rw) register accessor: Port 0 EEE Idle to LPI counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_idle2lpi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_idle2lpi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_idle2lpi_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_IDLE2LPI_REG")]
pub type CpswNcCppiP0Idle2lpiReg =
    crate::Reg<cpsw_nc_cppi_p0_idle2lpi_reg::CpswNcCppiP0Idle2lpiRegSpec>;
#[doc = "Port 0 EEE Idle to LPI counter"]
pub mod cpsw_nc_cppi_p0_idle2lpi_reg;
#[doc = "CPSW_NC_CPPI_P0_LPI2WAKE_REG (rw) register accessor: Port 0 EEE LPI to wake counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_lpi2wake_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_lpi2wake_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_lpi2wake_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_LPI2WAKE_REG")]
pub type CpswNcCppiP0Lpi2wakeReg =
    crate::Reg<cpsw_nc_cppi_p0_lpi2wake_reg::CpswNcCppiP0Lpi2wakeRegSpec>;
#[doc = "Port 0 EEE LPI to wake counter"]
pub mod cpsw_nc_cppi_p0_lpi2wake_reg;
#[doc = "CPSW_NC_CPPI_P0_EEE_STATUS_REG (rw) register accessor: Port 0 EEE status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_eee_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_eee_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_eee_status_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_EEE_STATUS_REG")]
pub type CpswNcCppiP0EeeStatusReg =
    crate::Reg<cpsw_nc_cppi_p0_eee_status_reg::CpswNcCppiP0EeeStatusRegSpec>;
#[doc = "Port 0 EEE status"]
pub mod cpsw_nc_cppi_p0_eee_status_reg;
#[doc = "CPSW_NC_CPPI_P0_RX_PKTS_PRI_REG (rw) register accessor: CPPI Port Receive Packets per priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_pkts_pri_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_pkts_pri_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_pkts_pri_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_PKTS_PRI_REG")]
pub type CpswNcCppiP0RxPktsPriReg =
    crate::Reg<cpsw_nc_cppi_p0_rx_pkts_pri_reg::CpswNcCppiP0RxPktsPriRegSpec>;
#[doc = "CPPI Port Receive Packets per priority"]
pub mod cpsw_nc_cppi_p0_rx_pkts_pri_reg;
#[doc = "CPSW_NC_CPPI_P0_RX_GAP_REG (rw) register accessor: Port 0 Receive Gap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_gap_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_gap_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_gap_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_GAP_REG")]
pub type CpswNcCppiP0RxGapReg = crate::Reg<cpsw_nc_cppi_p0_rx_gap_reg::CpswNcCppiP0RxGapRegSpec>;
#[doc = "Port 0 Receive Gap Register"]
pub mod cpsw_nc_cppi_p0_rx_gap_reg;
#[doc = "CPSW_NC_CPPI_P0_FIFO_STATUS_REG (rw) register accessor: Port 0 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_fifo_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_fifo_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_fifo_status_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_FIFO_STATUS_REG")]
pub type CpswNcCppiP0FifoStatusReg =
    crate::Reg<cpsw_nc_cppi_p0_fifo_status_reg::CpswNcCppiP0FifoStatusRegSpec>;
#[doc = "Port 0 FIFO Status"]
pub mod cpsw_nc_cppi_p0_fifo_status_reg;
#[doc = "CPSW_NC_CPPI_P0_MAX_BLKS_REG (rw) register accessor: Port 0 FIFO Max Blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_max_blks_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_max_blks_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_max_blks_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_MAX_BLKS_REG")]
pub type CpswNcCppiP0MaxBlksReg =
    crate::Reg<cpsw_nc_cppi_p0_max_blks_reg::CpswNcCppiP0MaxBlksRegSpec>;
#[doc = "Port 0 FIFO Max Blocks"]
pub mod cpsw_nc_cppi_p0_max_blks_reg;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_0 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_0`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_0")]
pub type CpswNcCppiP0RxDscpMapReg0 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_0::CpswNcCppiP0RxDscpMapReg0Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_0;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_1 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_1`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_1")]
pub type CpswNcCppiP0RxDscpMapReg1 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_1::CpswNcCppiP0RxDscpMapReg1Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_1;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_2 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_2`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_2")]
pub type CpswNcCppiP0RxDscpMapReg2 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_2::CpswNcCppiP0RxDscpMapReg2Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_2;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_3 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_3`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_3")]
pub type CpswNcCppiP0RxDscpMapReg3 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_3::CpswNcCppiP0RxDscpMapReg3Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_3;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_4 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_4`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_4")]
pub type CpswNcCppiP0RxDscpMapReg4 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_4::CpswNcCppiP0RxDscpMapReg4Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_4;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_5 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_5`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_5")]
pub type CpswNcCppiP0RxDscpMapReg5 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_5::CpswNcCppiP0RxDscpMapReg5Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_5;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_6 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_6`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_6")]
pub type CpswNcCppiP0RxDscpMapReg6 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_6::CpswNcCppiP0RxDscpMapReg6Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_6;
#[doc = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_7 (rw) register accessor: CPPI Port 0 Receive IPV4/IPV6 DSCP Map N\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_dscp_map_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_dscp_map_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_rx_dscp_map_reg_7`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_RX_DSCP_MAP_REG_7")]
pub type CpswNcCppiP0RxDscpMapReg7 =
    crate::Reg<cpsw_nc_cppi_p0_rx_dscp_map_reg_7::CpswNcCppiP0RxDscpMapReg7Spec>;
#[doc = "CPPI Port 0 Receive IPV4/IPV6 DSCP Map N"]
pub mod cpsw_nc_cppi_p0_rx_dscp_map_reg_7;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_0 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_0`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_0")]
pub type CpswNcCppiP0PriCirReg0 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_0::CpswNcCppiP0PriCirReg0Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_0;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_1 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_1`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_1")]
pub type CpswNcCppiP0PriCirReg1 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_1::CpswNcCppiP0PriCirReg1Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_1;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_2 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_2`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_2")]
pub type CpswNcCppiP0PriCirReg2 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_2::CpswNcCppiP0PriCirReg2Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_2;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_3 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_3`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_3")]
pub type CpswNcCppiP0PriCirReg3 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_3::CpswNcCppiP0PriCirReg3Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_3;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_4 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_4`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_4")]
pub type CpswNcCppiP0PriCirReg4 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_4::CpswNcCppiP0PriCirReg4Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_4;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_5 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_5`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_5")]
pub type CpswNcCppiP0PriCirReg5 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_5::CpswNcCppiP0PriCirReg5Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_5;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_6 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_6`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_6")]
pub type CpswNcCppiP0PriCirReg6 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_6::CpswNcCppiP0PriCirReg6Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_6;
#[doc = "CPSW_NC_CPPI_P0_PRI_CIR_REG_7 (rw) register accessor: CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_cir_reg_7`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_CIR_REG_7")]
pub type CpswNcCppiP0PriCirReg7 =
    crate::Reg<cpsw_nc_cppi_p0_pri_cir_reg_7::CpswNcCppiP0PriCirReg7Spec>;
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_cir_reg_7;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_0 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_0`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_0")]
pub type CpswNcCppiP0PriEirReg0 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_0::CpswNcCppiP0PriEirReg0Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_0;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_1 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_1`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_1")]
pub type CpswNcCppiP0PriEirReg1 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_1::CpswNcCppiP0PriEirReg1Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_1;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_2 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_2`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_2")]
pub type CpswNcCppiP0PriEirReg2 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_2::CpswNcCppiP0PriEirReg2Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_2;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_3 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_3`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_3")]
pub type CpswNcCppiP0PriEirReg3 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_3::CpswNcCppiP0PriEirReg3Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_3;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_4 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_4`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_4")]
pub type CpswNcCppiP0PriEirReg4 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_4::CpswNcCppiP0PriEirReg4Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_4;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_5 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_5`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_5")]
pub type CpswNcCppiP0PriEirReg5 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_5::CpswNcCppiP0PriEirReg5Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_5;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_6 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_6`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_6")]
pub type CpswNcCppiP0PriEirReg6 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_6::CpswNcCppiP0PriEirReg6Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_6;
#[doc = "CPSW_NC_CPPI_P0_PRI_EIR_REG_7 (rw) register accessor: CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_pri_eir_reg_7`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_PRI_EIR_REG_7")]
pub type CpswNcCppiP0PriEirReg7 =
    crate::Reg<cpsw_nc_cppi_p0_pri_eir_reg_7::CpswNcCppiP0PriEirReg7Spec>;
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate"]
pub mod cpsw_nc_cppi_p0_pri_eir_reg_7;
#[doc = "CPSW_NC_CPPI_P0_TX_D_THRESH_SET_L_REG (rw) register accessor: CPPI Port 0 Tx PFC Destination Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_D_THRESH_SET_L_REG")]
pub type CpswNcCppiP0TxDThreshSetLReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg::CpswNcCppiP0TxDThreshSetLRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Destination Threshold Set Low"]
pub mod cpsw_nc_cppi_p0_tx_d_thresh_set_l_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_D_THRESH_SET_H_REG (rw) register accessor: CPPI Port 0 Tx PFC Destination Threshold Set High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_D_THRESH_SET_H_REG")]
pub type CpswNcCppiP0TxDThreshSetHReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg::CpswNcCppiP0TxDThreshSetHRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Destination Threshold Set High"]
pub mod cpsw_nc_cppi_p0_tx_d_thresh_set_h_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_D_THRESH_CLR_L_REG (rw) register accessor: CPPI Port 0 Tx PFC Destination Threshold Clr Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_D_THRESH_CLR_L_REG")]
pub type CpswNcCppiP0TxDThreshClrLReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg::CpswNcCppiP0TxDThreshClrLRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Destination Threshold Clr Low"]
pub mod cpsw_nc_cppi_p0_tx_d_thresh_clr_l_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_D_THRESH_CLR_H_REG (rw) register accessor: CPPI Port 0 Tx PFC Destination Threshold Clr High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_D_THRESH_CLR_H_REG")]
pub type CpswNcCppiP0TxDThreshClrHReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg::CpswNcCppiP0TxDThreshClrHRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Destination Threshold Clr High"]
pub mod cpsw_nc_cppi_p0_tx_d_thresh_clr_h_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_SET_L_REG (rw) register accessor: CPPI Port 0 Tx PFC Global Buffer Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_SET_L_REG")]
pub type CpswNcCppiP0TxGBufThreshSetLReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg::CpswNcCppiP0TxGBufThreshSetLRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Global Buffer Threshold Set Low"]
pub mod cpsw_nc_cppi_p0_tx_g_buf_thresh_set_l_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_SET_H_REG (rw) register accessor: CPPI Port 0 Tx PFC Global Buffer Threshold Set High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_SET_H_REG")]
pub type CpswNcCppiP0TxGBufThreshSetHReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg::CpswNcCppiP0TxGBufThreshSetHRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Global Buffer Threshold Set High"]
pub mod cpsw_nc_cppi_p0_tx_g_buf_thresh_set_h_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_CLR_L_REG (rw) register accessor: CPPI Port 0 Tx PFC Global Buffer Threshold Clr Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_CLR_L_REG")]
pub type CpswNcCppiP0TxGBufThreshClrLReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg::CpswNcCppiP0TxGBufThreshClrLRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Global Buffer Threshold Clr Low"]
pub mod cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_l_reg;
#[doc = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_CLR_H_REG (rw) register accessor: CPPI Port 0 Tx PFC Global Buffer Threshold Clr High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_TX_G_BUF_THRESH_CLR_H_REG")]
pub type CpswNcCppiP0TxGBufThreshClrHReg =
    crate::Reg<cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg::CpswNcCppiP0TxGBufThreshClrHRegSpec>;
#[doc = "CPPI Port 0 Tx PFC Global Buffer Threshold Clr High"]
pub mod cpsw_nc_cppi_p0_tx_g_buf_thresh_clr_h_reg;
#[doc = "CPSW_NC_CPPI_P0_SRC_ID_A_REG (rw) register accessor: CPPI Port 0 CPPI Source ID A\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_src_id_a_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_src_id_a_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_src_id_a_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_SRC_ID_A_REG")]
pub type CpswNcCppiP0SrcIdAReg =
    crate::Reg<cpsw_nc_cppi_p0_src_id_a_reg::CpswNcCppiP0SrcIdARegSpec>;
#[doc = "CPPI Port 0 CPPI Source ID A"]
pub mod cpsw_nc_cppi_p0_src_id_a_reg;
#[doc = "CPSW_NC_CPPI_P0_SRC_ID_B_REG (rw) register accessor: CPPI Port 0 CPPI Source ID B\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_src_id_b_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_src_id_b_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_src_id_b_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_SRC_ID_B_REG")]
pub type CpswNcCppiP0SrcIdBReg =
    crate::Reg<cpsw_nc_cppi_p0_src_id_b_reg::CpswNcCppiP0SrcIdBRegSpec>;
#[doc = "CPPI Port 0 CPPI Source ID B"]
pub mod cpsw_nc_cppi_p0_src_id_b_reg;
#[doc = "CPSW_NC_CPPI_P0_HOST_BLKS_PRI_REG (rw) register accessor: CPPI Port 0 Host Blocks Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_host_blks_pri_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_host_blks_pri_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_cppi_p0_host_blks_pri_reg`]
module"]
#[doc(alias = "CPSW_NC_CPPI_P0_HOST_BLKS_PRI_REG")]
pub type CpswNcCppiP0HostBlksPriReg =
    crate::Reg<cpsw_nc_cppi_p0_host_blks_pri_reg::CpswNcCppiP0HostBlksPriRegSpec>;
#[doc = "CPPI Port 0 Host Blocks Priority"]
pub mod cpsw_nc_cppi_p0_host_blks_pri_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RESERVED_REG (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_reserved_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_reserved_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_reserved_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RESERVED_REG")]
pub type CpswNcEthMac0PnReservedReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_reserved_reg::CpswNcEthMac0PnReservedRegSpec>;
#[doc = "Reserved"]
pub mod cpsw_nc_eth_mac_0_pn_reserved_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_CONTROL_REG (rw) register accessor: Enet Port N Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_control_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_CONTROL_REG")]
pub type CpswNcEthMac0PnControlReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_control_reg::CpswNcEthMac0PnControlRegSpec>;
#[doc = "Enet Port N Control"]
pub mod cpsw_nc_eth_mac_0_pn_control_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAX_BLKS_REG (rw) register accessor: Enet Port N FIFO Max Blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_max_blks_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_max_blks_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_max_blks_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAX_BLKS_REG")]
pub type CpswNcEthMac0PnMaxBlksReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_max_blks_reg::CpswNcEthMac0PnMaxBlksRegSpec>;
#[doc = "Enet Port N FIFO Max Blocks"]
pub mod cpsw_nc_eth_mac_0_pn_max_blks_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_BLK_CNT_REG (rw) register accessor: Enet Port N FIFO Block Usage Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_blk_cnt_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_BLK_CNT_REG")]
pub type CpswNcEthMac0PnBlkCntReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_blk_cnt_reg::CpswNcEthMac0PnBlkCntRegSpec>;
#[doc = "Enet Port N FIFO Block Usage Count"]
pub mod cpsw_nc_eth_mac_0_pn_blk_cnt_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PORT_VLAN_REG (rw) register accessor: Enet Port N VLAN\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_port_vlan_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_port_vlan_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_port_vlan_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PORT_VLAN_REG")]
pub type CpswNcEthMac0PnPortVlanReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_port_vlan_reg::CpswNcEthMac0PnPortVlanRegSpec>;
#[doc = "Enet Port N VLAN"]
pub mod cpsw_nc_eth_mac_0_pn_port_vlan_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_PRI_MAP_REG (rw) register accessor: Enet Port N Tx Header Pri to Switch Pri Mapping\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_pri_map_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_pri_map_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_pri_map_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_PRI_MAP_REG")]
pub type CpswNcEthMac0PnTxPriMapReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_pri_map_reg::CpswNcEthMac0PnTxPriMapRegSpec>;
#[doc = "Enet Port N Tx Header Pri to Switch Pri Mapping"]
pub mod cpsw_nc_eth_mac_0_pn_tx_pri_map_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CTL_REG (rw) register accessor: Enet Port N Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_ctl_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CTL_REG")]
pub type CpswNcEthMac0PnPriCtlReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_ctl_reg::CpswNcEthMac0PnPriCtlRegSpec>;
#[doc = "Enet Port N Priority Control"]
pub mod cpsw_nc_eth_mac_0_pn_pri_ctl_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_PRI_MAP_REG (rw) register accessor: Enet Port N RX Pkt Pri to Header Pri Map\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_pri_map_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_PRI_MAP_REG")]
pub type CpswNcEthMac0PnRxPriMapReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::CpswNcEthMac0PnRxPriMapRegSpec>;
#[doc = "Enet Port N RX Pkt Pri to Header Pri Map"]
pub mod cpsw_nc_eth_mac_0_pn_rx_pri_map_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_MAXLEN_REG (rw) register accessor: Enet Port N Receive Frame Max Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_maxlen_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_maxlen_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_MAXLEN_REG")]
pub type CpswNcEthMac0PnRxMaxlenReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_maxlen_reg::CpswNcEthMac0PnRxMaxlenRegSpec>;
#[doc = "Enet Port N Receive Frame Max Length"]
pub mod cpsw_nc_eth_mac_0_pn_rx_maxlen_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_BLKS_PRI_REG (rw) register accessor: Enet Port N Transmit Block Sub Per Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_BLKS_PRI_REG")]
pub type CpswNcEthMac0PnTxBlksPriReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg::CpswNcEthMac0PnTxBlksPriRegSpec>;
#[doc = "Enet Port N Transmit Block Sub Per Priority"]
pub mod cpsw_nc_eth_mac_0_pn_tx_blks_pri_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_FLOW_THRESH_REG (rw) register accessor: Enet MAC Receive Flow Threshold in Receive Buffer Words\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_FLOW_THRESH_REG")]
pub type CpswNcEthMac0PnRxFlowThreshReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::CpswNcEthMac0PnRxFlowThreshRegSpec>;
#[doc = "Enet MAC Receive Flow Threshold in Receive Buffer Words"]
pub mod cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_IDLE2LPI_REG (rw) register accessor: Enet Port N EEE Idle to LPI counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_idle2lpi_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_IDLE2LPI_REG")]
pub type CpswNcEthMac0PnIdle2lpiReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_idle2lpi_reg::CpswNcEthMac0PnIdle2lpiRegSpec>;
#[doc = "Enet Port N EEE Idle to LPI counter"]
pub mod cpsw_nc_eth_mac_0_pn_idle2lpi_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_LPI2WAKE_REG (rw) register accessor: Enet Port N EEE LPI to wake counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_lpi2wake_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_lpi2wake_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_lpi2wake_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_LPI2WAKE_REG")]
pub type CpswNcEthMac0PnLpi2wakeReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_lpi2wake_reg::CpswNcEthMac0PnLpi2wakeRegSpec>;
#[doc = "Enet Port N EEE LPI to wake counter"]
pub mod cpsw_nc_eth_mac_0_pn_lpi2wake_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_EEE_STATUS_REG (rw) register accessor: Enet Port N EEE status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_eee_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_eee_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_eee_status_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_EEE_STATUS_REG")]
pub type CpswNcEthMac0PnEeeStatusReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_eee_status_reg::CpswNcEthMac0PnEeeStatusRegSpec>;
#[doc = "Enet Port N EEE status"]
pub mod cpsw_nc_eth_mac_0_pn_eee_status_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_FIFO_STATUS_REG (rw) register accessor: Enet Port N FIFO STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_fifo_status_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_FIFO_STATUS_REG")]
pub type CpswNcEthMac0PnFifoStatusReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_fifo_status_reg::CpswNcEthMac0PnFifoStatusRegSpec>;
#[doc = "Enet Port N FIFO STATUS"]
pub mod cpsw_nc_eth_mac_0_pn_fifo_status_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_EST_CONTROL_REG (rw) register accessor: Enet Port N EST CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_est_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_est_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_est_control_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_EST_CONTROL_REG")]
pub type CpswNcEthMac0PnEstControlReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_est_control_reg::CpswNcEthMac0PnEstControlRegSpec>;
#[doc = "Enet Port N EST CONTROL"]
pub mod cpsw_nc_eth_mac_0_pn_est_control_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_0 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_0")]
pub type CpswNcEthMac0PnRxDscpMapReg0 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0::CpswNcEthMac0PnRxDscpMapReg0Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_0;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_1 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_1")]
pub type CpswNcEthMac0PnRxDscpMapReg1 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1::CpswNcEthMac0PnRxDscpMapReg1Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_1;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_2 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_2")]
pub type CpswNcEthMac0PnRxDscpMapReg2 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::CpswNcEthMac0PnRxDscpMapReg2Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_3 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_3")]
pub type CpswNcEthMac0PnRxDscpMapReg3 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3::CpswNcEthMac0PnRxDscpMapReg3Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_3;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_4 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_4")]
pub type CpswNcEthMac0PnRxDscpMapReg4 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4::CpswNcEthMac0PnRxDscpMapReg4Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_4;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_5 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_5")]
pub type CpswNcEthMac0PnRxDscpMapReg5 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5::CpswNcEthMac0PnRxDscpMapReg5Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_5;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_6 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_6")]
pub type CpswNcEthMac0PnRxDscpMapReg6 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6::CpswNcEthMac0PnRxDscpMapReg6Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_6;
#[doc = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_7 (rw) register accessor: Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_7")]
pub type CpswNcEthMac0PnRxDscpMapReg7 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7::CpswNcEthMac0PnRxDscpMapReg7Spec>;
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M"]
pub mod cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_7;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_0 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_0`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_0")]
pub type CpswNcEthMac0PnPriCirReg0 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_0::CpswNcEthMac0PnPriCirReg0Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_0;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_1 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_1`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_1")]
pub type CpswNcEthMac0PnPriCirReg1 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_1::CpswNcEthMac0PnPriCirReg1Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_1;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_2 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_2`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_2")]
pub type CpswNcEthMac0PnPriCirReg2 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::CpswNcEthMac0PnPriCirReg2Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_2;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_3 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_3`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_3")]
pub type CpswNcEthMac0PnPriCirReg3 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_3::CpswNcEthMac0PnPriCirReg3Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_3;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_4 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_4`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_4")]
pub type CpswNcEthMac0PnPriCirReg4 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_4::CpswNcEthMac0PnPriCirReg4Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_4;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_5 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_5`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_5")]
pub type CpswNcEthMac0PnPriCirReg5 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_5::CpswNcEthMac0PnPriCirReg5Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_5;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_6 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_6`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_6")]
pub type CpswNcEthMac0PnPriCirReg6 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_6::CpswNcEthMac0PnPriCirReg6Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_6;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_7 (rw) register accessor: Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_cir_reg_7`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_7")]
pub type CpswNcEthMac0PnPriCirReg7 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_cir_reg_7::CpswNcEthMac0PnPriCirReg7Spec>;
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_cir_reg_7;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_0 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_0`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_0")]
pub type CpswNcEthMac0PnPriEirReg0 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_0::CpswNcEthMac0PnPriEirReg0Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_0;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_1 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_1`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_1")]
pub type CpswNcEthMac0PnPriEirReg1 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_1::CpswNcEthMac0PnPriEirReg1Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_1;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_2 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_2`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_2")]
pub type CpswNcEthMac0PnPriEirReg2 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_2::CpswNcEthMac0PnPriEirReg2Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_2;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_3 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_3`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_3")]
pub type CpswNcEthMac0PnPriEirReg3 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::CpswNcEthMac0PnPriEirReg3Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_3;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_4 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_4`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_4")]
pub type CpswNcEthMac0PnPriEirReg4 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_4::CpswNcEthMac0PnPriEirReg4Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_4;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_5 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_5`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_5")]
pub type CpswNcEthMac0PnPriEirReg5 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_5::CpswNcEthMac0PnPriEirReg5Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_5;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_6 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_6`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_6")]
pub type CpswNcEthMac0PnPriEirReg6 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_6::CpswNcEthMac0PnPriEirReg6Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_6;
#[doc = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_7 (rw) register accessor: Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_pri_eir_reg_7`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_7")]
pub type CpswNcEthMac0PnPriEirReg7 =
    crate::Reg<cpsw_nc_eth_mac_0_pn_pri_eir_reg_7::CpswNcEthMac0PnPriEirReg7Spec>;
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value"]
pub mod cpsw_nc_eth_mac_0_pn_pri_eir_reg_7;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_SET_L_REG (rw) register accessor: Enet Port N Tx PFC Destination Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_SET_L_REG")]
pub type CpswNcEthMac0PnTxDThreshSetLReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg::CpswNcEthMac0PnTxDThreshSetLRegSpec>;
#[doc = "Enet Port N Tx PFC Destination Threshold Set Low"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_SET_H_REG (rw) register accessor: Enet Port N Tx PFC Destination Threshold Set High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_SET_H_REG")]
pub type CpswNcEthMac0PnTxDThreshSetHReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg::CpswNcEthMac0PnTxDThreshSetHRegSpec>;
#[doc = "Enet Port N Tx PFC Destination Threshold Set High"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_thresh_set_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_CLR_L_REG (rw) register accessor: Enet Port N Tx PFC Destination Threshold Clr Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_CLR_L_REG")]
pub type CpswNcEthMac0PnTxDThreshClrLReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg::CpswNcEthMac0PnTxDThreshClrLRegSpec>;
#[doc = "Enet Port N Tx PFC Destination Threshold Clr Low"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_CLR_H_REG (rw) register accessor: Enet Port N Tx PFC Destination Threshold Clr High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_THRESH_CLR_H_REG")]
pub type CpswNcEthMac0PnTxDThreshClrHReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg::CpswNcEthMac0PnTxDThreshClrHRegSpec>;
#[doc = "Enet Port N Tx PFC Destination Threshold Clr High"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_thresh_clr_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_SET_L_REG (rw) register accessor: Enet Port N Tx PFC Global Buffer Threshold Set Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_SET_L_REG")]
pub type CpswNcEthMac0PnTxGBufThreshSetLReg = crate::Reg<
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg::CpswNcEthMac0PnTxGBufThreshSetLRegSpec,
>;
#[doc = "Enet Port N Tx PFC Global Buffer Threshold Set Low"]
pub mod cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_SET_H_REG (rw) register accessor: Enet Port N Tx PFC Global Buffer Threshold Set High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_SET_H_REG")]
pub type CpswNcEthMac0PnTxGBufThreshSetHReg = crate::Reg<
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg::CpswNcEthMac0PnTxGBufThreshSetHRegSpec,
>;
#[doc = "Enet Port N Tx PFC Global Buffer Threshold Set High"]
pub mod cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_set_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_L_REG (rw) register accessor: Enet Port N Tx PFC Global Buffer Threshold Clr Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_L_REG")]
pub type CpswNcEthMac0PnTxGBufThreshClrLReg = crate::Reg<
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::CpswNcEthMac0PnTxGBufThreshClrLRegSpec,
>;
#[doc = "Enet Port N Tx PFC Global Buffer Threshold Clr Low"]
pub mod cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_H_REG (rw) register accessor: Enet Port N Tx PFC Global Buffer Threshold Clr High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_H_REG")]
pub type CpswNcEthMac0PnTxGBufThreshClrHReg = crate::Reg<
    cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg::CpswNcEthMac0PnTxGBufThreshClrHRegSpec,
>;
#[doc = "Enet Port N Tx PFC Global Buffer Threshold Clr High"]
pub mod cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_L_REG (rw) register accessor: Enet Port N Tx Destination Out Flow Add Values Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_L_REG")]
pub type CpswNcEthMac0PnTxDOflowAddvalLReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::CpswNcEthMac0PnTxDOflowAddvalLRegSpec>;
#[doc = "Enet Port N Tx Destination Out Flow Add Values Low"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_H_REG (rw) register accessor: Enet Port N Tx Destination Out Flow Add Values High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_H_REG")]
pub type CpswNcEthMac0PnTxDOflowAddvalHReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg::CpswNcEthMac0PnTxDOflowAddvalHRegSpec>;
#[doc = "Enet Port N Tx Destination Out Flow Add Values High"]
pub mod cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_SA_L_REG (rw) register accessor: Enet Port N Tx Pause Frame Source Address Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_sa_l_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_sa_l_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_sa_l_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_SA_L_REG")]
pub type CpswNcEthMac0PnSaLReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_sa_l_reg::CpswNcEthMac0PnSaLRegSpec>;
#[doc = "Enet Port N Tx Pause Frame Source Address Low"]
pub mod cpsw_nc_eth_mac_0_pn_sa_l_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_SA_H_REG (rw) register accessor: Enet Port N Tx Pause Frame Source Address High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_sa_h_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_sa_h_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_sa_h_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_SA_H_REG")]
pub type CpswNcEthMac0PnSaHReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_sa_h_reg::CpswNcEthMac0PnSaHRegSpec>;
#[doc = "Enet Port N Tx Pause Frame Source Address High"]
pub mod cpsw_nc_eth_mac_0_pn_sa_h_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TS_CTL_REG (rw) register accessor: Enet Port N Time Sync Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_ts_ctl_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TS_CTL_REG")]
pub type CpswNcEthMac0PnTsCtlReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_ts_ctl_reg::CpswNcEthMac0PnTsCtlRegSpec>;
#[doc = "Enet Port N Time Sync Control"]
pub mod cpsw_nc_eth_mac_0_pn_ts_ctl_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TS_SEQ_LTYPE_REG (rw) register accessor: Enet Port N Time Sync LTYPE (and SEQ_ID_OFFSET)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TS_SEQ_LTYPE_REG")]
pub type CpswNcEthMac0PnTsSeqLtypeReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::CpswNcEthMac0PnTsSeqLtypeRegSpec>;
#[doc = "Enet Port N Time Sync LTYPE (and SEQ_ID_OFFSET)"]
pub mod cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TS_VLAN_LTYPE_REG (rw) register accessor: Enet Port N Time Sync VLAN2 and VLAN2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TS_VLAN_LTYPE_REG")]
pub type CpswNcEthMac0PnTsVlanLtypeReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::CpswNcEthMac0PnTsVlanLtypeRegSpec>;
#[doc = "Enet Port N Time Sync VLAN2 and VLAN2"]
pub mod cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TS_CTL_LTYPE2_REG (rw) register accessor: Enet Port N Time Sync Control and LTYPE 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TS_CTL_LTYPE2_REG")]
pub type CpswNcEthMac0PnTsCtlLtype2Reg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::CpswNcEthMac0PnTsCtlLtype2RegSpec>;
#[doc = "Enet Port N Time Sync Control and LTYPE 2"]
pub mod cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_TS_CTL2_REG (rw) register accessor: Enet Port N Time Sync Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_ts_ctl2_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_TS_CTL2_REG")]
pub type CpswNcEthMac0PnTsCtl2Reg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::CpswNcEthMac0PnTsCtl2RegSpec>;
#[doc = "Enet Port N Time Sync Control 2"]
pub mod cpsw_nc_eth_mac_0_pn_ts_ctl2_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_CONTROL_REG (rw) register accessor: Enet Port N Mac Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_control_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_CONTROL_REG")]
pub type CpswNcEthMac0PnMacControlReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_control_reg::CpswNcEthMac0PnMacControlRegSpec>;
#[doc = "Enet Port N Mac Control"]
pub mod cpsw_nc_eth_mac_0_pn_mac_control_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_STATUS_REG (rw) register accessor: Enet Port N Mac Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_status_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_STATUS_REG")]
pub type CpswNcEthMac0PnMacStatusReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_status_reg::CpswNcEthMac0PnMacStatusRegSpec>;
#[doc = "Enet Port N Mac Status"]
pub mod cpsw_nc_eth_mac_0_pn_mac_status_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_SOFT_RESET_REG (rw) register accessor: Enet Port N Mac Soft Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_SOFT_RESET_REG")]
pub type CpswNcEthMac0PnMacSoftResetReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::CpswNcEthMac0PnMacSoftResetRegSpec>;
#[doc = "Enet Port N Mac Soft Reset"]
pub mod cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_BOFFTEST_REG (rw) register accessor: Enet Port N Mac Backoff Test\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_bofftest_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_BOFFTEST_REG")]
pub type CpswNcEthMac0PnMacBofftestReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::CpswNcEthMac0PnMacBofftestRegSpec>;
#[doc = "Enet Port N Mac Backoff Test"]
pub mod cpsw_nc_eth_mac_0_pn_mac_bofftest_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RX_PAUSETIMER_REG (rw) register accessor: Enet Port N 802.3 Receive Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RX_PAUSETIMER_REG")]
pub type CpswNcEthMac0PnMacRxPausetimerReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::CpswNcEthMac0PnMacRxPausetimerRegSpec>;
#[doc = "Enet Port N 802.3 Receive Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_0 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_0")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg0 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0::CpswNcEthMac0PnMacRxnPausetimerReg0Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_0;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_1 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_1")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg1 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1::CpswNcEthMac0PnMacRxnPausetimerReg1Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_1;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_2 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_2")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg2 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2::CpswNcEthMac0PnMacRxnPausetimerReg2Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_2;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_3 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_3")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg3 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3::CpswNcEthMac0PnMacRxnPausetimerReg3Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_3;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_4 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_4")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg4 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4::CpswNcEthMac0PnMacRxnPausetimerReg4Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_4;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_5 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_5")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg5 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::CpswNcEthMac0PnMacRxnPausetimerReg5Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_6 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_6")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg6 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6::CpswNcEthMac0PnMacRxnPausetimerReg6Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_6;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_7 (rw) register accessor: Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_7")]
pub type CpswNcEthMac0PnMacRxnPausetimerReg7 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7::CpswNcEthMac0PnMacRxnPausetimerReg7Spec,
>;
#[doc = "Enet Port N PFC Priority P Rx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_7;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TX_PAUSETIMER_REG (rw) register accessor: Enet Port N 802.3 Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TX_PAUSETIMER_REG")]
pub type CpswNcEthMac0PnMacTxPausetimerReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::CpswNcEthMac0PnMacTxPausetimerRegSpec>;
#[doc = "Enet Port N 802.3 Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_0 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_0")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg0 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0::CpswNcEthMac0PnMacTxnPausetimerReg0Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_0;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_1 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_1")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg1 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1::CpswNcEthMac0PnMacTxnPausetimerReg1Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_1;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_2 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_2")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg2 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2::CpswNcEthMac0PnMacTxnPausetimerReg2Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_2;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_3 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_3")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg3 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3::CpswNcEthMac0PnMacTxnPausetimerReg3Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_3;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_4 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_4")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg4 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::CpswNcEthMac0PnMacTxnPausetimerReg4Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_5 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_5")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg5 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5::CpswNcEthMac0PnMacTxnPausetimerReg5Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_5;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_6 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_6")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg6 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6::CpswNcEthMac0PnMacTxnPausetimerReg6Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_6;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_7 (rw) register accessor: Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_7")]
pub type CpswNcEthMac0PnMacTxnPausetimerReg7 = crate::Reg<
    cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7::CpswNcEthMac0PnMacTxnPausetimerReg7Spec,
>;
#[doc = "Enet Port N PFC Priority P Tx Pause Timer"]
pub mod cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_7;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_EMCONTROL_REG (rw) register accessor: Enet Port N Emulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_EMCONTROL_REG")]
pub type CpswNcEthMac0PnMacEmcontrolReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::CpswNcEthMac0PnMacEmcontrolRegSpec>;
#[doc = "Enet Port N Emulation Control"]
pub mod cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_TX_GAP_REG (rw) register accessor: Enet Port N Tx Inter Packet Gap\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_TX_GAP_REG")]
pub type CpswNcEthMac0PnMacTxGapReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::CpswNcEthMac0PnMacTxGapRegSpec>;
#[doc = "Enet Port N Tx Inter Packet Gap"]
pub mod cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_MAC_PORT_CONFIG (rw) register accessor: Enet Port N Port Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_port_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_port_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_mac_port_config`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_MAC_PORT_CONFIG")]
pub type CpswNcEthMac0PnMacPortConfig =
    crate::Reg<cpsw_nc_eth_mac_0_pn_mac_port_config::CpswNcEthMac0PnMacPortConfigSpec>;
#[doc = "Enet Port N Port Configuration"]
pub mod cpsw_nc_eth_mac_0_pn_mac_port_config;
#[doc = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_POINTER_REG (rw) register accessor: Enet Port N Tx Egress InterVLAN Operation Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_POINTER_REG")]
pub type CpswNcEthMac0PnIntervlanOpxPointerReg = crate::Reg<
    cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::CpswNcEthMac0PnIntervlanOpxPointerRegSpec,
>;
#[doc = "Enet Port N Tx Egress InterVLAN Operation Pointer"]
pub mod cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_A_REG (rw) register accessor: Enet Port N Tx Egress InterVLAN A\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_A_REG")]
pub type CpswNcEthMac0PnIntervlanOpxAReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::CpswNcEthMac0PnIntervlanOpxARegSpec>;
#[doc = "Enet Port N Tx Egress InterVLAN A"]
pub mod cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_B_REG (rw) register accessor: Enet Port N Tx Egress InterVLAN B\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_B_REG")]
pub type CpswNcEthMac0PnIntervlanOpxBReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg::CpswNcEthMac0PnIntervlanOpxBRegSpec>;
#[doc = "Enet Port N Tx Egress InterVLAN B"]
pub mod cpsw_nc_eth_mac_0_pn_intervlan_opx_b_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_C_REG (rw) register accessor: Enet Port N Tx Egress InterVLAN C\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_C_REG")]
pub type CpswNcEthMac0PnIntervlanOpxCReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::CpswNcEthMac0PnIntervlanOpxCRegSpec>;
#[doc = "Enet Port N Tx Egress InterVLAN C"]
pub mod cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg;
#[doc = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_D_REG (rw) register accessor: Enet Port N Tx Egress InterVLAN D\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg`]
module"]
#[doc(alias = "CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_D_REG")]
pub type CpswNcEthMac0PnIntervlanOpxDReg =
    crate::Reg<cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::CpswNcEthMac0PnIntervlanOpxDRegSpec>;
#[doc = "Enet Port N Tx Egress InterVLAN D"]
pub mod cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg;
#[doc = "CPSW_NC_EST_FETCH_LOC_0 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_0`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_0")]
pub type CpswNcEstFetchLoc0 = crate::Reg<cpsw_nc_est_fetch_loc_0::CpswNcEstFetchLoc0Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_0;
#[doc = "CPSW_NC_EST_FETCH_LOC_1 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_1`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_1")]
pub type CpswNcEstFetchLoc1 = crate::Reg<cpsw_nc_est_fetch_loc_1::CpswNcEstFetchLoc1Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_1;
#[doc = "CPSW_NC_EST_FETCH_LOC_2 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_2`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_2")]
pub type CpswNcEstFetchLoc2 = crate::Reg<cpsw_nc_est_fetch_loc_2::CpswNcEstFetchLoc2Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_2;
#[doc = "CPSW_NC_EST_FETCH_LOC_3 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_3`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_3")]
pub type CpswNcEstFetchLoc3 = crate::Reg<cpsw_nc_est_fetch_loc_3::CpswNcEstFetchLoc3Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_3;
#[doc = "CPSW_NC_EST_FETCH_LOC_4 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_4`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_4")]
pub type CpswNcEstFetchLoc4 = crate::Reg<cpsw_nc_est_fetch_loc_4::CpswNcEstFetchLoc4Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_4;
#[doc = "CPSW_NC_EST_FETCH_LOC_5 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_5`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_5")]
pub type CpswNcEstFetchLoc5 = crate::Reg<cpsw_nc_est_fetch_loc_5::CpswNcEstFetchLoc5Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_5;
#[doc = "CPSW_NC_EST_FETCH_LOC_6 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_6`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_6")]
pub type CpswNcEstFetchLoc6 = crate::Reg<cpsw_nc_est_fetch_loc_6::CpswNcEstFetchLoc6Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_6;
#[doc = "CPSW_NC_EST_FETCH_LOC_7 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_7`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_7")]
pub type CpswNcEstFetchLoc7 = crate::Reg<cpsw_nc_est_fetch_loc_7::CpswNcEstFetchLoc7Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_7;
#[doc = "CPSW_NC_EST_FETCH_LOC_8 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_8`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_8")]
pub type CpswNcEstFetchLoc8 = crate::Reg<cpsw_nc_est_fetch_loc_8::CpswNcEstFetchLoc8Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_8;
#[doc = "CPSW_NC_EST_FETCH_LOC_9 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_9`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_9")]
pub type CpswNcEstFetchLoc9 = crate::Reg<cpsw_nc_est_fetch_loc_9::CpswNcEstFetchLoc9Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_9;
#[doc = "CPSW_NC_EST_FETCH_LOC_10 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_10`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_10")]
pub type CpswNcEstFetchLoc10 = crate::Reg<cpsw_nc_est_fetch_loc_10::CpswNcEstFetchLoc10Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_10;
#[doc = "CPSW_NC_EST_FETCH_LOC_11 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_11`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_11")]
pub type CpswNcEstFetchLoc11 = crate::Reg<cpsw_nc_est_fetch_loc_11::CpswNcEstFetchLoc11Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_11;
#[doc = "CPSW_NC_EST_FETCH_LOC_12 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_12`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_12")]
pub type CpswNcEstFetchLoc12 = crate::Reg<cpsw_nc_est_fetch_loc_12::CpswNcEstFetchLoc12Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_12;
#[doc = "CPSW_NC_EST_FETCH_LOC_13 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_13`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_13")]
pub type CpswNcEstFetchLoc13 = crate::Reg<cpsw_nc_est_fetch_loc_13::CpswNcEstFetchLoc13Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_13;
#[doc = "CPSW_NC_EST_FETCH_LOC_14 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_14`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_14")]
pub type CpswNcEstFetchLoc14 = crate::Reg<cpsw_nc_est_fetch_loc_14::CpswNcEstFetchLoc14Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_14;
#[doc = "CPSW_NC_EST_FETCH_LOC_15 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_15`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_15")]
pub type CpswNcEstFetchLoc15 = crate::Reg<cpsw_nc_est_fetch_loc_15::CpswNcEstFetchLoc15Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_15;
#[doc = "CPSW_NC_EST_FETCH_LOC_16 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_16`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_16")]
pub type CpswNcEstFetchLoc16 = crate::Reg<cpsw_nc_est_fetch_loc_16::CpswNcEstFetchLoc16Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_16;
#[doc = "CPSW_NC_EST_FETCH_LOC_17 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_17`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_17")]
pub type CpswNcEstFetchLoc17 = crate::Reg<cpsw_nc_est_fetch_loc_17::CpswNcEstFetchLoc17Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_17;
#[doc = "CPSW_NC_EST_FETCH_LOC_18 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_18`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_18")]
pub type CpswNcEstFetchLoc18 = crate::Reg<cpsw_nc_est_fetch_loc_18::CpswNcEstFetchLoc18Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_18;
#[doc = "CPSW_NC_EST_FETCH_LOC_19 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_19`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_19")]
pub type CpswNcEstFetchLoc19 = crate::Reg<cpsw_nc_est_fetch_loc_19::CpswNcEstFetchLoc19Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_19;
#[doc = "CPSW_NC_EST_FETCH_LOC_20 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_20`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_20")]
pub type CpswNcEstFetchLoc20 = crate::Reg<cpsw_nc_est_fetch_loc_20::CpswNcEstFetchLoc20Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_20;
#[doc = "CPSW_NC_EST_FETCH_LOC_21 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_21`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_21")]
pub type CpswNcEstFetchLoc21 = crate::Reg<cpsw_nc_est_fetch_loc_21::CpswNcEstFetchLoc21Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_21;
#[doc = "CPSW_NC_EST_FETCH_LOC_22 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_22`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_22")]
pub type CpswNcEstFetchLoc22 = crate::Reg<cpsw_nc_est_fetch_loc_22::CpswNcEstFetchLoc22Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_22;
#[doc = "CPSW_NC_EST_FETCH_LOC_23 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_23`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_23")]
pub type CpswNcEstFetchLoc23 = crate::Reg<cpsw_nc_est_fetch_loc_23::CpswNcEstFetchLoc23Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_23;
#[doc = "CPSW_NC_EST_FETCH_LOC_24 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_24`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_24")]
pub type CpswNcEstFetchLoc24 = crate::Reg<cpsw_nc_est_fetch_loc_24::CpswNcEstFetchLoc24Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_24;
#[doc = "CPSW_NC_EST_FETCH_LOC_25 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_25`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_25")]
pub type CpswNcEstFetchLoc25 = crate::Reg<cpsw_nc_est_fetch_loc_25::CpswNcEstFetchLoc25Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_25;
#[doc = "CPSW_NC_EST_FETCH_LOC_26 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_26`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_26")]
pub type CpswNcEstFetchLoc26 = crate::Reg<cpsw_nc_est_fetch_loc_26::CpswNcEstFetchLoc26Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_26;
#[doc = "CPSW_NC_EST_FETCH_LOC_27 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_27`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_27")]
pub type CpswNcEstFetchLoc27 = crate::Reg<cpsw_nc_est_fetch_loc_27::CpswNcEstFetchLoc27Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_27;
#[doc = "CPSW_NC_EST_FETCH_LOC_28 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_28`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_28")]
pub type CpswNcEstFetchLoc28 = crate::Reg<cpsw_nc_est_fetch_loc_28::CpswNcEstFetchLoc28Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_28;
#[doc = "CPSW_NC_EST_FETCH_LOC_29 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_29`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_29")]
pub type CpswNcEstFetchLoc29 = crate::Reg<cpsw_nc_est_fetch_loc_29::CpswNcEstFetchLoc29Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_29;
#[doc = "CPSW_NC_EST_FETCH_LOC_30 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_30`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_30")]
pub type CpswNcEstFetchLoc30 = crate::Reg<cpsw_nc_est_fetch_loc_30::CpswNcEstFetchLoc30Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_30;
#[doc = "CPSW_NC_EST_FETCH_LOC_31 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_31`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_31")]
pub type CpswNcEstFetchLoc31 = crate::Reg<cpsw_nc_est_fetch_loc_31::CpswNcEstFetchLoc31Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_31;
#[doc = "CPSW_NC_EST_FETCH_LOC_32 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_32`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_32")]
pub type CpswNcEstFetchLoc32 = crate::Reg<cpsw_nc_est_fetch_loc_32::CpswNcEstFetchLoc32Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_32;
#[doc = "CPSW_NC_EST_FETCH_LOC_33 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_33`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_33")]
pub type CpswNcEstFetchLoc33 = crate::Reg<cpsw_nc_est_fetch_loc_33::CpswNcEstFetchLoc33Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_33;
#[doc = "CPSW_NC_EST_FETCH_LOC_34 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_34`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_34")]
pub type CpswNcEstFetchLoc34 = crate::Reg<cpsw_nc_est_fetch_loc_34::CpswNcEstFetchLoc34Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_34;
#[doc = "CPSW_NC_EST_FETCH_LOC_35 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_35`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_35")]
pub type CpswNcEstFetchLoc35 = crate::Reg<cpsw_nc_est_fetch_loc_35::CpswNcEstFetchLoc35Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_35;
#[doc = "CPSW_NC_EST_FETCH_LOC_36 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_36`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_36")]
pub type CpswNcEstFetchLoc36 = crate::Reg<cpsw_nc_est_fetch_loc_36::CpswNcEstFetchLoc36Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_36;
#[doc = "CPSW_NC_EST_FETCH_LOC_37 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_37`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_37")]
pub type CpswNcEstFetchLoc37 = crate::Reg<cpsw_nc_est_fetch_loc_37::CpswNcEstFetchLoc37Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_37;
#[doc = "CPSW_NC_EST_FETCH_LOC_38 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_38`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_38")]
pub type CpswNcEstFetchLoc38 = crate::Reg<cpsw_nc_est_fetch_loc_38::CpswNcEstFetchLoc38Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_38;
#[doc = "CPSW_NC_EST_FETCH_LOC_39 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_39`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_39")]
pub type CpswNcEstFetchLoc39 = crate::Reg<cpsw_nc_est_fetch_loc_39::CpswNcEstFetchLoc39Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_39;
#[doc = "CPSW_NC_EST_FETCH_LOC_40 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_40`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_40")]
pub type CpswNcEstFetchLoc40 = crate::Reg<cpsw_nc_est_fetch_loc_40::CpswNcEstFetchLoc40Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_40;
#[doc = "CPSW_NC_EST_FETCH_LOC_41 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_41`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_41")]
pub type CpswNcEstFetchLoc41 = crate::Reg<cpsw_nc_est_fetch_loc_41::CpswNcEstFetchLoc41Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_41;
#[doc = "CPSW_NC_EST_FETCH_LOC_42 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_42`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_42")]
pub type CpswNcEstFetchLoc42 = crate::Reg<cpsw_nc_est_fetch_loc_42::CpswNcEstFetchLoc42Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_42;
#[doc = "CPSW_NC_EST_FETCH_LOC_43 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_43`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_43")]
pub type CpswNcEstFetchLoc43 = crate::Reg<cpsw_nc_est_fetch_loc_43::CpswNcEstFetchLoc43Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_43;
#[doc = "CPSW_NC_EST_FETCH_LOC_44 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_44`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_44")]
pub type CpswNcEstFetchLoc44 = crate::Reg<cpsw_nc_est_fetch_loc_44::CpswNcEstFetchLoc44Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_44;
#[doc = "CPSW_NC_EST_FETCH_LOC_45 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_45`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_45")]
pub type CpswNcEstFetchLoc45 = crate::Reg<cpsw_nc_est_fetch_loc_45::CpswNcEstFetchLoc45Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_45;
#[doc = "CPSW_NC_EST_FETCH_LOC_46 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_46`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_46")]
pub type CpswNcEstFetchLoc46 = crate::Reg<cpsw_nc_est_fetch_loc_46::CpswNcEstFetchLoc46Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_46;
#[doc = "CPSW_NC_EST_FETCH_LOC_47 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_47`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_47")]
pub type CpswNcEstFetchLoc47 = crate::Reg<cpsw_nc_est_fetch_loc_47::CpswNcEstFetchLoc47Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_47;
#[doc = "CPSW_NC_EST_FETCH_LOC_48 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_48`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_48")]
pub type CpswNcEstFetchLoc48 = crate::Reg<cpsw_nc_est_fetch_loc_48::CpswNcEstFetchLoc48Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_48;
#[doc = "CPSW_NC_EST_FETCH_LOC_49 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_49`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_49")]
pub type CpswNcEstFetchLoc49 = crate::Reg<cpsw_nc_est_fetch_loc_49::CpswNcEstFetchLoc49Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_49;
#[doc = "CPSW_NC_EST_FETCH_LOC_50 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_50`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_50")]
pub type CpswNcEstFetchLoc50 = crate::Reg<cpsw_nc_est_fetch_loc_50::CpswNcEstFetchLoc50Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_50;
#[doc = "CPSW_NC_EST_FETCH_LOC_51 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_51`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_51")]
pub type CpswNcEstFetchLoc51 = crate::Reg<cpsw_nc_est_fetch_loc_51::CpswNcEstFetchLoc51Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_51;
#[doc = "CPSW_NC_EST_FETCH_LOC_52 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_52`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_52")]
pub type CpswNcEstFetchLoc52 = crate::Reg<cpsw_nc_est_fetch_loc_52::CpswNcEstFetchLoc52Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_52;
#[doc = "CPSW_NC_EST_FETCH_LOC_53 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_53`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_53")]
pub type CpswNcEstFetchLoc53 = crate::Reg<cpsw_nc_est_fetch_loc_53::CpswNcEstFetchLoc53Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_53;
#[doc = "CPSW_NC_EST_FETCH_LOC_54 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_54`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_54")]
pub type CpswNcEstFetchLoc54 = crate::Reg<cpsw_nc_est_fetch_loc_54::CpswNcEstFetchLoc54Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_54;
#[doc = "CPSW_NC_EST_FETCH_LOC_55 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_55`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_55")]
pub type CpswNcEstFetchLoc55 = crate::Reg<cpsw_nc_est_fetch_loc_55::CpswNcEstFetchLoc55Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_55;
#[doc = "CPSW_NC_EST_FETCH_LOC_56 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_56`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_56")]
pub type CpswNcEstFetchLoc56 = crate::Reg<cpsw_nc_est_fetch_loc_56::CpswNcEstFetchLoc56Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_56;
#[doc = "CPSW_NC_EST_FETCH_LOC_57 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_57`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_57")]
pub type CpswNcEstFetchLoc57 = crate::Reg<cpsw_nc_est_fetch_loc_57::CpswNcEstFetchLoc57Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_57;
#[doc = "CPSW_NC_EST_FETCH_LOC_58 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_58`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_58")]
pub type CpswNcEstFetchLoc58 = crate::Reg<cpsw_nc_est_fetch_loc_58::CpswNcEstFetchLoc58Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_58;
#[doc = "CPSW_NC_EST_FETCH_LOC_59 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_59`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_59")]
pub type CpswNcEstFetchLoc59 = crate::Reg<cpsw_nc_est_fetch_loc_59::CpswNcEstFetchLoc59Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_59;
#[doc = "CPSW_NC_EST_FETCH_LOC_60 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_60`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_60")]
pub type CpswNcEstFetchLoc60 = crate::Reg<cpsw_nc_est_fetch_loc_60::CpswNcEstFetchLoc60Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_60;
#[doc = "CPSW_NC_EST_FETCH_LOC_61 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_61`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_61")]
pub type CpswNcEstFetchLoc61 = crate::Reg<cpsw_nc_est_fetch_loc_61::CpswNcEstFetchLoc61Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_61;
#[doc = "CPSW_NC_EST_FETCH_LOC_62 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_62`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_62")]
pub type CpswNcEstFetchLoc62 = crate::Reg<cpsw_nc_est_fetch_loc_62::CpswNcEstFetchLoc62Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_62;
#[doc = "CPSW_NC_EST_FETCH_LOC_63 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_63`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_63")]
pub type CpswNcEstFetchLoc63 = crate::Reg<cpsw_nc_est_fetch_loc_63::CpswNcEstFetchLoc63Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_63;
#[doc = "CPSW_NC_EST_FETCH_LOC_64 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_64`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_64")]
pub type CpswNcEstFetchLoc64 = crate::Reg<cpsw_nc_est_fetch_loc_64::CpswNcEstFetchLoc64Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_64;
#[doc = "CPSW_NC_EST_FETCH_LOC_65 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_65`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_65")]
pub type CpswNcEstFetchLoc65 = crate::Reg<cpsw_nc_est_fetch_loc_65::CpswNcEstFetchLoc65Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_65;
#[doc = "CPSW_NC_EST_FETCH_LOC_66 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_66`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_66")]
pub type CpswNcEstFetchLoc66 = crate::Reg<cpsw_nc_est_fetch_loc_66::CpswNcEstFetchLoc66Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_66;
#[doc = "CPSW_NC_EST_FETCH_LOC_67 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_67`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_67")]
pub type CpswNcEstFetchLoc67 = crate::Reg<cpsw_nc_est_fetch_loc_67::CpswNcEstFetchLoc67Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_67;
#[doc = "CPSW_NC_EST_FETCH_LOC_68 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_68`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_68")]
pub type CpswNcEstFetchLoc68 = crate::Reg<cpsw_nc_est_fetch_loc_68::CpswNcEstFetchLoc68Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_68;
#[doc = "CPSW_NC_EST_FETCH_LOC_69 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_69`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_69")]
pub type CpswNcEstFetchLoc69 = crate::Reg<cpsw_nc_est_fetch_loc_69::CpswNcEstFetchLoc69Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_69;
#[doc = "CPSW_NC_EST_FETCH_LOC_70 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_70`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_70")]
pub type CpswNcEstFetchLoc70 = crate::Reg<cpsw_nc_est_fetch_loc_70::CpswNcEstFetchLoc70Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_70;
#[doc = "CPSW_NC_EST_FETCH_LOC_71 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_71`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_71")]
pub type CpswNcEstFetchLoc71 = crate::Reg<cpsw_nc_est_fetch_loc_71::CpswNcEstFetchLoc71Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_71;
#[doc = "CPSW_NC_EST_FETCH_LOC_72 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_72`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_72")]
pub type CpswNcEstFetchLoc72 = crate::Reg<cpsw_nc_est_fetch_loc_72::CpswNcEstFetchLoc72Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_72;
#[doc = "CPSW_NC_EST_FETCH_LOC_73 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_73`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_73")]
pub type CpswNcEstFetchLoc73 = crate::Reg<cpsw_nc_est_fetch_loc_73::CpswNcEstFetchLoc73Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_73;
#[doc = "CPSW_NC_EST_FETCH_LOC_74 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_74`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_74")]
pub type CpswNcEstFetchLoc74 = crate::Reg<cpsw_nc_est_fetch_loc_74::CpswNcEstFetchLoc74Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_74;
#[doc = "CPSW_NC_EST_FETCH_LOC_75 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_75`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_75")]
pub type CpswNcEstFetchLoc75 = crate::Reg<cpsw_nc_est_fetch_loc_75::CpswNcEstFetchLoc75Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_75;
#[doc = "CPSW_NC_EST_FETCH_LOC_76 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_76`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_76")]
pub type CpswNcEstFetchLoc76 = crate::Reg<cpsw_nc_est_fetch_loc_76::CpswNcEstFetchLoc76Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_76;
#[doc = "CPSW_NC_EST_FETCH_LOC_77 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_77`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_77")]
pub type CpswNcEstFetchLoc77 = crate::Reg<cpsw_nc_est_fetch_loc_77::CpswNcEstFetchLoc77Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_77;
#[doc = "CPSW_NC_EST_FETCH_LOC_78 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_78`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_78")]
pub type CpswNcEstFetchLoc78 = crate::Reg<cpsw_nc_est_fetch_loc_78::CpswNcEstFetchLoc78Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_78;
#[doc = "CPSW_NC_EST_FETCH_LOC_79 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_79`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_79")]
pub type CpswNcEstFetchLoc79 = crate::Reg<cpsw_nc_est_fetch_loc_79::CpswNcEstFetchLoc79Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_79;
#[doc = "CPSW_NC_EST_FETCH_LOC_80 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_80`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_80")]
pub type CpswNcEstFetchLoc80 = crate::Reg<cpsw_nc_est_fetch_loc_80::CpswNcEstFetchLoc80Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_80;
#[doc = "CPSW_NC_EST_FETCH_LOC_81 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_81`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_81")]
pub type CpswNcEstFetchLoc81 = crate::Reg<cpsw_nc_est_fetch_loc_81::CpswNcEstFetchLoc81Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_81;
#[doc = "CPSW_NC_EST_FETCH_LOC_82 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_82`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_82")]
pub type CpswNcEstFetchLoc82 = crate::Reg<cpsw_nc_est_fetch_loc_82::CpswNcEstFetchLoc82Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_82;
#[doc = "CPSW_NC_EST_FETCH_LOC_83 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_83`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_83")]
pub type CpswNcEstFetchLoc83 = crate::Reg<cpsw_nc_est_fetch_loc_83::CpswNcEstFetchLoc83Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_83;
#[doc = "CPSW_NC_EST_FETCH_LOC_84 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_84`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_84")]
pub type CpswNcEstFetchLoc84 = crate::Reg<cpsw_nc_est_fetch_loc_84::CpswNcEstFetchLoc84Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_84;
#[doc = "CPSW_NC_EST_FETCH_LOC_85 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_85`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_85")]
pub type CpswNcEstFetchLoc85 = crate::Reg<cpsw_nc_est_fetch_loc_85::CpswNcEstFetchLoc85Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_85;
#[doc = "CPSW_NC_EST_FETCH_LOC_86 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_86`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_86")]
pub type CpswNcEstFetchLoc86 = crate::Reg<cpsw_nc_est_fetch_loc_86::CpswNcEstFetchLoc86Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_86;
#[doc = "CPSW_NC_EST_FETCH_LOC_87 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_87`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_87")]
pub type CpswNcEstFetchLoc87 = crate::Reg<cpsw_nc_est_fetch_loc_87::CpswNcEstFetchLoc87Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_87;
#[doc = "CPSW_NC_EST_FETCH_LOC_88 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_88`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_88")]
pub type CpswNcEstFetchLoc88 = crate::Reg<cpsw_nc_est_fetch_loc_88::CpswNcEstFetchLoc88Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_88;
#[doc = "CPSW_NC_EST_FETCH_LOC_89 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_89`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_89")]
pub type CpswNcEstFetchLoc89 = crate::Reg<cpsw_nc_est_fetch_loc_89::CpswNcEstFetchLoc89Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_89;
#[doc = "CPSW_NC_EST_FETCH_LOC_90 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_90`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_90")]
pub type CpswNcEstFetchLoc90 = crate::Reg<cpsw_nc_est_fetch_loc_90::CpswNcEstFetchLoc90Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_90;
#[doc = "CPSW_NC_EST_FETCH_LOC_91 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_91`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_91")]
pub type CpswNcEstFetchLoc91 = crate::Reg<cpsw_nc_est_fetch_loc_91::CpswNcEstFetchLoc91Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_91;
#[doc = "CPSW_NC_EST_FETCH_LOC_92 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_92`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_92")]
pub type CpswNcEstFetchLoc92 = crate::Reg<cpsw_nc_est_fetch_loc_92::CpswNcEstFetchLoc92Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_92;
#[doc = "CPSW_NC_EST_FETCH_LOC_93 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_93`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_93")]
pub type CpswNcEstFetchLoc93 = crate::Reg<cpsw_nc_est_fetch_loc_93::CpswNcEstFetchLoc93Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_93;
#[doc = "CPSW_NC_EST_FETCH_LOC_94 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_94`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_94")]
pub type CpswNcEstFetchLoc94 = crate::Reg<cpsw_nc_est_fetch_loc_94::CpswNcEstFetchLoc94Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_94;
#[doc = "CPSW_NC_EST_FETCH_LOC_95 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_95`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_95")]
pub type CpswNcEstFetchLoc95 = crate::Reg<cpsw_nc_est_fetch_loc_95::CpswNcEstFetchLoc95Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_95;
#[doc = "CPSW_NC_EST_FETCH_LOC_96 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_96`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_96")]
pub type CpswNcEstFetchLoc96 = crate::Reg<cpsw_nc_est_fetch_loc_96::CpswNcEstFetchLoc96Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_96;
#[doc = "CPSW_NC_EST_FETCH_LOC_97 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_97`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_97")]
pub type CpswNcEstFetchLoc97 = crate::Reg<cpsw_nc_est_fetch_loc_97::CpswNcEstFetchLoc97Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_97;
#[doc = "CPSW_NC_EST_FETCH_LOC_98 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_98`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_98")]
pub type CpswNcEstFetchLoc98 = crate::Reg<cpsw_nc_est_fetch_loc_98::CpswNcEstFetchLoc98Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_98;
#[doc = "CPSW_NC_EST_FETCH_LOC_99 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_99`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_99")]
pub type CpswNcEstFetchLoc99 = crate::Reg<cpsw_nc_est_fetch_loc_99::CpswNcEstFetchLoc99Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_99;
#[doc = "CPSW_NC_EST_FETCH_LOC_100 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_100`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_100")]
pub type CpswNcEstFetchLoc100 = crate::Reg<cpsw_nc_est_fetch_loc_100::CpswNcEstFetchLoc100Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_100;
#[doc = "CPSW_NC_EST_FETCH_LOC_101 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_101`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_101")]
pub type CpswNcEstFetchLoc101 = crate::Reg<cpsw_nc_est_fetch_loc_101::CpswNcEstFetchLoc101Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_101;
#[doc = "CPSW_NC_EST_FETCH_LOC_102 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_102`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_102")]
pub type CpswNcEstFetchLoc102 = crate::Reg<cpsw_nc_est_fetch_loc_102::CpswNcEstFetchLoc102Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_102;
#[doc = "CPSW_NC_EST_FETCH_LOC_103 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_103`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_103")]
pub type CpswNcEstFetchLoc103 = crate::Reg<cpsw_nc_est_fetch_loc_103::CpswNcEstFetchLoc103Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_103;
#[doc = "CPSW_NC_EST_FETCH_LOC_104 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_104`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_104")]
pub type CpswNcEstFetchLoc104 = crate::Reg<cpsw_nc_est_fetch_loc_104::CpswNcEstFetchLoc104Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_104;
#[doc = "CPSW_NC_EST_FETCH_LOC_105 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_105`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_105")]
pub type CpswNcEstFetchLoc105 = crate::Reg<cpsw_nc_est_fetch_loc_105::CpswNcEstFetchLoc105Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_105;
#[doc = "CPSW_NC_EST_FETCH_LOC_106 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_106`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_106")]
pub type CpswNcEstFetchLoc106 = crate::Reg<cpsw_nc_est_fetch_loc_106::CpswNcEstFetchLoc106Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_106;
#[doc = "CPSW_NC_EST_FETCH_LOC_107 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_107`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_107")]
pub type CpswNcEstFetchLoc107 = crate::Reg<cpsw_nc_est_fetch_loc_107::CpswNcEstFetchLoc107Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_107;
#[doc = "CPSW_NC_EST_FETCH_LOC_108 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_108`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_108")]
pub type CpswNcEstFetchLoc108 = crate::Reg<cpsw_nc_est_fetch_loc_108::CpswNcEstFetchLoc108Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_108;
#[doc = "CPSW_NC_EST_FETCH_LOC_109 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_109`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_109")]
pub type CpswNcEstFetchLoc109 = crate::Reg<cpsw_nc_est_fetch_loc_109::CpswNcEstFetchLoc109Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_109;
#[doc = "CPSW_NC_EST_FETCH_LOC_110 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_110`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_110")]
pub type CpswNcEstFetchLoc110 = crate::Reg<cpsw_nc_est_fetch_loc_110::CpswNcEstFetchLoc110Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_110;
#[doc = "CPSW_NC_EST_FETCH_LOC_111 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_111`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_111")]
pub type CpswNcEstFetchLoc111 = crate::Reg<cpsw_nc_est_fetch_loc_111::CpswNcEstFetchLoc111Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_111;
#[doc = "CPSW_NC_EST_FETCH_LOC_112 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_112`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_112")]
pub type CpswNcEstFetchLoc112 = crate::Reg<cpsw_nc_est_fetch_loc_112::CpswNcEstFetchLoc112Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_112;
#[doc = "CPSW_NC_EST_FETCH_LOC_113 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_113`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_113")]
pub type CpswNcEstFetchLoc113 = crate::Reg<cpsw_nc_est_fetch_loc_113::CpswNcEstFetchLoc113Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_113;
#[doc = "CPSW_NC_EST_FETCH_LOC_114 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_114`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_114")]
pub type CpswNcEstFetchLoc114 = crate::Reg<cpsw_nc_est_fetch_loc_114::CpswNcEstFetchLoc114Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_114;
#[doc = "CPSW_NC_EST_FETCH_LOC_115 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_115`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_115")]
pub type CpswNcEstFetchLoc115 = crate::Reg<cpsw_nc_est_fetch_loc_115::CpswNcEstFetchLoc115Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_115;
#[doc = "CPSW_NC_EST_FETCH_LOC_116 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_116`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_116")]
pub type CpswNcEstFetchLoc116 = crate::Reg<cpsw_nc_est_fetch_loc_116::CpswNcEstFetchLoc116Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_116;
#[doc = "CPSW_NC_EST_FETCH_LOC_117 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_117`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_117")]
pub type CpswNcEstFetchLoc117 = crate::Reg<cpsw_nc_est_fetch_loc_117::CpswNcEstFetchLoc117Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_117;
#[doc = "CPSW_NC_EST_FETCH_LOC_118 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_118`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_118")]
pub type CpswNcEstFetchLoc118 = crate::Reg<cpsw_nc_est_fetch_loc_118::CpswNcEstFetchLoc118Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_118;
#[doc = "CPSW_NC_EST_FETCH_LOC_119 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_119`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_119")]
pub type CpswNcEstFetchLoc119 = crate::Reg<cpsw_nc_est_fetch_loc_119::CpswNcEstFetchLoc119Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_119;
#[doc = "CPSW_NC_EST_FETCH_LOC_120 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_120`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_120")]
pub type CpswNcEstFetchLoc120 = crate::Reg<cpsw_nc_est_fetch_loc_120::CpswNcEstFetchLoc120Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_120;
#[doc = "CPSW_NC_EST_FETCH_LOC_121 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_121`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_121")]
pub type CpswNcEstFetchLoc121 = crate::Reg<cpsw_nc_est_fetch_loc_121::CpswNcEstFetchLoc121Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_121;
#[doc = "CPSW_NC_EST_FETCH_LOC_122 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_122`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_122")]
pub type CpswNcEstFetchLoc122 = crate::Reg<cpsw_nc_est_fetch_loc_122::CpswNcEstFetchLoc122Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_122;
#[doc = "CPSW_NC_EST_FETCH_LOC_123 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_123`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_123")]
pub type CpswNcEstFetchLoc123 = crate::Reg<cpsw_nc_est_fetch_loc_123::CpswNcEstFetchLoc123Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_123;
#[doc = "CPSW_NC_EST_FETCH_LOC_124 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_124`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_124")]
pub type CpswNcEstFetchLoc124 = crate::Reg<cpsw_nc_est_fetch_loc_124::CpswNcEstFetchLoc124Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_124;
#[doc = "CPSW_NC_EST_FETCH_LOC_125 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_125`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_125")]
pub type CpswNcEstFetchLoc125 = crate::Reg<cpsw_nc_est_fetch_loc_125::CpswNcEstFetchLoc125Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_125;
#[doc = "CPSW_NC_EST_FETCH_LOC_126 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_126`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_126")]
pub type CpswNcEstFetchLoc126 = crate::Reg<cpsw_nc_est_fetch_loc_126::CpswNcEstFetchLoc126Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_126;
#[doc = "CPSW_NC_EST_FETCH_LOC_127 (rw) register accessor: The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_est_fetch_loc_127`]
module"]
#[doc(alias = "CPSW_NC_EST_FETCH_LOC_127")]
pub type CpswNcEstFetchLoc127 = crate::Reg<cpsw_nc_est_fetch_loc_127::CpswNcEstFetchLoc127Spec>;
#[doc = "The Revision Register contains the ID and revision information."]
pub mod cpsw_nc_est_fetch_loc_127;
#[doc = "CPSW_CPDMA_REGS_CPDMA_FH_IDVER_REG (rw) register accessor: CPDMA FHost IDVER\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_idver_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_idver_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_fh_idver_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_FH_IDVER_REG")]
pub type CpswCpdmaRegsCpdmaFhIdverReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_fh_idver_reg::CpswCpdmaRegsCpdmaFhIdverRegSpec>;
#[doc = "CPDMA FHost IDVER"]
pub mod cpsw_cpdma_regs_cpdma_fh_idver_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_FH_CONTROL_REG (rw) register accessor: CPDMA FHost Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_fh_control_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_FH_CONTROL_REG")]
pub type CpswCpdmaRegsCpdmaFhControlReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_fh_control_reg::CpswCpdmaRegsCpdmaFhControlRegSpec>;
#[doc = "CPDMA FHost Control Register"]
pub mod cpsw_cpdma_regs_cpdma_fh_control_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_FH_TEARDOWN_REG (rw) register accessor: CPDMA FHost Teardown Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_teardown_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_fh_teardown_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_FH_TEARDOWN_REG")]
pub type CpswCpdmaRegsCpdmaFhTeardownReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_fh_teardown_reg::CpswCpdmaRegsCpdmaFhTeardownRegSpec>;
#[doc = "CPDMA FHost Teardown Register"]
pub mod cpsw_cpdma_regs_cpdma_fh_teardown_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_FH_EOQ_INT (rw) register accessor: CPDMA FHost Interrupt on EOQ only Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_eoq_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_eoq_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_fh_eoq_int`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_FH_EOQ_INT")]
pub type CpswCpdmaRegsCpdmaFhEoqInt =
    crate::Reg<cpsw_cpdma_regs_cpdma_fh_eoq_int::CpswCpdmaRegsCpdmaFhEoqIntSpec>;
#[doc = "CPDMA FHost Interrupt on EOQ only Register"]
pub mod cpsw_cpdma_regs_cpdma_fh_eoq_int;
#[doc = "CPSW_CPDMA_REGS_CPDMA_TH_IDVER_REG (rw) register accessor: CPDMA THost IDVER\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_idver_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_idver_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_th_idver_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_TH_IDVER_REG")]
pub type CpswCpdmaRegsCpdmaThIdverReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_th_idver_reg::CpswCpdmaRegsCpdmaThIdverRegSpec>;
#[doc = "CPDMA THost IDVER"]
pub mod cpsw_cpdma_regs_cpdma_th_idver_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_TH_CONTROL_REG (rw) register accessor: CPDMA THost Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_th_control_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_TH_CONTROL_REG")]
pub type CpswCpdmaRegsCpdmaThControlReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_th_control_reg::CpswCpdmaRegsCpdmaThControlRegSpec>;
#[doc = "CPDMA THost Control Register"]
pub mod cpsw_cpdma_regs_cpdma_th_control_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_TH_TEARDOWN_REG (rw) register accessor: CPDMA THost Teardown Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_teardown_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_teardown_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_th_teardown_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_TH_TEARDOWN_REG")]
pub type CpswCpdmaRegsCpdmaThTeardownReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_th_teardown_reg::CpswCpdmaRegsCpdmaThTeardownRegSpec>;
#[doc = "CPDMA THost Teardown Register"]
pub mod cpsw_cpdma_regs_cpdma_th_teardown_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_SOFT_RESET_REG (rw) register accessor: CPDMA Soft Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_soft_reset_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_soft_reset_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_soft_reset_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_SOFT_RESET_REG")]
pub type CpswCpdmaRegsCpdmaSoftResetReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_soft_reset_reg::CpswCpdmaRegsCpdmaSoftResetRegSpec>;
#[doc = "CPDMA Soft Reset Register"]
pub mod cpsw_cpdma_regs_cpdma_soft_reset_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_CONTROL_REG (rw) register accessor: CPDMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_control_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_CONTROL_REG")]
pub type CpswCpdmaRegsCpdmaControlReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_control_reg::CpswCpdmaRegsCpdmaControlRegSpec>;
#[doc = "CPDMA Control Register"]
pub mod cpsw_cpdma_regs_cpdma_control_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_STATUS_REG (rw) register accessor: CPDMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_status_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_STATUS_REG")]
pub type CpswCpdmaRegsCpdmaStatusReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_status_reg::CpswCpdmaRegsCpdmaStatusRegSpec>;
#[doc = "CPDMA Status Register"]
pub mod cpsw_cpdma_regs_cpdma_status_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_TH_BUFFER_OFFSET_REG (rw) register accessor: CPDMA THost Buffer Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_th_buffer_offset_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_TH_BUFFER_OFFSET_REG")]
pub type CpswCpdmaRegsCpdmaThBufferOffsetReg =
    crate::Reg<cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::CpswCpdmaRegsCpdmaThBufferOffsetRegSpec>;
#[doc = "CPDMA THost Buffer Offset Register"]
pub mod cpsw_cpdma_regs_cpdma_th_buffer_offset_reg;
#[doc = "CPSW_CPDMA_REGS_CPDMA_EMULATION_CONTROL_REG (rw) register accessor: CPDMA THost Buffer Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_emulation_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_emulation_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_regs_cpdma_emulation_control_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_REGS_CPDMA_EMULATION_CONTROL_REG")]
pub type CpswCpdmaRegsCpdmaEmulationControlReg = crate::Reg<
    cpsw_cpdma_regs_cpdma_emulation_control_reg::CpswCpdmaRegsCpdmaEmulationControlRegSpec,
>;
#[doc = "CPDMA THost Buffer Offset Register"]
pub mod cpsw_cpdma_regs_cpdma_emulation_control_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_FH_INTSTAT_RAW_REG (rw) register accessor: CPDMA FHost Interrupt Status RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_fh_intstat_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_fh_intstat_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_fh_intstat_raw_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_FH_INTSTAT_RAW_REG")]
pub type CpswCpdmaIntCpdmaFhIntstatRawReg =
    crate::Reg<cpsw_cpdma_int_cpdma_fh_intstat_raw_reg::CpswCpdmaIntCpdmaFhIntstatRawRegSpec>;
#[doc = "CPDMA FHost Interrupt Status RAW"]
pub mod cpsw_cpdma_int_cpdma_fh_intstat_raw_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_FH_INTSTAT_MASKED_REG (rw) register accessor: CPDMA FHost Interrupt Status MASKED\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_fh_intstat_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_fh_intstat_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_fh_intstat_masked_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_FH_INTSTAT_MASKED_REG")]
pub type CpswCpdmaIntCpdmaFhIntstatMaskedReg =
    crate::Reg<cpsw_cpdma_int_cpdma_fh_intstat_masked_reg::CpswCpdmaIntCpdmaFhIntstatMaskedRegSpec>;
#[doc = "CPDMA FHost Interrupt Status MASKED"]
pub mod cpsw_cpdma_int_cpdma_fh_intstat_masked_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG (rw) register accessor: CPDMA FHost Interrupt Masked SET\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_fh_intmask_set_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG")]
pub type CpswCpdmaIntCpdmaFhIntmaskSetReg =
    crate::Reg<cpsw_cpdma_int_cpdma_fh_intmask_set_reg::CpswCpdmaIntCpdmaFhIntmaskSetRegSpec>;
#[doc = "CPDMA FHost Interrupt Masked SET"]
pub mod cpsw_cpdma_int_cpdma_fh_intmask_set_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_FH_INTMASK_CLEAR_REG (rw) register accessor: CPDMA FHost Interrupt Masked CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_fh_intmask_clear_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_fh_intmask_clear_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_fh_intmask_clear_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_FH_INTMASK_CLEAR_REG")]
pub type CpswCpdmaIntCpdmaFhIntmaskClearReg =
    crate::Reg<cpsw_cpdma_int_cpdma_fh_intmask_clear_reg::CpswCpdmaIntCpdmaFhIntmaskClearRegSpec>;
#[doc = "CPDMA FHost Interrupt Masked CLR"]
pub mod cpsw_cpdma_int_cpdma_fh_intmask_clear_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_IN_VECTOR_REG (rw) register accessor: CPDMA DMA IN Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_in_vector_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_in_vector_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_in_vector_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_IN_VECTOR_REG")]
pub type CpswCpdmaIntCpdmaInVectorReg =
    crate::Reg<cpsw_cpdma_int_cpdma_in_vector_reg::CpswCpdmaIntCpdmaInVectorRegSpec>;
#[doc = "CPDMA DMA IN Vector"]
pub mod cpsw_cpdma_int_cpdma_in_vector_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_EOI_VECTOR_REG (rw) register accessor: CPDMA DMA EOI Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_eoi_vector_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_eoi_vector_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_eoi_vector_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_EOI_VECTOR_REG")]
pub type CpswCpdmaIntCpdmaEoiVectorReg =
    crate::Reg<cpsw_cpdma_int_cpdma_eoi_vector_reg::CpswCpdmaIntCpdmaEoiVectorRegSpec>;
#[doc = "CPDMA DMA EOI Vector"]
pub mod cpsw_cpdma_int_cpdma_eoi_vector_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH_INTSTAT_RAW_REG (rw) register accessor: CPDMA Receive Interrupt Status RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th_intstat_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th_intstat_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th_intstat_raw_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH_INTSTAT_RAW_REG")]
pub type CpswCpdmaIntCpdmaThIntstatRawReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th_intstat_raw_reg::CpswCpdmaIntCpdmaThIntstatRawRegSpec>;
#[doc = "CPDMA Receive Interrupt Status RAW"]
pub mod cpsw_cpdma_int_cpdma_th_intstat_raw_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH_INTSTAT_MASKED_REG (rw) register accessor: CPDMA Receive Interrupt Status MASKED\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th_intstat_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th_intstat_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th_intstat_masked_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH_INTSTAT_MASKED_REG")]
pub type CpswCpdmaIntCpdmaThIntstatMaskedReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th_intstat_masked_reg::CpswCpdmaIntCpdmaThIntstatMaskedRegSpec>;
#[doc = "CPDMA Receive Interrupt Status MASKED"]
pub mod cpsw_cpdma_int_cpdma_th_intstat_masked_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH_INTMASK_SET_REG (rw) register accessor: CPDMA THost Interrupt Masked SET\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th_intmask_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th_intmask_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th_intmask_set_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH_INTMASK_SET_REG")]
pub type CpswCpdmaIntCpdmaThIntmaskSetReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th_intmask_set_reg::CpswCpdmaIntCpdmaThIntmaskSetRegSpec>;
#[doc = "CPDMA THost Interrupt Masked SET"]
pub mod cpsw_cpdma_int_cpdma_th_intmask_set_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH_INTMASK_CLEAR_REG (rw) register accessor: CPDMA THost Interrupt Masked CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th_intmask_clear_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th_intmask_clear_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th_intmask_clear_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH_INTMASK_CLEAR_REG")]
pub type CpswCpdmaIntCpdmaThIntmaskClearReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th_intmask_clear_reg::CpswCpdmaIntCpdmaThIntmaskClearRegSpec>;
#[doc = "CPDMA THost Interrupt Masked CLR"]
pub mod cpsw_cpdma_int_cpdma_th_intmask_clear_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_INTSTAT_RAW_REG (rw) register accessor: CPDMA DMA Interrupt Status RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_intstat_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_intstat_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_intstat_raw_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_INTSTAT_RAW_REG")]
pub type CpswCpdmaIntCpdmaIntstatRawReg =
    crate::Reg<cpsw_cpdma_int_cpdma_intstat_raw_reg::CpswCpdmaIntCpdmaIntstatRawRegSpec>;
#[doc = "CPDMA DMA Interrupt Status RAW"]
pub mod cpsw_cpdma_int_cpdma_intstat_raw_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_INTSTAT_MASKED_REG (rw) register accessor: CPDMA DMA Interrupt Status MASKED\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_intstat_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_intstat_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_intstat_masked_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_INTSTAT_MASKED_REG")]
pub type CpswCpdmaIntCpdmaIntstatMaskedReg =
    crate::Reg<cpsw_cpdma_int_cpdma_intstat_masked_reg::CpswCpdmaIntCpdmaIntstatMaskedRegSpec>;
#[doc = "CPDMA DMA Interrupt Status MASKED"]
pub mod cpsw_cpdma_int_cpdma_intstat_masked_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_INTMASK_SET_REG (rw) register accessor: CPDMA DMA Interrupt Status SET\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_intmask_set_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_intmask_set_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_intmask_set_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_INTMASK_SET_REG")]
pub type CpswCpdmaIntCpdmaIntmaskSetReg =
    crate::Reg<cpsw_cpdma_int_cpdma_intmask_set_reg::CpswCpdmaIntCpdmaIntmaskSetRegSpec>;
#[doc = "CPDMA DMA Interrupt Status SET"]
pub mod cpsw_cpdma_int_cpdma_intmask_set_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_INTMASK_CLEAR_REG (rw) register accessor: CPDMA DMA Interrupt Status CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_intmask_clear_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_intmask_clear_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_intmask_clear_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_INTMASK_CLEAR_REG")]
pub type CpswCpdmaIntCpdmaIntmaskClearReg =
    crate::Reg<cpsw_cpdma_int_cpdma_intmask_clear_reg::CpswCpdmaIntCpdmaIntmaskClearRegSpec>;
#[doc = "CPDMA DMA Interrupt Status CLR"]
pub mod cpsw_cpdma_int_cpdma_intmask_clear_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH0_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th0_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th0_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th0_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH0_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh0PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th0_pendthresh_reg::CpswCpdmaIntCpdmaTh0PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th0_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH1_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th1_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th1_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th1_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH1_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh1PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th1_pendthresh_reg::CpswCpdmaIntCpdmaTh1PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th1_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH2_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th2_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH2_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh2PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th2_pendthresh_reg::CpswCpdmaIntCpdmaTh2PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th2_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH3_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th3_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th3_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th3_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH3_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh3PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th3_pendthresh_reg::CpswCpdmaIntCpdmaTh3PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th3_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH4_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th4_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th4_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th4_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH4_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh4PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th4_pendthresh_reg::CpswCpdmaIntCpdmaTh4PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th4_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH5_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th5_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th5_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th5_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH5_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh5PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th5_pendthresh_reg::CpswCpdmaIntCpdmaTh5PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th5_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH6_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th6_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th6_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th6_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH6_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh6PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th6_pendthresh_reg::CpswCpdmaIntCpdmaTh6PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th6_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH7_PENDTHRESH_REG (rw) register accessor: CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th7_pendthresh_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th7_pendthresh_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th7_pendthresh_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH7_PENDTHRESH_REG")]
pub type CpswCpdmaIntCpdmaTh7PendthreshReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th7_pendthresh_reg::CpswCpdmaIntCpdmaTh7PendthreshRegSpec>;
#[doc = "CPDMA THost Threshold Pending Register"]
pub mod cpsw_cpdma_int_cpdma_th7_pendthresh_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH0_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th0_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th0_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th0_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH0_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh0FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th0_freebuffer_reg::CpswCpdmaIntCpdmaTh0FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th0_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH1_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th1_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th1_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th1_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH1_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh1FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th1_freebuffer_reg::CpswCpdmaIntCpdmaTh1FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th1_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH2_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th2_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th2_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th2_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH2_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh2FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th2_freebuffer_reg::CpswCpdmaIntCpdmaTh2FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th2_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH3_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th3_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH3_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh3FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th3_freebuffer_reg::CpswCpdmaIntCpdmaTh3FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th3_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH4_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th4_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th4_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th4_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH4_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh4FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th4_freebuffer_reg::CpswCpdmaIntCpdmaTh4FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th4_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH5_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th5_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th5_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th5_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH5_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh5FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th5_freebuffer_reg::CpswCpdmaIntCpdmaTh5FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th5_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH6_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th6_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th6_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th6_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH6_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh6FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th6_freebuffer_reg::CpswCpdmaIntCpdmaTh6FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th6_freebuffer_reg;
#[doc = "CPSW_CPDMA_INT_CPDMA_TH7_FREEBUFFER_REG (rw) register accessor: CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th7_freebuffer_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th7_freebuffer_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_int_cpdma_th7_freebuffer_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_INT_CPDMA_TH7_FREEBUFFER_REG")]
pub type CpswCpdmaIntCpdmaTh7FreebufferReg =
    crate::Reg<cpsw_cpdma_int_cpdma_th7_freebuffer_reg::CpswCpdmaIntCpdmaTh7FreebufferRegSpec>;
#[doc = "CPDMA THost Free Buffer Count Register"]
pub mod cpsw_cpdma_int_cpdma_th7_freebuffer_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH0_HDP_REG (rw) register accessor: CPDMA FHost Channel 0 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh0_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh0_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh0_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH0_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh0HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh0_hdp_reg::CpswCpdmaSramCpdmaFh0HdpRegSpec>;
#[doc = "CPDMA FHost Channel 0 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh0_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH1_HDP_REG (rw) register accessor: CPDMA FHost Channel 1 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh1_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh1_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh1_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH1_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh1HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh1_hdp_reg::CpswCpdmaSramCpdmaFh1HdpRegSpec>;
#[doc = "CPDMA FHost Channel 1 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh1_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH2_HDP_REG (rw) register accessor: CPDMA FHost Channel 2 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh2_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh2_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh2_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH2_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh2HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh2_hdp_reg::CpswCpdmaSramCpdmaFh2HdpRegSpec>;
#[doc = "CPDMA FHost Channel 2 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh2_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH3_HDP_REG (rw) register accessor: CPDMA FHost Channel 3 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh3_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh3_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh3_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH3_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh3HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh3_hdp_reg::CpswCpdmaSramCpdmaFh3HdpRegSpec>;
#[doc = "CPDMA FHost Channel 3 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh3_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH4_HDP_REG (rw) register accessor: CPDMA FHost Channel 4 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh4_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh4_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh4_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH4_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh4HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh4_hdp_reg::CpswCpdmaSramCpdmaFh4HdpRegSpec>;
#[doc = "CPDMA FHost Channel 4 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh4_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH5_HDP_REG (rw) register accessor: CPDMA FHost Channel 5 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh5_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH5_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh5HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh5_hdp_reg::CpswCpdmaSramCpdmaFh5HdpRegSpec>;
#[doc = "CPDMA FHost Channel 5 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh5_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH6_HDP_REG (rw) register accessor: CPDMA FHost Channel 6 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh6_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh6_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh6_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH6_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh6HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh6_hdp_reg::CpswCpdmaSramCpdmaFh6HdpRegSpec>;
#[doc = "CPDMA FHost Channel 6 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh6_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH7_HDP_REG (rw) register accessor: CPDMA FHost Channel 7 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh7_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh7_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh7_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH7_HDP_REG")]
pub type CpswCpdmaSramCpdmaFh7HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh7_hdp_reg::CpswCpdmaSramCpdmaFh7HdpRegSpec>;
#[doc = "CPDMA FHost Channel 7 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh7_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH0_HDP_REG (rw) register accessor: CPDMA THost Channel 0 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th0_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th0_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th0_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH0_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh0HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th0_hdp_reg::CpswCpdmaSramCpdmaTh0HdpRegSpec>;
#[doc = "CPDMA THost Channel 0 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th0_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH1_HDP_REG (rw) register accessor: CPDMA THost Channel 1 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th1_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th1_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th1_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH1_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh1HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th1_hdp_reg::CpswCpdmaSramCpdmaTh1HdpRegSpec>;
#[doc = "CPDMA THost Channel 1 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th1_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH2_HDP_REG (rw) register accessor: CPDMA THost Channel 2 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th2_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th2_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th2_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH2_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh2HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th2_hdp_reg::CpswCpdmaSramCpdmaTh2HdpRegSpec>;
#[doc = "CPDMA THost Channel 2 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th2_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH3_HDP_REG (rw) register accessor: CPDMA THost Channel 3 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th3_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th3_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th3_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH3_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh3HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th3_hdp_reg::CpswCpdmaSramCpdmaTh3HdpRegSpec>;
#[doc = "CPDMA THost Channel 3 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th3_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH4_HDP_REG (rw) register accessor: CPDMA THost Channel 4 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th4_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th4_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th4_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH4_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh4HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th4_hdp_reg::CpswCpdmaSramCpdmaTh4HdpRegSpec>;
#[doc = "CPDMA THost Channel 4 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th4_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH5_HDP_REG (rw) register accessor: CPDMA THost Channel 5 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th5_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th5_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th5_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH5_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh5HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th5_hdp_reg::CpswCpdmaSramCpdmaTh5HdpRegSpec>;
#[doc = "CPDMA THost Channel 5 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th5_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH6_HDP_REG (rw) register accessor: CPDMA THost Channel 6 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th6_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th6_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th6_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH6_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh6HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th6_hdp_reg::CpswCpdmaSramCpdmaTh6HdpRegSpec>;
#[doc = "CPDMA THost Channel 6 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th6_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH7_HDP_REG (rw) register accessor: CPDMA THost Channel 7 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th7_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th7_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th7_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH7_HDP_REG")]
pub type CpswCpdmaSramCpdmaTh7HdpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th7_hdp_reg::CpswCpdmaSramCpdmaTh7HdpRegSpec>;
#[doc = "CPDMA THost Channel 7 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th7_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH0_CP_REG (rw) register accessor: CPDMA FHost Channel 0 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh0_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh0_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh0_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH0_CP_REG")]
pub type CpswCpdmaSramCpdmaFh0CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh0_cp_reg::CpswCpdmaSramCpdmaFh0CpRegSpec>;
#[doc = "CPDMA FHost Channel 0 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh0_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH1_CP_REG (rw) register accessor: CPDMA FHost Channel 1 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh1_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh1_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh1_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH1_CP_REG")]
pub type CpswCpdmaSramCpdmaFh1CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh1_cp_reg::CpswCpdmaSramCpdmaFh1CpRegSpec>;
#[doc = "CPDMA FHost Channel 1 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh1_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH2_CP_REG (rw) register accessor: CPDMA FHost Channel 2 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh2_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh2_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh2_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH2_CP_REG")]
pub type CpswCpdmaSramCpdmaFh2CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh2_cp_reg::CpswCpdmaSramCpdmaFh2CpRegSpec>;
#[doc = "CPDMA FHost Channel 2 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh2_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH3_CP_REG (rw) register accessor: CPDMA FHost Channel 3 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh3_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh3_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh3_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH3_CP_REG")]
pub type CpswCpdmaSramCpdmaFh3CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh3_cp_reg::CpswCpdmaSramCpdmaFh3CpRegSpec>;
#[doc = "CPDMA FHost Channel 3 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh3_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH4_CP_REG (rw) register accessor: CPDMA FHost Channel 4 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh4_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh4_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh4_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH4_CP_REG")]
pub type CpswCpdmaSramCpdmaFh4CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh4_cp_reg::CpswCpdmaSramCpdmaFh4CpRegSpec>;
#[doc = "CPDMA FHost Channel 4 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh4_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH5_CP_REG (rw) register accessor: CPDMA FHost Channel 5 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh5_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh5_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh5_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH5_CP_REG")]
pub type CpswCpdmaSramCpdmaFh5CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh5_cp_reg::CpswCpdmaSramCpdmaFh5CpRegSpec>;
#[doc = "CPDMA FHost Channel 5 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh5_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH6_CP_REG (rw) register accessor: CPDMA FHost Channel 6 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh6_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh6_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh6_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH6_CP_REG")]
pub type CpswCpdmaSramCpdmaFh6CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh6_cp_reg::CpswCpdmaSramCpdmaFh6CpRegSpec>;
#[doc = "CPDMA FHost Channel 6 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh6_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_FH7_CP_REG (rw) register accessor: CPDMA FHost Channel 7 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh7_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh7_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_fh7_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_FH7_CP_REG")]
pub type CpswCpdmaSramCpdmaFh7CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_fh7_cp_reg::CpswCpdmaSramCpdmaFh7CpRegSpec>;
#[doc = "CPDMA FHost Channel 7 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_fh7_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH0_CP_REG (rw) register accessor: CPDMA THost Channel 0 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th0_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th0_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th0_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH0_CP_REG")]
pub type CpswCpdmaSramCpdmaTh0CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th0_cp_reg::CpswCpdmaSramCpdmaTh0CpRegSpec>;
#[doc = "CPDMA THost Channel 0 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th0_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH1_CP_REG (rw) register accessor: CPDMA THost Channel 1 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th1_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th1_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th1_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH1_CP_REG")]
pub type CpswCpdmaSramCpdmaTh1CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th1_cp_reg::CpswCpdmaSramCpdmaTh1CpRegSpec>;
#[doc = "CPDMA THost Channel 1 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th1_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH2_CP_REG (rw) register accessor: CPDMA THost Channel 2 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th2_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th2_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th2_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH2_CP_REG")]
pub type CpswCpdmaSramCpdmaTh2CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th2_cp_reg::CpswCpdmaSramCpdmaTh2CpRegSpec>;
#[doc = "CPDMA THost Channel 2 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th2_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH3_CP_REG (rw) register accessor: CPDMA THost Channel 3 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th3_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th3_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th3_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH3_CP_REG")]
pub type CpswCpdmaSramCpdmaTh3CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th3_cp_reg::CpswCpdmaSramCpdmaTh3CpRegSpec>;
#[doc = "CPDMA THost Channel 3 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th3_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH4_CP_REG (rw) register accessor: CPDMA THost Channel 4 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th4_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th4_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th4_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH4_CP_REG")]
pub type CpswCpdmaSramCpdmaTh4CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th4_cp_reg::CpswCpdmaSramCpdmaTh4CpRegSpec>;
#[doc = "CPDMA THost Channel 4 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th4_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH5_CP_REG (rw) register accessor: CPDMA THost Channel 5 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th5_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th5_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th5_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH5_CP_REG")]
pub type CpswCpdmaSramCpdmaTh5CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th5_cp_reg::CpswCpdmaSramCpdmaTh5CpRegSpec>;
#[doc = "CPDMA THost Channel 5 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th5_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH6_CP_REG (rw) register accessor: CPDMA THost Channel 6 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th6_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th6_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th6_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH6_CP_REG")]
pub type CpswCpdmaSramCpdmaTh6CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th6_cp_reg::CpswCpdmaSramCpdmaTh6CpRegSpec>;
#[doc = "CPDMA THost Channel 6 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th6_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_CPDMA_TH7_CP_REG (rw) register accessor: CPDMA THost Channel 7 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_th7_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_th7_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_cpdma_th7_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_CPDMA_TH7_CP_REG")]
pub type CpswCpdmaSramCpdmaTh7CpReg =
    crate::Reg<cpsw_cpdma_sram_cpdma_th7_cp_reg::CpswCpdmaSramCpdmaTh7CpRegSpec>;
#[doc = "CPDMA THost Channel 7 Completion Pointer"]
pub mod cpsw_cpdma_sram_cpdma_th7_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH0_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 0 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH0_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh0HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg::CpswCpdmaSramTestCpdmaFh0HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 0 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh0_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH1_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 1 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH1_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh1HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg::CpswCpdmaSramTestCpdmaFh1HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 1 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh1_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH2_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 2 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH2_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh2HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg::CpswCpdmaSramTestCpdmaFh2HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 2 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh2_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH3_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 3 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH3_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh3HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg::CpswCpdmaSramTestCpdmaFh3HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 3 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh3_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH4_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 4 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH4_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh4HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg::CpswCpdmaSramTestCpdmaFh4HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 4 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh4_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 5 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh5HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg::CpswCpdmaSramTestCpdmaFh5HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 5 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh5_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH6_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 6 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH6_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh6HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg::CpswCpdmaSramTestCpdmaFh6HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 6 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh6_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH7_HDP_REG (rw) register accessor: Test CPDMA FHost Channel 7 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH7_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaFh7HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg::CpswCpdmaSramTestCpdmaFh7HdpRegSpec>;
#[doc = "Test CPDMA FHost Channel 7 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh7_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH0_HDP_REG (rw) register accessor: Test CPDMA THost Channel 0 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th0_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th0_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th0_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH0_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh0HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th0_hdp_reg::CpswCpdmaSramTestCpdmaTh0HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 0 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th0_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH1_HDP_REG (rw) register accessor: Test CPDMA THost Channel 1 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th1_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th1_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th1_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH1_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh1HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th1_hdp_reg::CpswCpdmaSramTestCpdmaTh1HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 1 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th1_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH2_HDP_REG (rw) register accessor: Test CPDMA THost Channel 2 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th2_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th2_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th2_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH2_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh2HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th2_hdp_reg::CpswCpdmaSramTestCpdmaTh2HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 2 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th2_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_HDP_REG (rw) register accessor: Test CPDMA THost Channel 3 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th3_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh3HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::CpswCpdmaSramTestCpdmaTh3HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 3 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th3_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH4_HDP_REG (rw) register accessor: Test CPDMA THost Channel 4 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th4_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th4_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th4_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH4_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh4HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th4_hdp_reg::CpswCpdmaSramTestCpdmaTh4HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 4 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th4_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH5_HDP_REG (rw) register accessor: Test CPDMA THost Channel 5 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th5_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th5_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th5_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH5_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh5HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th5_hdp_reg::CpswCpdmaSramTestCpdmaTh5HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 5 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th5_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH6_HDP_REG (rw) register accessor: Test CPDMA THost Channel 6 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th6_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th6_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th6_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH6_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh6HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th6_hdp_reg::CpswCpdmaSramTestCpdmaTh6HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 6 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th6_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH7_HDP_REG (rw) register accessor: Test CPDMA THost Channel 7 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th7_hdp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th7_hdp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th7_hdp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH7_HDP_REG")]
pub type CpswCpdmaSramTestCpdmaTh7HdpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th7_hdp_reg::CpswCpdmaSramTestCpdmaTh7HdpRegSpec>;
#[doc = "Test CPDMA THost Channel 7 Head Descriptor Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th7_hdp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH0_CP_REG (rw) register accessor: Test CPDMA FHost Channel 0 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh0_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh0_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh0_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH0_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh0CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh0_cp_reg::CpswCpdmaSramTestCpdmaFh0CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 0 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh0_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH1_CP_REG (rw) register accessor: Test CPDMA FHost Channel 1 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh1_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh1_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh1_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH1_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh1CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh1_cp_reg::CpswCpdmaSramTestCpdmaFh1CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 1 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh1_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH2_CP_REG (rw) register accessor: Test CPDMA FHost Channel 2 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh2_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh2_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh2_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH2_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh2CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh2_cp_reg::CpswCpdmaSramTestCpdmaFh2CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 2 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh2_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH3_CP_REG (rw) register accessor: Test CPDMA FHost Channel 3 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh3_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh3_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh3_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH3_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh3CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh3_cp_reg::CpswCpdmaSramTestCpdmaFh3CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 3 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh3_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH4_CP_REG (rw) register accessor: Test CPDMA FHost Channel 4 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh4_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh4_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh4_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH4_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh4CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh4_cp_reg::CpswCpdmaSramTestCpdmaFh4CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 4 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh4_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_CP_REG (rw) register accessor: Test CPDMA FHost Channel 5 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh5_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh5CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::CpswCpdmaSramTestCpdmaFh5CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 5 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh5_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH6_CP_REG (rw) register accessor: Test CPDMA FHost Channel 6 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh6_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh6_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh6_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH6_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh6CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh6_cp_reg::CpswCpdmaSramTestCpdmaFh6CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 6 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh6_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH7_CP_REG (rw) register accessor: Test CPDMA FHost Channel 7 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh7_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh7_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_fh7_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_FH7_CP_REG")]
pub type CpswCpdmaSramTestCpdmaFh7CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_fh7_cp_reg::CpswCpdmaSramTestCpdmaFh7CpRegSpec>;
#[doc = "Test CPDMA FHost Channel 7 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_fh7_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH0_CP_REG (rw) register accessor: Test CPDMA THost Channel 0 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th0_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th0_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th0_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH0_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh0CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th0_cp_reg::CpswCpdmaSramTestCpdmaTh0CpRegSpec>;
#[doc = "Test CPDMA THost Channel 0 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th0_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH1_CP_REG (rw) register accessor: Test CPDMA THost Channel 1 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th1_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th1_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th1_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH1_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh1CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th1_cp_reg::CpswCpdmaSramTestCpdmaTh1CpRegSpec>;
#[doc = "Test CPDMA THost Channel 1 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th1_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH2_CP_REG (rw) register accessor: Test CPDMA THost Channel 2 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th2_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th2_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th2_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH2_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh2CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th2_cp_reg::CpswCpdmaSramTestCpdmaTh2CpRegSpec>;
#[doc = "Test CPDMA THost Channel 2 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th2_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_CP_REG (rw) register accessor: Test CPDMA THost Channel 3 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th3_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th3_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th3_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh3CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th3_cp_reg::CpswCpdmaSramTestCpdmaTh3CpRegSpec>;
#[doc = "Test CPDMA THost Channel 3 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th3_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH4_CP_REG (rw) register accessor: Test CPDMA THost Channel 4 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th4_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th4_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th4_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH4_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh4CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th4_cp_reg::CpswCpdmaSramTestCpdmaTh4CpRegSpec>;
#[doc = "Test CPDMA THost Channel 4 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th4_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH5_CP_REG (rw) register accessor: Test CPDMA THost Channel 5 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th5_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th5_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th5_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH5_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh5CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th5_cp_reg::CpswCpdmaSramTestCpdmaTh5CpRegSpec>;
#[doc = "Test CPDMA THost Channel 5 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th5_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH6_CP_REG (rw) register accessor: Test CPDMA THost Channel 6 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th6_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th6_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th6_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH6_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh6CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th6_cp_reg::CpswCpdmaSramTestCpdmaTh6CpRegSpec>;
#[doc = "Test CPDMA THost Channel 6 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th6_cp_reg;
#[doc = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH7_CP_REG (rw) register accessor: Test CPDMA THost Channel 7 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th7_cp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th7_cp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_cpdma_sram_test_cpdma_th7_cp_reg`]
module"]
#[doc(alias = "CPSW_CPDMA_SRAM_TEST_CPDMA_TH7_CP_REG")]
pub type CpswCpdmaSramTestCpdmaTh7CpReg =
    crate::Reg<cpsw_cpdma_sram_test_cpdma_th7_cp_reg::CpswCpdmaSramTestCpdmaTh7CpRegSpec>;
#[doc = "Test CPDMA THost Channel 7 Completion Pointer"]
pub mod cpsw_cpdma_sram_test_cpdma_th7_cp_reg;
#[doc = "CPSW_NC_STAT_0_RXGOODFRAMES (rw) register accessor: Total number of good frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxgoodframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxgoodframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxgoodframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXGOODFRAMES")]
pub type CpswNcStat0Rxgoodframes =
    crate::Reg<cpsw_nc_stat_0_rxgoodframes::CpswNcStat0RxgoodframesSpec>;
#[doc = "Total number of good frames received"]
pub mod cpsw_nc_stat_0_rxgoodframes;
#[doc = "CPSW_NC_STAT_0_RXBROADCASTFRAMES (rw) register accessor: Total number of good broadcast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxbroadcastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxbroadcastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxbroadcastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXBROADCASTFRAMES")]
pub type CpswNcStat0Rxbroadcastframes =
    crate::Reg<cpsw_nc_stat_0_rxbroadcastframes::CpswNcStat0RxbroadcastframesSpec>;
#[doc = "Total number of good broadcast frames received"]
pub mod cpsw_nc_stat_0_rxbroadcastframes;
#[doc = "CPSW_NC_STAT_0_RXMULTICASTFRAMES (rw) register accessor: Total number of good multicast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxmulticastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxmulticastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxmulticastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXMULTICASTFRAMES")]
pub type CpswNcStat0Rxmulticastframes =
    crate::Reg<cpsw_nc_stat_0_rxmulticastframes::CpswNcStat0RxmulticastframesSpec>;
#[doc = "Total number of good multicast frames received"]
pub mod cpsw_nc_stat_0_rxmulticastframes;
#[doc = "CPSW_NC_STAT_0_RXCRCERRORS (rw) register accessor: Total number of CRC errors frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxcrcerrors::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxcrcerrors::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxcrcerrors`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXCRCERRORS")]
pub type CpswNcStat0Rxcrcerrors =
    crate::Reg<cpsw_nc_stat_0_rxcrcerrors::CpswNcStat0RxcrcerrorsSpec>;
#[doc = "Total number of CRC errors frames received"]
pub mod cpsw_nc_stat_0_rxcrcerrors;
#[doc = "CPSW_NC_STAT_0_RXOVERSIZEDFRAMES (rw) register accessor: Total number of oversized frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxoversizedframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxoversizedframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxoversizedframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXOVERSIZEDFRAMES")]
pub type CpswNcStat0Rxoversizedframes =
    crate::Reg<cpsw_nc_stat_0_rxoversizedframes::CpswNcStat0RxoversizedframesSpec>;
#[doc = "Total number of oversized frames received"]
pub mod cpsw_nc_stat_0_rxoversizedframes;
#[doc = "CPSW_NC_STAT_0_RXUNDERSIZEDFRAMES (rw) register accessor: Total number of undersized frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxundersizedframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxundersizedframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxundersizedframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXUNDERSIZEDFRAMES")]
pub type CpswNcStat0Rxundersizedframes =
    crate::Reg<cpsw_nc_stat_0_rxundersizedframes::CpswNcStat0RxundersizedframesSpec>;
#[doc = "Total number of undersized frames received"]
pub mod cpsw_nc_stat_0_rxundersizedframes;
#[doc = "CPSW_NC_STAT_0_RXFRAGMENTS (rw) register accessor: Total number of fragmented frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxfragments::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxfragments::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxfragments`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXFRAGMENTS")]
pub type CpswNcStat0Rxfragments =
    crate::Reg<cpsw_nc_stat_0_rxfragments::CpswNcStat0RxfragmentsSpec>;
#[doc = "Total number of fragmented frames received"]
pub mod cpsw_nc_stat_0_rxfragments;
#[doc = "CPSW_NC_STAT_0_ALE_DROP (rw) register accessor: Total number of frames dropped by the ALE\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_DROP")]
pub type CpswNcStat0AleDrop = crate::Reg<cpsw_nc_stat_0_ale_drop::CpswNcStat0AleDropSpec>;
#[doc = "Total number of frames dropped by the ALE"]
pub mod cpsw_nc_stat_0_ale_drop;
#[doc = "CPSW_NC_STAT_0_ALE_OVERRUN_DROP (rw) register accessor: Total number of overrun frames dropped by the ALE\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_overrun_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_overrun_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_overrun_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_OVERRUN_DROP")]
pub type CpswNcStat0AleOverrunDrop =
    crate::Reg<cpsw_nc_stat_0_ale_overrun_drop::CpswNcStat0AleOverrunDropSpec>;
#[doc = "Total number of overrun frames dropped by the ALE"]
pub mod cpsw_nc_stat_0_ale_overrun_drop;
#[doc = "CPSW_NC_STAT_0_RXOCTETS (rw) register accessor: Total number of received bytes in good frames\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rxoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RXOCTETS")]
pub type CpswNcStat0Rxoctets = crate::Reg<cpsw_nc_stat_0_rxoctets::CpswNcStat0RxoctetsSpec>;
#[doc = "Total number of received bytes in good frames"]
pub mod cpsw_nc_stat_0_rxoctets;
#[doc = "CPSW_NC_STAT_0_TXGOODFRAMES (rw) register accessor: Total number of good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txgoodframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txgoodframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txgoodframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXGOODFRAMES")]
pub type CpswNcStat0Txgoodframes =
    crate::Reg<cpsw_nc_stat_0_txgoodframes::CpswNcStat0TxgoodframesSpec>;
#[doc = "Total number of good frames transmitted"]
pub mod cpsw_nc_stat_0_txgoodframes;
#[doc = "CPSW_NC_STAT_0_TXBROADCASTFRAMES (rw) register accessor: Total number of good broadcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txbroadcastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txbroadcastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txbroadcastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXBROADCASTFRAMES")]
pub type CpswNcStat0Txbroadcastframes =
    crate::Reg<cpsw_nc_stat_0_txbroadcastframes::CpswNcStat0TxbroadcastframesSpec>;
#[doc = "Total number of good broadcast frames transmitted"]
pub mod cpsw_nc_stat_0_txbroadcastframes;
#[doc = "CPSW_NC_STAT_0_TXMULTICASTFRAMES (rw) register accessor: Total number of good multicast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txmulticastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txmulticastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txmulticastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXMULTICASTFRAMES")]
pub type CpswNcStat0Txmulticastframes =
    crate::Reg<cpsw_nc_stat_0_txmulticastframes::CpswNcStat0TxmulticastframesSpec>;
#[doc = "Total number of good multicast frames transmitted"]
pub mod cpsw_nc_stat_0_txmulticastframes;
#[doc = "CPSW_NC_STAT_0_TXSINGLECOLLFRAMES (rw) register accessor: Total number of transmitted frames experiencing a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txsinglecollframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txsinglecollframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txsinglecollframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXSINGLECOLLFRAMES")]
pub type CpswNcStat0Txsinglecollframes =
    crate::Reg<cpsw_nc_stat_0_txsinglecollframes::CpswNcStat0TxsinglecollframesSpec>;
#[doc = "Total number of transmitted frames experiencing a single collision"]
pub mod cpsw_nc_stat_0_txsinglecollframes;
#[doc = "CPSW_NC_STAT_0_TXMULTCOLLFRAMES (rw) register accessor: Total number of transmitted frames experiencing multiple collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txmultcollframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txmultcollframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txmultcollframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXMULTCOLLFRAMES")]
pub type CpswNcStat0Txmultcollframes =
    crate::Reg<cpsw_nc_stat_0_txmultcollframes::CpswNcStat0TxmultcollframesSpec>;
#[doc = "Total number of transmitted frames experiencing multiple collisions"]
pub mod cpsw_nc_stat_0_txmultcollframes;
#[doc = "CPSW_NC_STAT_0_TXOCTETS (rw) register accessor: Total number of bytes in all good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_txoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_txoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_txoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TXOCTETS")]
pub type CpswNcStat0Txoctets = crate::Reg<cpsw_nc_stat_0_txoctets::CpswNcStat0TxoctetsSpec>;
#[doc = "Total number of bytes in all good frames transmitted"]
pub mod cpsw_nc_stat_0_txoctets;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES64 (rw) register accessor: Total number of 64-byte frames received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes64`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES64")]
pub type CpswNcStat0Octetframes64 =
    crate::Reg<cpsw_nc_stat_0_octetframes64::CpswNcStat0Octetframes64Spec>;
#[doc = "Total number of 64-byte frames received and transmitted"]
pub mod cpsw_nc_stat_0_octetframes64;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES65T127 (rw) register accessor: Total number of frames of size 65 to 127 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes65t127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes65t127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes65t127`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES65T127")]
pub type CpswNcStat0Octetframes65t127 =
    crate::Reg<cpsw_nc_stat_0_octetframes65t127::CpswNcStat0Octetframes65t127Spec>;
#[doc = "Total number of frames of size 65 to 127 bytes received and transmitted"]
pub mod cpsw_nc_stat_0_octetframes65t127;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES128T255 (rw) register accessor: Total number of frames of size 128 to 255 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes128t255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes128t255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes128t255`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES128T255")]
pub type CpswNcStat0Octetframes128t255 =
    crate::Reg<cpsw_nc_stat_0_octetframes128t255::CpswNcStat0Octetframes128t255Spec>;
#[doc = "Total number of frames of size 128 to 255 bytes received and transmitted"]
pub mod cpsw_nc_stat_0_octetframes128t255;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES256T511 (rw) register accessor: Total number of frames of size 256 to 511 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes256t511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes256t511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes256t511`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES256T511")]
pub type CpswNcStat0Octetframes256t511 =
    crate::Reg<cpsw_nc_stat_0_octetframes256t511::CpswNcStat0Octetframes256t511Spec>;
#[doc = "Total number of frames of size 256 to 511 bytes received and transmitted"]
pub mod cpsw_nc_stat_0_octetframes256t511;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES512T1023 (rw) register accessor: Total number of frames of size 512 to 1023 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes512t1023::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes512t1023::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes512t1023`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES512T1023")]
pub type CpswNcStat0Octetframes512t1023 =
    crate::Reg<cpsw_nc_stat_0_octetframes512t1023::CpswNcStat0Octetframes512t1023Spec>;
#[doc = "Total number of frames of size 512 to 1023 bytes received and transmitted"]
pub mod cpsw_nc_stat_0_octetframes512t1023;
#[doc = "CPSW_NC_STAT_0_OCTETFRAMES1024TUP (rw) register accessor: Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes1024tup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes1024tup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_octetframes1024tup`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_OCTETFRAMES1024TUP")]
pub type CpswNcStat0Octetframes1024tup =
    crate::Reg<cpsw_nc_stat_0_octetframes1024tup::CpswNcStat0Octetframes1024tupSpec>;
#[doc = "Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
pub mod cpsw_nc_stat_0_octetframes1024tup;
#[doc = "CPSW_NC_STAT_0_NETOCTETS (rw) register accessor: Total number of bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_netoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_netoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_netoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_NETOCTETS")]
pub type CpswNcStat0Netoctets = crate::Reg<cpsw_nc_stat_0_netoctets::CpswNcStat0NetoctetsSpec>;
#[doc = "Total number of bytes received and transmitted"]
pub mod cpsw_nc_stat_0_netoctets;
#[doc = "CPSW_NC_STAT_0_RX_BOTTOM_OF_FIFO_DROP (rw) register accessor: Receive Bottom of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rx_bottom_of_fifo_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rx_bottom_of_fifo_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rx_bottom_of_fifo_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RX_BOTTOM_OF_FIFO_DROP")]
pub type CpswNcStat0RxBottomOfFifoDrop =
    crate::Reg<cpsw_nc_stat_0_rx_bottom_of_fifo_drop::CpswNcStat0RxBottomOfFifoDropSpec>;
#[doc = "Receive Bottom of FIFO Drop"]
pub mod cpsw_nc_stat_0_rx_bottom_of_fifo_drop;
#[doc = "CPSW_NC_STAT_0_PORTMASK_DROP (rw) register accessor: Total number of dropped frames received due to portmask\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_portmask_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_portmask_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_portmask_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_PORTMASK_DROP")]
pub type CpswNcStat0PortmaskDrop =
    crate::Reg<cpsw_nc_stat_0_portmask_drop::CpswNcStat0PortmaskDropSpec>;
#[doc = "Total number of dropped frames received due to portmask"]
pub mod cpsw_nc_stat_0_portmask_drop;
#[doc = "CPSW_NC_STAT_0_RX_TOP_OF_FIFO_DROP (rw) register accessor: Receive Top of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rx_top_of_fifo_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rx_top_of_fifo_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_rx_top_of_fifo_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_RX_TOP_OF_FIFO_DROP")]
pub type CpswNcStat0RxTopOfFifoDrop =
    crate::Reg<cpsw_nc_stat_0_rx_top_of_fifo_drop::CpswNcStat0RxTopOfFifoDropSpec>;
#[doc = "Receive Top of FIFO Drop"]
pub mod cpsw_nc_stat_0_rx_top_of_fifo_drop;
#[doc = "CPSW_NC_STAT_0_ALE_RATE_LIMIT_DROP (rw) register accessor: Total number of dropped frames due to ALE Rate Limiting\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_rate_limit_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_rate_limit_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_rate_limit_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_RATE_LIMIT_DROP")]
pub type CpswNcStat0AleRateLimitDrop =
    crate::Reg<cpsw_nc_stat_0_ale_rate_limit_drop::CpswNcStat0AleRateLimitDropSpec>;
#[doc = "Total number of dropped frames due to ALE Rate Limiting"]
pub mod cpsw_nc_stat_0_ale_rate_limit_drop;
#[doc = "CPSW_NC_STAT_0_ALE_VID_INGRESS_DROP (rw) register accessor: Total number of dropped frames due to ALE VID Ingress\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_vid_ingress_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_vid_ingress_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_vid_ingress_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_VID_INGRESS_DROP")]
pub type CpswNcStat0AleVidIngressDrop =
    crate::Reg<cpsw_nc_stat_0_ale_vid_ingress_drop::CpswNcStat0AleVidIngressDropSpec>;
#[doc = "Total number of dropped frames due to ALE VID Ingress"]
pub mod cpsw_nc_stat_0_ale_vid_ingress_drop;
#[doc = "CPSW_NC_STAT_0_ALE_DA_EQ_SA_DROP (rw) register accessor: Total number of dropped frames due to DA=SA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_da_eq_sa_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_da_eq_sa_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_da_eq_sa_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_DA_EQ_SA_DROP")]
pub type CpswNcStat0AleDaEqSaDrop =
    crate::Reg<cpsw_nc_stat_0_ale_da_eq_sa_drop::CpswNcStat0AleDaEqSaDropSpec>;
#[doc = "Total number of dropped frames due to DA=SA"]
pub mod cpsw_nc_stat_0_ale_da_eq_sa_drop;
#[doc = "CPSW_NC_STAT_0_ALE_BLOCK_DROP (rw) register accessor: Total number of dropped frames due to ALE Block Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_block_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_block_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_block_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_BLOCK_DROP")]
pub type CpswNcStat0AleBlockDrop =
    crate::Reg<cpsw_nc_stat_0_ale_block_drop::CpswNcStat0AleBlockDropSpec>;
#[doc = "Total number of dropped frames due to ALE Block Mode"]
pub mod cpsw_nc_stat_0_ale_block_drop;
#[doc = "CPSW_NC_STAT_0_ALE_SECURE_DROP (rw) register accessor: Total number of dropped frames due to ALE Secure Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_secure_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_secure_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_secure_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_SECURE_DROP")]
pub type CpswNcStat0AleSecureDrop =
    crate::Reg<cpsw_nc_stat_0_ale_secure_drop::CpswNcStat0AleSecureDropSpec>;
#[doc = "Total number of dropped frames due to ALE Secure Mode"]
pub mod cpsw_nc_stat_0_ale_secure_drop;
#[doc = "CPSW_NC_STAT_0_ALE_AUTH_DROP (rw) register accessor: Total number of dropped frames due to ALE Authentication\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_auth_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_auth_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_auth_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_AUTH_DROP")]
pub type CpswNcStat0AleAuthDrop =
    crate::Reg<cpsw_nc_stat_0_ale_auth_drop::CpswNcStat0AleAuthDropSpec>;
#[doc = "Total number of dropped frames due to ALE Authentication"]
pub mod cpsw_nc_stat_0_ale_auth_drop;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_UNI (rw) register accessor: ALE Receive Unknown Unicast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_uni::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_uni::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_uni`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_UNI")]
pub type CpswNcStat0AleUnknUni = crate::Reg<cpsw_nc_stat_0_ale_unkn_uni::CpswNcStat0AleUnknUniSpec>;
#[doc = "ALE Receive Unknown Unicast"]
pub mod cpsw_nc_stat_0_ale_unkn_uni;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_UNI_BCNT (rw) register accessor: ALE Receive Unknown Unicast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_uni_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_uni_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_uni_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_UNI_BCNT")]
pub type CpswNcStat0AleUnknUniBcnt =
    crate::Reg<cpsw_nc_stat_0_ale_unkn_uni_bcnt::CpswNcStat0AleUnknUniBcntSpec>;
#[doc = "ALE Receive Unknown Unicast Bytecount"]
pub mod cpsw_nc_stat_0_ale_unkn_uni_bcnt;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_MLT (rw) register accessor: ALE Receive Unknown Multicast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_mlt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_mlt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_mlt`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_MLT")]
pub type CpswNcStat0AleUnknMlt = crate::Reg<cpsw_nc_stat_0_ale_unkn_mlt::CpswNcStat0AleUnknMltSpec>;
#[doc = "ALE Receive Unknown Multicast"]
pub mod cpsw_nc_stat_0_ale_unkn_mlt;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_MLT_BCNT (rw) register accessor: ALE Receive Unknown Multicast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_mlt_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_mlt_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_mlt_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_MLT_BCNT")]
pub type CpswNcStat0AleUnknMltBcnt =
    crate::Reg<cpsw_nc_stat_0_ale_unkn_mlt_bcnt::CpswNcStat0AleUnknMltBcntSpec>;
#[doc = "ALE Receive Unknown Multicast Bytecount"]
pub mod cpsw_nc_stat_0_ale_unkn_mlt_bcnt;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_BRD (rw) register accessor: ALE Receive Unknown Broadcast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_brd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_brd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_brd`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_BRD")]
pub type CpswNcStat0AleUnknBrd = crate::Reg<cpsw_nc_stat_0_ale_unkn_brd::CpswNcStat0AleUnknBrdSpec>;
#[doc = "ALE Receive Unknown Broadcast"]
pub mod cpsw_nc_stat_0_ale_unkn_brd;
#[doc = "CPSW_NC_STAT_0_ALE_UNKN_BRD_BCNT (rw) register accessor: ALE Receive Unknown Broadcast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_unkn_brd_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_unkn_brd_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_unkn_brd_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_UNKN_BRD_BCNT")]
pub type CpswNcStat0AleUnknBrdBcnt =
    crate::Reg<cpsw_nc_stat_0_ale_unkn_brd_bcnt::CpswNcStat0AleUnknBrdBcntSpec>;
#[doc = "ALE Receive Unknown Broadcast Bytecount"]
pub mod cpsw_nc_stat_0_ale_unkn_brd_bcnt;
#[doc = "CPSW_NC_STAT_0_ALE_POL_MATCH (rw) register accessor: ALE Policer Matched\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_pol_match::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_pol_match::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_pol_match`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_POL_MATCH")]
pub type CpswNcStat0AlePolMatch =
    crate::Reg<cpsw_nc_stat_0_ale_pol_match::CpswNcStat0AlePolMatchSpec>;
#[doc = "ALE Policer Matched"]
pub mod cpsw_nc_stat_0_ale_pol_match;
#[doc = "CPSW_NC_STAT_0_ALE_POL_MATCH_RED (rw) register accessor: ALE Policer Matched and Condition Red\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_pol_match_red::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_pol_match_red::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_pol_match_red`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_POL_MATCH_RED")]
pub type CpswNcStat0AlePolMatchRed =
    crate::Reg<cpsw_nc_stat_0_ale_pol_match_red::CpswNcStat0AlePolMatchRedSpec>;
#[doc = "ALE Policer Matched and Condition Red"]
pub mod cpsw_nc_stat_0_ale_pol_match_red;
#[doc = "CPSW_NC_STAT_0_ALE_POL_MATCH_YELLOW (rw) register accessor: ALE Policer Matched and Condition Yellow\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_pol_match_yellow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_pol_match_yellow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_pol_match_yellow`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_POL_MATCH_YELLOW")]
pub type CpswNcStat0AlePolMatchYellow =
    crate::Reg<cpsw_nc_stat_0_ale_pol_match_yellow::CpswNcStat0AlePolMatchYellowSpec>;
#[doc = "ALE Policer Matched and Condition Yellow"]
pub mod cpsw_nc_stat_0_ale_pol_match_yellow;
#[doc = "CPSW_NC_STAT_0_ALE_MULT_SA_DROP (rw) register accessor: ALE Multicast Source Address Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_mult_sa_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_mult_sa_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_mult_sa_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_MULT_SA_DROP")]
pub type CpswNcStat0AleMultSaDrop =
    crate::Reg<cpsw_nc_stat_0_ale_mult_sa_drop::CpswNcStat0AleMultSaDropSpec>;
#[doc = "ALE Multicast Source Address Drop"]
pub mod cpsw_nc_stat_0_ale_mult_sa_drop;
#[doc = "CPSW_NC_STAT_0_ALE_DUAL_VLAN_DROP (rw) register accessor: ALE Dual VLAN Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_dual_vlan_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_dual_vlan_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_dual_vlan_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_DUAL_VLAN_DROP")]
pub type CpswNcStat0AleDualVlanDrop =
    crate::Reg<cpsw_nc_stat_0_ale_dual_vlan_drop::CpswNcStat0AleDualVlanDropSpec>;
#[doc = "ALE Dual VLAN Drop"]
pub mod cpsw_nc_stat_0_ale_dual_vlan_drop;
#[doc = "CPSW_NC_STAT_0_ALE_LEN_ERROR_DROP (rw) register accessor: ALE Length Error Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_len_error_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_len_error_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_len_error_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_LEN_ERROR_DROP")]
pub type CpswNcStat0AleLenErrorDrop =
    crate::Reg<cpsw_nc_stat_0_ale_len_error_drop::CpswNcStat0AleLenErrorDropSpec>;
#[doc = "ALE Length Error Drop"]
pub mod cpsw_nc_stat_0_ale_len_error_drop;
#[doc = "CPSW_NC_STAT_0_ALE_IP_NEXT_HDR_DROP (rw) register accessor: ALE IP Next Header Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_ip_next_hdr_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_IP_NEXT_HDR_DROP")]
pub type CpswNcStat0AleIpNextHdrDrop =
    crate::Reg<cpsw_nc_stat_0_ale_ip_next_hdr_drop::CpswNcStat0AleIpNextHdrDropSpec>;
#[doc = "ALE IP Next Header Drop"]
pub mod cpsw_nc_stat_0_ale_ip_next_hdr_drop;
#[doc = "CPSW_NC_STAT_0_ALE_IPV4_FRAG_DROP (rw) register accessor: ALE IPV4 Frag Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_ipv4_frag_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_ipv4_frag_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_ale_ipv4_frag_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_ALE_IPV4_FRAG_DROP")]
pub type CpswNcStat0AleIpv4FragDrop =
    crate::Reg<cpsw_nc_stat_0_ale_ipv4_frag_drop::CpswNcStat0AleIpv4FragDropSpec>;
#[doc = "ALE IPV4 Frag Drop"]
pub mod cpsw_nc_stat_0_ale_ipv4_frag_drop;
#[doc = "CPSW_NC_STAT_0_TX_MEMORY_PROTECT_ERROR (rw) register accessor: Transmit Memory Protect CRC Error\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_tx_memory_protect_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_tx_memory_protect_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_0_tx_memory_protect_error`]
module"]
#[doc(alias = "CPSW_NC_STAT_0_TX_MEMORY_PROTECT_ERROR")]
pub type CpswNcStat0TxMemoryProtectError =
    crate::Reg<cpsw_nc_stat_0_tx_memory_protect_error::CpswNcStat0TxMemoryProtectErrorSpec>;
#[doc = "Transmit Memory Protect CRC Error"]
pub mod cpsw_nc_stat_0_tx_memory_protect_error;
#[doc = "CPSW_NC_STAT_1_RXGOODFRAMES (rw) register accessor: Total number of good frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxgoodframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxgoodframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxgoodframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXGOODFRAMES")]
pub type CpswNcStat1Rxgoodframes =
    crate::Reg<cpsw_nc_stat_1_rxgoodframes::CpswNcStat1RxgoodframesSpec>;
#[doc = "Total number of good frames received"]
pub mod cpsw_nc_stat_1_rxgoodframes;
#[doc = "CPSW_NC_STAT_1_RXBROADCASTFRAMES (rw) register accessor: Total number of good broadcast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxbroadcastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxbroadcastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxbroadcastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXBROADCASTFRAMES")]
pub type CpswNcStat1Rxbroadcastframes =
    crate::Reg<cpsw_nc_stat_1_rxbroadcastframes::CpswNcStat1RxbroadcastframesSpec>;
#[doc = "Total number of good broadcast frames received"]
pub mod cpsw_nc_stat_1_rxbroadcastframes;
#[doc = "CPSW_NC_STAT_1_RXMULTICASTFRAMES (rw) register accessor: Total number of good multicast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxmulticastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxmulticastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxmulticastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXMULTICASTFRAMES")]
pub type CpswNcStat1Rxmulticastframes =
    crate::Reg<cpsw_nc_stat_1_rxmulticastframes::CpswNcStat1RxmulticastframesSpec>;
#[doc = "Total number of good multicast frames received"]
pub mod cpsw_nc_stat_1_rxmulticastframes;
#[doc = "CPSW_NC_STAT_1_RXPAUSEFRAMES (rw) register accessor: Total number of pause frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxpauseframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxpauseframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxpauseframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXPAUSEFRAMES")]
pub type CpswNcStat1Rxpauseframes =
    crate::Reg<cpsw_nc_stat_1_rxpauseframes::CpswNcStat1RxpauseframesSpec>;
#[doc = "Total number of pause frames received"]
pub mod cpsw_nc_stat_1_rxpauseframes;
#[doc = "CPSW_NC_STAT_1_RXCRCERRORS (rw) register accessor: Total number of CRC errors frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxcrcerrors::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxcrcerrors::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxcrcerrors`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXCRCERRORS")]
pub type CpswNcStat1Rxcrcerrors =
    crate::Reg<cpsw_nc_stat_1_rxcrcerrors::CpswNcStat1RxcrcerrorsSpec>;
#[doc = "Total number of CRC errors frames received"]
pub mod cpsw_nc_stat_1_rxcrcerrors;
#[doc = "CPSW_NC_STAT_1_RXALIGNCODEERRORS (rw) register accessor: Total number of alignment/code errors received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxaligncodeerrors::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxaligncodeerrors::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxaligncodeerrors`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXALIGNCODEERRORS")]
pub type CpswNcStat1Rxaligncodeerrors =
    crate::Reg<cpsw_nc_stat_1_rxaligncodeerrors::CpswNcStat1RxaligncodeerrorsSpec>;
#[doc = "Total number of alignment/code errors received"]
pub mod cpsw_nc_stat_1_rxaligncodeerrors;
#[doc = "CPSW_NC_STAT_1_RXOVERSIZEDFRAMES (rw) register accessor: Total number of oversized frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxoversizedframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxoversizedframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxoversizedframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXOVERSIZEDFRAMES")]
pub type CpswNcStat1Rxoversizedframes =
    crate::Reg<cpsw_nc_stat_1_rxoversizedframes::CpswNcStat1RxoversizedframesSpec>;
#[doc = "Total number of oversized frames received"]
pub mod cpsw_nc_stat_1_rxoversizedframes;
#[doc = "CPSW_NC_STAT_1_RXJABBERFRAMES (rw) register accessor: Total number of jabber frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxjabberframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxjabberframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxjabberframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXJABBERFRAMES")]
pub type CpswNcStat1Rxjabberframes =
    crate::Reg<cpsw_nc_stat_1_rxjabberframes::CpswNcStat1RxjabberframesSpec>;
#[doc = "Total number of jabber frames received"]
pub mod cpsw_nc_stat_1_rxjabberframes;
#[doc = "CPSW_NC_STAT_1_RXUNDERSIZEDFRAMES (rw) register accessor: Total number of undersized frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxundersizedframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxundersizedframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxundersizedframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXUNDERSIZEDFRAMES")]
pub type CpswNcStat1Rxundersizedframes =
    crate::Reg<cpsw_nc_stat_1_rxundersizedframes::CpswNcStat1RxundersizedframesSpec>;
#[doc = "Total number of undersized frames received"]
pub mod cpsw_nc_stat_1_rxundersizedframes;
#[doc = "CPSW_NC_STAT_1_RXFRAGMENTS (rw) register accessor: Total number of fragmented frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxfragments::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxfragments::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxfragments`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXFRAGMENTS")]
pub type CpswNcStat1Rxfragments =
    crate::Reg<cpsw_nc_stat_1_rxfragments::CpswNcStat1RxfragmentsSpec>;
#[doc = "Total number of fragmented frames received"]
pub mod cpsw_nc_stat_1_rxfragments;
#[doc = "CPSW_NC_STAT_1_ALE_DROP (rw) register accessor: Total number of frames dropped by the ALE\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_DROP")]
pub type CpswNcStat1AleDrop = crate::Reg<cpsw_nc_stat_1_ale_drop::CpswNcStat1AleDropSpec>;
#[doc = "Total number of frames dropped by the ALE"]
pub mod cpsw_nc_stat_1_ale_drop;
#[doc = "CPSW_NC_STAT_1_ALE_OVERRUN_DROP (rw) register accessor: Total number of overrun frames dropped by the ALE\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_overrun_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_overrun_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_overrun_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_OVERRUN_DROP")]
pub type CpswNcStat1AleOverrunDrop =
    crate::Reg<cpsw_nc_stat_1_ale_overrun_drop::CpswNcStat1AleOverrunDropSpec>;
#[doc = "Total number of overrun frames dropped by the ALE"]
pub mod cpsw_nc_stat_1_ale_overrun_drop;
#[doc = "CPSW_NC_STAT_1_RXOCTETS (rw) register accessor: Total number of received bytes in good frames\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXOCTETS")]
pub type CpswNcStat1Rxoctets = crate::Reg<cpsw_nc_stat_1_rxoctets::CpswNcStat1RxoctetsSpec>;
#[doc = "Total number of received bytes in good frames"]
pub mod cpsw_nc_stat_1_rxoctets;
#[doc = "CPSW_NC_STAT_1_TXGOODFRAMES (rw) register accessor: Total number of good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txgoodframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txgoodframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txgoodframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXGOODFRAMES")]
pub type CpswNcStat1Txgoodframes =
    crate::Reg<cpsw_nc_stat_1_txgoodframes::CpswNcStat1TxgoodframesSpec>;
#[doc = "Total number of good frames transmitted"]
pub mod cpsw_nc_stat_1_txgoodframes;
#[doc = "CPSW_NC_STAT_1_TXBROADCASTFRAMES (rw) register accessor: Total number of good broadcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txbroadcastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txbroadcastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txbroadcastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXBROADCASTFRAMES")]
pub type CpswNcStat1Txbroadcastframes =
    crate::Reg<cpsw_nc_stat_1_txbroadcastframes::CpswNcStat1TxbroadcastframesSpec>;
#[doc = "Total number of good broadcast frames transmitted"]
pub mod cpsw_nc_stat_1_txbroadcastframes;
#[doc = "CPSW_NC_STAT_1_TXMULTICASTFRAMES (rw) register accessor: Total number of good multicast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txmulticastframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txmulticastframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txmulticastframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXMULTICASTFRAMES")]
pub type CpswNcStat1Txmulticastframes =
    crate::Reg<cpsw_nc_stat_1_txmulticastframes::CpswNcStat1TxmulticastframesSpec>;
#[doc = "Total number of good multicast frames transmitted"]
pub mod cpsw_nc_stat_1_txmulticastframes;
#[doc = "CPSW_NC_STAT_1_TXPAUSEFRAMES (rw) register accessor: Total number of pause frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txpauseframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txpauseframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txpauseframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXPAUSEFRAMES")]
pub type CpswNcStat1Txpauseframes =
    crate::Reg<cpsw_nc_stat_1_txpauseframes::CpswNcStat1TxpauseframesSpec>;
#[doc = "Total number of pause frames transmitted"]
pub mod cpsw_nc_stat_1_txpauseframes;
#[doc = "CPSW_NC_STAT_1_TXDEFERREDFRAMES (rw) register accessor: Total number of deferred frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txdeferredframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txdeferredframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txdeferredframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXDEFERREDFRAMES")]
pub type CpswNcStat1Txdeferredframes =
    crate::Reg<cpsw_nc_stat_1_txdeferredframes::CpswNcStat1TxdeferredframesSpec>;
#[doc = "Total number of deferred frames transmitted"]
pub mod cpsw_nc_stat_1_txdeferredframes;
#[doc = "CPSW_NC_STAT_1_TXCOLLISIONFRAMES (rw) register accessor: Total number of transmitted frames experiencing a collision\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txcollisionframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txcollisionframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txcollisionframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXCOLLISIONFRAMES")]
pub type CpswNcStat1Txcollisionframes =
    crate::Reg<cpsw_nc_stat_1_txcollisionframes::CpswNcStat1TxcollisionframesSpec>;
#[doc = "Total number of transmitted frames experiencing a collision"]
pub mod cpsw_nc_stat_1_txcollisionframes;
#[doc = "CPSW_NC_STAT_1_TXSINGLECOLLFRAMES (rw) register accessor: Total number of transmitted frames experiencing a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txsinglecollframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txsinglecollframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txsinglecollframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXSINGLECOLLFRAMES")]
pub type CpswNcStat1Txsinglecollframes =
    crate::Reg<cpsw_nc_stat_1_txsinglecollframes::CpswNcStat1TxsinglecollframesSpec>;
#[doc = "Total number of transmitted frames experiencing a single collision"]
pub mod cpsw_nc_stat_1_txsinglecollframes;
#[doc = "CPSW_NC_STAT_1_TXMULTCOLLFRAMES (rw) register accessor: Total number of transmitted frames experiencing multiple collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txmultcollframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txmultcollframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txmultcollframes`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXMULTCOLLFRAMES")]
pub type CpswNcStat1Txmultcollframes =
    crate::Reg<cpsw_nc_stat_1_txmultcollframes::CpswNcStat1TxmultcollframesSpec>;
#[doc = "Total number of transmitted frames experiencing multiple collisions"]
pub mod cpsw_nc_stat_1_txmultcollframes;
#[doc = "CPSW_NC_STAT_1_TXEXCESSIVECOLLISIONS (rw) register accessor: Total number of transmitted frames abandoned due to excessive collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txexcessivecollisions::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txexcessivecollisions::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txexcessivecollisions`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXEXCESSIVECOLLISIONS")]
pub type CpswNcStat1Txexcessivecollisions =
    crate::Reg<cpsw_nc_stat_1_txexcessivecollisions::CpswNcStat1TxexcessivecollisionsSpec>;
#[doc = "Total number of transmitted frames abandoned due to excessive collisions"]
pub mod cpsw_nc_stat_1_txexcessivecollisions;
#[doc = "CPSW_NC_STAT_1_TXLATECOLLISIONS (rw) register accessor: Total number of transmitted frames abandoned due to a late collision\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txlatecollisions::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txlatecollisions::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txlatecollisions`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXLATECOLLISIONS")]
pub type CpswNcStat1Txlatecollisions =
    crate::Reg<cpsw_nc_stat_1_txlatecollisions::CpswNcStat1TxlatecollisionsSpec>;
#[doc = "Total number of transmitted frames abandoned due to a late collision"]
pub mod cpsw_nc_stat_1_txlatecollisions;
#[doc = "CPSW_NC_STAT_1_RXIPGERROR (rw) register accessor: Total number of receive inter-packet gap errors (10G only)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rxipgerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rxipgerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rxipgerror`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RXIPGERROR")]
pub type CpswNcStat1Rxipgerror = crate::Reg<cpsw_nc_stat_1_rxipgerror::CpswNcStat1RxipgerrorSpec>;
#[doc = "Total number of receive inter-packet gap errors (10G only)"]
pub mod cpsw_nc_stat_1_rxipgerror;
#[doc = "CPSW_NC_STAT_1_TXCARRIERSENSEERRORS (rw) register accessor: Total number of transmitted frames that experienced a carrier loss\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txcarriersenseerrors::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txcarriersenseerrors::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txcarriersenseerrors`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXCARRIERSENSEERRORS")]
pub type CpswNcStat1Txcarriersenseerrors =
    crate::Reg<cpsw_nc_stat_1_txcarriersenseerrors::CpswNcStat1TxcarriersenseerrorsSpec>;
#[doc = "Total number of transmitted frames that experienced a carrier loss"]
pub mod cpsw_nc_stat_1_txcarriersenseerrors;
#[doc = "CPSW_NC_STAT_1_TXOCTETS (rw) register accessor: Total number of bytes in all good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_txoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_txoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_txoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TXOCTETS")]
pub type CpswNcStat1Txoctets = crate::Reg<cpsw_nc_stat_1_txoctets::CpswNcStat1TxoctetsSpec>;
#[doc = "Total number of bytes in all good frames transmitted"]
pub mod cpsw_nc_stat_1_txoctets;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES64 (rw) register accessor: Total number of 64-byte frames received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes64`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES64")]
pub type CpswNcStat1Octetframes64 =
    crate::Reg<cpsw_nc_stat_1_octetframes64::CpswNcStat1Octetframes64Spec>;
#[doc = "Total number of 64-byte frames received and transmitted"]
pub mod cpsw_nc_stat_1_octetframes64;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES65T127 (rw) register accessor: Total number of frames of size 65 to 127 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes65t127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes65t127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes65t127`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES65T127")]
pub type CpswNcStat1Octetframes65t127 =
    crate::Reg<cpsw_nc_stat_1_octetframes65t127::CpswNcStat1Octetframes65t127Spec>;
#[doc = "Total number of frames of size 65 to 127 bytes received and transmitted"]
pub mod cpsw_nc_stat_1_octetframes65t127;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES128T255 (rw) register accessor: Total number of frames of size 128 to 255 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes128t255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes128t255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes128t255`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES128T255")]
pub type CpswNcStat1Octetframes128t255 =
    crate::Reg<cpsw_nc_stat_1_octetframes128t255::CpswNcStat1Octetframes128t255Spec>;
#[doc = "Total number of frames of size 128 to 255 bytes received and transmitted"]
pub mod cpsw_nc_stat_1_octetframes128t255;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES256T511 (rw) register accessor: Total number of frames of size 256 to 511 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes256t511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes256t511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes256t511`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES256T511")]
pub type CpswNcStat1Octetframes256t511 =
    crate::Reg<cpsw_nc_stat_1_octetframes256t511::CpswNcStat1Octetframes256t511Spec>;
#[doc = "Total number of frames of size 256 to 511 bytes received and transmitted"]
pub mod cpsw_nc_stat_1_octetframes256t511;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES512T1023 (rw) register accessor: Total number of frames of size 512 to 1023 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes512t1023::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes512t1023::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes512t1023`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES512T1023")]
pub type CpswNcStat1Octetframes512t1023 =
    crate::Reg<cpsw_nc_stat_1_octetframes512t1023::CpswNcStat1Octetframes512t1023Spec>;
#[doc = "Total number of frames of size 512 to 1023 bytes received and transmitted"]
pub mod cpsw_nc_stat_1_octetframes512t1023;
#[doc = "CPSW_NC_STAT_1_OCTETFRAMES1024TUP (rw) register accessor: Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes1024tup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes1024tup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_octetframes1024tup`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_OCTETFRAMES1024TUP")]
pub type CpswNcStat1Octetframes1024tup =
    crate::Reg<cpsw_nc_stat_1_octetframes1024tup::CpswNcStat1Octetframes1024tupSpec>;
#[doc = "Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
pub mod cpsw_nc_stat_1_octetframes1024tup;
#[doc = "CPSW_NC_STAT_1_NETOCTETS (rw) register accessor: Total number of bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_netoctets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_netoctets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_netoctets`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_NETOCTETS")]
pub type CpswNcStat1Netoctets = crate::Reg<cpsw_nc_stat_1_netoctets::CpswNcStat1NetoctetsSpec>;
#[doc = "Total number of bytes received and transmitted"]
pub mod cpsw_nc_stat_1_netoctets;
#[doc = "CPSW_NC_STAT_1_RX_BOTTOM_OF_FIFO_DROP (rw) register accessor: Receive Bottom of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rx_bottom_of_fifo_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RX_BOTTOM_OF_FIFO_DROP")]
pub type CpswNcStat1RxBottomOfFifoDrop =
    crate::Reg<cpsw_nc_stat_1_rx_bottom_of_fifo_drop::CpswNcStat1RxBottomOfFifoDropSpec>;
#[doc = "Receive Bottom of FIFO Drop"]
pub mod cpsw_nc_stat_1_rx_bottom_of_fifo_drop;
#[doc = "CPSW_NC_STAT_1_PORTMASK_DROP (rw) register accessor: Total number of dropped frames received due to portmask\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_portmask_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_portmask_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_portmask_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_PORTMASK_DROP")]
pub type CpswNcStat1PortmaskDrop =
    crate::Reg<cpsw_nc_stat_1_portmask_drop::CpswNcStat1PortmaskDropSpec>;
#[doc = "Total number of dropped frames received due to portmask"]
pub mod cpsw_nc_stat_1_portmask_drop;
#[doc = "CPSW_NC_STAT_1_RX_TOP_OF_FIFO_DROP (rw) register accessor: Receive Top of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rx_top_of_fifo_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rx_top_of_fifo_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_rx_top_of_fifo_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_RX_TOP_OF_FIFO_DROP")]
pub type CpswNcStat1RxTopOfFifoDrop =
    crate::Reg<cpsw_nc_stat_1_rx_top_of_fifo_drop::CpswNcStat1RxTopOfFifoDropSpec>;
#[doc = "Receive Top of FIFO Drop"]
pub mod cpsw_nc_stat_1_rx_top_of_fifo_drop;
#[doc = "CPSW_NC_STAT_1_ALE_RATE_LIMIT_DROP (rw) register accessor: Total number of dropped frames due to ALE Rate Limiting\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_rate_limit_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_rate_limit_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_rate_limit_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_RATE_LIMIT_DROP")]
pub type CpswNcStat1AleRateLimitDrop =
    crate::Reg<cpsw_nc_stat_1_ale_rate_limit_drop::CpswNcStat1AleRateLimitDropSpec>;
#[doc = "Total number of dropped frames due to ALE Rate Limiting"]
pub mod cpsw_nc_stat_1_ale_rate_limit_drop;
#[doc = "CPSW_NC_STAT_1_ALE_VID_INGRESS_DROP (rw) register accessor: Total number of dropped frames due to ALE VID Ingress\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_vid_ingress_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_vid_ingress_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_vid_ingress_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_VID_INGRESS_DROP")]
pub type CpswNcStat1AleVidIngressDrop =
    crate::Reg<cpsw_nc_stat_1_ale_vid_ingress_drop::CpswNcStat1AleVidIngressDropSpec>;
#[doc = "Total number of dropped frames due to ALE VID Ingress"]
pub mod cpsw_nc_stat_1_ale_vid_ingress_drop;
#[doc = "CPSW_NC_STAT_1_ALE_DA_EQ_SA_DROP (rw) register accessor: Total number of dropped frames due to DA=SA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_da_eq_sa_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_da_eq_sa_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_da_eq_sa_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_DA_EQ_SA_DROP")]
pub type CpswNcStat1AleDaEqSaDrop =
    crate::Reg<cpsw_nc_stat_1_ale_da_eq_sa_drop::CpswNcStat1AleDaEqSaDropSpec>;
#[doc = "Total number of dropped frames due to DA=SA"]
pub mod cpsw_nc_stat_1_ale_da_eq_sa_drop;
#[doc = "CPSW_NC_STAT_1_ALE_BLOCK_DROP (rw) register accessor: Total number of dropped frames due to ALE Block Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_block_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_block_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_block_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_BLOCK_DROP")]
pub type CpswNcStat1AleBlockDrop =
    crate::Reg<cpsw_nc_stat_1_ale_block_drop::CpswNcStat1AleBlockDropSpec>;
#[doc = "Total number of dropped frames due to ALE Block Mode"]
pub mod cpsw_nc_stat_1_ale_block_drop;
#[doc = "CPSW_NC_STAT_1_ALE_SECURE_DROP (rw) register accessor: Total number of dropped frames due to ALE Secure Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_secure_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_secure_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_secure_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_SECURE_DROP")]
pub type CpswNcStat1AleSecureDrop =
    crate::Reg<cpsw_nc_stat_1_ale_secure_drop::CpswNcStat1AleSecureDropSpec>;
#[doc = "Total number of dropped frames due to ALE Secure Mode"]
pub mod cpsw_nc_stat_1_ale_secure_drop;
#[doc = "CPSW_NC_STAT_1_ALE_AUTH_DROP (rw) register accessor: Total number of dropped frames due to ALE Authentication\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_auth_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_auth_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_auth_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_AUTH_DROP")]
pub type CpswNcStat1AleAuthDrop =
    crate::Reg<cpsw_nc_stat_1_ale_auth_drop::CpswNcStat1AleAuthDropSpec>;
#[doc = "Total number of dropped frames due to ALE Authentication"]
pub mod cpsw_nc_stat_1_ale_auth_drop;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_UNI (rw) register accessor: ALE Receive Unknown Unicast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_uni::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_uni::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_uni`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_UNI")]
pub type CpswNcStat1AleUnknUni = crate::Reg<cpsw_nc_stat_1_ale_unkn_uni::CpswNcStat1AleUnknUniSpec>;
#[doc = "ALE Receive Unknown Unicast"]
pub mod cpsw_nc_stat_1_ale_unkn_uni;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_UNI_BCNT (rw) register accessor: ALE Receive Unknown Unicast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_uni_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_uni_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_uni_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_UNI_BCNT")]
pub type CpswNcStat1AleUnknUniBcnt =
    crate::Reg<cpsw_nc_stat_1_ale_unkn_uni_bcnt::CpswNcStat1AleUnknUniBcntSpec>;
#[doc = "ALE Receive Unknown Unicast Bytecount"]
pub mod cpsw_nc_stat_1_ale_unkn_uni_bcnt;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_MLT (rw) register accessor: ALE Receive Unknown Multicast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_mlt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_mlt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_mlt`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_MLT")]
pub type CpswNcStat1AleUnknMlt = crate::Reg<cpsw_nc_stat_1_ale_unkn_mlt::CpswNcStat1AleUnknMltSpec>;
#[doc = "ALE Receive Unknown Multicast"]
pub mod cpsw_nc_stat_1_ale_unkn_mlt;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_MLT_BCNT (rw) register accessor: ALE Receive Unknown Multicast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_mlt_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_MLT_BCNT")]
pub type CpswNcStat1AleUnknMltBcnt =
    crate::Reg<cpsw_nc_stat_1_ale_unkn_mlt_bcnt::CpswNcStat1AleUnknMltBcntSpec>;
#[doc = "ALE Receive Unknown Multicast Bytecount"]
pub mod cpsw_nc_stat_1_ale_unkn_mlt_bcnt;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_BRD (rw) register accessor: ALE Receive Unknown Broadcast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_brd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_brd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_brd`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_BRD")]
pub type CpswNcStat1AleUnknBrd = crate::Reg<cpsw_nc_stat_1_ale_unkn_brd::CpswNcStat1AleUnknBrdSpec>;
#[doc = "ALE Receive Unknown Broadcast"]
pub mod cpsw_nc_stat_1_ale_unkn_brd;
#[doc = "CPSW_NC_STAT_1_ALE_UNKN_BRD_BCNT (rw) register accessor: ALE Receive Unknown Broadcast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_brd_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_brd_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_unkn_brd_bcnt`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_UNKN_BRD_BCNT")]
pub type CpswNcStat1AleUnknBrdBcnt =
    crate::Reg<cpsw_nc_stat_1_ale_unkn_brd_bcnt::CpswNcStat1AleUnknBrdBcntSpec>;
#[doc = "ALE Receive Unknown Broadcast Bytecount"]
pub mod cpsw_nc_stat_1_ale_unkn_brd_bcnt;
#[doc = "CPSW_NC_STAT_1_ALE_POL_MATCH (rw) register accessor: ALE Policer Matched\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_pol_match::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_pol_match::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_pol_match`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_POL_MATCH")]
pub type CpswNcStat1AlePolMatch =
    crate::Reg<cpsw_nc_stat_1_ale_pol_match::CpswNcStat1AlePolMatchSpec>;
#[doc = "ALE Policer Matched"]
pub mod cpsw_nc_stat_1_ale_pol_match;
#[doc = "CPSW_NC_STAT_1_ALE_POL_MATCH_RED (rw) register accessor: ALE Policer Matched and Condition Red\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_pol_match_red::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_pol_match_red::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_pol_match_red`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_POL_MATCH_RED")]
pub type CpswNcStat1AlePolMatchRed =
    crate::Reg<cpsw_nc_stat_1_ale_pol_match_red::CpswNcStat1AlePolMatchRedSpec>;
#[doc = "ALE Policer Matched and Condition Red"]
pub mod cpsw_nc_stat_1_ale_pol_match_red;
#[doc = "CPSW_NC_STAT_1_ALE_POL_MATCH_YELLOW (rw) register accessor: ALE Policer Matched and Condition Yellow\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_pol_match_yellow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_pol_match_yellow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_pol_match_yellow`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_POL_MATCH_YELLOW")]
pub type CpswNcStat1AlePolMatchYellow =
    crate::Reg<cpsw_nc_stat_1_ale_pol_match_yellow::CpswNcStat1AlePolMatchYellowSpec>;
#[doc = "ALE Policer Matched and Condition Yellow"]
pub mod cpsw_nc_stat_1_ale_pol_match_yellow;
#[doc = "CPSW_NC_STAT_1_ALE_MULT_SA_DROP (rw) register accessor: ALE Multicast Source Address Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_mult_sa_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_mult_sa_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_mult_sa_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_MULT_SA_DROP")]
pub type CpswNcStat1AleMultSaDrop =
    crate::Reg<cpsw_nc_stat_1_ale_mult_sa_drop::CpswNcStat1AleMultSaDropSpec>;
#[doc = "ALE Multicast Source Address Drop"]
pub mod cpsw_nc_stat_1_ale_mult_sa_drop;
#[doc = "CPSW_NC_STAT_1_ALE_DUAL_VLAN_DROP (rw) register accessor: ALE Dual VLAN Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_dual_vlan_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_dual_vlan_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_dual_vlan_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_DUAL_VLAN_DROP")]
pub type CpswNcStat1AleDualVlanDrop =
    crate::Reg<cpsw_nc_stat_1_ale_dual_vlan_drop::CpswNcStat1AleDualVlanDropSpec>;
#[doc = "ALE Dual VLAN Drop"]
pub mod cpsw_nc_stat_1_ale_dual_vlan_drop;
#[doc = "CPSW_NC_STAT_1_ALE_LEN_ERROR_DROP (rw) register accessor: ALE Length Error Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_len_error_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_len_error_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_len_error_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_LEN_ERROR_DROP")]
pub type CpswNcStat1AleLenErrorDrop =
    crate::Reg<cpsw_nc_stat_1_ale_len_error_drop::CpswNcStat1AleLenErrorDropSpec>;
#[doc = "ALE Length Error Drop"]
pub mod cpsw_nc_stat_1_ale_len_error_drop;
#[doc = "CPSW_NC_STAT_1_ALE_IP_NEXT_HDR_DROP (rw) register accessor: ALE IP Next Header Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_ip_next_hdr_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_ip_next_hdr_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_ip_next_hdr_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_IP_NEXT_HDR_DROP")]
pub type CpswNcStat1AleIpNextHdrDrop =
    crate::Reg<cpsw_nc_stat_1_ale_ip_next_hdr_drop::CpswNcStat1AleIpNextHdrDropSpec>;
#[doc = "ALE IP Next Header Drop"]
pub mod cpsw_nc_stat_1_ale_ip_next_hdr_drop;
#[doc = "CPSW_NC_STAT_1_ALE_IPV4_FRAG_DROP (rw) register accessor: ALE IPV4 Frag Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_ipv4_frag_drop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_ipv4_frag_drop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_ale_ipv4_frag_drop`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ALE_IPV4_FRAG_DROP")]
pub type CpswNcStat1AleIpv4FragDrop =
    crate::Reg<cpsw_nc_stat_1_ale_ipv4_frag_drop::CpswNcStat1AleIpv4FragDropSpec>;
#[doc = "ALE IPV4 Frag Drop"]
pub mod cpsw_nc_stat_1_ale_ipv4_frag_drop;
#[doc = "CPSW_NC_STAT_1_TX_MEMORY_PROTECT_ERROR (rw) register accessor: Transmit Memory Protect CRC Error\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_tx_memory_protect_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_tx_memory_protect_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_tx_memory_protect_error`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_TX_MEMORY_PROTECT_ERROR")]
pub type CpswNcStat1TxMemoryProtectError =
    crate::Reg<cpsw_nc_stat_1_tx_memory_protect_error::CpswNcStat1TxMemoryProtectErrorSpec>;
#[doc = "Transmit Memory Protect CRC Error"]
pub mod cpsw_nc_stat_1_tx_memory_protect_error;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_0 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_0`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_0")]
pub type CpswNcStat1EnetPnTxPriReg0 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_0::CpswNcStat1EnetPnTxPriReg0Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_0;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_1 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_1`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_1")]
pub type CpswNcStat1EnetPnTxPriReg1 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_1::CpswNcStat1EnetPnTxPriReg1Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_1;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_2 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_2`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_2")]
pub type CpswNcStat1EnetPnTxPriReg2 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_2::CpswNcStat1EnetPnTxPriReg2Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_2;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_3 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_3`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_3")]
pub type CpswNcStat1EnetPnTxPriReg3 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::CpswNcStat1EnetPnTxPriReg3Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_3;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_4 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_4`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_4")]
pub type CpswNcStat1EnetPnTxPriReg4 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_4::CpswNcStat1EnetPnTxPriReg4Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_4;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_5 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_5`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_5")]
pub type CpswNcStat1EnetPnTxPriReg5 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_5::CpswNcStat1EnetPnTxPriReg5Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_5;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_6 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_6`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_6")]
pub type CpswNcStat1EnetPnTxPriReg6 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_6::CpswNcStat1EnetPnTxPriReg6Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_6;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_7 (rw) register accessor: ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_reg_7`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_7")]
pub type CpswNcStat1EnetPnTxPriReg7 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_reg_7::CpswNcStat1EnetPnTxPriReg7Spec>;
#[doc = "ENET Port n PRIORITY N Packet Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_reg_7;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_0 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_0")]
pub type CpswNcStat1EnetPnTxPriBcntReg0 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0::CpswNcStat1EnetPnTxPriBcntReg0Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_0;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_1 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_1")]
pub type CpswNcStat1EnetPnTxPriBcntReg1 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1::CpswNcStat1EnetPnTxPriBcntReg1Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_1;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_2 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_2")]
pub type CpswNcStat1EnetPnTxPriBcntReg2 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2::CpswNcStat1EnetPnTxPriBcntReg2Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_2;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_3 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_3")]
pub type CpswNcStat1EnetPnTxPriBcntReg3 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3::CpswNcStat1EnetPnTxPriBcntReg3Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_3;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_4 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_4")]
pub type CpswNcStat1EnetPnTxPriBcntReg4 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4::CpswNcStat1EnetPnTxPriBcntReg4Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_4;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_5 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_5")]
pub type CpswNcStat1EnetPnTxPriBcntReg5 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5::CpswNcStat1EnetPnTxPriBcntReg5Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_5;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_6 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_6")]
pub type CpswNcStat1EnetPnTxPriBcntReg6 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6::CpswNcStat1EnetPnTxPriBcntReg6Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_6;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_7 (rw) register accessor: ENET Port n PRIORITY N Packet Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_BCNT_REG_7")]
pub type CpswNcStat1EnetPnTxPriBcntReg7 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7::CpswNcStat1EnetPnTxPriBcntReg7Spec>;
#[doc = "ENET Port n PRIORITY N Packet Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_bcnt_reg_7;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_0 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_0")]
pub type CpswNcStat1EnetPnTxPriDropReg0 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0::CpswNcStat1EnetPnTxPriDropReg0Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_0;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_1 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_1")]
pub type CpswNcStat1EnetPnTxPriDropReg1 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1::CpswNcStat1EnetPnTxPriDropReg1Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_1;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_2 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_2")]
pub type CpswNcStat1EnetPnTxPriDropReg2 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2::CpswNcStat1EnetPnTxPriDropReg2Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_2;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_3 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_3")]
pub type CpswNcStat1EnetPnTxPriDropReg3 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3::CpswNcStat1EnetPnTxPriDropReg3Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_3;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_4 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_4")]
pub type CpswNcStat1EnetPnTxPriDropReg4 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4::CpswNcStat1EnetPnTxPriDropReg4Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_4;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_5 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_5")]
pub type CpswNcStat1EnetPnTxPriDropReg5 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5::CpswNcStat1EnetPnTxPriDropReg5Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_5;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_6 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_6")]
pub type CpswNcStat1EnetPnTxPriDropReg6 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6::CpswNcStat1EnetPnTxPriDropReg6Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_6;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_7 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_REG_7")]
pub type CpswNcStat1EnetPnTxPriDropReg7 =
    crate::Reg<cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7::CpswNcStat1EnetPnTxPriDropReg7Spec>;
#[doc = "ENET Port n PRIORITY N Packet Drop Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_reg_7;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_0 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_0")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg0 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0::CpswNcStat1EnetPnTxPriDropBcntReg0Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_0;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_1 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_1")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg1 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1::CpswNcStat1EnetPnTxPriDropBcntReg1Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_1;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_2 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_2")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg2 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2::CpswNcStat1EnetPnTxPriDropBcntReg2Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_2;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_3 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_3")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg3 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3::CpswNcStat1EnetPnTxPriDropBcntReg3Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_3;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_4 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_4")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg4 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4::CpswNcStat1EnetPnTxPriDropBcntReg4Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_4;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_5 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_5")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg5 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5::CpswNcStat1EnetPnTxPriDropBcntReg5Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_5;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_6 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_6")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg6 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::CpswNcStat1EnetPnTxPriDropBcntReg6Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6;
#[doc = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_7 (rw) register accessor: ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7`]
module"]
#[doc(alias = "CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_7")]
pub type CpswNcStat1EnetPnTxPriDropBcntReg7 = crate::Reg<
    cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7::CpswNcStat1EnetPnTxPriDropBcntReg7Spec,
>;
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count"]
pub mod cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_7;
#[doc = "IDVER_REG (rw) register accessor: Identification and Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idver_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idver_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idver_reg`]
module"]
#[doc(alias = "IDVER_REG")]
pub type IdverReg = crate::Reg<idver_reg::IdverRegSpec>;
#[doc = "Identification and Version Register"]
pub mod idver_reg;
#[doc = "CPTS_CONTROL_REG (rw) register accessor: Time Sync Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_control_reg`]
module"]
#[doc(alias = "CPTS_CONTROL_REG")]
pub type CptsControlReg = crate::Reg<cpts_control_reg::CptsControlRegSpec>;
#[doc = "Time Sync Control Register"]
pub mod cpts_control_reg;
#[doc = "CPTS_RFTCLK_SEL_REG (rw) register accessor: RFTCLK Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_rftclk_sel_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_rftclk_sel_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_rftclk_sel_reg`]
module"]
#[doc(alias = "CPTS_RFTCLK_SEL_REG")]
pub type CptsRftclkSelReg = crate::Reg<cpts_rftclk_sel_reg::CptsRftclkSelRegSpec>;
#[doc = "RFTCLK Select Register"]
pub mod cpts_rftclk_sel_reg;
#[doc = "CPTS_TS_PUSH_REG (rw) register accessor: Time Stamp Event Push Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_push_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_push_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_push_reg`]
module"]
#[doc(alias = "CPTS_TS_PUSH_REG")]
pub type CptsTsPushReg = crate::Reg<cpts_ts_push_reg::CptsTsPushRegSpec>;
#[doc = "Time Stamp Event Push Register"]
pub mod cpts_ts_push_reg;
#[doc = "TS_LOAD_VAL_REG (rw) register accessor: Time Stamp Load Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_load_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_load_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_load_val_reg`]
module"]
#[doc(alias = "TS_LOAD_VAL_REG")]
pub type TsLoadValReg = crate::Reg<ts_load_val_reg::TsLoadValRegSpec>;
#[doc = "Time Stamp Load Low Value Register"]
pub mod ts_load_val_reg;
#[doc = "CPTS_TS_LOAD_EN_REG (rw) register accessor: Time Stamp Load Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_load_en_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_load_en_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_load_en_reg`]
module"]
#[doc(alias = "CPTS_TS_LOAD_EN_REG")]
pub type CptsTsLoadEnReg = crate::Reg<cpts_ts_load_en_reg::CptsTsLoadEnRegSpec>;
#[doc = "Time Stamp Load Enable Register"]
pub mod cpts_ts_load_en_reg;
#[doc = "TS_COMP_VAL_REG (rw) register accessor: Time Stamp Comparison Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_comp_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_comp_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_comp_val_reg`]
module"]
#[doc(alias = "TS_COMP_VAL_REG")]
pub type TsCompValReg = crate::Reg<ts_comp_val_reg::TsCompValRegSpec>;
#[doc = "Time Stamp Comparison Low Value Register"]
pub mod ts_comp_val_reg;
#[doc = "CPTS_TS_COMP_LEN_REG (rw) register accessor: Time Stamp Comparison Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_comp_len_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_comp_len_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_comp_len_reg`]
module"]
#[doc(alias = "CPTS_TS_COMP_LEN_REG")]
pub type CptsTsCompLenReg = crate::Reg<cpts_ts_comp_len_reg::CptsTsCompLenRegSpec>;
#[doc = "Time Stamp Comparison Length Register"]
pub mod cpts_ts_comp_len_reg;
#[doc = "CPTS_INTSTAT_RAW_REG (rw) register accessor: Interrupt Status Register Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_intstat_raw_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_intstat_raw_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_intstat_raw_reg`]
module"]
#[doc(alias = "CPTS_INTSTAT_RAW_REG")]
pub type CptsIntstatRawReg = crate::Reg<cpts_intstat_raw_reg::CptsIntstatRawRegSpec>;
#[doc = "Interrupt Status Register Raw"]
pub mod cpts_intstat_raw_reg;
#[doc = "CPTS_INTSTAT_MASKED_REG (rw) register accessor: Interrupt Status Register Masked\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_intstat_masked_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_intstat_masked_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_intstat_masked_reg`]
module"]
#[doc(alias = "CPTS_INTSTAT_MASKED_REG")]
pub type CptsIntstatMaskedReg = crate::Reg<cpts_intstat_masked_reg::CptsIntstatMaskedRegSpec>;
#[doc = "Interrupt Status Register Masked"]
pub mod cpts_intstat_masked_reg;
#[doc = "CPTS_INT_ENABLE_REG (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_int_enable_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_int_enable_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_int_enable_reg`]
module"]
#[doc(alias = "CPTS_INT_ENABLE_REG")]
pub type CptsIntEnableReg = crate::Reg<cpts_int_enable_reg::CptsIntEnableRegSpec>;
#[doc = "Interrupt Enable Register"]
pub mod cpts_int_enable_reg;
#[doc = "CPTS_TS_COMP_NUDGE_REG (rw) register accessor: Time Stamp Comparison Nudge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_comp_nudge_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_comp_nudge_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_comp_nudge_reg`]
module"]
#[doc(alias = "CPTS_TS_COMP_NUDGE_REG")]
pub type CptsTsCompNudgeReg = crate::Reg<cpts_ts_comp_nudge_reg::CptsTsCompNudgeRegSpec>;
#[doc = "Time Stamp Comparison Nudge Register"]
pub mod cpts_ts_comp_nudge_reg;
#[doc = "CPTS_EVENT_POP_REG (rw) register accessor: Event Pop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_pop_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_pop_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_event_pop_reg`]
module"]
#[doc(alias = "CPTS_EVENT_POP_REG")]
pub type CptsEventPopReg = crate::Reg<cpts_event_pop_reg::CptsEventPopRegSpec>;
#[doc = "Event Pop Register"]
pub mod cpts_event_pop_reg;
#[doc = "CPTS_EVENT_0_REG (rw) register accessor: Event 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_0_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_0_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_event_0_reg`]
module"]
#[doc(alias = "CPTS_EVENT_0_REG")]
pub type CptsEvent0Reg = crate::Reg<cpts_event_0_reg::CptsEvent0RegSpec>;
#[doc = "Event 0 Register"]
pub mod cpts_event_0_reg;
#[doc = "CPTS_EVENT_1_REG (rw) register accessor: Event 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_1_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_1_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_event_1_reg`]
module"]
#[doc(alias = "CPTS_EVENT_1_REG")]
pub type CptsEvent1Reg = crate::Reg<cpts_event_1_reg::CptsEvent1RegSpec>;
#[doc = "Event 1 Register"]
pub mod cpts_event_1_reg;
#[doc = "CPTS_EVENT_2_REG (rw) register accessor: Event 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_event_2_reg`]
module"]
#[doc(alias = "CPTS_EVENT_2_REG")]
pub type CptsEvent2Reg = crate::Reg<cpts_event_2_reg::CptsEvent2RegSpec>;
#[doc = "Event 2 Register"]
pub mod cpts_event_2_reg;
#[doc = "CPTS_EVENT_3_REG (rw) register accessor: Event 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_3_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_3_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_event_3_reg`]
module"]
#[doc(alias = "CPTS_EVENT_3_REG")]
pub type CptsEvent3Reg = crate::Reg<cpts_event_3_reg::CptsEvent3RegSpec>;
#[doc = "Event 3 Register"]
pub mod cpts_event_3_reg;
#[doc = "CPTS_TS_LOAD_HIGH_VAL_REG (rw) register accessor: Time Stamp Load High Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_load_high_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_load_high_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_load_high_val_reg`]
module"]
#[doc(alias = "CPTS_TS_LOAD_HIGH_VAL_REG")]
pub type CptsTsLoadHighValReg = crate::Reg<cpts_ts_load_high_val_reg::CptsTsLoadHighValRegSpec>;
#[doc = "Time Stamp Load High Value Register"]
pub mod cpts_ts_load_high_val_reg;
#[doc = "CPTS_TS_COMP_HIGH_VAL_REG (rw) register accessor: Time Stamp Comparison High Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_comp_high_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_comp_high_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_comp_high_val_reg`]
module"]
#[doc(alias = "CPTS_TS_COMP_HIGH_VAL_REG")]
pub type CptsTsCompHighValReg = crate::Reg<cpts_ts_comp_high_val_reg::CptsTsCompHighValRegSpec>;
#[doc = "Time Stamp Comparison High Value Register"]
pub mod cpts_ts_comp_high_val_reg;
#[doc = "CPTS_TS_ADD_VAL_REG (rw) register accessor: TS Add Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_add_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_add_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_add_val_reg`]
module"]
#[doc(alias = "CPTS_TS_ADD_VAL_REG")]
pub type CptsTsAddValReg = crate::Reg<cpts_ts_add_val_reg::CptsTsAddValRegSpec>;
#[doc = "TS Add Value Register"]
pub mod cpts_ts_add_val_reg;
#[doc = "CPTS_TS_PPM_LOW_VAL_REG (rw) register accessor: Time Stamp PPM Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_ppm_low_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_ppm_low_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_ppm_low_val_reg`]
module"]
#[doc(alias = "CPTS_TS_PPM_LOW_VAL_REG")]
pub type CptsTsPpmLowValReg = crate::Reg<cpts_ts_ppm_low_val_reg::CptsTsPpmLowValRegSpec>;
#[doc = "Time Stamp PPM Low Value Register"]
pub mod cpts_ts_ppm_low_val_reg;
#[doc = "CPTS_TS_PPM_HIGH_VAL_REG (rw) register accessor: Time Stamp PPM High Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_ppm_high_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_ppm_high_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_ppm_high_val_reg`]
module"]
#[doc(alias = "CPTS_TS_PPM_HIGH_VAL_REG")]
pub type CptsTsPpmHighValReg = crate::Reg<cpts_ts_ppm_high_val_reg::CptsTsPpmHighValRegSpec>;
#[doc = "Time Stamp PPM High Value Register"]
pub mod cpts_ts_ppm_high_val_reg;
#[doc = "CPTS_TS_NUDGE_VAL_REG (rw) register accessor: Time Stamp Nudge Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_nudge_val_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_nudge_val_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_nudge_val_reg`]
module"]
#[doc(alias = "CPTS_TS_NUDGE_VAL_REG")]
pub type CptsTsNudgeValReg = crate::Reg<cpts_ts_nudge_val_reg::CptsTsNudgeValRegSpec>;
#[doc = "Time Stamp Nudge Value Register"]
pub mod cpts_ts_nudge_val_reg;
#[doc = "CPTS_TS_CONFIG (rw) register accessor: Time Stamp Configuration Read\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_ts_config`]
module"]
#[doc(alias = "CPTS_TS_CONFIG")]
pub type CptsTsConfig = crate::Reg<cpts_ts_config::CptsTsConfigSpec>;
#[doc = "Time Stamp Configuration Read"]
pub mod cpts_ts_config;
#[doc = "TS_GENF0_COMP_LOW_REG (rw) register accessor: Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_comp_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_comp_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_comp_low_reg`]
module"]
#[doc(alias = "TS_GENF0_COMP_LOW_REG")]
pub type TsGenf0CompLowReg = crate::Reg<ts_genf0_comp_low_reg::TsGenf0CompLowRegSpec>;
#[doc = "Time Stamp Generate Function Comparison Low Value"]
pub mod ts_genf0_comp_low_reg;
#[doc = "TS_GENF0_COMP_HIGH_REG (rw) register accessor: Time Stamp Generate Function Comparison high Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_comp_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_comp_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_comp_high_reg`]
module"]
#[doc(alias = "TS_GENF0_COMP_HIGH_REG")]
pub type TsGenf0CompHighReg = crate::Reg<ts_genf0_comp_high_reg::TsGenf0CompHighRegSpec>;
#[doc = "Time Stamp Generate Function Comparison high Value"]
pub mod ts_genf0_comp_high_reg;
#[doc = "TS_GENF0_CONTROL_REG (rw) register accessor: Time Stamp Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_control_reg`]
module"]
#[doc(alias = "TS_GENF0_CONTROL_REG")]
pub type TsGenf0ControlReg = crate::Reg<ts_genf0_control_reg::TsGenf0ControlRegSpec>;
#[doc = "Time Stamp Generate Function Control"]
pub mod ts_genf0_control_reg;
#[doc = "TS_GENF0_LENGTH_REG (rw) register accessor: Time Stamp Generate Function Length Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_length_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_length_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_length_reg`]
module"]
#[doc(alias = "TS_GENF0_LENGTH_REG")]
pub type TsGenf0LengthReg = crate::Reg<ts_genf0_length_reg::TsGenf0LengthRegSpec>;
#[doc = "Time Stamp Generate Function Length Value"]
pub mod ts_genf0_length_reg;
#[doc = "TS_GENF0_PPM_LOW_REG (rw) register accessor: Time Stamp Generate Function PPM Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_ppm_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_ppm_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_ppm_low_reg`]
module"]
#[doc(alias = "TS_GENF0_PPM_LOW_REG")]
pub type TsGenf0PpmLowReg = crate::Reg<ts_genf0_ppm_low_reg::TsGenf0PpmLowRegSpec>;
#[doc = "Time Stamp Generate Function PPM Low Value"]
pub mod ts_genf0_ppm_low_reg;
#[doc = "TS_GENF0_PPM_HIGH_REG (rw) register accessor: Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_ppm_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_ppm_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_ppm_high_reg`]
module"]
#[doc(alias = "TS_GENF0_PPM_HIGH_REG")]
pub type TsGenf0PpmHighReg = crate::Reg<ts_genf0_ppm_high_reg::TsGenf0PpmHighRegSpec>;
#[doc = "Time Stamp Generate Function PPM High Value"]
pub mod ts_genf0_ppm_high_reg;
#[doc = "TS_GENF0_NUDGE_REG (rw) register accessor: Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_nudge_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_nudge_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf0_nudge_reg`]
module"]
#[doc(alias = "TS_GENF0_NUDGE_REG")]
pub type TsGenf0NudgeReg = crate::Reg<ts_genf0_nudge_reg::TsGenf0NudgeRegSpec>;
#[doc = "Time Stamp Generate Function Nudge Value"]
pub mod ts_genf0_nudge_reg;
#[doc = "TS_GENF1_COMP_LOW_REG (rw) register accessor: Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_comp_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_comp_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_comp_low_reg`]
module"]
#[doc(alias = "TS_GENF1_COMP_LOW_REG")]
pub type TsGenf1CompLowReg = crate::Reg<ts_genf1_comp_low_reg::TsGenf1CompLowRegSpec>;
#[doc = "Time Stamp Generate Function Comparison Low Value"]
pub mod ts_genf1_comp_low_reg;
#[doc = "TS_GENF1_COMP_HIGH_REG (rw) register accessor: Time Stamp Generate Function Comparison high Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_comp_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_comp_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_comp_high_reg`]
module"]
#[doc(alias = "TS_GENF1_COMP_HIGH_REG")]
pub type TsGenf1CompHighReg = crate::Reg<ts_genf1_comp_high_reg::TsGenf1CompHighRegSpec>;
#[doc = "Time Stamp Generate Function Comparison high Value"]
pub mod ts_genf1_comp_high_reg;
#[doc = "TS_GENF1_CONTROL_REG (rw) register accessor: Time Stamp Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_control_reg`]
module"]
#[doc(alias = "TS_GENF1_CONTROL_REG")]
pub type TsGenf1ControlReg = crate::Reg<ts_genf1_control_reg::TsGenf1ControlRegSpec>;
#[doc = "Time Stamp Generate Function Control"]
pub mod ts_genf1_control_reg;
#[doc = "TS_GENF1_LENGTH_REG (rw) register accessor: Time Stamp Generate Function Length Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_length_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_length_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_length_reg`]
module"]
#[doc(alias = "TS_GENF1_LENGTH_REG")]
pub type TsGenf1LengthReg = crate::Reg<ts_genf1_length_reg::TsGenf1LengthRegSpec>;
#[doc = "Time Stamp Generate Function Length Value"]
pub mod ts_genf1_length_reg;
#[doc = "TS_GENF1_PPM_LOW_REG (rw) register accessor: Time Stamp Generate Function PPM Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_ppm_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_ppm_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_ppm_low_reg`]
module"]
#[doc(alias = "TS_GENF1_PPM_LOW_REG")]
pub type TsGenf1PpmLowReg = crate::Reg<ts_genf1_ppm_low_reg::TsGenf1PpmLowRegSpec>;
#[doc = "Time Stamp Generate Function PPM Low Value"]
pub mod ts_genf1_ppm_low_reg;
#[doc = "TS_GENF1_PPM_HIGH_REG (rw) register accessor: Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_ppm_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_ppm_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_ppm_high_reg`]
module"]
#[doc(alias = "TS_GENF1_PPM_HIGH_REG")]
pub type TsGenf1PpmHighReg = crate::Reg<ts_genf1_ppm_high_reg::TsGenf1PpmHighRegSpec>;
#[doc = "Time Stamp Generate Function PPM High Value"]
pub mod ts_genf1_ppm_high_reg;
#[doc = "TS_GENF1_NUDGE_REG (rw) register accessor: Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_nudge_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_nudge_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf1_nudge_reg`]
module"]
#[doc(alias = "TS_GENF1_NUDGE_REG")]
pub type TsGenf1NudgeReg = crate::Reg<ts_genf1_nudge_reg::TsGenf1NudgeRegSpec>;
#[doc = "Time Stamp Generate Function Nudge Value"]
pub mod ts_genf1_nudge_reg;
#[doc = "TS_GENF2_COMP_LOW_REG (rw) register accessor: Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_comp_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_comp_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_comp_low_reg`]
module"]
#[doc(alias = "TS_GENF2_COMP_LOW_REG")]
pub type TsGenf2CompLowReg = crate::Reg<ts_genf2_comp_low_reg::TsGenf2CompLowRegSpec>;
#[doc = "Time Stamp Generate Function Comparison Low Value"]
pub mod ts_genf2_comp_low_reg;
#[doc = "TS_GENF2_COMP_HIGH_REG (rw) register accessor: Time Stamp Generate Function Comparison high Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_comp_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_comp_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_comp_high_reg`]
module"]
#[doc(alias = "TS_GENF2_COMP_HIGH_REG")]
pub type TsGenf2CompHighReg = crate::Reg<ts_genf2_comp_high_reg::TsGenf2CompHighRegSpec>;
#[doc = "Time Stamp Generate Function Comparison high Value"]
pub mod ts_genf2_comp_high_reg;
#[doc = "TS_GENF2_CONTROL_REG (rw) register accessor: Time Stamp Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_control_reg`]
module"]
#[doc(alias = "TS_GENF2_CONTROL_REG")]
pub type TsGenf2ControlReg = crate::Reg<ts_genf2_control_reg::TsGenf2ControlRegSpec>;
#[doc = "Time Stamp Generate Function Control"]
pub mod ts_genf2_control_reg;
#[doc = "TS_GENF2_LENGTH_REG (rw) register accessor: Time Stamp Generate Function Length Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_length_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_length_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_length_reg`]
module"]
#[doc(alias = "TS_GENF2_LENGTH_REG")]
pub type TsGenf2LengthReg = crate::Reg<ts_genf2_length_reg::TsGenf2LengthRegSpec>;
#[doc = "Time Stamp Generate Function Length Value"]
pub mod ts_genf2_length_reg;
#[doc = "TS_GENF2_PPM_LOW_REG (rw) register accessor: Time Stamp Generate Function PPM Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_ppm_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_ppm_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_ppm_low_reg`]
module"]
#[doc(alias = "TS_GENF2_PPM_LOW_REG")]
pub type TsGenf2PpmLowReg = crate::Reg<ts_genf2_ppm_low_reg::TsGenf2PpmLowRegSpec>;
#[doc = "Time Stamp Generate Function PPM Low Value"]
pub mod ts_genf2_ppm_low_reg;
#[doc = "TS_GENF2_PPM_HIGH_REG (rw) register accessor: Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_ppm_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_ppm_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_ppm_high_reg`]
module"]
#[doc(alias = "TS_GENF2_PPM_HIGH_REG")]
pub type TsGenf2PpmHighReg = crate::Reg<ts_genf2_ppm_high_reg::TsGenf2PpmHighRegSpec>;
#[doc = "Time Stamp Generate Function PPM High Value"]
pub mod ts_genf2_ppm_high_reg;
#[doc = "TS_GENF2_NUDGE_REG (rw) register accessor: Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_nudge_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_nudge_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_genf2_nudge_reg`]
module"]
#[doc(alias = "TS_GENF2_NUDGE_REG")]
pub type TsGenf2NudgeReg = crate::Reg<ts_genf2_nudge_reg::TsGenf2NudgeRegSpec>;
#[doc = "Time Stamp Generate Function Nudge Value"]
pub mod ts_genf2_nudge_reg;
#[doc = "TS_ESTF_COMP_LOW_REG (rw) register accessor: Time Stamp ESTF Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_comp_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_comp_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_comp_low_reg`]
module"]
#[doc(alias = "TS_ESTF_COMP_LOW_REG")]
pub type TsEstfCompLowReg = crate::Reg<ts_estf_comp_low_reg::TsEstfCompLowRegSpec>;
#[doc = "Time Stamp ESTF Generate Function Comparison Low Value"]
pub mod ts_estf_comp_low_reg;
#[doc = "TS_ESTF_COMP_HIGH_REG (rw) register accessor: Time Stamp ESTF Generate Function Comparison high Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_comp_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_comp_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_comp_high_reg`]
module"]
#[doc(alias = "TS_ESTF_COMP_HIGH_REG")]
pub type TsEstfCompHighReg = crate::Reg<ts_estf_comp_high_reg::TsEstfCompHighRegSpec>;
#[doc = "Time Stamp ESTF Generate Function Comparison high Value"]
pub mod ts_estf_comp_high_reg;
#[doc = "TS_ESTF_CONTROL_REG (rw) register accessor: Time Stamp ESTF Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_control_reg`]
module"]
#[doc(alias = "TS_ESTF_CONTROL_REG")]
pub type TsEstfControlReg = crate::Reg<ts_estf_control_reg::TsEstfControlRegSpec>;
#[doc = "Time Stamp ESTF Generate Function Control"]
pub mod ts_estf_control_reg;
#[doc = "TS_ESTF_LENGTH_REG (rw) register accessor: Time Stamp ESTF Generate Function Length Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_length_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_length_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_length_reg`]
module"]
#[doc(alias = "TS_ESTF_LENGTH_REG")]
pub type TsEstfLengthReg = crate::Reg<ts_estf_length_reg::TsEstfLengthRegSpec>;
#[doc = "Time Stamp ESTF Generate Function Length Value"]
pub mod ts_estf_length_reg;
#[doc = "TS_ESTF_PPM_LOW_REG (rw) register accessor: Time Stamp ESTF Generate Function PPM Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_ppm_low_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_ppm_low_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_ppm_low_reg`]
module"]
#[doc(alias = "TS_ESTF_PPM_LOW_REG")]
pub type TsEstfPpmLowReg = crate::Reg<ts_estf_ppm_low_reg::TsEstfPpmLowRegSpec>;
#[doc = "Time Stamp ESTF Generate Function PPM Low Value"]
pub mod ts_estf_ppm_low_reg;
#[doc = "TS_ESTF_PPM_HIGH_REG (rw) register accessor: Time Stamp ESTF Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_ppm_high_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_ppm_high_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_ppm_high_reg`]
module"]
#[doc(alias = "TS_ESTF_PPM_HIGH_REG")]
pub type TsEstfPpmHighReg = crate::Reg<ts_estf_ppm_high_reg::TsEstfPpmHighRegSpec>;
#[doc = "Time Stamp ESTF Generate Function PPM High Value"]
pub mod ts_estf_ppm_high_reg;
#[doc = "TS_ESTF_NUDGE_REG (rw) register accessor: Time Stamp ESTF Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_nudge_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_nudge_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_estf_nudge_reg`]
module"]
#[doc(alias = "TS_ESTF_NUDGE_REG")]
pub type TsEstfNudgeReg = crate::Reg<ts_estf_nudge_reg::TsEstfNudgeRegSpec>;
#[doc = "Time Stamp ESTF Generate Function Nudge Value"]
pub mod ts_estf_nudge_reg;
#[doc = "ALE_MOD_VER (rw) register accessor: The Module and Version Register identifies the module identifier and revision of the ALE_2g32 module.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_mod_ver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_mod_ver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_mod_ver`]
module"]
#[doc(alias = "ALE_MOD_VER")]
pub type AleModVer = crate::Reg<ale_mod_ver::AleModVerSpec>;
#[doc = "The Module and Version Register identifies the module identifier and revision of the ALE_2g32 module."]
pub mod ale_mod_ver;
#[doc = "ALE_ALE_STATUS (rw) register accessor: The ALE status provides information on the ALE configuration and state. The ~iramdepth is used to determine how IPv6 entries are stored in the table. IPv6 entries are stored in two entries where IPv6 Entry hi is designated by the odd slice index and lo is designated by the even slice index. The slice index is above the ram depth like {SlixeIndex,RamIndex}. So for a 64 deep RAM index of 0x005, the Hi portion of the IPv6 entry is located at 0x005|0x040 and the Lo portion is located at 0x005&amp;amp;(~0x040).\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_status`]
module"]
#[doc(alias = "ALE_ALE_STATUS")]
pub type AleAleStatus = crate::Reg<ale_ale_status::AleAleStatusSpec>;
#[doc = "The ALE status provides information on the ALE configuration and state. The ~iramdepth is used to determine how IPv6 entries are stored in the table. IPv6 entries are stored in two entries where IPv6 Entry hi is designated by the odd slice index and lo is designated by the even slice index. The slice index is above the ram depth like {SlixeIndex,RamIndex}. So for a 64 deep RAM index of 0x005, the Hi portion of the IPv6 entry is located at 0x005|0x040 and the Lo portion is located at 0x005&amp;amp;(~0x040)."]
pub mod ale_ale_status;
#[doc = "ALE_ALE_CONTROL (rw) register accessor: The ALE Control Register is used to set the ALE modes used for all ports.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_control`]
module"]
#[doc(alias = "ALE_ALE_CONTROL")]
pub type AleAleControl = crate::Reg<ale_ale_control::AleAleControlSpec>;
#[doc = "The ALE Control Register is used to set the ALE modes used for all ports."]
pub mod ale_ale_control;
#[doc = "ALE_ALE_CTRL2 (rw) register accessor: The ALE Control 2 Register is used to set the extended features used for all ports.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_ctrl2`]
module"]
#[doc(alias = "ALE_ALE_CTRL2")]
pub type AleAleCtrl2 = crate::Reg<ale_ale_ctrl2::AleAleCtrl2Spec>;
#[doc = "The ALE Control 2 Register is used to set the extended features used for all ports."]
pub mod ale_ale_ctrl2;
#[doc = "ALE_ALE_PRESCALE (rw) register accessor: The ALE Prescale Register is used to set the Broadcast and Multicast rate limiting prescaler value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_prescale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_prescale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_prescale`]
module"]
#[doc(alias = "ALE_ALE_PRESCALE")]
pub type AleAlePrescale = crate::Reg<ale_ale_prescale::AleAlePrescaleSpec>;
#[doc = "The ALE Prescale Register is used to set the Broadcast and Multicast rate limiting prescaler value."]
pub mod ale_ale_prescale;
#[doc = "ALE_ALE_AGING_CTRL (rw) register accessor: The ALE Aging Control sets the aging interval which will cause periodic aging to occur. This value specifies the minimum time between aging starts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_aging_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_aging_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_aging_ctrl`]
module"]
#[doc(alias = "ALE_ALE_AGING_CTRL")]
pub type AleAleAgingCtrl = crate::Reg<ale_ale_aging_ctrl::AleAleAgingCtrlSpec>;
#[doc = "The ALE Aging Control sets the aging interval which will cause periodic aging to occur. This value specifies the minimum time between aging starts."]
pub mod ale_ale_aging_ctrl;
#[doc = "ALE_ALE_NXT_HDR (rw) register accessor: The ALE Next Header is used to limit the IPv6 Next header or IPv4 Protocol values found in the IP header. It is enabled via the ~iLmtNxtHdr bit in the VLAN entry. All four ~iip_nxt_hdr0-3 are compared when enabled, so if only one is required, set them all to the one value to be tested.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_nxt_hdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_nxt_hdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_nxt_hdr`]
module"]
#[doc(alias = "ALE_ALE_NXT_HDR")]
pub type AleAleNxtHdr = crate::Reg<ale_ale_nxt_hdr::AleAleNxtHdrSpec>;
#[doc = "The ALE Next Header is used to limit the IPv6 Next header or IPv4 Protocol values found in the IP header. It is enabled via the ~iLmtNxtHdr bit in the VLAN entry. All four ~iip_nxt_hdr0-3 are compared when enabled, so if only one is required, set them all to the one value to be tested."]
pub mod ale_ale_nxt_hdr;
#[doc = "ALE_ALE_TBLCTL (rw) register accessor: The ALE table control register is used to read or write that ALE table entries. After writing to this register any read or write to any ALE register will be stalled until the read or write operation completes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_tblctl`]
module"]
#[doc(alias = "ALE_ALE_TBLCTL")]
pub type AleAleTblctl = crate::Reg<ale_ale_tblctl::AleAleTblctlSpec>;
#[doc = "The ALE table control register is used to read or write that ALE table entries. After writing to this register any read or write to any ALE register will be stalled until the read or write operation completes."]
pub mod ale_ale_tblctl;
#[doc = "ALE_ALE_TBLW2 (rw) register accessor: The ALE Table Word 2 is the most significant word of an ALE table entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_tblw2`]
module"]
#[doc(alias = "ALE_ALE_TBLW2")]
pub type AleAleTblw2 = crate::Reg<ale_ale_tblw2::AleAleTblw2Spec>;
#[doc = "The ALE Table Word 2 is the most significant word of an ALE table entry."]
pub mod ale_ale_tblw2;
#[doc = "ALE_ALE_TBLW1 (rw) register accessor: The ALE Table Word 1 is the middle word of an ALE table entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_tblw1`]
module"]
#[doc(alias = "ALE_ALE_TBLW1")]
pub type AleAleTblw1 = crate::Reg<ale_ale_tblw1::AleAleTblw1Spec>;
#[doc = "The ALE Table Word 1 is the middle word of an ALE table entry."]
pub mod ale_ale_tblw1;
#[doc = "ALE_ALE_TBLW0 (rw) register accessor: The ALE Table Word 0 is the least significant word of an ALE table entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_tblw0`]
module"]
#[doc(alias = "ALE_ALE_TBLW0")]
pub type AleAleTblw0 = crate::Reg<ale_ale_tblw0::AleAleTblw0Spec>;
#[doc = "The ALE Table Word 0 is the least significant word of an ALE table entry."]
pub mod ale_ale_tblw0;
#[doc = "ALE_I0_ALE_PORTCTL0_0 (rw) register accessor: The ALE Port Control Register sets the port specific modes of operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i0_ale_portctl0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i0_ale_portctl0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_i0_ale_portctl0_0`]
module"]
#[doc(alias = "ALE_I0_ALE_PORTCTL0_0")]
pub type AleI0AlePortctl0_0 = crate::Reg<ale_i0_ale_portctl0_0::AleI0AlePortctl0_0Spec>;
#[doc = "The ALE Port Control Register sets the port specific modes of operation."]
pub mod ale_i0_ale_portctl0_0;
#[doc = "ALE_I0_ALE_PORTCTL0_1 (rw) register accessor: The ALE Port Control Register sets the port specific modes of operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i0_ale_portctl0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i0_ale_portctl0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_i0_ale_portctl0_1`]
module"]
#[doc(alias = "ALE_I0_ALE_PORTCTL0_1")]
pub type AleI0AlePortctl0_1 = crate::Reg<ale_i0_ale_portctl0_1::AleI0AlePortctl0_1Spec>;
#[doc = "The ALE Port Control Register sets the port specific modes of operation."]
pub mod ale_i0_ale_portctl0_1;
#[doc = "ALE_ALE_UVLAN_MEMBER (rw) register accessor: The ALE Unknown VLAN Member Mask Register is used to specify the member list for unknown VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_member::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_member::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_uvlan_member`]
module"]
#[doc(alias = "ALE_ALE_UVLAN_MEMBER")]
pub type AleAleUvlanMember = crate::Reg<ale_ale_uvlan_member::AleAleUvlanMemberSpec>;
#[doc = "The ALE Unknown VLAN Member Mask Register is used to specify the member list for unknown VLAN ID."]
pub mod ale_ale_uvlan_member;
#[doc = "ALE_ALE_UVLAN_URCAST (rw) register accessor: The ALE Unknown VLAN Unregistered Multicast Flood Mask Register is used to specify which egress ports unregistered multicast addresses egress for the unregistered VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_urcast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_urcast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_uvlan_urcast`]
module"]
#[doc(alias = "ALE_ALE_UVLAN_URCAST")]
pub type AleAleUvlanUrcast = crate::Reg<ale_ale_uvlan_urcast::AleAleUvlanUrcastSpec>;
#[doc = "The ALE Unknown VLAN Unregistered Multicast Flood Mask Register is used to specify which egress ports unregistered multicast addresses egress for the unregistered VLAN ID."]
pub mod ale_ale_uvlan_urcast;
#[doc = "ALE_ALE_UVLAN_RMCAST (rw) register accessor: The ALE Unknown VLAN Registered Multicast Flood Mask Register is used to specify which egress ports registered multicast addresses egress for the unregistered VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_rmcast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_rmcast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_uvlan_rmcast`]
module"]
#[doc(alias = "ALE_ALE_UVLAN_RMCAST")]
pub type AleAleUvlanRmcast = crate::Reg<ale_ale_uvlan_rmcast::AleAleUvlanRmcastSpec>;
#[doc = "The ALE Unknown VLAN Registered Multicast Flood Mask Register is used to specify which egress ports registered multicast addresses egress for the unregistered VLAN ID."]
pub mod ale_ale_uvlan_rmcast;
#[doc = "ALE_ALE_UVLAN_UNTAG (rw) register accessor: The ALE Unknown VLAN force Untagged Egress Mask Register is used to specify which egress ports the VLAN ID will be removed.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_untag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_untag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_uvlan_untag`]
module"]
#[doc(alias = "ALE_ALE_UVLAN_UNTAG")]
pub type AleAleUvlanUntag = crate::Reg<ale_ale_uvlan_untag::AleAleUvlanUntagSpec>;
#[doc = "The ALE Unknown VLAN force Untagged Egress Mask Register is used to specify which egress ports the VLAN ID will be removed."]
pub mod ale_ale_uvlan_untag;
#[doc = "ALE_ALE_FAST_LUT (rw) register accessor: The Fast LUT registers allows the ports to be placed in Fast LUT mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_fast_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_fast_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_fast_lut`]
module"]
#[doc(alias = "ALE_ALE_FAST_LUT")]
pub type AleAleFastLut = crate::Reg<ale_ale_fast_lut::AleAleFastLutSpec>;
#[doc = "The Fast LUT registers allows the ports to be placed in Fast LUT mode."]
pub mod ale_ale_fast_lut;
#[doc = "ALE_ALE_STAT_DIAG (rw) register accessor: The ALE Statistic Output Diagnostic Register allows the output statistics to diagnose the SW counters. This register is for diagnostice only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_stat_diag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_stat_diag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_stat_diag`]
module"]
#[doc(alias = "ALE_ALE_STAT_DIAG")]
pub type AleAleStatDiag = crate::Reg<ale_ale_stat_diag::AleAleStatDiagSpec>;
#[doc = "The ALE Statistic Output Diagnostic Register allows the output statistics to diagnose the SW counters. This register is for diagnostice only."]
pub mod ale_ale_stat_diag;
#[doc = "ALE_ALE_OAM_LB_CTRL (rw) register accessor: The ALE OAM Control allows ports to be put into OAM Loopback, only non-supervisor packet are looped back to the source port.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_oam_lb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_oam_lb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_oam_lb_ctrl`]
module"]
#[doc(alias = "ALE_ALE_OAM_LB_CTRL")]
pub type AleAleOamLbCtrl = crate::Reg<ale_ale_oam_lb_ctrl::AleAleOamLbCtrlSpec>;
#[doc = "The ALE OAM Control allows ports to be put into OAM Loopback, only non-supervisor packet are looped back to the source port."]
pub mod ale_ale_oam_lb_ctrl;
#[doc = "ALE_ALE_MSK_MUX0 (rw) register accessor: VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_msk_mux0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_msk_mux0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_ale_msk_mux0`]
module"]
#[doc(alias = "ALE_ALE_MSK_MUX0")]
pub type AleAleMskMux0 = crate::Reg<ale_ale_msk_mux0::AleAleMskMux0Spec>;
#[doc = "VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
pub mod ale_ale_msk_mux0;
#[doc = "ALE_I1_ALE_MSK_MUX1_0 (rw) register accessor: VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i1_ale_msk_mux1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i1_ale_msk_mux1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_i1_ale_msk_mux1_0`]
module"]
#[doc(alias = "ALE_I1_ALE_MSK_MUX1_0")]
pub type AleI1AleMskMux1_0 = crate::Reg<ale_i1_ale_msk_mux1_0::AleI1AleMskMux1_0Spec>;
#[doc = "VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
pub mod ale_i1_ale_msk_mux1_0;
#[doc = "ALE_I1_ALE_MSK_MUX1_1 (rw) register accessor: VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i1_ale_msk_mux1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i1_ale_msk_mux1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_i1_ale_msk_mux1_1`]
module"]
#[doc(alias = "ALE_I1_ALE_MSK_MUX1_1")]
pub type AleI1AleMskMux1_1 = crate::Reg<ale_i1_ale_msk_mux1_1::AleI1AleMskMux1_1Spec>;
#[doc = "VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
pub mod ale_i1_ale_msk_mux1_1;
#[doc = "ALE_I1_ALE_MSK_MUX1_2 (rw) register accessor: VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i1_ale_msk_mux1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i1_ale_msk_mux1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_i1_ale_msk_mux1_2`]
module"]
#[doc(alias = "ALE_I1_ALE_MSK_MUX1_2")]
pub type AleI1AleMskMux1_2 = crate::Reg<ale_i1_ale_msk_mux1_2::AleI1AleMskMux1_2Spec>;
#[doc = "VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively."]
pub mod ale_i1_ale_msk_mux1_2;
#[doc = "ALE_EGRESSOP (rw) register accessor: The Egress Operation register allows enabled classifiers with any match like IPSA or IPDA match to use the CPSW Egress Packet Operations Inter VLAN Routing sub functions. If the packet was destined for the host or is destined to any port without any errors, but matches a clasifier that has a programmed egress opcode, it will be forwarded to the destination ports where the destination ports will use the thier egress opcode entry to modify the packet. InterVLAN Routing and mirroring need to be understood, they are orthogonal functions. Care must be taken not to violate VLAN rules as this can redirect packets based on classifier matches.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_egressop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_egressop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_egressop`]
module"]
#[doc(alias = "ALE_EGRESSOP")]
pub type AleEgressop = crate::Reg<ale_egressop::AleEgressopSpec>;
#[doc = "The Egress Operation register allows enabled classifiers with any match like IPSA or IPDA match to use the CPSW Egress Packet Operations Inter VLAN Routing sub functions. If the packet was destined for the host or is destined to any port without any errors, but matches a clasifier that has a programmed egress opcode, it will be forwarded to the destination ports where the destination ports will use the thier egress opcode entry to modify the packet. InterVLAN Routing and mirroring need to be understood, they are orthogonal functions. Care must be taken not to violate VLAN rules as this can redirect packets based on classifier matches."]
pub mod ale_egressop;
#[doc = "ALE_POLICECFG0 (rw) register accessor: The Policing Config 0 holds the port, frame priority and ONU address index as well as match enables for port, frame priority and ONU address matching.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg0`]
module"]
#[doc(alias = "ALE_POLICECFG0")]
pub type AlePolicecfg0 = crate::Reg<ale_policecfg0::AlePolicecfg0Spec>;
#[doc = "The Policing Config 0 holds the port, frame priority and ONU address index as well as match enables for port, frame priority and ONU address matching."]
pub mod ale_policecfg0;
#[doc = "ALE_POLICECFG1 (rw) register accessor: The Policing Config 1 holds the match enable/match index for the L2 Destination and L2 source addresses\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg1`]
module"]
#[doc(alias = "ALE_POLICECFG1")]
pub type AlePolicecfg1 = crate::Reg<ale_policecfg1::AlePolicecfg1Spec>;
#[doc = "The Policing Config 1 holds the match enable/match index for the L2 Destination and L2 source addresses"]
pub mod ale_policecfg1;
#[doc = "ALE_POLICECFG2 (rw) register accessor: The Policing Config 2 holds the match enable/match index for the Outer VLAN and Inner VLAN addresses\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg2`]
module"]
#[doc(alias = "ALE_POLICECFG2")]
pub type AlePolicecfg2 = crate::Reg<ale_policecfg2::AlePolicecfg2Spec>;
#[doc = "The Policing Config 2 holds the match enable/match index for the Outer VLAN and Inner VLAN addresses"]
pub mod ale_policecfg2;
#[doc = "ALE_POLICECFG3 (rw) register accessor: The Policing Config 3 holds the match enable/match index for the Ether Type and IP Source address\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg3`]
module"]
#[doc(alias = "ALE_POLICECFG3")]
pub type AlePolicecfg3 = crate::Reg<ale_policecfg3::AlePolicecfg3Spec>;
#[doc = "The Policing Config 3 holds the match enable/match index for the Ether Type and IP Source address"]
pub mod ale_policecfg3;
#[doc = "ALE_POLICECFG4 (rw) register accessor: The Policing Config 4 holds the match enable/match index for the IP Destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg4`]
module"]
#[doc(alias = "ALE_POLICECFG4")]
pub type AlePolicecfg4 = crate::Reg<ale_policecfg4::AlePolicecfg4Spec>;
#[doc = "The Policing Config 4 holds the match enable/match index for the IP Destination address"]
pub mod ale_policecfg4;
#[doc = "ALE_POLICECFG6 (rw) register accessor: The PIR counter is a 37 bit internal counter where ~ipir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED at LUT time. If the counter is negative the packet will be marked RED, else it can be YELLOW or GREEN based on the CIR counter. If only this counter is used (aka cir_idle_inc_val==0) Packet are marked RED or GREEN based on PIR counter only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg6`]
module"]
#[doc(alias = "ALE_POLICECFG6")]
pub type AlePolicecfg6 = crate::Reg<ale_policecfg6::AlePolicecfg6Spec>;
#[doc = "The PIR counter is a 37 bit internal counter where ~ipir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED at LUT time. If the counter is negative the packet will be marked RED, else it can be YELLOW or GREEN based on the CIR counter. If only this counter is used (aka cir_idle_inc_val==0) Packet are marked RED or GREEN based on PIR counter only."]
pub mod ale_policecfg6;
#[doc = "ALE_POLICECFG7 (rw) register accessor: The CIR counter is a 37 bit internal counter where ~icir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED or YELLOW at LUT time. If the counter is positive the packet will be marked GREEN, else it can be YELLOW or RED based on the PIR counter. If only this counter is used (aka pir_idle_inc_val==0) Packet are marked YELLOW or GREEN based on CIR counter only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecfg7`]
module"]
#[doc(alias = "ALE_POLICECFG7")]
pub type AlePolicecfg7 = crate::Reg<ale_policecfg7::AlePolicecfg7Spec>;
#[doc = "The CIR counter is a 37 bit internal counter where ~icir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED or YELLOW at LUT time. If the counter is positive the packet will be marked GREEN, else it can be YELLOW or RED based on the PIR counter. If only this counter is used (aka pir_idle_inc_val==0) Packet are marked YELLOW or GREEN based on CIR counter only."]
pub mod ale_policecfg7;
#[doc = "ALE_POLICETBLCTL (rw) register accessor: The Policing Table Control is used to read or write the selected policing/classifier entry. The selected policing/classifier entry is only read or written after this register is written based on the value of the ~iwrite_enable bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policetblctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policetblctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policetblctl`]
module"]
#[doc(alias = "ALE_POLICETBLCTL")]
pub type AlePolicetblctl = crate::Reg<ale_policetblctl::AlePolicetblctlSpec>;
#[doc = "The Policing Table Control is used to read or write the selected policing/classifier entry. The selected policing/classifier entry is only read or written after this register is written based on the value of the ~iwrite_enable bit."]
pub mod ale_policetblctl;
#[doc = "ALE_POLICECONTROL (rw) register accessor: The Control Enables color marking as well as internal ALE packet dropping rules.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policecontrol`]
module"]
#[doc(alias = "ALE_POLICECONTROL")]
pub type AlePolicecontrol = crate::Reg<ale_policecontrol::AlePolicecontrolSpec>;
#[doc = "The Control Enables color marking as well as internal ALE packet dropping rules."]
pub mod ale_policecontrol;
#[doc = "ALE_POLICETESTCTL (rw) register accessor: The Policing Test Control enables the ability to determine which policing entry has been hit and whether they reported a red or yellow rate condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policetestctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policetestctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policetestctl`]
module"]
#[doc(alias = "ALE_POLICETESTCTL")]
pub type AlePolicetestctl = crate::Reg<ale_policetestctl::AlePolicetestctlSpec>;
#[doc = "The Policing Test Control enables the ability to determine which policing entry has been hit and whether they reported a red or yellow rate condition."]
pub mod ale_policetestctl;
#[doc = "ALE_POLICEHSTAT (rw) register accessor: The policing hit status is a read only register that reads the hit bits of the selected policing/classifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policehstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policehstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_policehstat`]
module"]
#[doc(alias = "ALE_POLICEHSTAT")]
pub type AlePolicehstat = crate::Reg<ale_policehstat::AlePolicehstatSpec>;
#[doc = "The policing hit status is a read only register that reads the hit bits of the selected policing/classifier."]
pub mod ale_policehstat;
#[doc = "ALE_THREADMAPDEF (rw) register accessor: The THREAD Mapping Default Value register is used to set the default thread ID when no classifier is matched,\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapdef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapdef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_threadmapdef`]
module"]
#[doc(alias = "ALE_THREADMAPDEF")]
pub type AleThreadmapdef = crate::Reg<ale_threadmapdef::AleThreadmapdefSpec>;
#[doc = "The THREAD Mapping Default Value register is used to set the default thread ID when no classifier is matched,"]
pub mod ale_threadmapdef;
#[doc = "ALE_THREADMAPCTL (rw) register accessor: The THREAD Mapping Control register allows the highest matched classifier to return a particular thread ID for traffic sent to the host. This allows particular classifier matched traffic to be placed an a particular hosts queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_threadmapctl`]
module"]
#[doc(alias = "ALE_THREADMAPCTL")]
pub type AleThreadmapctl = crate::Reg<ale_threadmapctl::AleThreadmapctlSpec>;
#[doc = "The THREAD Mapping Control register allows the highest matched classifier to return a particular thread ID for traffic sent to the host. This allows particular classifier matched traffic to be placed an a particular hosts queue."]
pub mod ale_threadmapctl;
#[doc = "ALE_THREADMAPVAL (rw) register accessor: The THREAD Mapping Value register is used to set the thread ID for a particular classifier entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale_threadmapval`]
module"]
#[doc(alias = "ALE_THREADMAPVAL")]
pub type AleThreadmapval = crate::Reg<ale_threadmapval::AleThreadmapvalSpec>;
#[doc = "The THREAD Mapping Value register is used to set the thread ID for a particular classifier entry."]
pub mod ale_threadmapval;
#[doc = "rev (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
#[doc(alias = "rev")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "Revision parameters"]
pub mod rev;
#[doc = "vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vector`]
module"]
#[doc(alias = "vector")]
pub type Vector = crate::Reg<vector::VectorSpec>;
#[doc = "ECC Vector Register"]
pub mod vector;
#[doc = "stat (rw) register accessor: Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "stat")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Misc Status"]
pub mod stat;
#[doc = "ECC_reserved_svbus_0 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_0`]
module"]
#[doc(alias = "ECC_reserved_svbus_0")]
pub type EccReservedSvbus0 = crate::Reg<ecc_reserved_svbus_0::EccReservedSvbus0Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_0;
#[doc = "ECC_reserved_svbus_1 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_1`]
module"]
#[doc(alias = "ECC_reserved_svbus_1")]
pub type EccReservedSvbus1 = crate::Reg<ecc_reserved_svbus_1::EccReservedSvbus1Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_1;
#[doc = "ECC_reserved_svbus_2 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_2`]
module"]
#[doc(alias = "ECC_reserved_svbus_2")]
pub type EccReservedSvbus2 = crate::Reg<ecc_reserved_svbus_2::EccReservedSvbus2Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_2;
#[doc = "ECC_reserved_svbus_3 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_3`]
module"]
#[doc(alias = "ECC_reserved_svbus_3")]
pub type EccReservedSvbus3 = crate::Reg<ecc_reserved_svbus_3::EccReservedSvbus3Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_3;
#[doc = "ECC_reserved_svbus_4 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_4`]
module"]
#[doc(alias = "ECC_reserved_svbus_4")]
pub type EccReservedSvbus4 = crate::Reg<ecc_reserved_svbus_4::EccReservedSvbus4Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_4;
#[doc = "ECC_reserved_svbus_5 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_5`]
module"]
#[doc(alias = "ECC_reserved_svbus_5")]
pub type EccReservedSvbus5 = crate::Reg<ecc_reserved_svbus_5::EccReservedSvbus5Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_5;
#[doc = "ECC_reserved_svbus_6 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_6`]
module"]
#[doc(alias = "ECC_reserved_svbus_6")]
pub type EccReservedSvbus6 = crate::Reg<ecc_reserved_svbus_6::EccReservedSvbus6Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_6;
#[doc = "ECC_reserved_svbus_7 (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_reserved_svbus_7`]
module"]
#[doc(alias = "ECC_reserved_svbus_7")]
pub type EccReservedSvbus7 = crate::Reg<ecc_reserved_svbus_7::EccReservedSvbus7Spec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_reserved_svbus_7;
#[doc = "ECC_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sec_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_sec_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_sec_eoi_reg")]
pub type EccSecEoiReg = crate::Reg<ecc_sec_eoi_reg::EccSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_sec_eoi_reg;
#[doc = "ECC_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sec_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_sec_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sec_status_reg0`]
module"]
#[doc(alias = "ECC_sec_status_reg0")]
pub type EccSecStatusReg0 = crate::Reg<ecc_sec_status_reg0::EccSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_sec_status_reg0;
#[doc = "ECC_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sec_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_sec_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_sec_enable_set_reg0")]
pub type EccSecEnableSetReg0 = crate::Reg<ecc_sec_enable_set_reg0::EccSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_sec_enable_set_reg0;
#[doc = "ECC_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sec_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_sec_enable_clr_reg0")]
pub type EccSecEnableClrReg0 = crate::Reg<ecc_sec_enable_clr_reg0::EccSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_sec_enable_clr_reg0;
#[doc = "ECC_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ded_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ded_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_ded_eoi_reg")]
pub type EccDedEoiReg = crate::Reg<ecc_ded_eoi_reg::EccDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_ded_eoi_reg;
#[doc = "ECC_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ded_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ded_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ded_status_reg0`]
module"]
#[doc(alias = "ECC_ded_status_reg0")]
pub type EccDedStatusReg0 = crate::Reg<ecc_ded_status_reg0::EccDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_ded_status_reg0;
#[doc = "ECC_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ded_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ded_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_ded_enable_set_reg0")]
pub type EccDedEnableSetReg0 = crate::Reg<ecc_ded_enable_set_reg0::EccDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_ded_enable_set_reg0;
#[doc = "ECC_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ded_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_ded_enable_clr_reg0")]
pub type EccDedEnableClrReg0 = crate::Reg<ecc_ded_enable_clr_reg0::EccDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_ded_enable_clr_reg0;
#[doc = "aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_set`]
module"]
#[doc(alias = "aggr_enable_set")]
pub type AggrEnableSet = crate::Reg<aggr_enable_set::AggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod aggr_enable_set;
#[doc = "aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_clr`]
module"]
#[doc(alias = "aggr_enable_clr")]
pub type AggrEnableClr = crate::Reg<aggr_enable_clr::AggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod aggr_enable_clr;
#[doc = "aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_set`]
module"]
#[doc(alias = "aggr_status_set")]
pub type AggrStatusSet = crate::Reg<aggr_status_set::AggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod aggr_status_set;
#[doc = "aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_clr`]
module"]
#[doc(alias = "aggr_status_clr")]
pub type AggrStatusClr = crate::Reg<aggr_status_clr::AggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod aggr_status_clr;
