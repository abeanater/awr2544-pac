#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_STATUS_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacStatusRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_STATUS_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacStatusRegSpec>;
#[doc = "Field `TRANSMIT_FLOW_CONTROL` reader - 0:0\\]
Transmit Flow Control Active"]
pub type TransmitFlowControlR = crate::BitReader;
#[doc = "Field `TRANSMIT_FLOW_CONTROL` writer - 0:0\\]
Transmit Flow Control Active"]
pub type TransmitFlowControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_FLOW_CONTROL` reader - 1:1\\]
Receive Flow Control Active"]
pub type ReceiveFlowControlR = crate::BitReader;
#[doc = "Field `RECEIVE_FLOW_CONTROL` writer - 1:1\\]
Receive Flow Control Active"]
pub type ReceiveFlowControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_FULLDUPLEX` reader - 3:3\\]
External Fullduplex"]
pub type ExternalFullduplexR = crate::BitReader;
#[doc = "Field `EXTERNAL_FULLDUPLEX` writer - 3:3\\]
External Fullduplex"]
pub type ExternalFullduplexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_GIG_MODE` reader - 4:4\\]
External GIG mode"]
pub type ExternalGigModeR = crate::BitReader;
#[doc = "Field `EXTERNAL_GIG_MODE` writer - 4:4\\]
External GIG mode"]
pub type ExternalGigModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_RECEIVE_FLOW` reader - 5:5\\]
External Receive Flow Control Enable"]
pub type ExternalReceiveFlowR = crate::BitReader;
#[doc = "Field `EXTERNAL_RECEIVE_FLOW` writer - 5:5\\]
External Receive Flow Control Enable"]
pub type ExternalReceiveFlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL_TRANSMIT_FLOW` reader - 6:6\\]
External Transmit Flow Control Enable"]
pub type ExternalTransmitFlowR = crate::BitReader;
#[doc = "Field `EXTERNAL_TRANSMIT_FLOW` writer - 6:6\\]
External Transmit Flow Control Enable"]
pub type ExternalTransmitFlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_PRIORITY_BASED` reader - 15:8\\]
Receive Priority Based Flow Control Active (priority 7 down to 0)"]
pub type ReceivePriorityBasedR = crate::FieldReader;
#[doc = "Field `RECEIVE_PRIORITY_BASED` writer - 15:8\\]
Receive Priority Based Flow Control Active (priority 7 down to 0)"]
pub type ReceivePriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSMIT_PRIORITY_BASED` reader - 23:16\\]
Transmit Priority Based Flow Control Active (priority 7 down to 0)"]
pub type TransmitPriorityBasedR = crate::FieldReader;
#[doc = "Field `TRANSMIT_PRIORITY_BASED` writer - 23:16\\]
Transmit Priority Based Flow Control Active (priority 7 down to 0)"]
pub type TransmitPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_LOWEST_PRIORITY` reader - 26:24\\]
The lowest priority that caused top of receive FIFO flow control trigger since the last write to clear. This field is write 0x7 to clear."]
pub type TheLowestPriorityR = crate::FieldReader;
#[doc = "Field `THE_LOWEST_PRIORITY` writer - 26:24\\]
The lowest priority that caused top of receive FIFO flow control trigger since the last write to clear. This field is write 0x7 to clear."]
pub type TheLowestPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOP_OF_RECEIVE` reader - 27:27\\]
Top of receive FIFO flow control trigger occurred. This bit is write one to clear."]
pub type TopOfReceiveR = crate::BitReader;
#[doc = "Field `TOP_OF_RECEIVE` writer - 27:27\\]
Top of receive FIFO flow control trigger occurred. This bit is write one to clear."]
pub type TopOfReceiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREMPT_AND_EXPRESS` reader - 28:28\\]
Prempt and Express cpxmac_sl Transmit IDLE"]
pub type PremptAndExpressR = crate::BitReader;
#[doc = "Field `PREMPT_AND_EXPRESS` writer - 28:28\\]
Prempt and Express cpxmac_sl Transmit IDLE"]
pub type PremptAndExpressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXPRESS_CPXMAC_SL_IDLE` reader - 30:30\\]
Express cpxmac_sl IDLE"]
pub type ExpressCpxmacSlIdleR = crate::BitReader;
#[doc = "Field `EXPRESS_CPXMAC_SL_IDLE` writer - 30:30\\]
Express cpxmac_sl IDLE"]
pub type ExpressCpxmacSlIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPXMAC_SL_IDLE` reader - 31:31\\]
cpxmac_sl IDLE"]
pub type CpxmacSlIdleR = crate::BitReader;
#[doc = "Field `CPXMAC_SL_IDLE` writer - 31:31\\]
cpxmac_sl IDLE"]
pub type CpxmacSlIdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit Flow Control Active"]
    #[inline(always)]
    pub fn transmit_flow_control(&self) -> TransmitFlowControlR {
        TransmitFlowControlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive Flow Control Active"]
    #[inline(always)]
    pub fn receive_flow_control(&self) -> ReceiveFlowControlR {
        ReceiveFlowControlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
External Fullduplex"]
    #[inline(always)]
    pub fn external_fullduplex(&self) -> ExternalFullduplexR {
        ExternalFullduplexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
External GIG mode"]
    #[inline(always)]
    pub fn external_gig_mode(&self) -> ExternalGigModeR {
        ExternalGigModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
External Receive Flow Control Enable"]
    #[inline(always)]
    pub fn external_receive_flow(&self) -> ExternalReceiveFlowR {
        ExternalReceiveFlowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
External Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn external_transmit_flow(&self) -> ExternalTransmitFlowR {
        ExternalTransmitFlowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Receive Priority Based Flow Control Active (priority 7 down to 0)"]
    #[inline(always)]
    pub fn receive_priority_based(&self) -> ReceivePriorityBasedR {
        ReceivePriorityBasedR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Transmit Priority Based Flow Control Active (priority 7 down to 0)"]
    #[inline(always)]
    pub fn transmit_priority_based(&self) -> TransmitPriorityBasedR {
        TransmitPriorityBasedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The lowest priority that caused top of receive FIFO flow control trigger since the last write to clear. This field is write 0x7 to clear."]
    #[inline(always)]
    pub fn the_lowest_priority(&self) -> TheLowestPriorityR {
        TheLowestPriorityR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Top of receive FIFO flow control trigger occurred. This bit is write one to clear."]
    #[inline(always)]
    pub fn top_of_receive(&self) -> TopOfReceiveR {
        TopOfReceiveR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Prempt and Express cpxmac_sl Transmit IDLE"]
    #[inline(always)]
    pub fn prempt_and_express(&self) -> PremptAndExpressR {
        PremptAndExpressR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Express cpxmac_sl IDLE"]
    #[inline(always)]
    pub fn express_cpxmac_sl_idle(&self) -> ExpressCpxmacSlIdleR {
        ExpressCpxmacSlIdleR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
cpxmac_sl IDLE"]
    #[inline(always)]
    pub fn cpxmac_sl_idle(&self) -> CpxmacSlIdleR {
        CpxmacSlIdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit Flow Control Active"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_flow_control(
        &mut self,
    ) -> TransmitFlowControlW<CpswNcEthMac0PnMacStatusRegSpec> {
        TransmitFlowControlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive Flow Control Active"]
    #[inline(always)]
    #[must_use]
    pub fn receive_flow_control(&mut self) -> ReceiveFlowControlW<CpswNcEthMac0PnMacStatusRegSpec> {
        ReceiveFlowControlW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
External Fullduplex"]
    #[inline(always)]
    #[must_use]
    pub fn external_fullduplex(&mut self) -> ExternalFullduplexW<CpswNcEthMac0PnMacStatusRegSpec> {
        ExternalFullduplexW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
External GIG mode"]
    #[inline(always)]
    #[must_use]
    pub fn external_gig_mode(&mut self) -> ExternalGigModeW<CpswNcEthMac0PnMacStatusRegSpec> {
        ExternalGigModeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
External Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn external_receive_flow(
        &mut self,
    ) -> ExternalReceiveFlowW<CpswNcEthMac0PnMacStatusRegSpec> {
        ExternalReceiveFlowW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
External Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn external_transmit_flow(
        &mut self,
    ) -> ExternalTransmitFlowW<CpswNcEthMac0PnMacStatusRegSpec> {
        ExternalTransmitFlowW::new(self, 6)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Receive Priority Based Flow Control Active (priority 7 down to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn receive_priority_based(
        &mut self,
    ) -> ReceivePriorityBasedW<CpswNcEthMac0PnMacStatusRegSpec> {
        ReceivePriorityBasedW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Transmit Priority Based Flow Control Active (priority 7 down to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_priority_based(
        &mut self,
    ) -> TransmitPriorityBasedW<CpswNcEthMac0PnMacStatusRegSpec> {
        TransmitPriorityBasedW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The lowest priority that caused top of receive FIFO flow control trigger since the last write to clear. This field is write 0x7 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn the_lowest_priority(&mut self) -> TheLowestPriorityW<CpswNcEthMac0PnMacStatusRegSpec> {
        TheLowestPriorityW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Top of receive FIFO flow control trigger occurred. This bit is write one to clear."]
    #[inline(always)]
    #[must_use]
    pub fn top_of_receive(&mut self) -> TopOfReceiveW<CpswNcEthMac0PnMacStatusRegSpec> {
        TopOfReceiveW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Prempt and Express cpxmac_sl Transmit IDLE"]
    #[inline(always)]
    #[must_use]
    pub fn prempt_and_express(&mut self) -> PremptAndExpressW<CpswNcEthMac0PnMacStatusRegSpec> {
        PremptAndExpressW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Express cpxmac_sl IDLE"]
    #[inline(always)]
    #[must_use]
    pub fn express_cpxmac_sl_idle(
        &mut self,
    ) -> ExpressCpxmacSlIdleW<CpswNcEthMac0PnMacStatusRegSpec> {
        ExpressCpxmacSlIdleW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
cpxmac_sl IDLE"]
    #[inline(always)]
    #[must_use]
    pub fn cpxmac_sl_idle(&mut self) -> CpxmacSlIdleW<CpswNcEthMac0PnMacStatusRegSpec> {
        CpxmacSlIdleW::new(self, 31)
    }
}
#[doc = "Enet Port N Mac Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacStatusRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_STATUS_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
