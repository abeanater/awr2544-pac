#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_FLOW_THRESH_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnRxFlowThreshRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_FLOW_THRESH_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnRxFlowThreshRegSpec>;
#[doc = "Field `RECEIVE_FLOW_THRESHOLD` reader - 8:0\\]
Receive Flow Threshold in Words"]
pub type ReceiveFlowThresholdR = crate::FieldReader<u16>;
#[doc = "Field `RECEIVE_FLOW_THRESHOLD` writer - 8:0\\]
Receive Flow Threshold in Words"]
pub type ReceiveFlowThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Receive Flow Threshold in Words"]
    #[inline(always)]
    pub fn receive_flow_threshold(&self) -> ReceiveFlowThresholdR {
        ReceiveFlowThresholdR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Receive Flow Threshold in Words"]
    #[inline(always)]
    #[must_use]
    pub fn receive_flow_threshold(
        &mut self,
    ) -> ReceiveFlowThresholdW<CpswNcEthMac0PnRxFlowThreshRegSpec> {
        ReceiveFlowThresholdW::new(self, 0)
    }
}
#[doc = "Enet MAC Receive Flow Threshold in Receive Buffer Words\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnRxFlowThreshRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnRxFlowThreshRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnRxFlowThreshRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_rx_flow_thresh_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnRxFlowThreshRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_RX_FLOW_THRESH_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnRxFlowThreshRegSpec {
    const RESET_VALUE: u32 = 0;
}
