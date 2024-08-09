#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_CONTROL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacControlRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_CONTROL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacControlRegSpec>;
#[doc = "Field `FULL_DUPLEX_MODE` reader - 0:0\\]
Full Duplex mode"]
pub type FullDuplexModeR = crate::BitReader;
#[doc = "Field `FULL_DUPLEX_MODE` writer - 0:0\\]
Full Duplex mode"]
pub type FullDuplexModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP_BACK_MODE` reader - 1:1\\]
Loop Back Mode"]
pub type LoopBackModeR = crate::BitReader;
#[doc = "Field `LOOP_BACK_MODE` writer - 1:1\\]
Loop Back Mode"]
pub type LoopBackModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANUFACTURING_TEST_MODE` reader - 2:2\\]
Manufacturing Test Mode"]
pub type ManufacturingTestModeR = crate::BitReader;
#[doc = "Field `MANUFACTURING_TEST_MODE` writer - 2:2\\]
Manufacturing Test Mode"]
pub type ManufacturingTestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_FLOW_CONTROL` reader - 3:3\\]
Receive Flow Control Enable"]
pub type ReceiveFlowControlR = crate::BitReader;
#[doc = "Field `RECEIVE_FLOW_CONTROL` writer - 3:3\\]
Receive Flow Control Enable"]
pub type ReceiveFlowControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FLOW_CONTROL` reader - 4:4\\]
Transmit Flow Control Enable"]
pub type TransmitFlowControlR = crate::BitReader;
#[doc = "Field `TRANSMIT_FLOW_CONTROL` writer - 4:4\\]
Transmit Flow Control Enable"]
pub type TransmitFlowControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMII_ENABLE` reader - 5:5\\]
GMII Enable"]
pub type GmiiEnableR = crate::BitReader;
#[doc = "Field `GMII_ENABLE` writer - 5:5\\]
GMII Enable"]
pub type GmiiEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_PACING_ENABLE` reader - 6:6\\]
Transmit Pacing Enable"]
pub type TransmitPacingEnableR = crate::BitReader;
#[doc = "Field `TRANSMIT_PACING_ENABLE` writer - 6:6\\]
Transmit Pacing Enable"]
pub type TransmitPacingEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIGABIT_MODE` reader - 7:7\\]
Gigabit Mode"]
pub type GigabitModeR = crate::BitReader;
#[doc = "Field `GIGABIT_MODE` writer - 7:7\\]
Gigabit Mode"]
pub type GigabitModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_SHORT_GAP` reader - 10:10\\]
Transmit Short Gap Enable"]
pub type TransmitShortGapR = crate::BitReader;
#[doc = "Field `TRANSMIT_SHORT_GAP` writer - 10:10\\]
Transmit Short Gap Enable"]
pub type TransmitShortGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_IDLE` reader - 11:11\\]
Command Idle"]
pub type CommandIdleR = crate::BitReader;
#[doc = "Field `COMMAND_IDLE` writer - 11:11\\]
Command Idle"]
pub type CommandIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_CRC_TYPE` reader - 12:12\\]
Port CRC Type"]
pub type PortCrcTypeR = crate::BitReader;
#[doc = "Field `PORT_CRC_TYPE` writer - 12:12\\]
Port CRC Type"]
pub type PortCrcTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERFACE_CONTROL_A` reader - 15:15\\]
Interface Control A"]
pub type InterfaceControlAR = crate::BitReader;
#[doc = "Field `INTERFACE_CONTROL_A` writer - 15:15\\]
Interface Control A"]
pub type InterfaceControlAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERFACE_CONTROL_B` reader - 16:16\\]
Interface Control B"]
pub type InterfaceControlBR = crate::BitReader;
#[doc = "Field `INTERFACE_CONTROL_B` writer - 16:16\\]
Interface Control B"]
pub type InterfaceControlBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIGABIT_MODE_FORCE` reader - 17:17\\]
Gigabit Mode Force"]
pub type GigabitModeForceR = crate::BitReader;
#[doc = "Field `GIGABIT_MODE_FORCE` writer - 17:17\\]
Gigabit Mode Force"]
pub type GigabitModeForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_ENABLE` reader - 18:18\\]
External Enable"]
pub type ExternalEnableR = crate::BitReader;
#[doc = "Field `EXTERNAL_ENABLE` writer - 18:18\\]
External Enable"]
pub type ExternalEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_RECEIVE_FLOW` reader - 19:19\\]
External Receive Flow Control Enable"]
pub type ExternalReceiveFlowR = crate::BitReader;
#[doc = "Field `EXTERNAL_RECEIVE_FLOW` writer - 19:19\\]
External Receive Flow Control Enable"]
pub type ExternalReceiveFlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_TRANSMIT_FLOW` reader - 20:20\\]
External Transmit Flow Control Enable"]
pub type ExternalTransmitFlowR = crate::BitReader;
#[doc = "Field `EXTERNAL_TRANSMIT_FLOW` writer - 20:20\\]
External Transmit Flow Control Enable"]
pub type ExternalTransmitFlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_SHORT_GAP` reader - 21:21\\]
Transmit Short Gap Limit Enable"]
pub type TransmitShortGapR = crate::BitReader;
#[doc = "Field `TRANSMIT_SHORT_GAP` writer - 21:21\\]
Transmit Short Gap Limit Enable"]
pub type TransmitShortGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_COPY_ERROR` reader - 22:22\\]
RX Copy Error Frames Enable"]
pub type RxCopyErrorR = crate::BitReader;
#[doc = "Field `RX_COPY_ERROR` writer - 22:22\\]
RX Copy Error Frames Enable"]
pub type RxCopyErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_COPY_SHORT` reader - 23:23\\]
RX Copy Short Frames Enable"]
pub type RxCopyShortR = crate::BitReader;
#[doc = "Field `RX_COPY_SHORT` writer - 23:23\\]
RX Copy Short Frames Enable"]
pub type RxCopyShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_COPY_MAC` reader - 24:24\\]
RX Copy MAC Control Frames Enable"]
pub type RxCopyMacR = crate::BitReader;
#[doc = "Field `RX_COPY_MAC` writer - 24:24\\]
RX Copy MAC Control Frames Enable"]
pub type RxCopyMacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Full Duplex mode"]
    #[inline(always)]
    pub fn full_duplex_mode(&self) -> FullDuplexModeR {
        FullDuplexModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop Back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&self) -> LoopBackModeR {
        LoopBackModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Manufacturing Test Mode"]
    #[inline(always)]
    pub fn manufacturing_test_mode(&self) -> ManufacturingTestModeR {
        ManufacturingTestModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive Flow Control Enable"]
    #[inline(always)]
    pub fn receive_flow_control(&self) -> ReceiveFlowControlR {
        ReceiveFlowControlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn transmit_flow_control(&self) -> TransmitFlowControlR {
        TransmitFlowControlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GMII Enable"]
    #[inline(always)]
    pub fn gmii_enable(&self) -> GmiiEnableR {
        GmiiEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit Pacing Enable"]
    #[inline(always)]
    pub fn transmit_pacing_enable(&self) -> TransmitPacingEnableR {
        TransmitPacingEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Gigabit Mode"]
    #[inline(always)]
    pub fn gigabit_mode(&self) -> GigabitModeR {
        GigabitModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit Short Gap Enable"]
    #[inline(always)]
    pub fn transmit_short_gap(&self) -> TransmitShortGapR {
        TransmitShortGapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Command Idle"]
    #[inline(always)]
    pub fn command_idle(&self) -> CommandIdleR {
        CommandIdleR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port CRC Type"]
    #[inline(always)]
    pub fn port_crc_type(&self) -> PortCrcTypeR {
        PortCrcTypeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interface Control A"]
    #[inline(always)]
    pub fn interface_control_a(&self) -> InterfaceControlAR {
        InterfaceControlAR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interface Control B"]
    #[inline(always)]
    pub fn interface_control_b(&self) -> InterfaceControlBR {
        InterfaceControlBR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Gigabit Mode Force"]
    #[inline(always)]
    pub fn gigabit_mode_force(&self) -> GigabitModeForceR {
        GigabitModeForceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
External Enable"]
    #[inline(always)]
    pub fn external_enable(&self) -> ExternalEnableR {
        ExternalEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
External Receive Flow Control Enable"]
    #[inline(always)]
    pub fn external_receive_flow(&self) -> ExternalReceiveFlowR {
        ExternalReceiveFlowR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
External Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn external_transmit_flow(&self) -> ExternalTransmitFlowR {
        ExternalTransmitFlowR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Transmit Short Gap Limit Enable"]
    #[inline(always)]
    pub fn transmit_short_gap(&self) -> TransmitShortGapR {
        TransmitShortGapR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
RX Copy Error Frames Enable"]
    #[inline(always)]
    pub fn rx_copy_error(&self) -> RxCopyErrorR {
        RxCopyErrorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
RX Copy Short Frames Enable"]
    #[inline(always)]
    pub fn rx_copy_short(&self) -> RxCopyShortR {
        RxCopyShortR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
RX Copy MAC Control Frames Enable"]
    #[inline(always)]
    pub fn rx_copy_mac(&self) -> RxCopyMacR {
        RxCopyMacR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Full Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn full_duplex_mode(&mut self) -> FullDuplexModeW<CpswNcEthMac0PnMacControlRegSpec> {
        FullDuplexModeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_back_mode(&mut self) -> LoopBackModeW<CpswNcEthMac0PnMacControlRegSpec> {
        LoopBackModeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Manufacturing Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn manufacturing_test_mode(
        &mut self,
    ) -> ManufacturingTestModeW<CpswNcEthMac0PnMacControlRegSpec> {
        ManufacturingTestModeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn receive_flow_control(
        &mut self,
    ) -> ReceiveFlowControlW<CpswNcEthMac0PnMacControlRegSpec> {
        ReceiveFlowControlW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_flow_control(
        &mut self,
    ) -> TransmitFlowControlW<CpswNcEthMac0PnMacControlRegSpec> {
        TransmitFlowControlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
GMII Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gmii_enable(&mut self) -> GmiiEnableW<CpswNcEthMac0PnMacControlRegSpec> {
        GmiiEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit Pacing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_pacing_enable(
        &mut self,
    ) -> TransmitPacingEnableW<CpswNcEthMac0PnMacControlRegSpec> {
        TransmitPacingEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Gigabit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gigabit_mode(&mut self) -> GigabitModeW<CpswNcEthMac0PnMacControlRegSpec> {
        GigabitModeW::new(self, 7)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmit Short Gap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_short_gap(&mut self) -> TransmitShortGapW<CpswNcEthMac0PnMacControlRegSpec> {
        TransmitShortGapW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Command Idle"]
    #[inline(always)]
    #[must_use]
    pub fn command_idle(&mut self) -> CommandIdleW<CpswNcEthMac0PnMacControlRegSpec> {
        CommandIdleW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Port CRC Type"]
    #[inline(always)]
    #[must_use]
    pub fn port_crc_type(&mut self) -> PortCrcTypeW<CpswNcEthMac0PnMacControlRegSpec> {
        PortCrcTypeW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
Interface Control A"]
    #[inline(always)]
    #[must_use]
    pub fn interface_control_a(&mut self) -> InterfaceControlAW<CpswNcEthMac0PnMacControlRegSpec> {
        InterfaceControlAW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interface Control B"]
    #[inline(always)]
    #[must_use]
    pub fn interface_control_b(&mut self) -> InterfaceControlBW<CpswNcEthMac0PnMacControlRegSpec> {
        InterfaceControlBW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Gigabit Mode Force"]
    #[inline(always)]
    #[must_use]
    pub fn gigabit_mode_force(&mut self) -> GigabitModeForceW<CpswNcEthMac0PnMacControlRegSpec> {
        GigabitModeForceW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
External Enable"]
    #[inline(always)]
    #[must_use]
    pub fn external_enable(&mut self) -> ExternalEnableW<CpswNcEthMac0PnMacControlRegSpec> {
        ExternalEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
External Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn external_receive_flow(
        &mut self,
    ) -> ExternalReceiveFlowW<CpswNcEthMac0PnMacControlRegSpec> {
        ExternalReceiveFlowW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
External Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn external_transmit_flow(
        &mut self,
    ) -> ExternalTransmitFlowW<CpswNcEthMac0PnMacControlRegSpec> {
        ExternalTransmitFlowW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Transmit Short Gap Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_short_gap(&mut self) -> TransmitShortGapW<CpswNcEthMac0PnMacControlRegSpec> {
        TransmitShortGapW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
RX Copy Error Frames Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_copy_error(&mut self) -> RxCopyErrorW<CpswNcEthMac0PnMacControlRegSpec> {
        RxCopyErrorW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
RX Copy Short Frames Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_copy_short(&mut self) -> RxCopyShortW<CpswNcEthMac0PnMacControlRegSpec> {
        RxCopyShortW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
RX Copy MAC Control Frames Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_copy_mac(&mut self) -> RxCopyMacW<CpswNcEthMac0PnMacControlRegSpec> {
        RxCopyMacW::new(self, 24)
    }
}
#[doc = "Enet Port N Mac Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacControlRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
