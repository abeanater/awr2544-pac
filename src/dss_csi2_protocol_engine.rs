#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csi2_revision: Csi2Revision,
    _reserved1: [u8; 0x0c],
    csi2_sysconfig: Csi2Sysconfig,
    csi2_sysstatus: Csi2Sysstatus,
    csi2_irqstatus: Csi2Irqstatus,
    csi2_irqenable: Csi2Irqenable,
    _reserved5: [u8; 0x20],
    csi2_ctrl: Csi2Ctrl,
    csi2_gnq: Csi2Gnq,
    csi2_complexio_cfg1: Csi2ComplexioCfg1,
    csi2_complexio_irqstatus: Csi2ComplexioIrqstatus,
    csi2_complexio_irqenable: Csi2ComplexioIrqenable,
    csi2_clk_ctrl: Csi2ClkCtrl,
    csi2_timing1: Csi2Timing1,
    csi2_timing2: Csi2Timing2,
    csi2_vm_timing1: Csi2VmTiming1,
    csi2_vm_timing2: Csi2VmTiming2,
    csi2_vm_timing3: Csi2VmTiming3,
    csi2_clk_timing: Csi2ClkTiming,
    csi2_tx_fifo_vc_size: Csi2TxFifoVcSize,
    csi2_rx_fifo_vc_size: Csi2RxFifoVcSize,
    csi2_complexio_cfg2: Csi2ComplexioCfg2,
    csi2_rx_fifo_vc_fullness: Csi2RxFifoVcFullness,
    csi2_vm_timing4: Csi2VmTiming4,
    csi2_tx_fifo_vc_emptiness: Csi2TxFifoVcEmptiness,
    csi2_vm_timing5: Csi2VmTiming5,
    csi2_vm_timing6: Csi2VmTiming6,
    csi2_vm_timing7: Csi2VmTiming7,
    csi2_stopclk_timing: Csi2StopclkTiming,
    csi2_ctrl2: Csi2Ctrl2,
    csi2_vm_timing8: Csi2VmTiming8,
    csi2_te_hsync_width_0: Csi2TeHsyncWidth0,
    csi2_te_vsync_width_0: Csi2TeVsyncWidth0,
    csi2_te_hsync_number_0: Csi2TeHsyncNumber0,
    csi2_te_hsync_width_1: Csi2TeHsyncWidth1,
    csi2_te_vsync_width_1: Csi2TeVsyncWidth1,
    csi2_te_hsync_number_1: Csi2TeHsyncNumber1,
    _reserved35: [u8; 0x48],
    csi2_vc_ctrl_0: Csi2VcCtrl0,
    csi2_vc_te_0: Csi2VcTe0,
    csi2_vc_long_packet_header_0: Csi2VcLongPacketHeader0,
    csi2_vc_long_packet_payload_0: Csi2VcLongPacketPayload0,
    csi2_vc_short_packet_header_0: Csi2VcShortPacketHeader0,
    _reserved40: [u8; 0x04],
    csi2_vc_irqstatus_0: Csi2VcIrqstatus0,
    csi2_vc_irqenable_0: Csi2VcIrqenable0,
    csi2_vc_ctrl_1: Csi2VcCtrl1,
    csi2_vc_te_1: Csi2VcTe1,
    csi2_vc_long_packet_header_1: Csi2VcLongPacketHeader1,
    csi2_vc_long_packet_payload_1: Csi2VcLongPacketPayload1,
    csi2_vc_short_packet_header_1: Csi2VcShortPacketHeader1,
    _reserved47: [u8; 0x04],
    csi2_vc_irqstatus_1: Csi2VcIrqstatus1,
    csi2_vc_irqenable_1: Csi2VcIrqenable1,
    csi2_vc_ctrl_2: Csi2VcCtrl2,
    csi2_vc_te_2: Csi2VcTe2,
    csi2_vc_long_packet_header_2: Csi2VcLongPacketHeader2,
    csi2_vc_long_packet_payload_2: Csi2VcLongPacketPayload2,
    csi2_vc_short_packet_header_2: Csi2VcShortPacketHeader2,
    _reserved54: [u8; 0x04],
    csi2_vc_irqstatus_2: Csi2VcIrqstatus2,
    csi2_vc_irqenable_2: Csi2VcIrqenable2,
    csi2_vc_ctrl_3: Csi2VcCtrl3,
    csi2_vc_te_3: Csi2VcTe3,
    csi2_vc_long_packet_header_3: Csi2VcLongPacketHeader3,
    csi2_vc_long_packet_payload_3: Csi2VcLongPacketPayload3,
    csi2_vc_short_packet_header_3: Csi2VcShortPacketHeader3,
    _reserved61: [u8; 0x04],
    csi2_vc_irqstatus_3: Csi2VcIrqstatus3,
    csi2_vc_irqenable_3: Csi2VcIrqenable3,
}
impl RegisterBlock {
    #[doc = "0x00 - MODULE REVISION This register contains the IP revision code in binary coded digital. For example, we have: 0x01 = revision 0.1 and 0x21 = revision 2.1"]
    #[inline(always)]
    pub const fn csi2_revision(&self) -> &Csi2Revision {
        &self.csi2_revision
    }
    #[doc = "0x10 - SYSTEM CONFIGURATION REGISTER This register is the OCP-socket system configuration register."]
    #[inline(always)]
    pub const fn csi2_sysconfig(&self) -> &Csi2Sysconfig {
        &self.csi2_sysconfig
    }
    #[doc = "0x14 - SYSTEM STATUS REGISTER This register provides status information about the module, excluding the interrupt status register."]
    #[inline(always)]
    pub const fn csi2_sysstatus(&self) -> &Csi2Sysstatus {
        &self.csi2_sysstatus
    }
    #[doc = "0x18 - INTERRUPT STATUS REGISTER - All virtual channels + Complex IO + PLL This register associates one bit for each virtual channel in order to determine which virtual channel has generated the interrupt. The virtual channel shall be enabled for events to be generated on that virtual channel. If the virtual channel is disabled, the interrupt is not generated."]
    #[inline(always)]
    pub const fn csi2_irqstatus(&self) -> &Csi2Irqstatus {
        &self.csi2_irqstatus
    }
    #[doc = "0x1c - INTERRUPT ENABLE REGISTER - This register associates one bit for each virtual channel in order to enable/disable each virtual channel individually."]
    #[inline(always)]
    pub const fn csi2_irqenable(&self) -> &Csi2Irqenable {
        &self.csi2_irqenable
    }
    #[doc = "0x40 - GLOBAL CONTROL REGISTER This register controls the CSI2 Protocol Engine module. This register shall not be modified dynamically (except IF_EN bit fields)."]
    #[inline(always)]
    pub const fn csi2_ctrl(&self) -> &Csi2Ctrl {
        &self.csi2_ctrl
    }
    #[doc = "0x44 - GENERIC PARAMETER REGISTER This register provide a way to read the generic parameters used in the design."]
    #[inline(always)]
    pub const fn csi2_gnq(&self) -> &Csi2Gnq {
        &self.csi2_gnq
    }
    #[doc = "0x48 - COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the order and position of the lanes (clock and data) and the polarity order for the control of the PHY differential signals in addition to the control bit for the power FSM."]
    #[inline(always)]
    pub const fn csi2_complexio_cfg1(&self) -> &Csi2ComplexioCfg1 {
        &self.csi2_complexio_cfg1
    }
    #[doc = "0x4c - INTERRUPT STATUS REGISTER - All errors from complex IO"]
    #[inline(always)]
    pub const fn csi2_complexio_irqstatus(&self) -> &Csi2ComplexioIrqstatus {
        &self.csi2_complexio_irqstatus
    }
    #[doc = "0x50 - INTERRUPT ENABLE REGISTER - All errors from complex IO"]
    #[inline(always)]
    pub const fn csi2_complexio_irqenable(&self) -> &Csi2ComplexioIrqenable {
        &self.csi2_complexio_irqenable
    }
    #[doc = "0x54 - CLOCK CONTROL This register controls the CLOCK GENERATION. The register can be modified only when IF_EN is reset."]
    #[inline(always)]
    pub const fn csi2_clk_ctrl(&self) -> &Csi2ClkCtrl {
        &self.csi2_clk_ctrl
    }
    #[doc = "0x58 - TIMING1 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers FORCE_TX_STOP_TIMER and TA_TO_TIMER"]
    #[inline(always)]
    pub const fn csi2_timing1(&self) -> &Csi2Timing1 {
        &self.csi2_timing1
    }
    #[doc = "0x5c - TIMING2 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers HS_TX_TIMER and LP_RX_TIMER"]
    #[inline(always)]
    pub const fn csi2_timing2(&self) -> &Csi2Timing2 {
        &self.csi2_timing2
    }
    #[doc = "0x60 - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing1(&self) -> &Csi2VmTiming1 {
        &self.csi2_vm_timing1
    }
    #[doc = "0x64 - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing2(&self) -> &Csi2VmTiming2 {
        &self.csi2_vm_timing2
    }
    #[doc = "0x68 - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing3(&self) -> &Csi2VmTiming3 {
        &self.csi2_vm_timing3
    }
    #[doc = "0x6c - CLOCK TIMING REGISTER This register controls the CSI2 Protocol Engine module timers. This register shall not be modified while CSI2_CTRL.IF_EN is set to '1'."]
    #[inline(always)]
    pub const fn csi2_clk_timing(&self) -> &Csi2ClkTiming {
        &self.csi2_clk_timing
    }
    #[doc = "0x70 - Defines the corresponding memory entries allocated for each virtual channel. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the TX FIFO."]
    #[inline(always)]
    pub const fn csi2_tx_fifo_vc_size(&self) -> &Csi2TxFifoVcSize {
        &self.csi2_tx_fifo_vc_size
    }
    #[doc = "0x74 - Defines the corresponding memory entries allocated for each virtual channel and the addresses. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the RX FIFO."]
    #[inline(always)]
    pub const fn csi2_rx_fifo_vc_size(&self) -> &Csi2RxFifoVcSize {
        &self.csi2_rx_fifo_vc_size
    }
    #[doc = "0x78 - COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the ULPS for each lane."]
    #[inline(always)]
    pub const fn csi2_complexio_cfg2(&self) -> &Csi2ComplexioCfg2 {
        &self.csi2_complexio_cfg2
    }
    #[doc = "0x7c - Defines the fullness of each space allocated for each virtual channel."]
    #[inline(always)]
    pub const fn csi2_rx_fifo_vc_fullness(&self) -> &Csi2RxFifoVcFullness {
        &self.csi2_rx_fifo_vc_fullness
    }
    #[doc = "0x80 - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing4(&self) -> &Csi2VmTiming4 {
        &self.csi2_vm_timing4
    }
    #[doc = "0x84 - Defines the emptiness of each space allocated for each virtual channel."]
    #[inline(always)]
    pub const fn csi2_tx_fifo_vc_emptiness(&self) -> &Csi2TxFifoVcEmptiness {
        &self.csi2_tx_fifo_vc_emptiness
    }
    #[doc = "0x88 - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing5(&self) -> &Csi2VmTiming5 {
        &self.csi2_vm_timing5
    }
    #[doc = "0x8c - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing6(&self) -> &Csi2VmTiming6 {
        &self.csi2_vm_timing6
    }
    #[doc = "0x90 - Defines the minimum number of HS bytes clock cycles that are required to allow for the delays in entering and exiting HS mode. The supported values are from 0 to 65535"]
    #[inline(always)]
    pub const fn csi2_vm_timing7(&self) -> &Csi2VmTiming7 {
        &self.csi2_vm_timing7
    }
    #[doc = "0x94 - Number of functional clock cycles to wait for TxByteClkHS to stop/start after change in CSI2StopClk signal"]
    #[inline(always)]
    pub const fn csi2_stopclk_timing(&self) -> &Csi2StopclkTiming {
        &self.csi2_stopclk_timing
    }
    #[doc = "0x98 - Additional control bits for use with Video Port 2"]
    #[inline(always)]
    pub const fn csi2_ctrl2(&self) -> &Csi2Ctrl2 {
        &self.csi2_ctrl2
    }
    #[doc = "0x9c - VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
    #[inline(always)]
    pub const fn csi2_vm_timing8(&self) -> &Csi2VmTiming8 {
        &self.csi2_vm_timing8
    }
    #[doc = "0xa0 - The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_hsync_width_0(&self) -> &Csi2TeHsyncWidth0 {
        &self.csi2_te_hsync_width_0
    }
    #[doc = "0xa4 - The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_vsync_width_0(&self) -> &Csi2TeVsyncWidth0 {
        &self.csi2_te_vsync_width_0
    }
    #[doc = "0xa8 - The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_hsync_number_0(&self) -> &Csi2TeHsyncNumber0 {
        &self.csi2_te_hsync_number_0
    }
    #[doc = "0xac - The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_hsync_width_1(&self) -> &Csi2TeHsyncWidth1 {
        &self.csi2_te_hsync_width_1
    }
    #[doc = "0xb0 - The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_vsync_width_1(&self) -> &Csi2TeVsyncWidth1 {
        &self.csi2_te_vsync_width_1
    }
    #[doc = "0xb4 - The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
    #[inline(always)]
    pub const fn csi2_te_hsync_number_1(&self) -> &Csi2TeHsyncNumber1 {
        &self.csi2_te_hsync_number_1
    }
    #[doc = "0x100 - CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_ctrl_0(&self) -> &Csi2VcCtrl0 {
        &self.csi2_vc_ctrl_0
    }
    #[doc = "0x104 - CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
    #[inline(always)]
    pub const fn csi2_vc_te_0(&self) -> &Csi2VcTe0 {
        &self.csi2_vc_te_0
    }
    #[doc = "0x108 - LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_header_0(&self) -> &Csi2VcLongPacketHeader0 {
        &self.csi2_vc_long_packet_header_0
    }
    #[doc = "0x10c - LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_payload_0(&self) -> &Csi2VcLongPacketPayload0 {
        &self.csi2_vc_long_packet_payload_0
    }
    #[doc = "0x110 - SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_short_packet_header_0(&self) -> &Csi2VcShortPacketHeader0 {
        &self.csi2_vc_short_packet_header_0
    }
    #[doc = "0x118 - INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqstatus_0(&self) -> &Csi2VcIrqstatus0 {
        &self.csi2_vc_irqstatus_0
    }
    #[doc = "0x11c - INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqenable_0(&self) -> &Csi2VcIrqenable0 {
        &self.csi2_vc_irqenable_0
    }
    #[doc = "0x120 - CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_ctrl_1(&self) -> &Csi2VcCtrl1 {
        &self.csi2_vc_ctrl_1
    }
    #[doc = "0x124 - CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
    #[inline(always)]
    pub const fn csi2_vc_te_1(&self) -> &Csi2VcTe1 {
        &self.csi2_vc_te_1
    }
    #[doc = "0x128 - LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_header_1(&self) -> &Csi2VcLongPacketHeader1 {
        &self.csi2_vc_long_packet_header_1
    }
    #[doc = "0x12c - LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_payload_1(&self) -> &Csi2VcLongPacketPayload1 {
        &self.csi2_vc_long_packet_payload_1
    }
    #[doc = "0x130 - SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_short_packet_header_1(&self) -> &Csi2VcShortPacketHeader1 {
        &self.csi2_vc_short_packet_header_1
    }
    #[doc = "0x138 - INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqstatus_1(&self) -> &Csi2VcIrqstatus1 {
        &self.csi2_vc_irqstatus_1
    }
    #[doc = "0x13c - INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqenable_1(&self) -> &Csi2VcIrqenable1 {
        &self.csi2_vc_irqenable_1
    }
    #[doc = "0x140 - CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_ctrl_2(&self) -> &Csi2VcCtrl2 {
        &self.csi2_vc_ctrl_2
    }
    #[doc = "0x144 - CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
    #[inline(always)]
    pub const fn csi2_vc_te_2(&self) -> &Csi2VcTe2 {
        &self.csi2_vc_te_2
    }
    #[doc = "0x148 - LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_header_2(&self) -> &Csi2VcLongPacketHeader2 {
        &self.csi2_vc_long_packet_header_2
    }
    #[doc = "0x14c - LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_payload_2(&self) -> &Csi2VcLongPacketPayload2 {
        &self.csi2_vc_long_packet_payload_2
    }
    #[doc = "0x150 - SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_short_packet_header_2(&self) -> &Csi2VcShortPacketHeader2 {
        &self.csi2_vc_short_packet_header_2
    }
    #[doc = "0x158 - INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqstatus_2(&self) -> &Csi2VcIrqstatus2 {
        &self.csi2_vc_irqstatus_2
    }
    #[doc = "0x15c - INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqenable_2(&self) -> &Csi2VcIrqenable2 {
        &self.csi2_vc_irqenable_2
    }
    #[doc = "0x160 - CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_ctrl_3(&self) -> &Csi2VcCtrl3 {
        &self.csi2_vc_ctrl_3
    }
    #[doc = "0x164 - CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
    #[inline(always)]
    pub const fn csi2_vc_te_3(&self) -> &Csi2VcTe3 {
        &self.csi2_vc_te_3
    }
    #[doc = "0x168 - LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_header_3(&self) -> &Csi2VcLongPacketHeader3 {
        &self.csi2_vc_long_packet_header_3
    }
    #[doc = "0x16c - LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_long_packet_payload_3(&self) -> &Csi2VcLongPacketPayload3 {
        &self.csi2_vc_long_packet_payload_3
    }
    #[doc = "0x170 - SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
    #[inline(always)]
    pub const fn csi2_vc_short_packet_header_3(&self) -> &Csi2VcShortPacketHeader3 {
        &self.csi2_vc_short_packet_header_3
    }
    #[doc = "0x178 - INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqstatus_3(&self) -> &Csi2VcIrqstatus3 {
        &self.csi2_vc_irqstatus_3
    }
    #[doc = "0x17c - INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
    #[inline(always)]
    pub const fn csi2_vc_irqenable_3(&self) -> &Csi2VcIrqenable3 {
        &self.csi2_vc_irqenable_3
    }
}
#[doc = "CSI2_REVISION (rw) register accessor: MODULE REVISION This register contains the IP revision code in binary coded digital. For example, we have: 0x01 = revision 0.1 and 0x21 = revision 2.1\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_revision`]
module"]
#[doc(alias = "CSI2_REVISION")]
pub type Csi2Revision = crate::Reg<csi2_revision::Csi2RevisionSpec>;
#[doc = "MODULE REVISION This register contains the IP revision code in binary coded digital. For example, we have: 0x01 = revision 0.1 and 0x21 = revision 2.1"]
pub mod csi2_revision;
#[doc = "CSI2_SYSCONFIG (rw) register accessor: SYSTEM CONFIGURATION REGISTER This register is the OCP-socket system configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_sysconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_sysconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_sysconfig`]
module"]
#[doc(alias = "CSI2_SYSCONFIG")]
pub type Csi2Sysconfig = crate::Reg<csi2_sysconfig::Csi2SysconfigSpec>;
#[doc = "SYSTEM CONFIGURATION REGISTER This register is the OCP-socket system configuration register."]
pub mod csi2_sysconfig;
#[doc = "CSI2_SYSSTATUS (rw) register accessor: SYSTEM STATUS REGISTER This register provides status information about the module, excluding the interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_sysstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_sysstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_sysstatus`]
module"]
#[doc(alias = "CSI2_SYSSTATUS")]
pub type Csi2Sysstatus = crate::Reg<csi2_sysstatus::Csi2SysstatusSpec>;
#[doc = "SYSTEM STATUS REGISTER This register provides status information about the module, excluding the interrupt status register."]
pub mod csi2_sysstatus;
#[doc = "CSI2_IRQSTATUS (rw) register accessor: INTERRUPT STATUS REGISTER - All virtual channels + Complex IO + PLL This register associates one bit for each virtual channel in order to determine which virtual channel has generated the interrupt. The virtual channel shall be enabled for events to be generated on that virtual channel. If the virtual channel is disabled, the interrupt is not generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_irqstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_irqstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_irqstatus`]
module"]
#[doc(alias = "CSI2_IRQSTATUS")]
pub type Csi2Irqstatus = crate::Reg<csi2_irqstatus::Csi2IrqstatusSpec>;
#[doc = "INTERRUPT STATUS REGISTER - All virtual channels + Complex IO + PLL This register associates one bit for each virtual channel in order to determine which virtual channel has generated the interrupt. The virtual channel shall be enabled for events to be generated on that virtual channel. If the virtual channel is disabled, the interrupt is not generated."]
pub mod csi2_irqstatus;
#[doc = "CSI2_IRQENABLE (rw) register accessor: INTERRUPT ENABLE REGISTER - This register associates one bit for each virtual channel in order to enable/disable each virtual channel individually.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_irqenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_irqenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_irqenable`]
module"]
#[doc(alias = "CSI2_IRQENABLE")]
pub type Csi2Irqenable = crate::Reg<csi2_irqenable::Csi2IrqenableSpec>;
#[doc = "INTERRUPT ENABLE REGISTER - This register associates one bit for each virtual channel in order to enable/disable each virtual channel individually."]
pub mod csi2_irqenable;
#[doc = "CSI2_CTRL (rw) register accessor: GLOBAL CONTROL REGISTER This register controls the CSI2 Protocol Engine module. This register shall not be modified dynamically (except IF_EN bit fields).\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_ctrl`]
module"]
#[doc(alias = "CSI2_CTRL")]
pub type Csi2Ctrl = crate::Reg<csi2_ctrl::Csi2CtrlSpec>;
#[doc = "GLOBAL CONTROL REGISTER This register controls the CSI2 Protocol Engine module. This register shall not be modified dynamically (except IF_EN bit fields)."]
pub mod csi2_ctrl;
#[doc = "CSI2_GNQ (rw) register accessor: GENERIC PARAMETER REGISTER This register provide a way to read the generic parameters used in the design.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_gnq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_gnq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_gnq`]
module"]
#[doc(alias = "CSI2_GNQ")]
pub type Csi2Gnq = crate::Reg<csi2_gnq::Csi2GnqSpec>;
#[doc = "GENERIC PARAMETER REGISTER This register provide a way to read the generic parameters used in the design."]
pub mod csi2_gnq;
#[doc = "CSI2_COMPLEXIO_CFG1 (rw) register accessor: COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the order and position of the lanes (clock and data) and the polarity order for the control of the PHY differential signals in addition to the control bit for the power FSM.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_complexio_cfg1`]
module"]
#[doc(alias = "CSI2_COMPLEXIO_CFG1")]
pub type Csi2ComplexioCfg1 = crate::Reg<csi2_complexio_cfg1::Csi2ComplexioCfg1Spec>;
#[doc = "COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the order and position of the lanes (clock and data) and the polarity order for the control of the PHY differential signals in addition to the control bit for the power FSM."]
pub mod csi2_complexio_cfg1;
#[doc = "CSI2_COMPLEXIO_IRQSTATUS (rw) register accessor: INTERRUPT STATUS REGISTER - All errors from complex IO\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_irqstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_irqstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_complexio_irqstatus`]
module"]
#[doc(alias = "CSI2_COMPLEXIO_IRQSTATUS")]
pub type Csi2ComplexioIrqstatus = crate::Reg<csi2_complexio_irqstatus::Csi2ComplexioIrqstatusSpec>;
#[doc = "INTERRUPT STATUS REGISTER - All errors from complex IO"]
pub mod csi2_complexio_irqstatus;
#[doc = "CSI2_COMPLEXIO_IRQENABLE (rw) register accessor: INTERRUPT ENABLE REGISTER - All errors from complex IO\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_irqenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_irqenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_complexio_irqenable`]
module"]
#[doc(alias = "CSI2_COMPLEXIO_IRQENABLE")]
pub type Csi2ComplexioIrqenable = crate::Reg<csi2_complexio_irqenable::Csi2ComplexioIrqenableSpec>;
#[doc = "INTERRUPT ENABLE REGISTER - All errors from complex IO"]
pub mod csi2_complexio_irqenable;
#[doc = "CSI2_CLK_CTRL (rw) register accessor: CLOCK CONTROL This register controls the CLOCK GENERATION. The register can be modified only when IF_EN is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_clk_ctrl`]
module"]
#[doc(alias = "CSI2_CLK_CTRL")]
pub type Csi2ClkCtrl = crate::Reg<csi2_clk_ctrl::Csi2ClkCtrlSpec>;
#[doc = "CLOCK CONTROL This register controls the CLOCK GENERATION. The register can be modified only when IF_EN is reset."]
pub mod csi2_clk_ctrl;
#[doc = "CSI2_TIMING1 (rw) register accessor: TIMING1 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers FORCE_TX_STOP_TIMER and TA_TO_TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_timing1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_timing1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_timing1`]
module"]
#[doc(alias = "CSI2_TIMING1")]
pub type Csi2Timing1 = crate::Reg<csi2_timing1::Csi2Timing1Spec>;
#[doc = "TIMING1 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers FORCE_TX_STOP_TIMER and TA_TO_TIMER"]
pub mod csi2_timing1;
#[doc = "CSI2_TIMING2 (rw) register accessor: TIMING2 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers HS_TX_TIMER and LP_RX_TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_timing2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_timing2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_timing2`]
module"]
#[doc(alias = "CSI2_TIMING2")]
pub type Csi2Timing2 = crate::Reg<csi2_timing2::Csi2Timing2Spec>;
#[doc = "TIMING2 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers HS_TX_TIMER and LP_RX_TIMER"]
pub mod csi2_timing2;
#[doc = "CSI2_VM_TIMING1 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing1`]
module"]
#[doc(alias = "CSI2_VM_TIMING1")]
pub type Csi2VmTiming1 = crate::Reg<csi2_vm_timing1::Csi2VmTiming1Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing1;
#[doc = "CSI2_VM_TIMING2 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing2`]
module"]
#[doc(alias = "CSI2_VM_TIMING2")]
pub type Csi2VmTiming2 = crate::Reg<csi2_vm_timing2::Csi2VmTiming2Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing2;
#[doc = "CSI2_VM_TIMING3 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing3`]
module"]
#[doc(alias = "CSI2_VM_TIMING3")]
pub type Csi2VmTiming3 = crate::Reg<csi2_vm_timing3::Csi2VmTiming3Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing3;
#[doc = "CSI2_CLK_TIMING (rw) register accessor: CLOCK TIMING REGISTER This register controls the CSI2 Protocol Engine module timers. This register shall not be modified while CSI2_CTRL.IF_EN is set to '1'.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_clk_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_clk_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_clk_timing`]
module"]
#[doc(alias = "CSI2_CLK_TIMING")]
pub type Csi2ClkTiming = crate::Reg<csi2_clk_timing::Csi2ClkTimingSpec>;
#[doc = "CLOCK TIMING REGISTER This register controls the CSI2 Protocol Engine module timers. This register shall not be modified while CSI2_CTRL.IF_EN is set to '1'."]
pub mod csi2_clk_timing;
#[doc = "CSI2_TX_FIFO_VC_SIZE (rw) register accessor: Defines the corresponding memory entries allocated for each virtual channel. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the TX FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_fifo_vc_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_fifo_vc_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_tx_fifo_vc_size`]
module"]
#[doc(alias = "CSI2_TX_FIFO_VC_SIZE")]
pub type Csi2TxFifoVcSize = crate::Reg<csi2_tx_fifo_vc_size::Csi2TxFifoVcSizeSpec>;
#[doc = "Defines the corresponding memory entries allocated for each virtual channel. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the TX FIFO."]
pub mod csi2_tx_fifo_vc_size;
#[doc = "CSI2_RX_FIFO_VC_SIZE (rw) register accessor: Defines the corresponding memory entries allocated for each virtual channel and the addresses. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the RX FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_rx_fifo_vc_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_rx_fifo_vc_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_rx_fifo_vc_size`]
module"]
#[doc(alias = "CSI2_RX_FIFO_VC_SIZE")]
pub type Csi2RxFifoVcSize = crate::Reg<csi2_rx_fifo_vc_size::Csi2RxFifoVcSizeSpec>;
#[doc = "Defines the corresponding memory entries allocated for each virtual channel and the addresses. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the RX FIFO."]
pub mod csi2_rx_fifo_vc_size;
#[doc = "CSI2_COMPLEXIO_CFG2 (rw) register accessor: COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the ULPS for each lane.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_complexio_cfg2`]
module"]
#[doc(alias = "CSI2_COMPLEXIO_CFG2")]
pub type Csi2ComplexioCfg2 = crate::Reg<csi2_complexio_cfg2::Csi2ComplexioCfg2Spec>;
#[doc = "COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the ULPS for each lane."]
pub mod csi2_complexio_cfg2;
#[doc = "CSI2_RX_FIFO_VC_FULLNESS (rw) register accessor: Defines the fullness of each space allocated for each virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_rx_fifo_vc_fullness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_rx_fifo_vc_fullness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_rx_fifo_vc_fullness`]
module"]
#[doc(alias = "CSI2_RX_FIFO_VC_FULLNESS")]
pub type Csi2RxFifoVcFullness = crate::Reg<csi2_rx_fifo_vc_fullness::Csi2RxFifoVcFullnessSpec>;
#[doc = "Defines the fullness of each space allocated for each virtual channel."]
pub mod csi2_rx_fifo_vc_fullness;
#[doc = "CSI2_VM_TIMING4 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing4`]
module"]
#[doc(alias = "CSI2_VM_TIMING4")]
pub type Csi2VmTiming4 = crate::Reg<csi2_vm_timing4::Csi2VmTiming4Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing4;
#[doc = "CSI2_TX_FIFO_VC_EMPTINESS (rw) register accessor: Defines the emptiness of each space allocated for each virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_fifo_vc_emptiness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_fifo_vc_emptiness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_tx_fifo_vc_emptiness`]
module"]
#[doc(alias = "CSI2_TX_FIFO_VC_EMPTINESS")]
pub type Csi2TxFifoVcEmptiness = crate::Reg<csi2_tx_fifo_vc_emptiness::Csi2TxFifoVcEmptinessSpec>;
#[doc = "Defines the emptiness of each space allocated for each virtual channel."]
pub mod csi2_tx_fifo_vc_emptiness;
#[doc = "CSI2_VM_TIMING5 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing5`]
module"]
#[doc(alias = "CSI2_VM_TIMING5")]
pub type Csi2VmTiming5 = crate::Reg<csi2_vm_timing5::Csi2VmTiming5Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing5;
#[doc = "CSI2_VM_TIMING6 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing6`]
module"]
#[doc(alias = "CSI2_VM_TIMING6")]
pub type Csi2VmTiming6 = crate::Reg<csi2_vm_timing6::Csi2VmTiming6Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing6;
#[doc = "CSI2_VM_TIMING7 (rw) register accessor: Defines the minimum number of HS bytes clock cycles that are required to allow for the delays in entering and exiting HS mode. The supported values are from 0 to 65535\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing7`]
module"]
#[doc(alias = "CSI2_VM_TIMING7")]
pub type Csi2VmTiming7 = crate::Reg<csi2_vm_timing7::Csi2VmTiming7Spec>;
#[doc = "Defines the minimum number of HS bytes clock cycles that are required to allow for the delays in entering and exiting HS mode. The supported values are from 0 to 65535"]
pub mod csi2_vm_timing7;
#[doc = "CSI2_STOPCLK_TIMING (rw) register accessor: Number of functional clock cycles to wait for TxByteClkHS to stop/start after change in CSI2StopClk signal\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_stopclk_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_stopclk_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_stopclk_timing`]
module"]
#[doc(alias = "CSI2_STOPCLK_TIMING")]
pub type Csi2StopclkTiming = crate::Reg<csi2_stopclk_timing::Csi2StopclkTimingSpec>;
#[doc = "Number of functional clock cycles to wait for TxByteClkHS to stop/start after change in CSI2StopClk signal"]
pub mod csi2_stopclk_timing;
#[doc = "CSI2_CTRL2 (rw) register accessor: Additional control bits for use with Video Port 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_ctrl2`]
module"]
#[doc(alias = "CSI2_CTRL2")]
pub type Csi2Ctrl2 = crate::Reg<csi2_ctrl2::Csi2Ctrl2Spec>;
#[doc = "Additional control bits for use with Video Port 2"]
pub mod csi2_ctrl2;
#[doc = "CSI2_VM_TIMING8 (rw) register accessor: VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vm_timing8`]
module"]
#[doc(alias = "CSI2_VM_TIMING8")]
pub type Csi2VmTiming8 = crate::Reg<csi2_vm_timing8::Csi2VmTiming8Spec>;
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing."]
pub mod csi2_vm_timing8;
#[doc = "CSI2_TE_HSYNC_WIDTH_0 (rw) register accessor: The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_width_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_width_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_hsync_width_0`]
module"]
#[doc(alias = "CSI2_TE_HSYNC_WIDTH_0")]
pub type Csi2TeHsyncWidth0 = crate::Reg<csi2_te_hsync_width_0::Csi2TeHsyncWidth0Spec>;
#[doc = "The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_hsync_width_0;
#[doc = "CSI2_TE_VSYNC_WIDTH_0 (rw) register accessor: The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_vsync_width_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_vsync_width_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_vsync_width_0`]
module"]
#[doc(alias = "CSI2_TE_VSYNC_WIDTH_0")]
pub type Csi2TeVsyncWidth0 = crate::Reg<csi2_te_vsync_width_0::Csi2TeVsyncWidth0Spec>;
#[doc = "The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_vsync_width_0;
#[doc = "CSI2_TE_HSYNC_NUMBER_0 (rw) register accessor: The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_number_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_number_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_hsync_number_0`]
module"]
#[doc(alias = "CSI2_TE_HSYNC_NUMBER_0")]
pub type Csi2TeHsyncNumber0 = crate::Reg<csi2_te_hsync_number_0::Csi2TeHsyncNumber0Spec>;
#[doc = "The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_hsync_number_0;
#[doc = "CSI2_TE_HSYNC_WIDTH_1 (rw) register accessor: The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_width_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_width_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_hsync_width_1`]
module"]
#[doc(alias = "CSI2_TE_HSYNC_WIDTH_1")]
pub type Csi2TeHsyncWidth1 = crate::Reg<csi2_te_hsync_width_1::Csi2TeHsyncWidth1Spec>;
#[doc = "The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_hsync_width_1;
#[doc = "CSI2_TE_VSYNC_WIDTH_1 (rw) register accessor: The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_vsync_width_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_vsync_width_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_vsync_width_1`]
module"]
#[doc(alias = "CSI2_TE_VSYNC_WIDTH_1")]
pub type Csi2TeVsyncWidth1 = crate::Reg<csi2_te_vsync_width_1::Csi2TeVsyncWidth1Spec>;
#[doc = "The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_vsync_width_1;
#[doc = "CSI2_TE_HSYNC_NUMBER_1 (rw) register accessor: The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_number_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_number_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_te_hsync_number_1`]
module"]
#[doc(alias = "CSI2_TE_HSYNC_NUMBER_1")]
pub type Csi2TeHsyncNumber1 = crate::Reg<csi2_te_hsync_number_1::Csi2TeHsyncNumber1Spec>;
#[doc = "The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain."]
pub mod csi2_te_hsync_number_1;
#[doc = "CSI2_VC_CTRL_0 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_ctrl_0`]
module"]
#[doc(alias = "CSI2_VC_CTRL_0")]
pub type Csi2VcCtrl0 = crate::Reg<csi2_vc_ctrl_0::Csi2VcCtrl0Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
pub mod csi2_vc_ctrl_0;
#[doc = "CSI2_VC_TE_0 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_te_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_te_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_te_0`]
module"]
#[doc(alias = "CSI2_VC_TE_0")]
pub type Csi2VcTe0 = crate::Reg<csi2_vc_te_0::Csi2VcTe0Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
pub mod csi2_vc_te_0;
#[doc = "CSI2_VC_LONG_PACKET_HEADER_0 (rw) register accessor: LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_header_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_header_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_header_0`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_HEADER_0")]
pub type Csi2VcLongPacketHeader0 =
    crate::Reg<csi2_vc_long_packet_header_0::Csi2VcLongPacketHeader0Spec>;
#[doc = "LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_header_0;
#[doc = "CSI2_VC_LONG_PACKET_PAYLOAD_0 (rw) register accessor: LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_payload_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_payload_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_payload_0`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_PAYLOAD_0")]
pub type Csi2VcLongPacketPayload0 =
    crate::Reg<csi2_vc_long_packet_payload_0::Csi2VcLongPacketPayload0Spec>;
#[doc = "LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_payload_0;
#[doc = "CSI2_VC_SHORT_PACKET_HEADER_0 (rw) register accessor: SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_short_packet_header_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_short_packet_header_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_short_packet_header_0`]
module"]
#[doc(alias = "CSI2_VC_SHORT_PACKET_HEADER_0")]
pub type Csi2VcShortPacketHeader0 =
    crate::Reg<csi2_vc_short_packet_header_0::Csi2VcShortPacketHeader0Spec>;
#[doc = "SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_short_packet_header_0;
#[doc = "CSI2_VC_IRQSTATUS_0 (rw) register accessor: INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqstatus_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqstatus_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqstatus_0`]
module"]
#[doc(alias = "CSI2_VC_IRQSTATUS_0")]
pub type Csi2VcIrqstatus0 = crate::Reg<csi2_vc_irqstatus_0::Csi2VcIrqstatus0Spec>;
#[doc = "INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
pub mod csi2_vc_irqstatus_0;
#[doc = "CSI2_VC_IRQENABLE_0 (rw) register accessor: INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqenable_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqenable_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqenable_0`]
module"]
#[doc(alias = "CSI2_VC_IRQENABLE_0")]
pub type Csi2VcIrqenable0 = crate::Reg<csi2_vc_irqenable_0::Csi2VcIrqenable0Spec>;
#[doc = "INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
pub mod csi2_vc_irqenable_0;
#[doc = "CSI2_VC_CTRL_1 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_ctrl_1`]
module"]
#[doc(alias = "CSI2_VC_CTRL_1")]
pub type Csi2VcCtrl1 = crate::Reg<csi2_vc_ctrl_1::Csi2VcCtrl1Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
pub mod csi2_vc_ctrl_1;
#[doc = "CSI2_VC_TE_1 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_te_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_te_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_te_1`]
module"]
#[doc(alias = "CSI2_VC_TE_1")]
pub type Csi2VcTe1 = crate::Reg<csi2_vc_te_1::Csi2VcTe1Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
pub mod csi2_vc_te_1;
#[doc = "CSI2_VC_LONG_PACKET_HEADER_1 (rw) register accessor: LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_header_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_header_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_header_1`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_HEADER_1")]
pub type Csi2VcLongPacketHeader1 =
    crate::Reg<csi2_vc_long_packet_header_1::Csi2VcLongPacketHeader1Spec>;
#[doc = "LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_header_1;
#[doc = "CSI2_VC_LONG_PACKET_PAYLOAD_1 (rw) register accessor: LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_payload_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_payload_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_payload_1`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_PAYLOAD_1")]
pub type Csi2VcLongPacketPayload1 =
    crate::Reg<csi2_vc_long_packet_payload_1::Csi2VcLongPacketPayload1Spec>;
#[doc = "LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_payload_1;
#[doc = "CSI2_VC_SHORT_PACKET_HEADER_1 (rw) register accessor: SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_short_packet_header_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_short_packet_header_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_short_packet_header_1`]
module"]
#[doc(alias = "CSI2_VC_SHORT_PACKET_HEADER_1")]
pub type Csi2VcShortPacketHeader1 =
    crate::Reg<csi2_vc_short_packet_header_1::Csi2VcShortPacketHeader1Spec>;
#[doc = "SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_short_packet_header_1;
#[doc = "CSI2_VC_IRQSTATUS_1 (rw) register accessor: INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqstatus_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqstatus_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqstatus_1`]
module"]
#[doc(alias = "CSI2_VC_IRQSTATUS_1")]
pub type Csi2VcIrqstatus1 = crate::Reg<csi2_vc_irqstatus_1::Csi2VcIrqstatus1Spec>;
#[doc = "INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
pub mod csi2_vc_irqstatus_1;
#[doc = "CSI2_VC_IRQENABLE_1 (rw) register accessor: INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqenable_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqenable_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqenable_1`]
module"]
#[doc(alias = "CSI2_VC_IRQENABLE_1")]
pub type Csi2VcIrqenable1 = crate::Reg<csi2_vc_irqenable_1::Csi2VcIrqenable1Spec>;
#[doc = "INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
pub mod csi2_vc_irqenable_1;
#[doc = "CSI2_VC_CTRL_2 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_ctrl_2`]
module"]
#[doc(alias = "CSI2_VC_CTRL_2")]
pub type Csi2VcCtrl2 = crate::Reg<csi2_vc_ctrl_2::Csi2VcCtrl2Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
pub mod csi2_vc_ctrl_2;
#[doc = "CSI2_VC_TE_2 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_te_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_te_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_te_2`]
module"]
#[doc(alias = "CSI2_VC_TE_2")]
pub type Csi2VcTe2 = crate::Reg<csi2_vc_te_2::Csi2VcTe2Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
pub mod csi2_vc_te_2;
#[doc = "CSI2_VC_LONG_PACKET_HEADER_2 (rw) register accessor: LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_header_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_header_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_header_2`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_HEADER_2")]
pub type Csi2VcLongPacketHeader2 =
    crate::Reg<csi2_vc_long_packet_header_2::Csi2VcLongPacketHeader2Spec>;
#[doc = "LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_header_2;
#[doc = "CSI2_VC_LONG_PACKET_PAYLOAD_2 (rw) register accessor: LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_payload_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_payload_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_payload_2`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_PAYLOAD_2")]
pub type Csi2VcLongPacketPayload2 =
    crate::Reg<csi2_vc_long_packet_payload_2::Csi2VcLongPacketPayload2Spec>;
#[doc = "LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_payload_2;
#[doc = "CSI2_VC_SHORT_PACKET_HEADER_2 (rw) register accessor: SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_short_packet_header_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_short_packet_header_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_short_packet_header_2`]
module"]
#[doc(alias = "CSI2_VC_SHORT_PACKET_HEADER_2")]
pub type Csi2VcShortPacketHeader2 =
    crate::Reg<csi2_vc_short_packet_header_2::Csi2VcShortPacketHeader2Spec>;
#[doc = "SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_short_packet_header_2;
#[doc = "CSI2_VC_IRQSTATUS_2 (rw) register accessor: INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqstatus_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqstatus_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqstatus_2`]
module"]
#[doc(alias = "CSI2_VC_IRQSTATUS_2")]
pub type Csi2VcIrqstatus2 = crate::Reg<csi2_vc_irqstatus_2::Csi2VcIrqstatus2Spec>;
#[doc = "INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
pub mod csi2_vc_irqstatus_2;
#[doc = "CSI2_VC_IRQENABLE_2 (rw) register accessor: INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqenable_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqenable_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqenable_2`]
module"]
#[doc(alias = "CSI2_VC_IRQENABLE_2")]
pub type Csi2VcIrqenable2 = crate::Reg<csi2_vc_irqenable_2::Csi2VcIrqenable2Spec>;
#[doc = "INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
pub mod csi2_vc_irqenable_2;
#[doc = "CSI2_VC_CTRL_3 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_ctrl_3`]
module"]
#[doc(alias = "CSI2_VC_CTRL_3")]
pub type Csi2VcCtrl3 = crate::Reg<csi2_vc_ctrl_3::Csi2VcCtrl3Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the virtual channel."]
pub mod csi2_vc_ctrl_3;
#[doc = "CSI2_VC_TE_3 (rw) register accessor: CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_te_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_te_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_te_3`]
module"]
#[doc(alias = "CSI2_VC_TE_3")]
pub type Csi2VcTe3 = crate::Reg<csi2_vc_te_3::Csi2VcTe3Spec>;
#[doc = "CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode."]
pub mod csi2_vc_te_3;
#[doc = "CSI2_VC_LONG_PACKET_HEADER_3 (rw) register accessor: LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_header_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_header_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_header_3`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_HEADER_3")]
pub type Csi2VcLongPacketHeader3 =
    crate::Reg<csi2_vc_long_packet_header_3::Csi2VcLongPacketHeader3Spec>;
#[doc = "LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_header_3;
#[doc = "CSI2_VC_LONG_PACKET_PAYLOAD_3 (rw) register accessor: LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_payload_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_payload_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_long_packet_payload_3`]
module"]
#[doc(alias = "CSI2_VC_LONG_PACKET_PAYLOAD_3")]
pub type Csi2VcLongPacketPayload3 =
    crate::Reg<csi2_vc_long_packet_payload_3::Csi2VcLongPacketPayload3Spec>;
#[doc = "LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)"]
pub mod csi2_vc_long_packet_payload_3;
#[doc = "CSI2_VC_SHORT_PACKET_HEADER_3 (rw) register accessor: SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_short_packet_header_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_short_packet_header_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_short_packet_header_3`]
module"]
#[doc(alias = "CSI2_VC_SHORT_PACKET_HEADER_3")]
pub type Csi2VcShortPacketHeader3 =
    crate::Reg<csi2_vc_short_packet_header_3::Csi2VcShortPacketHeader3Spec>;
#[doc = "SHORT PACKET HEADER INFORMATION -Virtual channel This register sets the 24-bit DATA_ID + Short Packet Data Field + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
Short Packet Data field is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)"]
pub mod csi2_vc_short_packet_header_3;
#[doc = "CSI2_VC_IRQSTATUS_3 (rw) register accessor: INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqstatus_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqstatus_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqstatus_3`]
module"]
#[doc(alias = "CSI2_VC_IRQSTATUS_3")]
pub type Csi2VcIrqstatus3 = crate::Reg<csi2_vc_irqstatus_3::Csi2VcIrqstatus3Spec>;
#[doc = "INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel."]
pub mod csi2_vc_irqstatus_3;
#[doc = "CSI2_VC_IRQENABLE_3 (rw) register accessor: INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqenable_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqenable_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_vc_irqenable_3`]
module"]
#[doc(alias = "CSI2_VC_IRQENABLE_3")]
pub type Csi2VcIrqenable3 = crate::Reg<csi2_vc_irqenable_3::Csi2VcIrqenable3Spec>;
#[doc = "INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel."]
pub mod csi2_vc_irqenable_3;
